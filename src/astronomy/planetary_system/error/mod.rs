use crate::astronomy::host_star::error::Error as HostStarError;
use crate::astronomy::satellite_systems::error::Error as SatelliteSystemsError;

/// Star system errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Host Star
  HostStarError(HostStarError),
  /// Satellite Systems
  SatelliteSystemsError(SatelliteSystemsError),
}

impl From<HostStarError> for Error {
  #[named]
  fn from(error: HostStarError) -> Self {
    Error::HostStarError(error)
  }
}

impl From<SatelliteSystemsError> for Error {
  #[named]
  fn from(error: SatelliteSystemsError) -> Self {
    Error::SatelliteSystemsError(error)
  }
}
