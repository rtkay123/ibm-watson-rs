use std::borrow::Cow;
pub mod errors;

use hyper::{
    header::{HeaderValue, AUTHORIZATION},
    Body, Client, Method, Request, StatusCode,
};
use url::{form_urlencoded::byte_serialize, Url};

use self::errors::SynthesisError;

use super::TextToSpeech;

#[derive(Clone, Copy)]
pub enum AudioFormat {
    AudioAlaw {
        sample_rate: u16,
    },
    AudioBasic,
    AudioFlac {
        sample_rate: Option<u16>,
    },
    AudioL16 {
        sample_rate: u16,
        endianess: Option<AudioEndianess>,
    },
    AudioOgg {
        sample_rate: Option<u16>,
    },
    AudioOggCodecsOpus {
        sample_rate: Option<u16>,
    },
    AudioOggCodecsVorbis {
        sample_rate: Option<u16>,
    },
    AudioMp3 {
        sample_rate: Option<u16>,
    },
    AudioMpeg {
        sample_rate: Option<u16>,
    },
    AudioMulaw {
        sample_rate: u16,
    },
    AudioWav {
        sample_rate: Option<u16>,
    },
    AudioWebm,
    AudioWebmCodecsOpus,
    AudioWebmCodecsVorbis {
        sample_rate: Option<u16>,
    },
}

impl Default for AudioFormat {
    fn default() -> Self {
        AudioFormat::AudioOggCodecsOpus {
            sample_rate: Some(48000),
        }
    }
}

impl AudioFormat {
    pub fn id(&self) -> Cow<'static, str> {
        match &self {
            AudioFormat::AudioAlaw { sample_rate } => {
                let url = format!("audio/alaw;rate={sample_rate}");
                serialise_bytes(&url)
            }
            AudioFormat::AudioBasic => Cow::from("audio/basic"),
            AudioFormat::AudioFlac { sample_rate } => {
                let url = format!("audio/flac;rate={}", sample_rate.unwrap_or(22050));
                serialise_bytes(&url)
            }
            AudioFormat::AudioL16 {
                sample_rate,
                endianess: endianness,
            } => {
                let url = match endianness {
                    Some(endianness) => {
                        format!(
                            "audio/flac;rate={sample_rate};endianness={}",
                            endianness.id()
                        )
                    }
                    None => {
                        format!("audio/flac;rate={sample_rate}")
                    }
                };
                serialise_bytes(&url)
            }
            AudioFormat::AudioOgg { sample_rate } => {
                let url = format!("audio/ogg;rate={}", sample_rate.unwrap_or(22050));
                serialise_bytes(&url)
            }
            AudioFormat::AudioOggCodecsOpus { sample_rate } => {
                let url = format!(
                    "audio/ogg;codecs=opus;rate={}",
                    match sample_rate {
                        Some(rate) => *rate,
                        None => 48000,
                    }
                );
                serialise_bytes(&url)
            }
            AudioFormat::AudioOggCodecsVorbis { sample_rate } => {
                let url = format!(
                    "audio/ogg;codecs=vorbis;rate={}",
                    sample_rate.unwrap_or(22050)
                );
                serialise_bytes(&url)
            }
            AudioFormat::AudioMp3 { sample_rate } => {
                let url = format!("audio/mp3;rate={}", sample_rate.unwrap_or(22050));
                serialise_bytes(&url)
            }
            AudioFormat::AudioMpeg { sample_rate } => {
                let url = format!("audio/mpeg;rate={}", sample_rate.unwrap_or(22050));
                serialise_bytes(&url)
            }
            AudioFormat::AudioMulaw { sample_rate } => {
                let url = format!("audio/mulaw;rate={}", sample_rate);
                serialise_bytes(&url)
            }
            AudioFormat::AudioWav { sample_rate } => {
                let url = format!("audio/wav;rate={}", sample_rate.unwrap_or(22050));
                serialise_bytes(&url)
            }
            AudioFormat::AudioWebm => serialise_bytes("audio/webm"),
            AudioFormat::AudioWebmCodecsOpus => serialise_bytes("audio/webm;codecs=opus"),
            AudioFormat::AudioWebmCodecsVorbis { sample_rate } => {
                let url = format!(
                    "audio/webm;codecs=vorbis/rate={}",
                    sample_rate.unwrap_or(22050)
                );
                serialise_bytes(&url)
            }
        }
    }
}

fn serialise_bytes(url: &str) -> Cow<'static, str> {
    let url: String = byte_serialize(url.as_bytes()).collect();
    Cow::from(url)
}

#[derive(Default, Clone, Copy)]
pub enum AudioEndianess {
    BigEndian,
    #[default]
    LittleEndian,
}

impl AudioEndianess {
    pub fn id(&self) -> &str {
        match self {
            AudioEndianess::BigEndian => "big-endian",
            AudioEndianess::LittleEndian => "little-endian",
        }
    }
}

impl TextToSpeech<'_> {
    pub async fn synthesise(
        &self,
        text: impl AsRef<str>,
        accept: Option<AudioFormat>,
        customisation_id: Option<&str>,
    ) -> Result<bytes::Bytes, SynthesisError> {
        let mut url = Url::parse(self.service_url).unwrap();
        url.set_path("v1/synthesize");
        url.set_query(customisation_id);
        url.query_pairs_mut().append_pair("text", text.as_ref());
        url.query_pairs_mut().append_pair("voice", self.voice.id());
        if let Some(format) = accept {
            url.query_pairs_mut().append_pair("accept", &format.id());
        }
        let req = Request::builder()
            .uri(url.to_string())
            .header(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", self.access_token)).unwrap(),
            )
            .method(Method::GET)
            .body(Body::empty())
            .map_err(|e| SynthesisError::ConnectionError(e.to_string()))?;
        let client = self.get_client();
        let response = client
            .request(req)
            .await
            .map_err(|e| SynthesisError::ConnectionError(e.to_string()))?;
        assert_eq!(response.status(), 200);
        match response.status() {
            StatusCode::OK => {
                let bytes = hyper::body::to_bytes(response).await.unwrap();
                Ok(bytes)
            }
            StatusCode::NOT_ACCEPTABLE => Err(SynthesisError::NotAcceptable406),
            StatusCode::UNSUPPORTED_MEDIA_TYPE => Err(SynthesisError::UnsupportedMediaType415),
            StatusCode::INTERNAL_SERVER_ERROR => Err(SynthesisError::InternalServerError500),
            StatusCode::SERVICE_UNAVAILABLE => Err(SynthesisError::ServiceUnavailable500),
            StatusCode::BAD_REQUEST => Err(SynthesisError::BadRequest400),
            StatusCode::NOT_FOUND => Err(SynthesisError::NotFound404),
            _ => {
                unreachable!()
            }
        }
    }
}
