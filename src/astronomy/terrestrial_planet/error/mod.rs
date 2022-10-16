use crate::astronomy::host_star::error::Error as HostStarError;

/// TerrestrialPlanet errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Host Star.
  HostStarError(HostStarError),
  /// Pluto, also Minnesota.
  TooColdToSupportConventionalLife,
  /// Hell, or Las Vegas.
  TooHotToSupportConventionalLife,
  /// Hard to fight when people keep floating off into space.
  GravityTooLowToSupportConventionalLife,
  /// Just sounds kinda lame.
  GravityTooHighToSupportConventionalLife,
}

honeyholt_define_brief!(Error, |error: &Error| {
  use Error::*;
  match error {
    HostStarError(host_star_error) => format!(
      "an error occurred in the host star ({})",
      honeyholt_brief!(host_star_error)
    ),
    TooColdToSupportConventionalLife => "not habitable because it is too cold".to_string(),
    TooHotToSupportConventionalLife => "not habitable because it is too hot".to_string(),
    GravityTooLowToSupportConventionalLife => "not habitable because its gravity is too low".to_string(),
    GravityTooHighToSupportConventionalLife => "not habitable because its gravity is too high".to_string(),
  }
});

impl From<HostStarError> for Error {
  #[named]
  fn from(error: HostStarError) -> Self {
    Error::HostStarError(error)
  }
}
