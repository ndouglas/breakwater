use rand::prelude::*;
use std::default::Default;

use crate::astronomy::distant_binary_star::error::Error;
use crate::astronomy::distant_binary_star::DistantBinaryStar;
use crate::astronomy::planetary_system::constraints::Constraints as PlanetarySystemConstraints;

/// Constraints for creating a main-sequence star subsystem.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {}

impl Constraints {
  /// Generate a distant binary star with at least one habitable system.
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
  pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<DistantBinaryStar, Error> {
    trace_enter!();
    let primary_constraints = PlanetarySystemConstraints::default();
    trace_var!(primary_constraints);
    let primary = primary_constraints.generate(rng)?;
    trace_var!(primary);
    let secondary_constraints = PlanetarySystemConstraints::default();
    trace_var!(secondary_constraints);
    let secondary = secondary_constraints.generate(rng)?;
    trace_var!(secondary);
    let result = DistantBinaryStar { primary, secondary };
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
