use thiserror::Error;
#[derive(Error, Debug)]
pub enum DeleteLabeledDataError {
    /// There was an error establishing the connection
    #[error("{0}")]
    ConnectionError(String),
    /// The request did not pass a customer ID
    #[error("The request did not pass a customer ID")]
    BadRequest400,
    #[error("The service is currently unavailable")]
    /// The service is currently unavailable
    ServiceUnavailable503,
    #[error("The service experienced an internal error")]
    /// The service experienced an internal error
    InternalServerError500,
}
