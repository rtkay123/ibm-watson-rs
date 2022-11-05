use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client, ClientBuilder,
};

use crate::auth::IamAuthenticator;

pub mod models;

/// Creates a client used to send requests to your Text To Speech endpoint
pub struct SpeechToText<'a> {
    service_url: &'a str,
    client: Client,
}

impl<'a> SpeechToText<'a> {
    pub(crate) fn get_client(&self) -> Client {
        self.client.clone()
    }

    pub fn new(authenticator: &'a IamAuthenticator, service_url: &'a str) -> Self {
        let client = ClientBuilder::new();
        let default_headers = Self::default_headers(authenticator.token_response().access_token());
        let client = client.default_headers(default_headers);

        #[cfg(feature = "http2")]
        let client = ClientBuilder::use_rustls_tls(client);

        #[cfg(feature = "http2")]
        let client = client.http2_prior_knowledge();

        let client = client.build().unwrap();

        Self {
            service_url,
            client,
        }
    }

    fn default_headers(token: &str) -> HeaderMap<HeaderValue> {
        let mut headers = HeaderMap::new();
        let mut auth_value = HeaderValue::from_str(&format!("Bearer {}", token)).unwrap();
        auth_value.set_sensitive(true);
        headers.insert(AUTHORIZATION, auth_value);
        headers
    }
}
