use thiserror::Error;

#[derive(Error, Debug)]
pub enum OrkError {
    #[error("DB error.")]
    DbError(#[from] sled::Error),

    #[error("String conversion from UTF8 error")]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error("Soemthing went wrong running a state machine.")]
    StateMachineError(#[from] shakemyleg::SML_Error),
}

pub type OrkResult<T> = Result<T, OrkError>;
