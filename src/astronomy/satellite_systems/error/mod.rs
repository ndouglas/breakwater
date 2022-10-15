use crate::astronomy::satellite_system::error::Error as SatelliteSystemError;

/// Satellite systems errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Satellite System.
  SatelliteSystemError(SatelliteSystemError),
}

honeyholt_define_brief!(Error, |error: &Error| {
  use Error::*;
  match error {
    SatelliteSystemError(satellite_system_error) => format!(
      "an error occurred in the satellite system ({})",
      honeyholt_brief!(satellite_system_error)
    ),
  }
});

impl From<SatelliteSystemError> for Error {
  #[named]
  fn from(error: SatelliteSystemError) -> Self {
    Error::SatelliteSystemError(error)
  }
}
