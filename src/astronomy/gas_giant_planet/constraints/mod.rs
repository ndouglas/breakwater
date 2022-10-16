use rand::prelude::*;

use crate::astronomy::gas_giant_planet::constants::*;
use crate::astronomy::gas_giant_planet::error::Error;
use crate::astronomy::gas_giant_planet::GasGiantPlanet;

/// Constraints for creating a planet.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// The minimum mass.
  pub minimum_mass: Option<f64>,
  /// The maximum mass.
  pub maximum_mass: Option<f64>,
}

impl Constraints {
  /// Generate.
  #[named]
  pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<GasGiantPlanet, Error> {
    trace_enter!();
    let minimum_mass = self.minimum_mass.unwrap_or(MINIMUM_MASS);
    trace_var!(minimum_mass);
    let maximum_mass = self.maximum_mass.unwrap_or(MAXIMUM_MASS);
    trace_var!(maximum_mass);
    let mass = rng.gen_range(minimum_mass..maximum_mass);
    trace_var!(mass);
    let result = GasGiantPlanet { mass };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let minimum_mass = None;
    let maximum_mass = None;
    Self {
      minimum_mass,
      maximum_mass,
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
    let planet = &Constraints::default().generate(&mut rng)?;
    trace_var!(planet);
    print_var!(planet);
    trace_exit!();
    Ok(())
  }
}
