use bytes::Buf;
use hyper::{
    header::{HeaderValue, AUTHORIZATION},
    Body, Client, Method, Request, StatusCode,
};
use serde::{Deserialize, Serialize};
use url::Url;
pub mod errors;

use self::errors::PronunciationError;

use super::{voices::WatsonVoice, TextToSpeech};
#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Pronunciation {
    #[serde(rename = "pronunciation")]
    pub pronunciation: String,
}

#[derive(Clone, Copy, Debug, Default)]
pub enum PhonemeFormat {
    IBM,
    #[default]
    IPA,
}

impl PhonemeFormat {
    pub fn id(&self) -> &str {
        match self {
            PhonemeFormat::IBM => "ibm",
            PhonemeFormat::IPA => "ipa",
        }
    }
}

impl TextToSpeech<'_> {
    pub async fn get_pronunciation(
        &self,
        text: impl AsRef<str>,
        voice: Option<WatsonVoice>,
        format: Option<PhonemeFormat>,
        customisation_id: Option<impl AsRef<str>>,
    ) -> Result<Pronunciation, PronunciationError> {
        let mut url = Url::parse(self.service_url).unwrap();
        url.set_path("v1/pronunciation");
        url.query_pairs_mut()
            .append_pair("text", text.as_ref())
            .append_pair("format", format.unwrap_or_default().id())
            .append_pair(
                "voice",
                match &voice {
                    Some(voice) => voice.id(),
                    None => self.voice.id(),
                },
            );
        if let Some(c_id) = &customisation_id {
            url.query_pairs_mut()
                .append_pair("customization_id", c_id.as_ref());
        }
        let req = Request::builder()
            .uri(url.to_string())
            .header(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", self.access_token)).unwrap(),
            )
            .method(Method::GET)
            .body(Body::empty())
            .map_err(|e| PronunciationError::ConnectionError(e.to_string()))?;
        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http1()
            .build();
        let client = Client::builder().build(https);
        let response = client
            .request(req)
            .await
            .map_err(|e| PronunciationError::ConnectionError(e.to_string()))?;
        match response.status() {
            StatusCode::OK => {
                let body = hyper::body::aggregate(response).await.unwrap();
                let root: Pronunciation = serde_json::from_reader(body.reader()).unwrap();
                Ok(root)
            }
            StatusCode::NOT_ACCEPTABLE => Err(PronunciationError::NotAcceptable406),
            StatusCode::UNAUTHORIZED => Err(PronunciationError::Unuathorised401(
                customisation_id.unwrap().as_ref().to_string(),
            )),
            StatusCode::NOT_FOUND => Err(PronunciationError::NotFound404),
            StatusCode::SERVICE_UNAVAILABLE => Err(PronunciationError::ServiceUnavailable503),
            StatusCode::BAD_REQUEST => Err(PronunciationError::BadRequest400),
            StatusCode::INTERNAL_SERVER_ERROR => Err(PronunciationError::InternalServerError500),
            _ => {
                unreachable!()
            }
        }
    }
}
