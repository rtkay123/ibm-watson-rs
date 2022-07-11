use thiserror::Error;
#[derive(Error, Debug)]
pub enum CreateModelError {
    #[error("")]
    ConnectionError(String),
}
