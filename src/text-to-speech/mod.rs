use hyper::{
    body::Buf,
    header::{HeaderValue, AUTHORIZATION},
    Body, Client, Method, Request, StatusCode,
};
use serde::Deserialize;
use url::Url;

use crate::auth::IamAuthenticator;

use self::voices::{GetVoiceError, ListVoicesError, Voice, WatsonVoice};
#[path = "custom-models/mod.rs"]
pub mod custom_models;
#[path = "custom-prompts/mod.rs"]
pub mod custom_prompts;
#[path = "custom-words/mod.rs"]
pub mod custom_words;
pub mod voices;

pub struct TextToSpeech<'a> {
    access_token: &'a str,
    service_url: String,
    voice: WatsonVoice,
}

impl<'a> TextToSpeech<'a> {
    pub fn new(authenticator: &'a IamAuthenticator, service_url: impl Into<String>) -> Self {
        let ac = authenticator.token_response();
        let access_token = ac.access_token();
        let service_url = service_url.into();
        Self {
            access_token,
            service_url,
            voice: WatsonVoice::default(),
        }
    }

    pub async fn list_voices(&self) -> Result<Vec<Voice>, ListVoicesError> {
        let mut url = Url::parse(&self.service_url).unwrap();
        Self::set_voices_path(&mut url);
        let req = Request::builder()
            .uri(url.to_string())
            .header(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", self.access_token)).unwrap(),
            )
            .method(Method::GET)
            .body(Body::empty())
            .map_err(|e| ListVoicesError::ConnectionError(e.to_string()))?;
        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http1()
            .build();
        let client = Client::builder().build(https);
        let response = client
            .request(req)
            .await
            .map_err(|e| ListVoicesError::ConnectionError(e.to_string()))?;
        match response.status() {
            StatusCode::OK => {
                #[derive(Deserialize)]
                struct Root {
                    voices: Vec<Voice>,
                }
                let body = hyper::body::aggregate(response).await.unwrap();
                let root: Root = serde_json::from_reader(body.reader()).unwrap();

                Ok(root.voices)
            }
            StatusCode::NOT_ACCEPTABLE => Err(ListVoicesError::NotAcceptable),
            StatusCode::UNSUPPORTED_MEDIA_TYPE => Err(ListVoicesError::UnsupportedMediaType),
            StatusCode::INTERNAL_SERVER_ERROR => Err(ListVoicesError::InternalServerError),
            StatusCode::SERVICE_UNAVAILABLE => Err(ListVoicesError::ServiceUnavailable),
            _ => {
                unreachable!()
            }
        }
    }

    fn set_voices_path(uri: &mut Url) {
        uri.set_path("v1/voices");
    }

    pub async fn get_voice(
        &self,
        voice: WatsonVoice,
        customisation_id: Option<&str>,
    ) -> Result<Voice, GetVoiceError> {
        let mut url = Url::parse(&self.service_url).unwrap();
        Self::set_voices_path(&mut url);
        url.set_query(customisation_id);
        let req = Request::builder()
            .uri(format!("{}/{}", url, voice.id()))
            .header(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", self.access_token)).unwrap(),
            )
            .method(Method::GET)
            .body(Body::empty())
            .map_err(|e| GetVoiceError::ConnectionError(e.to_string()))?;
        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http1()
            .build();
        let client = Client::builder().build(https);
        let response = client
            .request(req)
            .await
            .map_err(|e| GetVoiceError::ConnectionError(e.to_string()))?;
        assert_eq!(response.status(), 200);
        match response.status() {
            StatusCode::OK => {
                let body = hyper::body::aggregate(response).await.unwrap();
                let root: Voice = serde_json::from_reader(body.reader()).unwrap();

                Ok(root)
            }
            StatusCode::NOT_ACCEPTABLE => Err(GetVoiceError::NotAcceptable),
            StatusCode::UNSUPPORTED_MEDIA_TYPE => Err(GetVoiceError::UnsupportedMediaType),
            StatusCode::INTERNAL_SERVER_ERROR => Err(GetVoiceError::InternalServerError),
            StatusCode::SERVICE_UNAVAILABLE => Err(GetVoiceError::ServiceUnavailable),
            _ => {
                unreachable!()
            }
        }
    }
}
