use thiserror::Error;

#[derive(Error, Debug)]
pub enum ListPromptsError {
    /// There was an error establishing the connection
    #[error("{0}")]
    ConnectionError(String),
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
    #[error("{0}")]
    ConnectionError(String),
    #[error("A required input parameter is null or a specified input parameter or header value is invalid or not supported")]
    /// A required input parameter is null or a specified input parameter or header value is invalid or not supported
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
}
