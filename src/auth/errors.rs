use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthenticationError {
    #[error("Parameter validation failed. Response if required parameters are missing or if parameter values are invalid.")]
    ParameterValidationFailed,
    #[error("The incoming request did not contain a valid authentication information")]
    InvalidAPIKey,
    #[error(
        "The incoming request is valid but the user is not allowed to perform the requested action."
    )]
    NotAllowed,
    #[error("Internal Server error. Response if unexpected error situation happened.")]
    ServerError,
    #[error("{0}")]
    ConnectionError(String),
}
