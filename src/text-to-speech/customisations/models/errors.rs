use thiserror::Error;
#[derive(Error, Debug)]
pub enum CreateModelError {
    #[error("{0}")]
    ConnectionError(String),
    #[error("A required input parameter is null or a specified input parameter or header value is invalid or not supported")]
    BadRequest400,
    #[error("The service is currently unavailable")]
    ServiceUnavailable503,
    #[error("The service experienced an internal error")]
    InternalServerError500,
}

#[derive(Error, Debug)]
pub enum ListModelError {
    #[error("{0}")]
    ConnectionError(String),
    #[error("A required input parameter is null or a specified input parameter or header value is invalid or not supported")]
    BadRequest400,
    #[error("The service is currently unavailable")]
    ServiceUnavailable503,
    #[error("The service experienced an internal error")]
    InternalServerError500,
}

#[derive(Error, Debug)]
pub enum UpdateModelError {
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
pub enum GetModelError {
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
    #[error(" The requested resource has not been modified since the time specified by the If-Modified-Since header, as documented in the HTTP specification.")]
    NotModified304,
}

#[derive(Error, Debug)]
pub enum DeleteModelError {
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
