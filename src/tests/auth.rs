use reqwest::{
    header::{HeaderValue, CONTENT_TYPE},
    Body, Client, Method, Request, Url,
};

#[tokio::test]
async fn http1_iam_key() {
    let me = "api_key";
    let url = Url::parse("https://iam.cloud.ibm.com/identity/token").unwrap();
    let mut req = Request::new(Method::POST, url);
    let headers = req.headers_mut();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    *req.body_mut() = Some(Body::from(format!(
        "grant_type=urn:ibm:params:oauth:grant-type:apikey&apikey={}",
        me
    )));
    let client = Client::new();
    let c = client.execute(req).await.unwrap();
    assert_eq!(c.status(), 400);
}
