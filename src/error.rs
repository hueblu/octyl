#[derive(Debug)]
pub enum Error {
    Crossterm(crossterm::ErrorKind),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<crossterm::ErrorKind> for Error {
    fn from(error: crossterm::ErrorKind) -> Self {
        Error::Crossterm(error)
    }
}
