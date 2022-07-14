use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client, ClientBuilder,
};

use crate::auth::IamAuthenticator;

use self::voices::WatsonVoice;

/// Manage custom Prompts, Words, Models
pub mod customisations;
/// Manage word pronunciation
pub mod pronunciation;
#[path = "speaker-models/mod.rs"]
/// Manage speaker models
pub mod speaker_models;
/// Synthesise text to speech
pub mod synthesis;
#[path = "user-data/mod.rs"]
/// Delete user data
pub mod user_data;
/// View information about Watson voices
pub mod voices;

/// Creates a client used to send requests to your Text To Speech endpoint
pub struct TextToSpeech<'a> {
    service_url: &'a str,
    voice: WatsonVoice,
    client: Client,
}

impl<'a> TextToSpeech<'a> {
    /// Create a new Text To Speech instance. This instance will be used to make all the requests
    /// to the text to speech service.
    ///
    /// # Parameters
    /// * `authenticator` - The [`IamAuthenticator`] containing your IAM Access Token
    /// * `service_url` - The endpoint for your text to speech instance. All Text To Speech
    /// requests will be made to this endpoint
    ///
    /// # Examples
    /// ``` no_run
    /// # use ibm_watson::{
    /// #     auth::IamAuthenticator,
    /// #     tts::{voices::WatsonVoice, TextToSpeech},
    /// # };
    /// # async fn foo()-> Result<(), Box<dyn std::error::Error>> {
    /// let auth = IamAuthenticator::new("api_key").await?;
    /// let tts = TextToSpeech::new(&auth, "service_url");
    /// let voice = tts.get_voice(WatsonVoice::EnGbCharlotteV3, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`IamAuthenticator`]: super::auth::IamAuthenticator
    pub fn new(authenticator: &'a IamAuthenticator, service_url: &'a str) -> Self {
        let client = ClientBuilder::new();
        let default_headers = Self::default_headers(authenticator.token_response().access_token());
        let client = client.default_headers(default_headers);

        #[cfg(feature = "http2")]
        let client = client.http2_prior_knowledge();

        let client = client.build().unwrap();

        Self {
            service_url,
            voice: WatsonVoice::default(),
            client,
        }
    }

    /// Change the default voice to use for Text To Speech requests
    ///
    /// # Parameters
    ///
    /// * `voice` - Use this [`voice`] in place of the [`default`] one
    ///
    /// # Examples
    /// ``` no_run
    /// # use ibm_watson::{
    /// #     auth::IamAuthenticator,
    /// #     tts::{voices::WatsonVoice, TextToSpeech},
    /// # };
    /// # async fn foo()-> Result<(), Box<dyn std::error::Error>> {
    /// # let auth = IamAuthenticator::new("api_key").await?;
    /// let mut tts = TextToSpeech::new(&auth, "service_url");
    /// tts.set_voice(WatsonVoice::EnGbCharlotteV3);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`voice`]: self::voices::WatsonVoice
    /// [`default`]: self::voices::WatsonVoice::EnUsMichaelV3
    pub fn set_voice(&mut self, voice: WatsonVoice) {
        self.voice = voice;
    }

    pub(crate) fn get_client(&self) -> Client {
        self.client.clone()
    }

    fn default_headers(token: &str) -> HeaderMap<HeaderValue> {
        let mut headers = HeaderMap::new();
        let mut auth_value = HeaderValue::from_str(&format!("Bearer {}", token)).unwrap();
        auth_value.set_sensitive(true);
        headers.insert(AUTHORIZATION, auth_value);
        headers
    }
}
