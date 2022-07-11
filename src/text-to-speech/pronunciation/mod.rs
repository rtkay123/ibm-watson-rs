use bytes::Buf;
use hyper::{
    header::{HeaderValue, AUTHORIZATION},
    Body, Method, Request, StatusCode,
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
    /// Gets the phonetic [`Pronunciation`] for the specified word. You can request the pronunciation for a specific [`format`]. You can also request the pronunciation for a specific [`voice`] to see the default translation for the language of that voice or for a specific custom [`model`] to see the translation for that model.
    ///
    /// # Parameters
    ///
    /// * `text` - The word for which the pronunciation is requested
    /// * `voice` - A [`voice`] that specifies the language in which the pronunciation is to be returned. If [`None`], the voice you [`set`] for the service will be used. If none has been set, the [`default`] will be used
    /// * `format` - The [`PhonemeFormat`] in which to return the pronunciation. The Arabic, Chinese, Dutch, Australian English, and Korean languages support only IPA. Omit the parameter to obtain the pronunciation in the default format
    /// * `customisation_id` - The customisation ID (GUID) of a custom [`model`] for which the pronunciation is to be returned. The language of a specified custom model must match the language of the specified voice. If the word is not defined in the specified custom model, the service returns the default translation for the custom model's language. You must make the request with credentials for the instance of the service that owns the custom model. Omit the parameter to see the translation for the specified voice with no customisation
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
    /// let customisation_id = Some("cust-id");
    /// let pronunciation = tts.get_pronunciation("word", None, None, customisation_id).await?;
    /// println!("{:#?}", pronunciation);
    /// # Ok(())
    /// # }
    /// ```
    /// [`None`]: std::option::Option::None
    /// [`set`]: Self::set_voice()
    /// [`voice`]: super::voices::WatsonVoice
    /// [`default`]: super::voices::WatsonVoice::EnUsMichaelV3
    /// [`PhonemeFormat`]: self::PhonemeFormat
    /// [`format`]: self::PhonemeFormat
    /// [`model`]: self::customisations::Model
    /// [`Pronunciation`]: self::Pronunciation
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
        let client = self.get_client();
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
