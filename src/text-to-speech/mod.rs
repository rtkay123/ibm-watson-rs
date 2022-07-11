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

    pub fn set_voice(&mut self, voice: WatsonVoice) {
        self.voice = voice;
    }

    pub(crate) fn get_client(&self) -> Client<HttpsConnector<HttpConnector>> {
        self.client.clone()
    }
}
