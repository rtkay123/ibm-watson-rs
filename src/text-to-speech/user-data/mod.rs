use reqwest::{Method, Request, StatusCode, Url, Version};
pub mod errors;

use self::errors::DeleteLabeledDataError;

use super::TextToSpeech;

impl TextToSpeech<'_> {
    /// Deletes all data that is associated with a specified customer ID. The method deletes all data for the customer ID, regardless of the method by which the information was added. The method has no effect if no data is associated with the customer ID. You must issue the request with credentials for the same instance of the service that was used to associate the customer ID with the data
    ///
    /// # Parameters
    /// * `customer_id` - The customer ID for which all data is to be deleted
    /// # Example
    /// ``` no_run
    /// # use ibm_watson::{
    /// #     auth::IamAuthenticator,
    /// #     tts::{voices::WatsonVoice, TextToSpeech},
    /// # };
    /// # async fn foo()-> Result<(), Box<dyn std::error::Error>> {
    /// # let auth = IamAuthenticator::new("api_key").await?;
    /// # let tts = TextToSpeech::new(&auth, "service_url");
    /// if tts.delete_labeled_data("me-id").await.is_ok() {
    ///     println!("user data deleted");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_labeled_data(
        &self,
        customer_id: impl AsRef<str>,
    ) -> Result<(), DeleteLabeledDataError> {
        let mut url = Url::parse(self.service_url).unwrap();
        url.set_path(&format!("v1/user_data/{}", customer_id.as_ref()));
        let mut req = Request::new(Method::DELETE, url);

        if cfg!(feature = "http2") {
            *req.version_mut() = Version::HTTP_2;
        }

        let client = self.get_client();
        let response = client
            .execute(req)
            .await
            .map_err(|e| DeleteLabeledDataError::ConnectionError(e.to_string()))?;
        match response.status() {
            StatusCode::NO_CONTENT => Ok(()),
            StatusCode::BAD_REQUEST => Err(DeleteLabeledDataError::BadRequest400),
            StatusCode::INTERNAL_SERVER_ERROR => {
                Err(DeleteLabeledDataError::InternalServerError500)
            }
            StatusCode::SERVICE_UNAVAILABLE => Err(DeleteLabeledDataError::ServiceUnavailable503),
            _ => {
                unreachable!()
            }
        }
    }
}
