use rand::prelude::*;
use std::default::Default;

use crate::astronomy::star::constants::*;
use crate::astronomy::star::error::Error;
use crate::astronomy::star::Star;

/// Constraints for creating a main-sequence star.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// The minimum mass of the star, in Msol.
  pub minimum_mass: Option<f64>,
  /// The maximum mass of the star, in Msol.
  pub maximum_mass: Option<f64>,
  /// The minimum age of the star, in Gyr.
  pub minimum_age: Option<f64>,
  /// The maximum age of the star, in Gyr.
  pub maximum_age: Option<f64>,
}

impl Constraints {
  /// Generate a habitable star.
  pub fn habitable() -> Self {
    let minimum_mass = Some(MINIMUM_HABITABLE_MASS);
    let maximum_mass = Some(MAXIMUM_HABITABLE_MASS);
    let minimum_age = Some(MINIMUM_HABITABLE_AGE);
    Self {
      minimum_mass,
      maximum_mass,
      minimum_age,
      ..Constraints::default()
    }
  }

  /// Generate a star from our settings.

  /// Generate.
  #[named]
  pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<Star, Error> {
    trace_enter!();
    let result = Star::from_constraints(rng, self)?;
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let minimum_mass = Some(MINIMUM_MASS);
    let maximum_mass = Some(MAXIMUM_MASS);
    let minimum_age = None;
    let maximum_age = None;
    Self {
      minimum_mass,
      maximum_mass,
      minimum_age,
      maximum_age,
    }
  }
}
