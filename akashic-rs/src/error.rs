use thiserror::Error;

#[derive(Debug, Error)]
pub enum  AkashicError{
    #[error("Illegal text alignment string: {0}")]
    IllegalTextAlignmentString(String)
}