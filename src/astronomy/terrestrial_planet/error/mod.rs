/// TerrestrialPlanet errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Unknown.
  UnknownError,
}

honeyholt_define_brief!(Error, |error: &Error| {
  use Error::*;
  match error {
    UnknownError => "an unknown error occurred".to_string(),
  }
});
