use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Credentials not found: recieved this error {0}")]
    Credentials(String),
}