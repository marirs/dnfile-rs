#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    RegexError(#[from] regex::Error),
    #[error("goblin error")]
    ParseError(#[from] goblin::error::Error),
    #[error("unsupported  binary format")]
    UnsupportedBinaryFormat(&'static str),
    #[error("unsupported  binary format")]
    Bincode(#[from] Box<bincode::ErrorKind>),
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("{0}")]
    UnresolvedRvaError(u32),
    #[error("{0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error("undefined stream")]
    UndefinedStream,
    #[error("not implemented")]
    NoiImplementedError,
}
