use crate::astronomy::close_binary_star::error::Error as CloseBinaryStarError;
use crate::astronomy::star::error::Error as StarError;

/// Moons errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Close Binary Star Error
  CloseBinaryStarError(CloseBinaryStarError),
  /// Star Error
  StarError(StarError),
}

impl From<StarError> for Error {
  #[named]
  fn from(error: StarError) -> Self {
    Error::StarError(error)
  }
}

impl From<CloseBinaryStarError> for Error {
  #[named]
  fn from(error: CloseBinaryStarError) -> Self {
    Error::CloseBinaryStarError(error)
  }
}
