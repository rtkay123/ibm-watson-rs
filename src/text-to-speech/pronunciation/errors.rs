use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
/// Errors that may be returned in pronunciation requests
pub enum PronunciationError {
    /// The requested resource has not been modified since the time specified by the If-Modified-Since header, as documented in the HTTP specification
    #[error("The requested resource has not been modified since the time specified by the If-Modified-Since header, as documented in the HTTP specification")]
    NotModified304,
    #[error("The request specified an Accept header with an incompatible content type.")]
    /// The request specified an Accept header with an incompatible content type
    NotAcceptable406,
    #[error("A required input parameter is null or a specified input parameter or header value is invalid or not supported")]
    /// A required input parameter is null or a specified input parameter or header value is invalid or not supported
    BadRequest400,
    #[error("The specified customisation_id: {0} is invalid for the requesting credentials")]
    /// The specified customisation_id is invalid for the requesting credentials
    Unuathorised401(String),
    #[error("The specified voice does not exist")]
    /// The specified voice does not exist or, for IBM Cloud Pak for Data, the voice parameter was not specified but the default voice is not installed. The message is Model '{voice}' not found
    NotFound404,
    //    #[error("The request specified an unacceptable media type.")]
    //    UnsupportedMediaType415,
    #[error("The service experienced an internal error.")]
    /// The service experienced an internal error.
    InternalServerError500,
    /// The service is currently unavailable
    #[error("The service is currently unavailable.")]
    ServiceUnavailable503,
    /// There was an error making the request
    #[error("There was an error establishing the connection")]
    ConnectionError(#[from] reqwest::Error),
    /// The response code the server returnes
    #[error("{0}")]
    UnmappedResponse(u16),
}
