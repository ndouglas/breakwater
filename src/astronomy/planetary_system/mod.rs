use crate::astronomy::host_star::HostStar;
use crate::astronomy::satellite_systems::SatelliteSystems;

pub mod constraints;
pub mod error;
use error::Error;

/// A `PlanetarySystem` is a `HostStar` and 0+ `SatelliteSystem` objects.
///
/// So a `PlanetarySystem` does not necessarily include planets.  This is
/// confusing and I don't really like it, but I don't have a better name
/// for it.  Yet.
#[derive(Clone, Debug, PartialEq)]
pub struct PlanetarySystem {
  pub host_star: HostStar,
  pub satellite_systems: SatelliteSystems,
}

impl PlanetarySystem {
  /// Indicate whether this star is capable of supporting conventional life.
  #[named]
  pub fn check_habitable(&self) -> Result<(), Error> {
    trace_enter!();
    let result = Ok(self.host_star.check_habitable()?);
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Indicate whether this star is capable of supporting conventional life.
  #[named]
  pub fn is_habitable(&self) -> bool {
    trace_enter!();
    let result = match self.check_habitable() {
      Ok(()) => true,
      Err(_) => false,
    };
    trace_var!(result);
    trace_exit!();
    result
  }
}
