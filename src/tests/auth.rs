use hyper::{header::CONTENT_TYPE, Body, Client, Method, Request};
use url::Url;

#[tokio::test]
async fn http1_iam_key() {
    let me = "api_key";
    let url = Url::parse("https://iam.cloud.ibm.com/identity/token").unwrap();
    let req = Request::builder()
        .uri(url.to_string())
        .method(Method::POST)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(Body::from(format!(
            "grant_type=urn:ibm:params:oauth:grant-type:apikey&apikey={}",
            me
        )))
        .unwrap();
    let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_only()
        .enable_http1()
        .build();
    let client = Client::builder().build(https);
    let c = client.request(req).await.unwrap();
    assert_eq!(c.status(), 400);
}
