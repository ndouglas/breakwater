use std::default::Default;

use crate::astronomy::constants::RADIUS_OF_STELLAR_NEIGHBORHOOD;
use crate::astronomy::star_system::constraints::Constraints as SystemConstraints;

/// Constraints for creating a stellar neighborhood.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// The radius of the neighborhood, in light years.
  pub radius: Option<f64>,
  /// Star system constraints.
  pub system_constraints: Option<SystemConstraints>,
}

impl Constraints {
  /// Generate a main-sequence star system.
  pub fn main_sequence() -> Self {
    let radius = Some(RADIUS_OF_STELLAR_NEIGHBORHOOD);
    let system_constraints = Some(SystemConstraints::main_sequence());
    Self {
      radius,
      system_constraints,
    }
  }

  /// Generate a habitable star system.
  pub fn habitable() -> Self {
    let radius = Some(RADIUS_OF_STELLAR_NEIGHBORHOOD);
    let system_constraints = Some(SystemConstraints::habitable());
    Self {
      radius,
      system_constraints,
    }
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let radius = Some(RADIUS_OF_STELLAR_NEIGHBORHOOD);
    let system_constraints = Some(SystemConstraints::default());
    Self {
      radius,
      system_constraints,
    }
  }
}
