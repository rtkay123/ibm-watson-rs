use bytes::Buf;
use hyper::{
    header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Body, Method, Request, StatusCode,
};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::tts::TextToSpeech;

use super::errors::{AddWordError, DeleteWordError, GetWordError, ListWordsError};

#[derive(Clone, Debug, Eq, PartialEq, Default, Serialize, Deserialize)]
/// Defines words and their translations to be used in custom [`models`]
///
/// [`models`]: crate::tts::customisations::Model
pub struct Word {
    /// the word for the custom model. the maximum length of a word is 49 characters.
    #[serde(rename = "word")]
    pub word: String,
    /// the phonetic or sounds-like translation for the word. a phonetic translation is based on the ssml format for representing the phonetic string of a word either as an ipa or ibm spr translation. the arabic, chinese, dutch, australian english, and korean languages support only ipa. a sounds-like translation consists of one or more words that, when combined, sound like the word. the maximum length of a translation is 499 characters.
    #[serde(rename = "translation")]
    pub translation: String,
    /// japanese only. the part of speech for the word. the service uses the value to produce the correct intonation for the word. you can create only a single entry, with or without a single part of speech, for any word; you cannot create multiple entries with different parts of speech for the same word.
    #[serde(rename = "part_of_speech", skip_serializing_if = "Option::is_none")]
    pub part_of_speech: Option<String>,
}

impl TextToSpeech<'_> {
    /// Adds one or more words and their translations to the specified custom [`model`]. Adding a new translation for a word that already exists in a custom model overwrites the word's existing translation. A custom model can contain no more than 20,000 entries. You must use credentials for the instance of the service that owns a model to add words to it.
    ///
    /// # Parameters
    ///
    /// * `customisation_id` - The customisation ID (GUID) of the custom [`model`]. You must make the request with credentials for the instance of the service that owns the custom model
    /// * `words` - [`Words`] that are to be added or updated for the custom model and the translation for each specified word
    ///
    /// # Example
    /// ``` no_run
    /// # use ibm_watson::{
    /// #     auth::IamAuthenticator,
    /// #     tts::{voices::WatsonVoice, TextToSpeech,
    /// #     customisations::Word},
    /// # };
    /// # async fn foo()-> Result<(), Box<dyn std::error::Error>> {
    /// # let auth = IamAuthenticator::new("api_key").await?;
    /// # let tts = TextToSpeech::new(&auth, "service_url");
    /// let word = Word {
    ///     word: String::default(),
    ///     translation: String::default(),
    ///     part_of_speech: Option::default()
    /// };
    /// let mut words = vec![];
    /// words.push(word);
    /// if let Ok(_) = tts.add_custom_words("word", &words).await {
    ///     println!("word(s) added!");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`model`]: crate::tts::customisations::Model
    /// [`Words`]: self::Word
    /// [`AddWordError`]: super::errors::AddWordError
    pub async fn add_custom_words(
        &self,
        customisation_id: impl AsRef<str>,
        words: &[Word],
    ) -> Result<(), AddWordError> {
        let mut url = Url::parse(self.service_url).unwrap();
        Self::set_words_path(&mut url, &customisation_id);
        #[derive(Serialize, Deserialize)]
        struct FormBody {
            words: Vec<Word>,
        }
        impl FormBody {
            fn new(words: &[Word]) -> Self {
                Self {
                    words: words.to_owned(),
                }
            }
        }
        let body = serde_json::to_string(&FormBody::new(words)).unwrap();
        let req = Request::builder()
            .uri(url.to_string())
            .header(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", self.access_token)).unwrap(),
            )
            .header(CONTENT_TYPE, "application/json")
            .method(Method::POST)
            .body(Body::from(body))
            .map_err(|e| AddWordError::ConnectionError(e.to_string()))?;
        let client = self.get_client();
        let response = client
            .request(req)
            .await
            .map_err(|e| AddWordError::ConnectionError(e.to_string()))?;
        match response.status() {
            StatusCode::OK => Ok(()),
            StatusCode::BAD_REQUEST => Err(AddWordError::BadRequest400),
            StatusCode::UNAUTHORIZED => Err(AddWordError::Unauthorised401(
                customisation_id.as_ref().to_owned(),
            )),
            StatusCode::INTERNAL_SERVER_ERROR => Err(AddWordError::InternalServerError500),
            StatusCode::SERVICE_UNAVAILABLE => Err(AddWordError::ServiceUnavailable503),
            _ => {
                unreachable!()
            }
        }
    }

    /// Lists all of the words and their translations for the specified custom model. The output shows the translations as they are defined in the model. You must use credentials for the instance of the service that owns a model to list its words
    ///
    /// # Parameters
    ///
    /// * `customisation_id` -  The customization ID (GUID) of the custom [`model`]. You must make the request with credentials for the instance of the service that owns the custom model
    ///
    /// # Example
    /// ``` no_run
    /// # use ibm_watson::{
    /// #     auth::IamAuthenticator,
    /// #     tts::{voices::WatsonVoice, TextToSpeech,
    /// #     customisations::Word},
    /// # };
    /// # async fn foo()-> Result<(), Box<dyn std::error::Error>> {
    /// # let auth = IamAuthenticator::new("api_key").await?;
    /// # let tts = TextToSpeech::new(&auth, "service_url");
    /// let words = tts.list_custom_words("customisation_id").await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`model`]: crate::tts::customisations::Model
    /// [`Words`]: self::Word
    /// [`Error`]: super::errors::ListWordsError
    pub async fn list_custom_words(
        &self,
        customisation_id: impl AsRef<str>,
    ) -> Result<Vec<Word>, ListWordsError> {
        let mut url = Url::parse(self.service_url).unwrap();
        Self::set_words_path(&mut url, &customisation_id);
        let req = Request::builder()
            .uri(url.to_string())
            .header(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", self.access_token)).unwrap(),
            )
            .method(Method::GET)
            .body(Body::empty())
            .map_err(|e| ListWordsError::ConnectionError(e.to_string()))?;
        let client = self.get_client();
        let response = client
            .request(req)
            .await
            .map_err(|e| ListWordsError::ConnectionError(e.to_string()))?;
        match response.status() {
            StatusCode::OK => {
                let body = hyper::body::aggregate(response).await.unwrap();
                #[derive(Deserialize, Serialize)]
                struct Root {
                    words: Vec<Word>,
                }
                let root: Root = serde_json::from_reader(body.reader()).unwrap();
                Ok(root.words)
            }
            StatusCode::BAD_REQUEST => Err(ListWordsError::BadRequest400),
            StatusCode::INTERNAL_SERVER_ERROR => Err(ListWordsError::InternalServerError500),
            StatusCode::SERVICE_UNAVAILABLE => Err(ListWordsError::ServiceUnavailable503),
            StatusCode::UNAUTHORIZED => Err(ListWordsError::Unauthorised401(
                customisation_id.as_ref().to_owned(),
            )),
            _ => {
                unreachable!()
            }
        }
    }

    /// Adds a single [`word`] and its translation to the specified custom model. Adding a new translation for a word that already exists in a custom model overwrites the word's existing translation. A custom model can contain no more than 20,000 entries. You must use credentials for the instance of the service that owns a model to add a word to it
    ///
    /// # Parameters
    ///
    /// * `customisation_id` -  The customization ID (GUID) of the custom [`model`]. You must make the request with credentials for the instance of the service that owns the custom model
    /// * `word` -  The customization ID (GUID) of the custom [`model`]. You must make the request with credentials for the instance of the service that owns the custom model
    ///
    /// # Example
    /// ``` no_run
    /// # use ibm_watson::{
    /// #     auth::IamAuthenticator,
    /// #     tts::{voices::WatsonVoice, TextToSpeech,
    /// #     customisations::Word},
    /// # };
    /// # async fn foo()-> Result<(), Box<dyn std::error::Error>> {
    /// # let auth = IamAuthenticator::new("api_key").await?;
    /// # let tts = TextToSpeech::new(&auth, "service_url");
    /// let word = Word {
    ///     word: String::default(),
    ///     translation: String::default(),
    ///     part_of_speech: Option::default()
    /// };
    /// tts.add_custom_word("customisation_id", &word).await?;
    /// # Ok(())
    /// # }
    /// ```
    /// [`Words`]: self::Word
    /// [`word`]: self::Word
    /// [`model`]: crate::tts::customisations::Model
    pub async fn add_custom_word(
        &self,
        customisation_id: impl AsRef<str>,
        word: &Word,
    ) -> Result<(), AddWordError> {
        let mut url = Url::parse(self.service_url).unwrap();
        url.set_path(&format!(
            "v1/customizations/{}/words/{}",
            customisation_id.as_ref(),
            &word.word
        ));
        #[derive(Serialize, Deserialize)]
        struct FormBody {
            /// the phonetic or sounds-like translation for the word. a phonetic translation is based on the ssml format for representing the phonetic string of a word either as an ipa or ibm spr translation. the arabic, chinese, dutch, australian english, and korean languages support only ipa. a sounds-like translation consists of one or more words that, when combined, sound like the word. the maximum length of a translation is 499 characters.
            #[serde(rename = "translation")]
            translation: String,
            /// japanese only. the part of speech for the word. the service uses the value to produce the correct intonation for the word. you can create only a single entry, with or without a single part of speech, for any word; you cannot create multiple entries with different parts of speech for the same word.
            #[serde(rename = "part_of_speech", skip_serializing_if = "Option::is_none")]
            part_of_speech: Option<String>,
        }
        impl FormBody {
            fn new(words: &Word) -> Self {
                Self {
                    translation: words.translation.clone(),
                    part_of_speech: words.part_of_speech.clone(),
                }
            }
        }
        let body = serde_json::to_string(&FormBody::new(word)).unwrap();
        let req = Request::builder()
            .uri(url.to_string())
            .header(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", self.access_token)).unwrap(),
            )
            .header(CONTENT_TYPE, "application/json")
            .method(Method::PUT)
            .body(Body::from(body))
            .map_err(|e| AddWordError::ConnectionError(e.to_string()))?;
        let client = self.get_client();
        let response = client
            .request(req)
            .await
            .map_err(|e| AddWordError::ConnectionError(e.to_string()))?;
        match response.status() {
            StatusCode::OK => Ok(()),
            StatusCode::BAD_REQUEST => Err(AddWordError::BadRequest400),
            StatusCode::UNAUTHORIZED => Err(AddWordError::Unauthorised401(
                customisation_id.as_ref().to_owned(),
            )),
            StatusCode::INTERNAL_SERVER_ERROR => Err(AddWordError::InternalServerError500),
            StatusCode::SERVICE_UNAVAILABLE => Err(AddWordError::ServiceUnavailable503),
            _ => {
                unreachable!()
            }
        }
    }

    /// Gets the translation for a single word from the specified custom model. The output shows the translation as it is defined in the model. You must use credentials for the instance of the service that owns a model to list its words.
    ///
    /// # Parameters
    ///
    /// * `customisation_id` - The customization ID (GUID) of the custom [`model`]. You must make the request with credentials for the instance of the service that owns the custom model
    /// * `word` - The word that is to be queried from the custom [`model`]
    ///
    /// # Example
    /// ``` no_run
    /// # use ibm_watson::{
    /// #     auth::IamAuthenticator,
    /// #     tts::{voices::WatsonVoice, TextToSpeech,
    /// #     customisations::Word},
    /// # };
    /// # async fn foo()-> Result<(), Box<dyn std::error::Error>> {
    /// # let auth = IamAuthenticator::new("api_key").await?;
    /// # let tts = TextToSpeech::new(&auth, "service_url");
    /// let word = tts.get_custom_word("customisation_id", "foo").await?;
    /// println!("{:#?}", word);
    /// # Ok(())
    /// # }
    /// ```
    /// [`Words`]: self::Word
    /// [`word`]: self::Word
    /// [`model`]: crate::tts::customisations::Model
    pub async fn get_custom_word(
        &self,
        customisation_id: impl AsRef<str>,
        word: impl AsRef<str>,
    ) -> Result<Word, GetWordError> {
        let mut url = Url::parse(self.service_url).unwrap();
        url.set_path(&format!(
            "v1/customizations/{}/words/{}",
            customisation_id.as_ref(),
            word.as_ref()
        ));
        let req = Request::builder()
            .uri(url.to_string())
            .header(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", self.access_token)).unwrap(),
            )
            .method(Method::GET)
            .body(Body::empty())
            .map_err(|e| GetWordError::ConnectionError(e.to_string()))?;
        let client = self.get_client();
        let response = client
            .request(req)
            .await
            .map_err(|e| GetWordError::ConnectionError(e.to_string()))?;
        match response.status() {
            StatusCode::OK => {
                let body = hyper::body::aggregate(response).await.unwrap();
                let root: Word = serde_json::from_reader(body.reader()).unwrap();
                Ok(root)
            }
            StatusCode::BAD_REQUEST => Err(GetWordError::BadRequest400),
            StatusCode::INTERNAL_SERVER_ERROR => Err(GetWordError::InternalServerError500),
            StatusCode::SERVICE_UNAVAILABLE => Err(GetWordError::ServiceUnavailable503),
            StatusCode::UNAUTHORIZED => Err(GetWordError::Unauthorised401(
                customisation_id.as_ref().to_owned(),
            )),
            _ => {
                unreachable!()
            }
        }
    }

    /// Deletes a single word from the specified custom model. You must use credentials for the instance of the service that owns a model to delete its words.
    /// # Parameters
    ///
    /// * `customisation_id` - The customization ID (GUID) of the custom [`model`]. You must make the request with credentials for the instance of the service that owns the custom model
    /// * `word` - The word that is to be queried from the custom [`model`]
    ///
    /// # Example
    /// ``` no_run
    /// # use ibm_watson::{
    /// #     auth::IamAuthenticator,
    /// #     tts::{voices::WatsonVoice, TextToSpeech,
    /// #     customisations::Word},
    /// # };
    /// # async fn foo()-> Result<(), Box<dyn std::error::Error>> {
    /// # let auth = IamAuthenticator::new("api_key").await?;
    /// # let tts = TextToSpeech::new(&auth, "service_url");
    /// if tts.delete_custom_word("customisation_id", "foo").await.is_ok() {
    ///     println!("word deleted");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    /// [`Words`]: self::Word
    /// [`word`]: self::Word
    /// [`model`]: crate::tts::customisations::Model
    pub async fn delete_custom_word(
        &self,
        customisation_id: impl AsRef<str>,
        word: impl AsRef<str>,
    ) -> Result<(), DeleteWordError> {
        let mut url = Url::parse(self.service_url).unwrap();
        url.set_path(&format!(
            "v1/customizations/{}/words/{}",
            customisation_id.as_ref(),
            word.as_ref()
        ));
        let req = Request::builder()
            .uri(url.to_string())
            .header(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", self.access_token)).unwrap(),
            )
            .method(Method::DELETE)
            .body(Body::empty())
            .map_err(|e| DeleteWordError::ConnectionError(e.to_string()))?;
        let client = self.get_client();
        let response = client
            .request(req)
            .await
            .map_err(|e| DeleteWordError::ConnectionError(e.to_string()))?;
        match response.status() {
            StatusCode::NO_CONTENT => Ok(()),
            StatusCode::BAD_REQUEST => Err(DeleteWordError::BadRequest400(
                customisation_id.as_ref().to_owned(),
            )),
            StatusCode::INTERNAL_SERVER_ERROR => Err(DeleteWordError::InternalServerError500),
            StatusCode::SERVICE_UNAVAILABLE => Err(DeleteWordError::ServiceUnavailable503),
            StatusCode::UNAUTHORIZED => Err(DeleteWordError::Unauthorised401(
                customisation_id.as_ref().to_owned(),
            )),
            _ => {
                unreachable!()
            }
        }
    }

    fn set_words_path(uri: &mut Url, customisation_id: impl AsRef<str>) {
        uri.set_path(&format!(
            "v1/customizations/{}/words",
            customisation_id.as_ref()
        ));
    }
}
