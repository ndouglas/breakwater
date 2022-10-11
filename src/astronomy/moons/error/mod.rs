use crate::astronomy::moon::error::Error as MoonError;

/// Moon-related errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Moon Error.
  MoonError(MoonError),
}

impl From<MoonError> for Error {
  #[named]
  fn from(error: MoonError) -> Self {
    Error::MoonError(error)
  }
}
