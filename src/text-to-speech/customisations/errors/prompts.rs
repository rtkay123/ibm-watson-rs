use thiserror::Error;

#[derive(Error, Debug)]
pub enum ListPromptsError {
    /// There was an error establishing the connection
    #[error("There was an error establishing the connection")]
    ConnectionError(#[from] reqwest::Error),
    #[error("A required input parameter is null or a specified input parameter or header value is invalid or not supported")]
    /// A required input parameter is null or a specified input parameter or header value is invalid or not supported
    BadRequest400,
    #[error("The service is currently unavailable")]
    /// The service is currently unavailable
    ServiceUnavailable503,
    /// The service experienced an internal error
    #[error("The service experienced an internal error")]
    InternalServerError500,
}

#[derive(Error, Debug)]
pub enum AddPromptError {
    /// There was an error establishing the connection
    #[error("There was an error establishing the connection")]
    ConnectionError(#[from] reqwest::Error),
    #[error("A required input parameter is null or a specified input parameter or header value is invalid or not supported")]
    /// The request failed: Possible failure causes include:
    ///
    /// * The prompt name exceeds the 49-character limit or includes characters that are not alphanumeric or underscores
    /// * The audio has a media type other than audio/wav or a sampling rate of less than 16 kHz
    /// * The audio is longer than 30 seconds
    /// * The service cannot process the audio for any reason (for example, the audio is corrupt)
    /// * The service cannot align the text and the audio because differences between the two are too great
    /// * The request attempts to add a prompt to a custom model that is defined for a language other than US English
    /// * Invalid service credentials were passed with the request
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

#[derive(Error, Debug)]
pub enum GetPromptError {
    /// There was an error establishing the connection
    #[error("There was an error establishing the connection")]
    ConnectionError(#[from] reqwest::Error),
    #[error("A required input parameter is null or a specified input parameter or header value is invalid or not supported")]
    /// A required input parameter is null or a specified input parameter or header value is invalid or not supported
    BadRequest400(String),
    #[error("The service is currently unavailable")]
    /// The service is currently unavailable
    ServiceUnavailable503,
    #[error("The service experienced an internal error")]
    /// The service experienced an internal error
    InternalServerError500,
    /// "The specified customisation_id is invalid for the requesting credentials
    #[error("The specified customisation_id {0} is invalid for the requesting credentials")]
    Unauthorised401(String),
}

#[derive(Error, Debug)]
pub enum DeletePromptError {
    /// There was an error establishing the connection
    #[error("There was an error establishing the connection")]
    ConnectionError(#[from] reqwest::Error),
    /// A required input parameter is null or a specified input parameter or header value is invalid or not supported
    #[error("A required input parameter is null or a specified input parameter or header value is invalid or not supported")]
    BadRequest400(String),
    #[error("The service is currently unavailable")]
    /// The service is currently unavailable
    ServiceUnavailable503,
    #[error("The service experienced an internal error")]
    /// The service experienced an internal error
    InternalServerError500,
    #[error("The specified customisation_id {0} or prompt_id {1} is invalid for the requesting credentials")]
    /// The specified customisation_id is invalid for the requesting credentials
    Unauthorised401(String, String),
}
