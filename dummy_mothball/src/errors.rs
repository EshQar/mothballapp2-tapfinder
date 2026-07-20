use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Invalid syntax: {0}")]
    SyntaxError(String),

    #[error("Invalid name: {0}")]
    NameError(String),

    #[error("Invalid type: {0}")]
    TypeError(String),

    #[error("Invalid value: {0}")]
    ValueError(String),

    #[error("Interrupted: {0}")]
    InterruptedError(String),
}
