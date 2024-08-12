use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommemorateError {
    #[error("Failed to read file: {0}")]
    FileReadError(#[from] std::io::Error),

    #[error("Failed to parse YAML: {0}")]
    YamlParseError(#[from] serde_yaml::Error),

    #[error("Failed to encrypt data: {0}")]
    EncryptionError(String),

    #[error("Failed to decrypt data: {0}")]
    DecryptionError(String),

    #[error("Invalid password: {0}")]
    InvalidPassword(String),

    #[error("Invalid time format: {0}")]
    InvalidTimeFormat(String),

    #[error("Invalid timezone: {0}")]
    InvalidTimezone(String),

    #[error("Event not found: {0}")]
    EventNotFound(String),

    #[error("Failed to create directory: {0}")]
    DirectoryCreationError(String),

    #[error("Ah, maybe this isn't the place for memories, let's look somewhere else")]
    MemoryInvalid(String),

    #[error("Oops! We couldn't find that precious memory. Would you like to create a new one?")]
    MemoryNotFound,

    #[error("It seems we can't access this memory right now. Perhaps it's time to find a safer place for it?")]
    MemoryAccessError,

    #[error("Hmm, that password doesn't seem quite right. Take a deep breath and try to recall that special moment.")]
    IncorrectPassword,
}

pub type CommemorateResult<T> = Result<T, CommemorateError>;
