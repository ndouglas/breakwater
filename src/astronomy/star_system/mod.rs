use rand::prelude::*;

use crate::astronomy::AstronomicalError;
use crate::astronomy::star_system::subsystem::constraints::Constraints as SubsystemConstraints;
use crate::astronomy::RADIUS_OF_STELLAR_NEIGHBORHOOD;

pub mod constraints;
use constraints::*;
pub mod subsystem;
use subsystem::*;

/// The `StarSystem` type.
///
/// This is probably a good place to include some notes on terminology.
///
/// For ease of programming, I'm conflating the concepts of "star" or "stellar"
/// systems and "planetary" systems.
///
/// Here, a "star system" means one or more stars gravitationally connected in
/// some interesting way, along with all of the planets and other satellites
/// bound to those stars in some interesting way.
///
/// And I use "solar system" only to refer to our (your and my) star system.
#[derive(Clone, Debug, PartialEq)]
pub struct StarSystem {
  /// The basic configuration of the host star(s).
  pub subsystem: Subsystem,
}

impl StarSystem {
  /// Generate a random star system with the specified constraints.
  ///
  /// This may or may not be habitable.
  #[named]
  pub fn get_random_constrained<R: Rng + ?Sized>(
    rng: &mut R,
    constraints: &Constraints,
  ) -> Result<StarSystem, AstronomicalError> {
    trace_enter!();
    let subsystem_constraints = constraints
      .subsystem_constraints
      .unwrap_or(SubsystemConstraints::default());
    let subsystem = Subsystem::get_random_constrained(rng, &subsystem_constraints)?;
    trace_var!(subsystem);
    let result = StarSystem { subsystem };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Retrieve or calculate the total mass of the stars.
  ///
  /// Calculated in Msol.
  #[named]
  pub fn get_star_mass(&self) -> f64 {
    trace_enter!();
    let result = self.subsystem.get_mass();
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Retrieve or calculate the total number of stars in the system.
  #[named]
  pub fn get_star_count(&self) -> u8 {
    trace_enter!();
    let result = self.subsystem.get_count();
    trace_u8!(result);
    trace_exit!();
    result
  }

  /// Indicate whether this star system is capable of supporting conventional life.
  #[named]
  pub fn check_habitable(&self) -> Result<(), AstronomicalError> {
    trace_enter!();
    let result = self.subsystem.check_habitable();
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Indicate whether this star system is capable of supporting conventional life.
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

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn get_random() -> Result<(), AstronomicalError> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let constraints = Constraints::habitable();
    let star_system = StarSystem::get_random_constrained(&mut rng, &constraints)?;
    trace_var!(star_system);
    // println!("{:#?}", star_system);
    trace_exit!();
    Ok(())
  }
}
