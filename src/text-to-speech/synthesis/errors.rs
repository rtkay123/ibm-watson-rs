use thiserror::Error;

#[derive(Error, Debug)]
pub enum SynthesisError {
    #[error("A required input parameter is null or a specified input parameter or header value is invalid")]
    BadRequest400,
    #[error("The specified voice does not exist")]
    NotFound404,
    #[error("The request specified an incompatible content type or failed to specify a required sampling rate")]
    NotAcceptable406,
    #[error("The request specified an unacceptable media type.")]
    UnsupportedMediaType415,
    #[error("The service experienced an internal error.")]
    InternalServerError500,
    #[error("The service is currently unavailable.")]
    ServiceUnavailable500,
    #[error("Connection error: {0}")]
    ConnectionError(String),
}
