use std::borrow::Cow;

use bytes::Buf;
use hyper::{
    header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Body, Client, Method, Request, StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use url::Url;

use crate::tts::{
    customisations::errors::{ListModelError, UpdateModelError},
    TextToSpeech,
};

use self::errors::{CreateModelError, DeleteModelError, GetModelError};

use super::{prompts::Prompt, words::Word};

pub mod errors;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomModel {
    /// the customization id (guid) of the custom model. the create a custom model method returns only this field. it does not not return the other fields of this object.
    #[serde(rename = "customization_id")]
    pub customisation_id: String,
    /// the name of the custom model.
    #[serde(rename = "name")]
    pub name: String,
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
    pub fn id(&self) -> Cow<'static, str> {
        match self {
            Language::ArMs => Cow::from("ar-MS"),
            Language::CsCz => Cow::from("cs-CZ"),
            Language::DeDe => Cow::from("de-DE"),
            Language::EnAu => Cow::from("en-AU"),
            Language::EnGb => Cow::from("en-GB"),
            Language::EnUs => Cow::from("en-US"),
            Language::EsEs => Cow::from("es-ES"),
            Language::EsLa => Cow::from("es-LA"),
            Language::EsUs => Cow::from("es-US"),
            Language::FrCa => Cow::from("fr-CA"),
            Language::FrFr => Cow::from("fr-FR"),
            Language::ItIt => Cow::from("it-IT"),
            Language::JaJp => Cow::from("ja-JP"),
            Language::KoKr => Cow::from("ko-KR"),
            Language::NlBe => Cow::from("nl-BE"),
            Language::NlNl => Cow::from("nl-NL"),
            Language::PtBr => Cow::from("pt-BR"),
            Language::SvSe => Cow::from("sv-SE"),
            Language::ZhCn => Cow::from("zh-CN"),
        }
    }
}

impl TextToSpeech<'_> {
    pub async fn create_custom_model(
        &self,
        name: impl AsRef<str>,
        language: Option<Language>,
        description: Option<impl AsRef<str>>,
    ) -> Result<CustomModel, CreateModelError> {
        let mut url = Url::parse(self.service_url).unwrap();
        url.set_path("v1/customizations");
        #[derive(Serialize, Deserialize)]
        struct FormBody<'a> {
            name: &'a str,
            language: &'a str,
            description: &'a str,
        }
        let name = name.as_ref();
        let language = language.unwrap_or_default().id().to_owned();
        let description = match description {
            Some(s) => s.as_ref().to_owned(),
            None => String::default(),
        };
        let form_body = json!( {
            "name": name,
            "language": language,
            "description": description
        });
        let req = Request::builder()
            .uri(url.to_string())
            .header(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", self.access_token)).unwrap(),
            )
            .header(CONTENT_TYPE, "application/json")
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
            StatusCode::BAD_REQUEST => Err(CreateModelError::BadRequest400),
            StatusCode::INTERNAL_SERVER_ERROR => Err(CreateModelError::InternalServerError500),
            StatusCode::SERVICE_UNAVAILABLE => Err(CreateModelError::ServiceUnavailable503),
            _ => {
                unreachable!()
            }
        }
    }

    pub async fn list_custom_models(
        &self,
        language: Option<Language>,
    ) -> Result<Vec<CustomModel>, ListModelError> {
        let mut url = Url::parse(self.service_url).unwrap();
        url.set_path("v1/customizations");
        url.set_query(Some(&language.unwrap_or_default().id()));
        let req = Request::builder()
            .uri(url.to_string())
            .header(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", self.access_token)).unwrap(),
            )
            .method(Method::GET)
            .body(Body::empty())
            .map_err(|e| ListModelError::ConnectionError(e.to_string()))?;
        let client = self.get_client();
        let response = client
            .request(req)
            .await
            .map_err(|e| ListModelError::ConnectionError(e.to_string()))?;
        match response.status() {
            StatusCode::OK => {
                let body = hyper::body::aggregate(response).await.unwrap();
                #[derive(Deserialize, Serialize)]
                struct Root {
                    customizations: Vec<CustomModel>,
                }
                let root: Root = serde_json::from_reader(body.reader()).unwrap();
                Ok(root.customizations)
            }
            StatusCode::BAD_REQUEST => Err(ListModelError::BadRequest400),
            StatusCode::INTERNAL_SERVER_ERROR => Err(ListModelError::InternalServerError500),
            StatusCode::SERVICE_UNAVAILABLE => Err(ListModelError::ServiceUnavailable503),
            _ => {
                unreachable!()
            }
        }
    }

    pub async fn update_custom_model(
        &self,
        customisation_id: impl AsRef<str>,
        name: Option<&str>,
        description: Option<&str>,
        words: Option<&[Word]>,
    ) -> Result<(), UpdateModelError> {
        let mut url = Url::parse(self.service_url).unwrap();
        url.set_path(&format!("v1/customizations/{}", customisation_id.as_ref()));
        #[derive(Deserialize, Serialize)]
        struct Foo<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            words: Option<Vec<Word>>,
        }
        impl<'a> Foo<'a> {
            fn new(
                name: Option<&'a str>,
                description: Option<&'a str>,
                words: Option<&'a [Word]>,
            ) -> Self {
                Self {
                    name,
                    description,
                    words: words.map(|f| f.to_owned()),
                }
            }
        }
        let data = serde_json::to_string(&Foo::new(name, description, words)).unwrap();
        let req = Request::builder()
            .uri(url.to_string())
            .header(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", self.access_token)).unwrap(),
            )
            .header(CONTENT_TYPE, "application/json")
            .method(Method::POST)
            .body(Body::from(data))
            .map_err(|e| UpdateModelError::ConnectionError(e.to_string()))?;
        let client = self.get_client();
        let response = client
            .request(req)
            .await
            .map_err(|e| UpdateModelError::ConnectionError(e.to_string()))?;
        match response.status() {
            StatusCode::OK => Ok(()),
            StatusCode::BAD_REQUEST => Err(UpdateModelError::BadRequest400),
            StatusCode::INTERNAL_SERVER_ERROR => Err(UpdateModelError::InternalServerError500),
            StatusCode::SERVICE_UNAVAILABLE => Err(UpdateModelError::ServiceUnavailable503),
            StatusCode::UNAUTHORIZED => Err(UpdateModelError::Unauthorised401(
                customisation_id.as_ref().to_owned(),
            )),
            _ => {
                unreachable!()
            }
        }
    }

    pub async fn get_custom_model(
        &self,
        customisation_id: impl AsRef<str>,
    ) -> Result<CustomModel, GetModelError> {
        let mut url = Url::parse(self.service_url).unwrap();
        url.set_path(&format!("v1/customizations/{}", customisation_id.as_ref()));
        let req = Request::builder()
            .uri(url.to_string())
            .header(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", self.access_token)).unwrap(),
            )
            .method(Method::GET)
            .body(Body::empty())
            .map_err(|e| GetModelError::ConnectionError(e.to_string()))?;
        let client = self.get_client();
        let response = client
            .request(req)
            .await
            .map_err(|e| GetModelError::ConnectionError(e.to_string()))?;
        match response.status() {
            StatusCode::OK => {
                let body = hyper::body::aggregate(response).await.unwrap();
                let root: CustomModel = serde_json::from_reader(body.reader()).unwrap();
                Ok(root)
            }
            StatusCode::BAD_REQUEST => Err(GetModelError::BadRequest400(
                customisation_id.as_ref().to_owned(),
            )),
            StatusCode::INTERNAL_SERVER_ERROR => Err(GetModelError::InternalServerError500),
            StatusCode::SERVICE_UNAVAILABLE => Err(GetModelError::ServiceUnavailable503),
            StatusCode::NOT_MODIFIED => Err(GetModelError::NotModified304),
            StatusCode::UNAUTHORIZED => Err(GetModelError::Unauthorised401(
                customisation_id.as_ref().to_owned(),
            )),
            _ => {
                unreachable!()
            }
        }
    }

    pub async fn delete_custom_model(
        &self,
        customisation_id: impl AsRef<str>,
    ) -> Result<(), DeleteModelError> {
        let mut url = Url::parse(self.service_url).unwrap();
        url.set_path(&format!("v1/customizations/{}", customisation_id.as_ref()));
        let req = Request::builder()
            .uri(url.to_string())
            .header(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", self.access_token)).unwrap(),
            )
            .method(Method::DELETE)
            .body(Body::empty())
            .map_err(|e| DeleteModelError::ConnectionError(e.to_string()))?;
        let client = self.get_client();
        let response = client
            .request(req)
            .await
            .map_err(|e| DeleteModelError::ConnectionError(e.to_string()))?;
        match response.status() {
            StatusCode::NO_CONTENT => Ok(()),
            StatusCode::BAD_REQUEST => Err(DeleteModelError::BadRequest400(
                customisation_id.as_ref().to_owned(),
            )),
            StatusCode::INTERNAL_SERVER_ERROR => Err(DeleteModelError::InternalServerError500),
            StatusCode::SERVICE_UNAVAILABLE => Err(DeleteModelError::ServiceUnavailable503),
            StatusCode::UNAUTHORIZED => Err(DeleteModelError::Unauthorised401(
                customisation_id.as_ref().to_owned(),
            )),
            _ => {
                unreachable!()
            }
        }
    }
}
