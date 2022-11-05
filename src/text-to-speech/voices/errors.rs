use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
/// Errors that may be returned when listing [`Watson Voices`]
///
/// [`Watson Voices`]: crate::tts::voices::WatsonVoice
pub enum ListVoicesError {
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
    /// There was an error making the request
    #[error("There was an error establishing the connection")]
    ConnectionError(#[from] reqwest::Error),
    /// The response that the server sends back
    #[error("{0}")]
    UnmappedResponse(u16),
}

#[derive(Error, Debug)]
#[non_exhaustive]
/// Errors that may be returned when getting information about a particular [`WatsonVoice`]
///
/// [`WatsonVoice`]: crate::tts::voices::WatsonVoice
pub enum GetVoiceError {
    #[error("The requested resource has not been modified since the time specified by the If-Modified-Since header, as documented in the HTTP specification")]
    /// The requested resource has not been modified since the time specified by the If-Modified-Since header, as documented in the HTTP specification
    NotModified304,
    #[error("A required input parameter is null or a specified input parameter or header value is invalid or not supported. Please check your customisation id")]
    /// A required input parameter is null or a specified input parameter or header value is invalid or not supported. Please check your customisation id
    BadRequest400,
    #[error("The specified customisation_id {0} is invalid for the requesting credentials")]
    /// The specified customisation_id is invalid for the requesting credentials
    Unauthorised401(String),
    #[error("The request specified an Accept header with an incompatible content type.")]
    /// The request specified an Accept header with an incompatible content type
    NotAcceptable406,
    #[error("The request specified an unacceptable media type.")]
    /// The request specified an unacceptable media type
    UnsupportedMediaType415,
    #[error("The service experienced an internal error.")]
    /// The service experienced an internal error
    InternalServerError500,
    #[error("The service is currently unavailable.")]
    /// The service is currently unavailable
    ServiceUnavailable503,
    /// There was an error making the request
    #[error("There was an error establishing the connection")]
    ConnectionError(#[from] reqwest::Error),
    /// The response that the server sends back
    #[error("{0}")]
    UnmappedResponse(u16),
}
