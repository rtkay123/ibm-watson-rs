pub mod errors;

use self::errors::ListModelsError;

use super::SpeechToText;

use reqwest::{Method, Request, StatusCode, Url, Version};
use serde::Deserialize;

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Model {
    pub name: String,
    pub language: String,
    pub url: String,
    pub rate: i64,
    #[serde(rename = "supported_features")]
    pub supported_features: SupportedFeatures,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct SupportedFeatures {
    #[serde(rename = "custom_language_model")]
    pub custom_language_model: bool,
    #[serde(rename = "custom_acoustic_model")]
    pub custom_acoustic_model: bool,
    #[serde(rename = "speaker_labels")]
    pub speaker_labels: bool,
}

impl SpeechToText<'_> {
    pub async fn list_models(&self) -> Result<Vec<Model>, ListModelsError> {
        let mut url = Url::parse(self.service_url).unwrap();

        Self::set_models_path(&mut url);

        let mut req = Request::new(Method::GET, url);

        if cfg!(feature = "http2") {
            *req.version_mut() = Version::HTTP_2;
        }

        let client = self.get_client();
        let response = client.execute(req).await?;
        match response.status() {
            StatusCode::OK => {
                #[derive(Deserialize)]
                struct Root {
                    models: Vec<Model>,
                }
                let root: Root = response.json().await.unwrap();

                Ok(root.models)
            }
            StatusCode::NOT_ACCEPTABLE => Err(ListModelsError::NotAcceptable406),
            StatusCode::UNSUPPORTED_MEDIA_TYPE => Err(ListModelsError::UnsupportedMediaType415),
            StatusCode::INTERNAL_SERVER_ERROR => Err(ListModelsError::InternalServerError500),
            StatusCode::SERVICE_UNAVAILABLE => Err(ListModelsError::ServiceUnavailable503),
            _ => Err(ListModelsError::UnmappedResponse(response.status().into())),
        }
    }

    fn set_models_path(uri: &mut Url) {
        uri.set_path("v1/models");
    }
}
