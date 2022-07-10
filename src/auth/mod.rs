mod errors;
use hyper::{body::Buf, header::CONTENT_TYPE, Body, Client, Method, Request, StatusCode};
use serde::{Deserialize, Serialize};
use url::Url;

pub use errors::AuthenticationError;

const AUTH_URL: &str = "https://iam.cloud.ibm.com/identity/token";
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TokenResponse {
    #[serde(rename = "access_token")]
    access_token: String,
    #[serde(rename = "refresh_token")]
    refresh_token: String,
    #[serde(rename = "delegated_refresh_token")]
    delegated_refresh_token: Option<String>,
    #[serde(rename = "token_type")]
    token_type: String,
    #[serde(rename = "expires_in")]
    expires_in: i64,
    expiration: i64,
    scope: Option<String>,
}

impl TokenResponse {
    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    pub fn refresh_token(&self) -> &str {
        &self.refresh_token
    }

    pub fn token_type(&self) -> &str {
        &self.token_type
    }

    pub fn expires_in(&self) -> i64 {
        self.expires_in
    }

    pub fn expiration(&self) -> i64 {
        self.expiration
    }

    pub fn scope(&self) -> Option<&String> {
        self.scope.as_ref()
    }

    pub fn delegated_refresh_token(&self) -> Option<&String> {
        self.delegated_refresh_token.as_ref()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IamAuthenticator {
    access_token: TokenResponse,
}

impl IamAuthenticator {
    pub async fn new(api_key: impl AsRef<str>) -> Result<Self, AuthenticationError> {
        let url = Url::parse(AUTH_URL).unwrap();
        let req = Request::builder()
            .uri(url.to_string())
            .method(Method::POST)
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(Body::from(format!(
                "grant_type=urn:ibm:params:oauth:grant-type:apikey&apikey={}",
                api_key.as_ref()
            )))
            .unwrap();
        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http1()
            .build();
        let client = Client::builder().build(https);
        let response = client.request(req).await.unwrap();
        match response.status() {
            StatusCode::OK => {
                // asynchronously aggregate the chunks of the body
                let body = hyper::body::aggregate(response).await.unwrap();
                // try to parse as json with serde_json
                let access_token: TokenResponse = serde_json::from_reader(body.reader()).unwrap();
                Ok(Self { access_token })
            }
            StatusCode::BAD_REQUEST => Err(AuthenticationError::ParameterValidationFailed),
            _ => unreachable!(),
        }
    }
}
