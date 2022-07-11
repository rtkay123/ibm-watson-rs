use thiserror::Error;

#[derive(Error, Debug)]
/// Errors that may be returned in speech synethesis
pub enum SynthesisError {
    #[error("A required input parameter is null or a specified input parameter or header value is invalid")]
    ///  A required input parameter is null or a specified input parameter or header value is invalid. For example, prompt_id '{prompt_id}' not found in custom model '{customization_id}' if you attempt to use a nonexistent or deleted custom prompt
    BadRequest400,
    #[error("The specified voice does not exist")]
    /// The specified voice does not exist or, for IBM Cloud Pak for Data, the voice parameter was not specified but the default voice is not installed. The message is Model '{voice}' not found
    NotFound404,
    #[error("The request specified an incompatible content type or failed to specify a required sampling rate")]
    /// The request specified an incompatible content type or failed to specify a required sampling rate
    NotAcceptable406,
    #[error("The request specified an unacceptable media type.")]
    /// The request specified an unacceptable media type
    UnsupportedMediaType415,
    #[error("The service experienced an internal error.")]
    /// The service experienced an internal error
    InternalServerError500,
    #[error("The service is currently unavailable.")]
    /// The service is currently unavailable
    ServiceUnavailable500,
    #[error("Connection error: {0}")]
    /// Some other error occurred in the request
    ConnectionError(String),
}
