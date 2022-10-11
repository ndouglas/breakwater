use crate::astronomy::satellite_system::error::Error as SatelliteSystemError;

/// Satellite systems errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Satellite System.
  SatelliteSystemError(SatelliteSystemError),
  /// Unknown
  UnknownError,
}

impl From<SatelliteSystemError> for Error {
  #[named]
  fn from(error: SatelliteSystemError) -> Self {
    Error::SatelliteSystemError(error)
  }
}
