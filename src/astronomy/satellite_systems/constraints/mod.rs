use rand::prelude::*;
use std::default::Default;

use crate::astronomy::satellite_systems::error::Error;
use crate::astronomy::satellite_systems::SatelliteSystems;

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
  pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<SatelliteSystems, Error> {
    trace_enter!();
    let satellite_systems = vec![];
    let result = SatelliteSystems { satellite_systems };
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
