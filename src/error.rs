use std::{error, fmt, io, result};

#[derive(Debug)]
pub enum Error {
    Custom(String),
    Io(io::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Custom(..) => "syntax error",
            Error::Io(ref error) => error::Error::description(error),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Custom(ref err) => write!(fmt, "error: {}", err),
            Error::Io(ref error) => fmt::Display::fmt(error, fmt),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::Io(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        use serde_json::error::Category;
        match err.classify() {
            Category::Io => Error::Io(err.into()),
            Category::Syntax | Category::Data | Category::Eof => {
                Error::Custom(String::from(format!("{}", err)))
            }
        }
    }
}

impl serde::ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}
