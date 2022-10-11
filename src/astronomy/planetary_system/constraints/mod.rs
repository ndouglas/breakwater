use rand::prelude::*;
use std::default::Default;

use crate::astronomy::distant_binary_star::constraints::Constraints as DistantBinaryStarConstraints;
use crate::astronomy::host_star::constraints::Constraints as HostStarConstraints;
use crate::astronomy::planetary_system::error::Error;
use crate::astronomy::planetary_system::PlanetarySystem;
use crate::astronomy::satellite_systems::constraints::Constraints as SatelliteSystemsConstraints;
use crate::astronomy::star_subsystem::error::Error as StarSubsystemError;
use crate::astronomy::star_subsystem::Subsystem;

/// Constraints for creating a main-sequence star subsystem.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {}

impl Constraints {
  /// Generate a habitable star subsystem.
  #[named]
  pub fn habitable() -> Self {
    trace_enter!();
    let result = Self {
      ..Constraints::default()
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Generate.
  #[named]
  pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<PlanetarySystem, Error> {
    trace_enter!();
    let host_star_constraints = HostStarConstraints::default();
    trace_var!(host_star_constraints);
    let satellite_systems_constraints = SatelliteSystemsConstraints::default();
    trace_var!(satellite_systems_constraints);
    let host_star = host_star_constraints.generate(rng)?;
    trace_var!(host_star);
    let satellite_systems = satellite_systems_constraints.generate(rng)?;
    trace_var!(satellite_systems);
    let result = PlanetarySystem {
      host_star,
      satellite_systems,
    };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    Self {}
  }
}
