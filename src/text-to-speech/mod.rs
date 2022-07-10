use crate::auth::IamAuthenticator;

use self::voices::WatsonVoice;

#[path = "custom-models/mod.rs"]
pub mod custom_models;
#[path = "custom-prompts/mod.rs"]
pub mod custom_prompts;
#[path = "custom-words/mod.rs"]
pub mod custom_words;
pub mod voices;

pub struct TextToSpeech<'a> {
    access_token: &'a str,
    service_url: String,
    voice: WatsonVoice,
}

impl<'a> TextToSpeech<'a> {
    pub fn new(authenticator: &'a IamAuthenticator, service_url: impl Into<String>) -> Self {
        let ac = authenticator.token_response();
        let access_token = ac.access_token();
        let service_url = service_url.into();
        Self {
            access_token,
            service_url,
            voice: WatsonVoice::default(),
        }
    }
}
