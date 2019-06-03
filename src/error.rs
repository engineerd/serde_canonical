use std::{error, fmt, io, result};

#[derive(Debug)]
pub enum Error {
    Syntax(String, usize, usize),
    Io(io::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Syntax(..) => "syntax error",
            Error::Io(ref error) => error::Error::description(error),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Syntax(ref code, line, col) => {
                write!(fmt, "{} at line {} column {}", code, line, col)
            }
            Error::Io(ref error) => fmt::Display::fmt(error, fmt),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::Io(error)
    }
}

impl serde::ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Syntax(msg.to_string(), 0, 0)
    }
}
