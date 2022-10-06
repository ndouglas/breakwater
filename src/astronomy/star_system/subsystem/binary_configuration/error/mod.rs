use std::convert::From;

use crate::astronomy::star::error::Error as StarError;

/// Binary star subsystem configuration errors.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Star Error.
  StarError(StarError),
  /// Lower than MINIMUM_BINARY_STAR_SEPARATION.
  BinaryStarsTooCloseForComfort,
}

impl From<StarError> for Error {
  #[named]
  fn from(error: StarError) -> Self {
    Error::StarError(error)
  }
}
