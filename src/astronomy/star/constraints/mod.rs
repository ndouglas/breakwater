use std::default::Default;

use crate::astronomy::constants::MAIN_SEQUENCE_STAR_MASS_LOWER_BOUND;
use crate::astronomy::constants::MAIN_SEQUENCE_STAR_MASS_UPPER_BOUND;
use crate::astronomy::constants::MAXIMUM_STAR_MASS_TO_SUPPORT_LIFE;
use crate::astronomy::constants::MINIMUM_STAR_AGE_TO_SUPPORT_LIFE;
use crate::astronomy::constants::MINIMUM_STAR_MASS_TO_SUPPORT_LIFE;

/// Constraints for creating a star.
///
/// This is intended to ease creating stars with specific characteristics.
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
  /// Generate a main-sequence star.
  pub fn main_sequence() -> Self {
    let minimum_mass = Some(MAIN_SEQUENCE_STAR_MASS_LOWER_BOUND);
    let maximum_mass = Some(MAIN_SEQUENCE_STAR_MASS_UPPER_BOUND);
    let minimum_age = None;
    let maximum_age = None;
    Self {
      minimum_mass,
      maximum_mass,
      minimum_age,
      maximum_age,
    }
  }

  /// Generate a habitable star.
  pub fn habitable() -> Self {
    let minimum_mass = Some(MINIMUM_STAR_MASS_TO_SUPPORT_LIFE);
    let maximum_mass = Some(MAXIMUM_STAR_MASS_TO_SUPPORT_LIFE);
    let minimum_age = Some(MINIMUM_STAR_AGE_TO_SUPPORT_LIFE);
    let maximum_age = None;
    Self {
      minimum_mass,
      maximum_mass,
      minimum_age,
      maximum_age,
    }
  }

  /// Generate a habitable or weak star.
  pub fn habitable_or_weak() -> Self {
    let minimum_mass = None;
    let maximum_mass = Some(MAXIMUM_STAR_MASS_TO_SUPPORT_LIFE);
    let minimum_age = Some(MINIMUM_STAR_AGE_TO_SUPPORT_LIFE);
    let maximum_age = None;
    Self {
      minimum_mass,
      maximum_mass,
      minimum_age,
      maximum_age,
    }
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let minimum_mass = None;
    let maximum_mass = None;
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
