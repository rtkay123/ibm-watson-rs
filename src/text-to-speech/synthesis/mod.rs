use std::borrow::Cow;
/// Errors that may be returned in speech synthesis requests
pub mod errors;

use reqwest::{Method, Request, StatusCode, Url};
use url::form_urlencoded::byte_serialize;

use self::errors::SynthesisError;

use super::TextToSpeech;

/// The service can return audio in the following formats (MIME types):
#[derive(Clone, Copy)]
pub enum AudioFormat {
    /// You must specify the rate of the audio.
    AudioAlaw { sample_rate: u16 },
    /// The service returns audio with a sampling rate of 8000 Hz.
    AudioBasic,
    /// You can optionally specify the rate of the audio. The default sampling rate is 22,050 Hz.
    AudioFlac { sample_rate: Option<u16> },
    /// You must specify the rate of the audio. You can optionally specify the endianness of the audio. The default endianness is little-endian
    AudioL16 {
        sample_rate: u16,
        endianess: Option<AudioEndianness>,
    },
    /// You can optionally specify the rate of the audio. The default sampling rate is 22,050 Hz
    AudioOgg { sample_rate: Option<u16> },
    /// You can optionally specify the rate of the audio. The default sampling rate is 22,050 Hz
    AudioOggCodecsOpus { sample_rate: Option<u16> },
    /// You can optionally specify the rate of the audio. The default sampling rate is 22,050 Hz
    AudioOggCodecsVorbis { sample_rate: Option<u16> },
    /// You can optionally specify the rate of the audio. The default sampling rate is 22,050 Hz
    AudioMp3 { sample_rate: Option<u16> },
    /// You can optionally specify the rate of the audio. The default sampling rate is 22,050 Hz
    AudioMpeg { sample_rate: Option<u16> },
    /// You must specify the rate of the audio
    AudioMulaw { sample_rate: u16 },
    /// You can optionally specify the rate of the audio. The default sampling rate is 22,050 Hz
    AudioWav { sample_rate: Option<u16> },
    /// The service returns the audio in the opus codec. The service returns audio with a sampling rate of 48,000 Hz
    AudioWebm,
    /// The service returns audio with a sampling rate of 48,000 Hz
    AudioWebmCodecsOpus,
    /// You can optionally specify the rate of the audio. The default sampling rate is 22,050 Hz
    AudioWebmCodecsVorbis { sample_rate: Option<u16> },
}

impl Default for AudioFormat {
    /// The default audio format: [`AudioOggCodecsOpus`]
    ///
    /// [`AudioOggCodecsOpus`]: Self::AudioOggCodecsOpus
    fn default() -> Self {
        AudioFormat::AudioOggCodecsOpus {
            sample_rate: Some(48000),
        }
    }
}

impl AudioFormat {
    /// The value that the server expects for a particular format
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
/// The server expects the following values for audio endianness
pub enum AudioEndianness {
    /// Big Endian
    BigEndian,
    #[default]
    /// Little Endian
    LittleEndian,
}

impl AudioEndianness {
    /// The string value expected by the server for [`AudioEndianness`]
    ///
    /// [`AudioEndianness`]: Self
    pub fn id(&self) -> &str {
        match self {
            AudioEndianness::BigEndian => "big-endian",
            AudioEndianness::LittleEndian => "little-endian",
        }
    }
}

impl TextToSpeech<'_> {
    /// Synthesises text to audio that is spoken in the [`specified voice`]. The service bases its understanding of the language for the input text on the specified voice. Use a voice that matches the language of the input text.
    ///
    /// # Parameters
    ///
    /// * `text` - The text to synthesise
    /// * `format` - The requested [`AudioFormat`] (MIME type) of the audio. Defaults to [`AudioOggCodecsOpus`]
    /// * `customisation_id` - The customization ID (GUID) of a custom [`model`] to use for the synthesis. If a custom model is specified, it works only if it matches the [`language`] of the indicated voice. You must make the request with credentials for the instance of the service that owns the custom model. Omit the parameter to use the specified voice with no customization
    ///
    /// [`AudioFormat`]: super::synthesis::AudioFormat
    /// [`AudioOggCodecsOpus`]: super::synthesis::AudioFormat::AudioOggCodecsOpus
    /// [`name`]: super::voices::Voice::name
    /// [`language`]: super::voices::Voice::language
    /// [`gender`]: super::voices::Voice::gender
    /// [`specified voice`]: super::TextToSpeech::set_voice()
    /// [`model`]: super::customisations::Model
    ///
    /// # Example
    /// ``` no_run
    /// # use ibm_watson::{
    /// #     auth::IamAuthenticator,
    /// #     tts::{voices::WatsonVoice, TextToSpeech},
    /// # };
    /// # async fn foo()-> Result<(), Box<dyn std::error::Error>> {
    /// # let auth = IamAuthenticator::new("api_key").await?;
    /// # let tts = TextToSpeech::new(&auth, "service_url");
    /// let synth_bytes = tts.synthesise("Hey there", None, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn synthesise(
        &self,
        text: impl AsRef<str>,
        format: Option<AudioFormat>,
        customisation_id: Option<&str>,
    ) -> Result<bytes::Bytes, SynthesisError> {
        let mut url = Url::parse(self.service_url).unwrap();
        url.set_path("v1/synthesize");
        url.set_query(customisation_id);
        url.query_pairs_mut().append_pair("text", text.as_ref());
        url.query_pairs_mut().append_pair("voice", self.voice.id());
        if let Some(format) = format {
            url.query_pairs_mut().append_pair("accept", &format.id());
        }
        let req = Request::new(Method::GET, url);
        let client = self.get_client();
        let response = client
            .execute(req)
            .await
            .map_err(|e| SynthesisError::ConnectionError(e.to_string()))?;
        assert_eq!(response.status(), 200);
        match response.status() {
            StatusCode::OK => {
                let bytes = response.bytes().await.unwrap();
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
