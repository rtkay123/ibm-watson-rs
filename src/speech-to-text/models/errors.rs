use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
/// Errors that may be returned when listing [`Watson Models`]
///
/// [`Watson Models`]: crate::stt::models::Model
pub enum ListModelsError {
    #[error("The request specified an Accept header with an incompatible content type.")]
    /// The request specified an Accept header with an incompatible content type
    NotAcceptable406, //406
    #[error("The request specified an unacceptable media type.")]
    /// The request specified an unacceptable media type
    UnsupportedMediaType415, // 415
    #[error("The service experienced an internal error.")]
    /// The service experienced an internal error
    InternalServerError500, // 500
    #[error("The service is currently unavailable.")]
    /// The service is currently unavailable
    ServiceUnavailable503, // 503
    #[error("{0}")]
    /// There was an error making the request
    ConnectionError(#[from] reqwest::Error),

    #[error("{0}")]
    /// There was an error making the request
    UnmappedResponse(u16),
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum GetModelError {
    #[error("The specified model_id {0} was not found")]
    /// The specified `model_id` was not found
    NotFound404(String), //404
    #[error("The request specified an Accept header with an incompatible content type.")]
    /// The request specified an Accept header with an incompatible content type
    NotAcceptable406, //406
    #[error("The request specified an unacceptable media type.")]
    /// The request specified an unacceptable media type
    UnsupportedMediaType415, // 415
    #[error("The service experienced an internal error.")]
    /// The service experienced an internal error
    InternalServerError500, // 500
    #[error("The service is currently unavailable.")]
    /// The service is currently unavailable
    ServiceUnavailable503, // 503
    #[error("{0}")]
    /// There was an error making the request
    ConnectionError(#[from] reqwest::Error),
    #[error("{0}")]
    /// There was an error making the request
    UnmappedResponse(u16),
}
