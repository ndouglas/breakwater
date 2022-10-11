use rand::prelude::*;

use crate::astronomy::moon::constraints::Constraints as MoonConstraints;
use crate::astronomy::moons::constants::*;
use crate::astronomy::moons::error::Error;
use crate::astronomy::moons::Moons;

/// Constraints for creating a moon.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// The minimum number to generate.
  pub minimum_count: Option<usize>,
  /// The maximum number to generate.
  pub maximum_count: Option<usize>,
  /// A constraint for moons.
  pub moon_constraints: Option<MoonConstraints>,
}

impl Constraints {
  /// Generate.
  #[named]
  pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<Moons, Error> {
    trace_enter!();
    let minimum_count = self.minimum_count.unwrap_or(MINIMUM_MOONS);
    trace_var!(minimum_count);
    let maximum_count = self.maximum_count.unwrap_or(MAXIMUM_MOONS);
    trace_var!(maximum_count);
    let moon_constraints = self.moon_constraints.unwrap_or(MoonConstraints::default());
    trace_var!(moon_constraints);
    let moons = {
      let count = rng.gen_range(minimum_count..=maximum_count);
      trace_var!(count);
      let mut moons = vec![];
      for _ in 1..count {
        let moon = moon_constraints.generate(rng)?;
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
    let minimum_count = None;
    let maximum_count = None;
    let moon_constraints = None;
    Self {
      minimum_count,
      maximum_count,
      moon_constraints,
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
    let moon = &Constraints::default().generate(&mut rng)?;
    trace_var!(moon);
    print_var!(moon);
    trace_exit!();
    Ok(())
  }
}
