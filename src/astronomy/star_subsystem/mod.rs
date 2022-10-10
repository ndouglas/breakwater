use rand::prelude::*;

use crate::astronomy::distant_binary_star::DistantBinaryStar;
use crate::astronomy::planetary_system::PlanetarySystem;
use crate::astronomy::star_subsystem::constraints::Constraints as SubsystemConstraints;

pub mod constraints;
pub mod error;
use error::*;

/// The `Subsystem` type.
///
/// A subsystem is either one star or two subsystems.  Not three, because of
/// the 3-body problem.
#[derive(Clone, Debug, PartialEq)]
pub enum Subsystem {
  /// A distant binary system.
  DistantBinaryStar(DistantBinaryStar),
  /// Any other planetary system.
  PlanetarySystem(PlanetarySystem),
}

impl Subsystem {
  /// Indicate whether this star is capable of supporting conventional life.
  #[named]
  pub fn check_habitable(&self) -> Result<(), Error> {
    trace_enter!();
    use Subsystem::*;
    let result = match &self {
      DistantBinaryStar(distant_binary_star) => Ok(distant_binary_star.check_habitable()?),
      PlanetarySystem(planetary_system) => Ok(planetary_system.check_habitable()?),
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Indicate whether this star is capable of supporting conventional life.
  #[named]
  pub fn is_habitable(&self) -> bool {
    trace_enter!();
    let result = match self.check_habitable() {
      Ok(()) => true,
      Err(_) => false,
    };
    trace_var!(result);
    trace_exit!();
    result
  }
}

#[cfg(test)]
pub mod test {

  use rand::prelude::*;

  use super::constraints::Constraints;
  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn get_random() -> Result<(), Error> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let constraints = Constraints::default();
    let result = constraints.generate(&mut rng)?;
    trace_exit!();
    Ok(())
  }
}
