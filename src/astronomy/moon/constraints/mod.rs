use rand::prelude::*;

use crate::astronomy::moon::constants::*;
use crate::astronomy::moon::error::Error;
use crate::astronomy::moon::Moon;
use crate::astronomy::planet::Planet;

/// Constraints for creating a moon.
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
  pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R, planet: &Planet, distance: f64) -> Result<Moon, Error> {
    trace_enter!();
    trace_var!(planet);
    trace_var!(distance);
    let minimum_mass = self.minimum_mass.unwrap_or(MINIMUM_MASS);
    trace_var!(minimum_mass);
    let maximum_mass = self.maximum_mass.unwrap_or(MAXIMUM_MASS);
    trace_var!(maximum_mass);
    let mass = rng.gen_range(minimum_mass..maximum_mass);
    trace_var!(mass);
    let result = Moon::from_mass(mass, planet, distance)?;
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

  use crate::astronomy::host_star::constraints::Constraints as HostStarConstraints;
  use crate::astronomy::planet::constraints::Constraints as PlanetConstraints;
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
    let host_star = &HostStarConstraints::default().generate(&mut rng)?;
    trace_var!(host_star);
    let habitable_zone = host_star.get_habitable_zone();
    trace_var!(habitable_zone);
    let distance = rng.gen_range(habitable_zone.0..habitable_zone.1);
    trace_var!(distance);
    let planet = &PlanetConstraints::default().generate(&mut rng, &host_star, distance)?;
    trace_var!(planet);
    let moon = &Constraints::default().generate(&mut rng, &planet, 25_000.0)?;
    trace_var!(moon);
    print_var!(moon);
    trace_exit!();
    Ok(())
  }
}
