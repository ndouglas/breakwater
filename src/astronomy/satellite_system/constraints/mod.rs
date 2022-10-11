use rand::prelude::*;

use crate::astronomy::moons::constraints::Constraints as MoonsConstraints;
use crate::astronomy::planet::constraints::Constraints as PlanetConstraints;
use crate::astronomy::satellite_system::error::Error;
use crate::astronomy::satellite_system::SatelliteSystem;

/// Constraints for creating a planet and its moons.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// Planet constraints.
  pub planet_constraints: Option<PlanetConstraints>,
  /// Moons constraints.
  pub moons_constraints: Option<MoonsConstraints>,
}

impl Constraints {
  /// Generate.
  #[named]
  pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<SatelliteSystem, Error> {
    trace_enter!();
    let planet_constraints = self.planet_constraints.unwrap_or(PlanetConstraints::default());
    trace_var!(planet_constraints);
    let moons_constraints = self.moons_constraints.unwrap_or(MoonsConstraints::default());
    trace_var!(moons_constraints);
    let planet = planet_constraints.generate(rng)?;
    trace_var!(planet);
    let moons = moons_constraints.generate(rng)?;
    trace_var!(moons);
    let result = SatelliteSystem { planet, moons };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let planet_constraints = None;
    let moons_constraints = None;
    Self {
      planet_constraints,
      moons_constraints,
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
    let satellite_system = &Constraints::default().generate(&mut rng)?;
    trace_var!(satellite_system);
    print_var!(satellite_system);
    trace_exit!();
    Ok(())
  }
}
