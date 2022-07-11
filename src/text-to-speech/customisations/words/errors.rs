use thiserror::Error;
#[derive(Error, Debug)]
pub enum AddWordError {
    #[error("A required input parameter is null or a specified input parameter or header value is invalid or not supported")]
    BadRequest400,
    #[error("The specified customization_id {0} is invalid for the requesting credentials")]
    Unauthorised401(String),
    #[error("The service experienced an internal error")]
    InternalServerError500,
    #[error("The service is currently unavailable.")]
    ServiceUnavailable503,
    #[error("{0}")]
    ConnectionError(String),
}

#[derive(Error, Debug)]
pub enum ListWordsError {
    #[error("{0}")]
    ConnectionError(String),
    #[error("A required input parameter is null or a specified input parameter or header value is invalid or not supported")]
    BadRequest400,
    #[error("The service is currently unavailable")]
    ServiceUnavailable503,
    #[error("The service experienced an internal error")]
    InternalServerError500,
    #[error("The specified customization_id {0} is invalid for the requesting credentials")]
    Unauthorised401(String),
}

#[derive(Error, Debug)]
pub enum GetWordError {
    #[error("{0}")]
    ConnectionError(String),
    #[error("A required input parameter is null or a specified input parameter or header value is invalid or not supported")]
    BadRequest400,
    #[error("The service is currently unavailable")]
    ServiceUnavailable503,
    #[error("The service experienced an internal error")]
    InternalServerError500,
    #[error("The specified customization_id {0} is invalid for the requesting credentials")]
    Unauthorised401(String),
}

#[derive(Error, Debug)]
pub enum DeleteWordError {
    #[error("{0}")]
    ConnectionError(String),
    #[error("A required input parameter is null or a specified input parameter or header value is invalid or not supported")]
    BadRequest400(String),
    #[error("The service is currently unavailable")]
    ServiceUnavailable503,
    #[error("The service experienced an internal error")]
    InternalServerError500,
    #[error("The specified customization_id {0} is invalid for the requesting credentials")]
    Unauthorised401(String),
}
