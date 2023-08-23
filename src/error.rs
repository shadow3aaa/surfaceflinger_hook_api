use std::io;

use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Got an io error: {source:?}")]
    Io {
        #[from]
        source: io::Error,
    },
    #[error("Got an error when parsing named pipe")]
    NamedPipe,
    #[error("Got an error: {0}")]
    Other(&'static str),
}
