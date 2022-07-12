use std::borrow::Cow;

use bytes::Buf;
use hyper::{
    header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Body, Method, Request, StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use url::Url;

use crate::tts::TextToSpeech;

use super::{
    errors::{CreateModelError, DeleteModelError, GetModelError, ListModelError, UpdateModelError},
    prompts::Prompt,
    words::Word,
};

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
/// Defines a custom model
pub struct Model {
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
/// The language of the new custom model
pub enum Language {
    /// Arabic
    ArMs,
    /// Czech (Czechia)
    CsCz,
    /// German (Germany)
    DeDe,
    /// English (Australia)
    EnAu,
    /// English (United Kingdom)
    EnGb,
    #[default]
    /// English (United States)
    EnUs,
    /// Spanish (Spain)
    EsEs,
    /// Spanish (Latin America)
    EsLa,
    /// Spanish (United States)
    EsUs,
    /// French (Canada)
    FrCa,
    /// French (France)
    FrFr,
    /// Italian (Italy)
    ItIt,
    /// Japanese (Japan)
    JaJp,
    /// Koren (South Korea)
    KoKr,
    /// Dutch (Belgium)
    NlBe,
    /// Dutch (Netherlands)
    NlNl,
    /// Portuguese (Brazil)
    PtBr,
    /// Swedish (Sweden)
    SvSe,
    /// Chinese (PRC)
    ZhCn,
}

impl Language {
    /// The value that the server expects for a particular language
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
    /// Creates a new empty custom model. You must specify a name for the new custom model. You can optionally specify the language and a description for the new model. The model is owned by the instance of the service whose credentials are used to create it
    ///
    /// # Parameters
    ///
    /// * `name` - The name of the new custom model
    /// * `language` - The language of the new custom model. You create a custom model for a specific language, not for a specific voice. A custom model can be used with any voice for its specified language. If [`None`] is specified, the [`default language`] is used
    /// * `description` - A description of the new custom model. Specifying a description is recommended
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
    /// let model = tts.create_custom_model("new model", None, Some("example")).await?;
    /// println!("model: {:#?}", model);
    /// # Ok(())
    /// # }
    /// ```
    /// [`None`]: std::option::Option::None
    /// [`default language`]: self::Language::EnUs
    pub async fn create_custom_model(
        &self,
        name: impl AsRef<str>,
        language: Option<Language>,
        description: Option<impl AsRef<str>>,
    ) -> Result<Model, CreateModelError> {
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
        let client = self.get_client();
        let response = client
            .request(req)
            .await
            .map_err(|e| CreateModelError::ConnectionError(e.to_string()))?;
        match response.status() {
            StatusCode::OK => {
                let body = hyper::body::aggregate(response).await.unwrap();
                let root: Model = serde_json::from_reader(body.reader()).unwrap();
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

    /// Lists metadata such as the name and description for all custom models that are owned by an instance of the service. Specify a [`language`] to list the custom models for that language only. To see the words and prompts in addition to the metadata for a specific custom model, use [`get_custom_model()`]. You must use credentials for the instance of the service that owns a model to list information about it.
    ///
    /// # Parameters
    ///
    /// * `language` - The language for which custom models that are owned by the requesting credentials are to be returned. Pass [`None`] to see all custom models that are owned by the requester
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
    /// let models = tts.list_custom_models(None).await?;
    /// println!("found: {:#?} models", models.len());
    /// # Ok(())
    /// # }
    /// ```
    /// [`None`]: std::option::Option::None
    /// [`language`]: self::Language
    /// [`get_custom_model()`]: Self::get_custom_model()
    pub async fn list_custom_models(
        &self,
        language: Option<Language>,
    ) -> Result<Vec<Model>, ListModelError> {
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
                    customizations: Vec<Model>,
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

    /// Updates information for the specified custom model. You can update metadata such as the
    /// name and description of the model. You can also update the words in the model and their
    /// translations. Adding a new translation for a word that already exists in a custom model
    /// overwrites the word's existing translation. A custom model can contain no more than 20,000
    /// entries. You must use credentials for the instance of the service that owns a model to
    /// update it
    ///
    /// # Parameters
    ///
    /// * `customisation_id` - The customization ID (GUID) of the custom model. You must make the request with credentials for the instance of the service that owns the custom model
    /// * `name` - A new [`name`] for the custom model
    /// * `description` - A new [`description`] for the custom model
    /// * `words` - An array of [`Word`] objects that provides the words and their translations that are to be added or updated for the custom model. Pass an empty array to make no additions or updates
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
    /// tts.update_custom_model("cust-id", Some("foo"), None, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    /// [`name`]: crate::tts::customisations::Model::name
    /// [`description`]: crate::tts::customisations::Model::description
    /// [`Word`]: crate::tts::customisations::Word
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

    /// Gets all information about a specified custom model. In addition to metadata such as the name and description of the custom model, the output includes the words and their translations that are defined for the model, as well as any prompts that are defined for the model. To see just the metadata for a model, use [`list_custom_models()`].
    ///
    /// # Parameters
    ///
    /// * `customisation_id` - The customization ID (GUID) of the custom model. You must make the request with credentials for the instance of the service that owns the custom model
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
    /// let model = tts.get_custom_model("cust-id").await?;
    /// println!("{:#?}", model);
    /// # Ok(())
    /// # }
    /// ```
    /// [`language`]: self::Language
    /// [`list_custom_models()`]: Self::list_custom_models()
    pub async fn get_custom_model(
        &self,
        customisation_id: impl AsRef<str>,
    ) -> Result<Model, GetModelError> {
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
                let root: Model = serde_json::from_reader(body.reader()).unwrap();
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

    /// Deletes the specified custom model. You must use credentials for the instance of the service that owns a model to delete it.
    ///
    /// # Parameters
    ///
    /// * `customisation_id` - The customization ID (GUID) of the custom model. You must make the request with credentials for the instance of the service that owns the custom model
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
    /// if tts.delete_custom_model("cust-id").await.is_ok() {
    ///     println!("model deleted");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    /// [`language`]: self::Language
    /// [`list_custom_models()`]: Self::list_custom_models()
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
