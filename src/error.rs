#[derive(Debug)]
pub enum Error {
    Librespot(librespot::core::Error),
    Http(http::Error),
    Json(serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Json(error)
    }
}

impl From<librespot::core::Error> for Error {
    fn from(error: librespot::core::Error) -> Self {
        Error::Librespot(error)
    }
}

impl From<http::Error> for Error {
    fn from(error: http::Error) -> Self {
        Error::Http(error)
    }
}
