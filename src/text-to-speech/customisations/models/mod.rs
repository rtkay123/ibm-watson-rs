use std::collections::HashMap;

use bytes::Buf;
use hyper::{
    header::{HeaderValue, AUTHORIZATION},
    Body, Client, Method, Request, StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use url::{form_urlencoded::byte_serialize, Url};

use crate::tts::TextToSpeech;

use self::errors::CreateModelError;

use super::{prompts::Prompt, words::Word};

pub mod errors;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomModel {
    /// the customization id (guid) of the custom model. the create a custom model method returns only this field. it does not not return the other fields of this object.
    #[serde(rename = "customization_id")]
    pub customisation_id: String,
    /// the name of the custom model.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// the language identifier of the custom model (for example, en-us).
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// the guid of the credentials for the instance of the service that owns the custom model.
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// the date and time in coordinated universal time (utc) at which the custom model was created. the value is provided in full iso 8601 format (yyyy-mm-ddthh:mm:ss.stzd)
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// the date and time in coordinated universal time (utc) at which the custom model was last modified. the created and updated fields are equal when a model is first added but has yet to be updated. the value is provided in full iso 8601 format (yyyy-mm-ddthh:mm:ss.stzd).
    #[serde(rename = "last_modified", skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    /// the description of the custom model.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// an array of word objects that lists the words and their translations from the custom model. the words are listed in alphabetical order, with uppercase letters listed before lowercase letters. the array is empty if no words are defined for the custom model. this field is returned only by the get a custom model method.
    #[serde(rename = "words", skip_serializing_if = "Option::is_none")]
    pub words: Option<Vec<Word>>,
    /// an array of prompt objects that provides information about the prompts that are defined for the specified custom model. the array is empty if no prompts are defined for the custom model. this field is returned only by the get a custom model method.
    #[serde(rename = "prompts", skip_serializing_if = "Option::is_none")]
    pub prompts: Option<Vec<Prompt>>,
}
#[non_exhaustive]
#[derive(Default)]
pub enum Language {
    ArMs,
    CsCz,
    DeDe,
    EnAu,
    EnGb,
    #[default]
    EnUs,
    EsEs,
    EsLa,
    EsUs,
    FrCa,
    FrFr,
    ItIt,
    JaJp,
    KoKr,
    NlBe,
    NlNl,
    PtBr,
    SvSe,
    ZhCn,
}

impl Language {
    pub fn id(&self) -> &str {
        todo!()
    }
}

impl TextToSpeech<'_> {
    pub async fn create_custom_model(
        &self,
        name: impl AsRef<str>,
        language: Option<Language>,
        description: impl AsRef<str>,
    ) -> Result<CustomModel, CreateModelError> {
        let mut url = Url::parse(self.service_url).unwrap();
        url.set_path("v1/customizations");
        #[derive(Serialize, Deserialize)]
        struct FormBody<'a> {
            name: &'a str,
            language: &'a str,
            description: &'a str,
        }
        let form_body = json!( {
            "name": name.as_ref(),
            "language": language.unwrap_or_default().id(),
            "description": description.as_ref(),
        });
        let req = Request::builder()
            .uri(url.to_string())
            .header(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", self.access_token)).unwrap(),
            )
            .method(Method::POST)
            .body(Body::from(form_body.to_string()))
            .map_err(|e| CreateModelError::ConnectionError(e.to_string()))?;
        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http1()
            .build();
        let client = Client::builder().build(https);

        let response = client
            .request(req)
            .await
            .map_err(|e| CreateModelError::ConnectionError(e.to_string()))?;
        match response.status() {
            StatusCode::OK => {
                let body = hyper::body::aggregate(response).await.unwrap();
                let root: CustomModel = serde_json::from_reader(body.reader()).unwrap();

                Ok(root)
            }
            _ => {
                unreachable!()
            }
        }
    }
}
