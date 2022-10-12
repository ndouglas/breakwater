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
    let lower_bound_mass = self.minimum_mass.unwrap_or(MINIMUM_MASS);
    trace_var!(lower_bound_mass);
    let upper_bound_mass = self.maximum_mass.unwrap_or(MAXIMUM_MASS);
    trace_var!(upper_bound_mass);
    let mass = rng.gen_range(lower_bound_mass..upper_bound_mass);
    trace_var!(mass);
    let mut result = Star::from_mass(rng, mass)?;
    trace_var!(result);
    let minimum_age = self.minimum_age.unwrap_or(0.1 * result.life_expectancy);
    trace_var!(minimum_age);
    let maximum_age = self.maximum_age.unwrap_or(0.9 * result.life_expectancy);
    trace_var!(maximum_age);
    result.current_age = rng.gen_range(minimum_age..maximum_age);
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

#[cfg(test)]
pub mod test {

  use rand::prelude::*;

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn get_random_main_sequence() -> Result<(), Error> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let star = Constraints::default().generate(&mut rng)?;
    trace_var!(star);
    print_var!(star);
    trace_exit!();
    Ok(())
  }
}
