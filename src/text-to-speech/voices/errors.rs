use thiserror::Error;

#[derive(Error, Debug)]
pub enum ListVoicesError {
    #[error("The request specified an Accept header with an incompatible content type.")]
    NotAcceptable, //406
    #[error("The request specified an unacceptable media type.")]
    UnsupportedMediaType, // 415
    #[error("The service experienced an internal error.")]
    InternalServerError, // 500
    #[error("The service is currently unavailable.")]
    ServiceUnavailable, // 503
    #[error("{0}")]
    ConnectionError(String),
}

#[derive(Error, Debug)]
pub enum GetVoiceError {
    #[error("The requested resource has not been modified since the time specified by the If-Modified-Since header, as documented in the HTTP specification")]
    NotModified,
    #[error("A required input parameter is null or a specified input parameter or header value is invalid or not supported. Please check your customisation id")]
    BadRequest,
    #[error("The specified customization_id {0} is invalid for the requesting credentials")]
    Unauthorised(String),
    #[error("The request specified an Accept header with an incompatible content type.")]
    NotAcceptable,
    #[error("The request specified an unacceptable media type.")]
    UnsupportedMediaType,
    #[error("The service experienced an internal error.")]
    InternalServerError,
    #[error("The service is currently unavailable.")]
    ServiceUnavailable,
    #[error("{0}")]
    ConnectionError(String),
}
