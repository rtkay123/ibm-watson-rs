use thiserror::Error;
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum DeleteLabeledDataError {
    /// There was an error establishing the connection
    #[error("There was an error establishing the connection")]
    ConnectionError(#[from] reqwest::Error),
    /// The request did not pass a customer ID
    #[error("The request did not pass a customer ID")]
    BadRequest400,
    #[error("The service is currently unavailable")]
    /// The service is currently unavailable
    ServiceUnavailable503,
    #[error("The service experienced an internal error")]
    /// The service experienced an internal error
    InternalServerError500,
    /// The response that the server sends back
    #[error("{0}")]
    UnmappedResponse(u16),
}
