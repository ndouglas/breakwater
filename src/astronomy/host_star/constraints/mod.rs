use rand::prelude::*;
use std::default::Default;

use crate::astronomy::close_binary_star::constraints::Constraints as CloseBinaryStarConstraints;
use crate::astronomy::star::constraints::Constraints as StarConstraints;
use crate::astronomy::star_subsystem::Subsystem;
use crate::astronomy::host_star::HostStar;
use crate::astronomy::host_star::error::Error;

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
  pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<HostStar, Error> {
    trace_enter!();
    use HostStar::*;
    let is_solitary: bool = rng.gen();
    let result;
    if is_solitary {
      let constraints = StarConstraints::default();
      result = Star(constraints.generate(rng)?);
    } else {
      let constraints = CloseBinaryStarConstraints::default();
      result = CloseBinaryStar(constraints.generate(rng)?);
    }
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
