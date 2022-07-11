use thiserror::Error;

#[derive(Error, Debug)]
/// Errors that may be returned in creating an IAM access token for a user or service ID using an API key
pub enum AuthenticationError {
    #[error("Parameter validation failed. Required parameters are missing or parameter values are invalid.")]
    /// Parameter validation failed. Response if required parameters are missing or if parameter values are invalid.
    ParameterValidationFailed,
    #[error("The incoming request did not contain a valid authentication information")]
    /// The incoming request did not contain a valid authentication information.
    InvalidAPIKey,
    #[error(
        "The incoming request is valid but the user is not allowed to perform the requested action."
    )]
    /// The incoming request is valid but the user is not allowed to perform the requested action.
    NotAllowed,
    #[error("Internal Server error. Response if unexpected error situation happened.")]
    /// Internal Server error. Response if unexpected error situation happened.
    ServerError,
    #[error("{0}")]
    /// Network Error
    ConnectionError(String),
}
