use rand::prelude::*;

use crate::astronomy::moon::constraints::Constraints as MoonConstraints;
use crate::astronomy::moons::constants::*;
use crate::astronomy::moons::error::Error;
use crate::astronomy::moons::Moons;
use crate::astronomy::planet::Planet;

/// Constraints for creating a moon.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// A constraint for moons.
  pub moon_constraints: Option<MoonConstraints>,
}

impl Constraints {
  /// Generate.
  #[named]
  pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R, planet: &Planet) -> Result<Moons, Error> {
    trace_enter!();
    trace_var!(planet);
    let minimum_count;
    let maximum_count;
    use Planet::*;
    match planet {
      TerrestrialPlanet(_) => {
        minimum_count = MINIMUM_TERRESTRIAL_MOONS;
        maximum_count = MAXIMUM_TERRESTRIAL_MOONS;
      },
      GasGiantPlanet(_) => {
        minimum_count = MINIMUM_GAS_GIANT_MOONS;
        maximum_count = MAXIMUM_GAS_GIANT_MOONS;
      },
    }
    trace_var!(minimum_count);
    trace_var!(maximum_count);
    let moon_constraints = self.moon_constraints.unwrap_or(MoonConstraints::default());
    trace_var!(moon_constraints);
    let moons = {
      let count = rng.gen_range(minimum_count..=maximum_count);
      trace_var!(count);
      let mut moons = vec![];
      for _ in 1..count {
        let moon = moon_constraints.generate(rng, planet)?;
        moons.push(moon);
      }
      moons
    };
    trace_var!(moons);
    let result = Moons { moons };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let moon_constraints = None;
    Self { moon_constraints }
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
    let moon = &Constraints::default().generate(&mut rng, &planet)?;
    trace_var!(moon);
    print_var!(moon);
    trace_exit!();
    Ok(())
  }
}
