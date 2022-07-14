use thiserror::Error;
#[derive(Error, Debug)]
pub enum AddWordError {
    /// A required input parameter is null or a specified input parameter or header value is invalid or not supported
    #[error("A required input parameter is null or a specified input parameter or header value is invalid or not supported")]
    BadRequest400,
    /// The specified customisation_id is invalid for the requesting credentials
    #[error("The specified customisation_id {0} is invalid for the requesting credentials")]
    Unauthorised401(String),
    /// The service experienced an internal error
    #[error("The service experienced an internal error")]
    InternalServerError500,
    /// The service is currently unavailable
    #[error("The service is currently unavailable.")]
    ServiceUnavailable503,
    /// There was an error establishing the connection
    #[error("{0}")]
    ConnectionError(String),
}

#[derive(Error, Debug)]
pub enum ListWordsError {
    /// There was an error establishing the connection
    #[error("{0}")]
    ConnectionError(String),
    /// A required input parameter is null or a specified input parameter or header value is invalid or not supported
    #[error("A required input parameter is null or a specified input parameter or header value is invalid or not supported")]
    BadRequest400,
    /// The service is currently unavailable
    #[error("The service is currently unavailable")]
    ServiceUnavailable503,
    /// The service experienced an internal error
    #[error("The service experienced an internal error")]
    InternalServerError500,
    /// The specified customisation_id is invalid for the requesting credentials
    #[error("The specified customisation_id {0} is invalid for the requesting credentials")]
    Unauthorised401(String),
}

#[derive(Error, Debug)]
pub enum GetWordError {
    /// There was an error establishing the connection
    #[error("{0}")]
    ConnectionError(String),
    #[error("A required input parameter is null or a specified input parameter or header value is invalid or not supported")]
    BadRequest400,
    /// The service is currently unavailable
    #[error("The service is currently unavailable")]
    ServiceUnavailable503,
    #[error("The service experienced an internal error")]
    /// The service experienced an internal error
    InternalServerError500,
    #[error("The specified customisation_id {0} is invalid for the requesting credentials")]
    /// The specified customisation_id is invalid for the requesting credentials
    Unauthorised401(String),
}

#[derive(Error, Debug)]
pub enum DeleteWordError {
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
    #[error("The specified customisation_id {0} is invalid for the requesting credentials")]
    /// The specified customisation_id is invalid for the requesting credentials
    Unauthorised401(String),
}
