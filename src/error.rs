use thiserror::Error;

#[derive(Error, Debug)]
pub enum OrkError {
    #[error("DB error.")]
    DbError(#[from] sled::Error),

    #[error("String conversion from UTF8 error")]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error("Something went wrong running a state machine.")]
    StateMachineError(#[from] shakemyleg::SML_Error),

    #[error("IO error")]
    IOError(#[from] std::io::Error),

    #[error("Config file path doesn't point to a file!")]
    ConfigNotAFile,

    #[error("Failed to (de)serialize something")]
    DeserError(#[from] serde_yaml::Error),
}

pub type OrkResult<T> = Result<T, OrkError>;
