use super::super::orbit::error::Error as OrbitError;

/// Star orbits-related errors.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Unknown error.
  UnknownError,
  /// An error occurred calculating an orbit.
  OrbitError(OrbitError),
}

impl From<OrbitError> for Error {
  #[named]
  fn from(error: OrbitError) -> Self {
    Error::OrbitError(error)
  }
}

honeyholt_define_brief!(Error, |error: &Error| {
  use Error::*;
  match error {
    UnknownError => "we encountered an unknown error calculating orbits".to_string(),
    OrbitError(orbit_error) => format!(
      "we encountered an error calculating an orbit ({})",
      honeyholt_brief!(orbit_error)
    ),
  }
});
