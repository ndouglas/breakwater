use rand::prelude::*;

use crate::astronomy::star_subsystem::constraints::Constraints as SubsystemConstraints;
use crate::astronomy::star_system::error::Error;
use crate::astronomy::star_system::StarSystem;

/// Constraints for creating a star system.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// Star subsystem creation constraints.
  pub subsystem_constraints: Option<SubsystemConstraints>,
  /// Number of times to regenerate if requirements aren't met.
  pub retries: Option<u8>,
}

impl Constraints {
  /// Generate a main-sequence star system.
  pub fn main_sequence() -> Self {
    let subsystem_constraints = Some(SubsystemConstraints::default());
    let retries = None;
    Self {
      subsystem_constraints,
      retries,
    }
  }

  /// Generate a habitable star system.
  pub fn habitable() -> Self {
    let subsystem_constraints = Some(SubsystemConstraints::habitable());
    let retries = Some(10);
    Self {
      subsystem_constraints,
      retries,
    }
  }

  /// Generate a habitable star system.
  pub fn habitable_close_binary() -> Self {
    let subsystem_constraints = Some(SubsystemConstraints::habitable());
    let retries = Some(10);
    Self {
      subsystem_constraints,
      retries,
    }
  }

  /// Generate a habitable star system.
  pub fn habitable_distant_binary() -> Self {
    let subsystem_constraints = Some(SubsystemConstraints::habitable());
    let retries = Some(10);
    Self {
      subsystem_constraints,
      retries,
    }
  }

  /// Generate a random star system with the specified constraints.
  ///
  /// This may or may not be habitable.
  #[named]
  pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<StarSystem, Error> {
    trace_enter!();
    let subsystem_constraints = self.subsystem_constraints.unwrap_or(SubsystemConstraints::default());
    let subsystem = {
      let mut retries = self.retries.unwrap_or(10);
      let subsystem;
      loop {
        let candidate_result = subsystem_constraints.generate(rng);
        if let Ok(candidate) = candidate_result {
          subsystem = candidate;
          break;
        }
        if retries == 0 {
          return Err(Error::NoSuitableSubsystemsCouldBeGenerated);
        }
        retries -= 1;
      }
      subsystem
    };
    trace_var!(subsystem);
    let name = "Steve".to_string();
    trace_var!(name);
    let result = StarSystem { subsystem, name };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let subsystem_constraints = Some(SubsystemConstraints::default());
    let retries = None;
    Self {
      subsystem_constraints,
      retries,
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
    let star_system = &Constraints::default().generate(&mut rng)?;
    trace_var!(star_system);
    print_var!(star_system);
    trace_exit!();
    Ok(())
  }
}
