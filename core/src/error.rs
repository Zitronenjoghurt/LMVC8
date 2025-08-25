use thiserror::Error;

pub type LMVC8Result<T> = Result<T, LMVC8Error>;

#[derive(Debug, Error)]
pub enum LMVC8Error {
    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),
    #[error("The ROM size is too large")]
    ROMSizeExceeded,
}
