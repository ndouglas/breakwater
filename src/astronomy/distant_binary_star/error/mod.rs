use crate::astronomy::planetary_system::error::Error as PlanetarySystemError;

/// Star system errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Planetary System Error
  PlanetarySystemError(PlanetarySystemError),
}

impl From<PlanetarySystemError> for Error {
  #[named]
  fn from(error: PlanetarySystemError) -> Self {
    Error::PlanetarySystemError(error)
  }
}
