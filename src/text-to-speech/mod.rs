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
}

impl<'a> TextToSpeech<'a> {
    pub fn new(authenticator: &'a IamAuthenticator, service_url: &'a str) -> Self {
        let ac = authenticator.token_response();
        let access_token = ac.access_token();

        Self {
            access_token,
            service_url,
            voice: WatsonVoice::default(),
        }
    }

    pub fn set_voice(&mut self, voice: WatsonVoice) {
        self.voice = voice;
    }
}
