use thiserror::Error;

#[derive(Error, Debug)]
pub enum PronunciationError {
    #[error("The requested resource has not been modified since the time specified by the If-Modified-Since header, as documented in the HTTP specification")]
    NotModified304,
    #[error("The request specified an Accept header with an incompatible content type.")]
    NotAcceptable406,
    #[error("A required input parameter is null or a specified input parameter or header value is invalid or not supported")]
    BadRequest400,
    #[error("The specified customization_id: {0} is invalid for the requesting credentials")]
    Unuathorised401(String),
    #[error("The specified voice does not exist")]
    NotFound404,
    #[error("The request specified an unacceptable media type.")]
    UnsupportedMediaType415,
    #[error("The service experienced an internal error.")]
    InternalServerError500,
    #[error("The service is currently unavailable.")]
    ServiceUnavailable503,
    #[error("{0}")]
    ConnectionError(String),
}
