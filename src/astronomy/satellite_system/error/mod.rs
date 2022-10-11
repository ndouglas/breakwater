use crate::astronomy::moon::error::Error as MoonError;
use crate::astronomy::moons::error::Error as MoonsError;
use crate::astronomy::planet::error::Error as PlanetError;

/// Moon-related errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Moon Error.
  MoonError(MoonError),
  /// Moons Error.
  MoonsError(MoonsError),
  /// Planet Error.
  PlanetError(PlanetError),
  /// Unknown.
  UnknownError,
}

impl From<MoonError> for Error {
  #[named]
  fn from(error: MoonError) -> Self {
    Error::MoonError(error)
  }
}

impl From<MoonsError> for Error {
  #[named]
  fn from(error: MoonsError) -> Self {
    Error::MoonsError(error)
  }
}

impl From<PlanetError> for Error {
  #[named]
  fn from(error: PlanetError) -> Self {
    Error::PlanetError(error)
  }
}
