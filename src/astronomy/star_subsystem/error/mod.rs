use std::convert::From;

use crate::astronomy::distant_binary_star::error::Error as DistantBinaryStarError;
use crate::astronomy::planetary_system::error::Error as PlanetarySystemError;

/// Star system errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Wrap a Distant Binary Star error.
  DistantBinaryStarError(DistantBinaryStarError),
  /// Wrap a Planetary System error.
  PlanetarySystemError(PlanetarySystemError),
  /// Unknown
  UnknownError,
}

impl From<DistantBinaryStarError> for Error {
  #[named]
  fn from(error: DistantBinaryStarError) -> Self {
    Error::DistantBinaryStarError(error)
  }
}

impl From<PlanetarySystemError> for Error {
  #[named]
  fn from(error: PlanetarySystemError) -> Self {
    Error::PlanetarySystemError(error)
  }
}
