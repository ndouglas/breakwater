use rand::prelude::*;
use std::default::Default;

use crate::astronomy::star_subsystem::error::Error;
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
  pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<Subsystem, Error> {
    trace_enter!();
    Err(Error::UnknownError)
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    Self {}
  }
}
