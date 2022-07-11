use hyper::{client::HttpConnector, Client};
use hyper_rustls::HttpsConnector;

use crate::auth::IamAuthenticator;

use self::voices::WatsonVoice;

pub mod customisations;
pub mod pronunciation;
pub mod synthesis;
pub mod voices;

pub struct TextToSpeech<'a> {
    access_token: &'a str,
    service_url: &'a str,
    voice: WatsonVoice,
    client: Client<HttpsConnector<HttpConnector>>,
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
        let ac = authenticator.token_response();
        let access_token = ac.access_token();
        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http1()
            .build();
        let client = Client::builder().build(https);
        Self {
            access_token,
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

    pub(crate) fn get_client(&self) -> Client<HttpsConnector<HttpConnector>> {
        self.client.clone()
    }
}
