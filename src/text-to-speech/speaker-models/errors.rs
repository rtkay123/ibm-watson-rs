use thiserror::Error;
#[derive(Debug, Error)]
pub enum ListSpeakersError {
    #[error("The service experienced an internal error.")]
    /// The service experienced an internal error
    InternalServerError500, // 500
    #[error("The service is currently unavailable.")]
    /// The service is currently unavailable
    ServiceUnavailable503, // 503
    #[error("{0}")]
    /// There was an error making the request
    ConnectionError(String),
    /// The request failed. Possible failure causes include. Invalid service credentials were passed with the request
    #[error("The request failed. Possible failure causes include. Invalid service credentials were passed with the request")]
    BadRequest400,
}

#[derive(Debug, Error)]
pub enum CreateSpeakerError {
    #[error("")]
    BadRequest400,
    #[error("The service is currently unavailable")]
    /// The service is currently unavailable
    ServiceUnavailable503,
    /// The service experienced an internal error
    #[error("The service experienced an internal error")]
    InternalServerError500,
    /// There was an error reading the file
    #[error("There was an error reading the file: {0}")]
    FileReadError(String),
    /// The request passed an unacceptable media type with the Content-Type header. The header must pass a value of multipart/form-data
    #[error("The request passed an unacceptable media type with the Content-Type header. The header must pass a value of multipart/form-data")]
    UnsupportedMediaType415,
    /// The specified customisation_id is invalid for the requesting credentials
    #[error("The specified customisation_id  {0} is invalid for the requesting credentials")]
    Unauthorised401(String),
}

#[derive(Debug, Error)]
pub enum GetSpeakerError {
    /// There was an error establishing the connection
    #[error("{0}")]
    ConnectionError(String),
    #[error("A required input parameter is null or a specified input parameter or header value is invalid or not supported")]
    BadRequest400,
    #[error("The service is currently unavailable")]
    /// The service is currently unavailable
    ServiceUnavailable503,
    /// The service experienced an internal error
    #[error("The service experienced an internal error")]
    InternalServerError500,
    /// The specified speaker_id is invalid for the requesting credentials")]
    #[error("The specified speaker_id: {0} is invalid for the requesting credentials")]
    Unauthorised401(String),
    /// The requested resource has not been modified since the time specified by the If-Modified-Since header, as documented in the HTTP specification
    #[error("The requested resource has not been modified since the time specified by the If-Modified-Since header, as documented in the HTTP specification")]
    NotModified304,
}

#[derive(Error, Debug)]
pub enum DeleteSpeakerError {
    /// There was an error establishing the connection
    #[error("{0}")]
    ConnectionError(String),
    /// A required input parameter is null or a specified input parameter or header value is invalid or not supported
    #[error("A required input parameter is null or a specified input parameter or header value is invalid or not supported")]
    BadRequest400(String),
    #[error("The service is currently unavailable")]
    /// The service is currently unavailable
    ServiceUnavailable503,
    #[error("The service experienced an internal error")]
    /// The service experienced an internal error
    InternalServerError500,
    #[error("The specified speaker_id {0} is invalid for the requesting credentials")]
    /// The specified speaker_id is invalid for the requesting credentials
    Unauthorised401(String),
}
