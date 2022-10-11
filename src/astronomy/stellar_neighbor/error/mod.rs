use crate::astronomy::star_system::error::Error as StarSystemError;

/// Star system errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Star System Error.
  StarSystemError(StarSystemError),
}

impl From<StarSystemError> for Error {
  #[named]
  fn from(error: StarSystemError) -> Self {
    Error::StarSystemError(error)
  }
}
