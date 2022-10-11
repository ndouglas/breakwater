use rand::prelude::*;
use std::default::Default;

use crate::astronomy::satellite_system::constraints::Constraints as SatelliteSystemConstraints;
use crate::astronomy::satellite_systems::constants::*;
use crate::astronomy::satellite_systems::error::Error;
use crate::astronomy::satellite_systems::SatelliteSystems;

/// Constraints for creating a main-sequence star subsystem.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// The minimum number to generate.
  pub minimum_count: Option<usize>,
  /// The maximum number to generate.
  pub maximum_count: Option<usize>,
  /// Satellite System constraints.
  pub satellite_system_constraints: Option<SatelliteSystemConstraints>,
}

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
    let minimum_count = self.minimum_count.unwrap_or(MINIMUM_SATELLITE_SYSTEMS);
    trace_var!(minimum_count);
    let maximum_count = self.maximum_count.unwrap_or(MAXIMUM_SATELLITE_SYSTEMS);
    trace_var!(maximum_count);
    let satellite_system_constraints = self
      .satellite_system_constraints
      .unwrap_or(SatelliteSystemConstraints::default());
    trace_var!(satellite_system_constraints);
    let satellite_systems = {
      let count = rng.gen_range(minimum_count..=maximum_count);
      trace_var!(count);
      let mut satellite_systems = vec![];
      for _ in 1..count {
        let satellite_system = satellite_system_constraints.generate(rng)?;
        satellite_systems.push(satellite_system);
      }
      satellite_systems
    };
    trace_var!(satellite_systems);
    let result = SatelliteSystems { satellite_systems };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let minimum_count = None;
    let maximum_count = None;
    let satellite_system_constraints = None;
    Self {
      minimum_count,
      maximum_count,
      satellite_system_constraints,
    }
  }
}

#[cfg(test)]
pub mod test {

  use rand::prelude::*;

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn test_generate() -> Result<(), Error> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let satellite_systems = &Constraints::default().generate(&mut rng)?;
    trace_var!(satellite_systems);
    print_var!(satellite_systems);
    trace_exit!();
    Ok(())
  }
}
