use rand::prelude::*;

use crate::astronomy::star::constraints::Constraints as StarConstraints;
use crate::astronomy::star_system::subsystem::constraints::Constraints as SubsystemConstraints;
use crate::astronomy::Star;
use crate::astronomy::MAXIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION;
use crate::astronomy::MINIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION;
use crate::astronomy::PROBABILITY_OF_BINARY_STARS;

pub mod constraints;
use constraints::*;
pub mod error;
use error::*;
pub mod binary_configuration;
use binary_configuration::orbit_type::OrbitType;
use binary_configuration::*;
pub mod r#type;
use r#type::*;

/// The `Subsystem` type.
///
/// A subsystem is either one or two subsystems.  Not three, because of the
/// 3-body problem.
#[derive(Clone, Debug, PartialEq)]
pub struct Subsystem {
  /// The specific configuration of this star subsystem.
  pub r#type: Type,
  /// The number of stars in this subsystem.
  pub star_count: u8,
  /// The total mass of this subsystem in Msol.
  pub mass: f64,
  /// The overall luminosity of this subsystem in Lsol.
  pub luminosity: f64,
  /// The habitable zone of this subsystem, in AU.
  pub habitable_zone: (f64, f64),
  /// The satellite bounds of this subsystem, in AU.
  pub satellite_zone: (f64, f64),
  /// The frost line of this subsystem, in AU.
  pub frost_line: f64,
}

impl Subsystem {
  /// Generate a random subsystem with the specified constraints.
  ///
  /// This may or may not be habitable, depending on the constraints.
  #[named]
  pub fn get_random_constrained<R: Rng + ?Sized>(
    rng: &mut R,
    constraints: &SubsystemConstraints,
  ) -> Result<Subsystem, Error> {
    use Type::*;
    trace_enter!();
    trace_var!(constraints);
    let r#type = Type::get_random_constrained(rng, constraints)?;
    trace_var!(r#type);
    let star_count;
    let mass;
    let luminosity;
    let frost_line;
    let habitable_zone;
    let satellite_zone;
    match &r#type {
      Single(star) => {
        star_count = 1;
        mass = star.mass;
        luminosity = star.luminosity;
        frost_line = star.frost_line;
        habitable_zone = star.habitable_zone;
        satellite_zone = star.satellite_zone;
      },
      Double(binary) => {
        star_count = binary.star_count;
        mass = binary.mass;
        luminosity = binary.luminosity;
        frost_line = binary.frost_line;
        habitable_zone = binary.habitable_zone;
        satellite_zone = binary.satellite_zone;
      },
    };
    trace_var!(star_count);
    trace_var!(mass);
    trace_var!(luminosity);
    trace_var!(frost_line);
    trace_var!(habitable_zone);
    trace_var!(satellite_zone);
    let result = Subsystem {
      r#type,
      star_count,
      mass,
      luminosity,
      habitable_zone,
      satellite_zone,
      frost_line,
    };
    if constraints.enforce_habitability {
      result.check_habitable()?;
    }
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Indicate whether this star is capable of supporting conventional life.
  #[named]
  pub fn check_habitable(&self) -> Result<(), Error> {
    use Type::*;
    trace_enter!();
    let result = match &self.r#type {
      Single(star) => Ok(star.check_habitable()?),
      Double(binary) => Ok(binary.check_habitable()?),
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

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn get_random() -> Result<(), Error> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let constraints = Constraints::habitable_solitary_or_p_type_binary();
    let subsystem = {
      let mut retries = 10;
      let subsystem;
      loop {
        let candidate_result = Subsystem::get_random_constrained(&mut rng, &constraints);
        if let Ok(candidate) = candidate_result {
          subsystem = candidate;
          break;
        }
        if retries == 0 {
          panic!("Unable to generate a suitable subsystem.");
        }
        retries -= 1;
      }
      subsystem
    };
    trace_var!(subsystem);
    assert!(subsystem.is_habitable());
    // println!("{:#?}", subsystem);
    trace_exit!();
    Ok(())
  }

  #[named]
  #[test]
  pub fn get_random2() -> Result<(), Error> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let constraints = Constraints::habitable_solitary_or_s_type_binary();
    let subsystem = {
      let mut retries = 10;
      let subsystem;
      loop {
        let candidate_result = Subsystem::get_random_constrained(&mut rng, &constraints);
        if let Ok(candidate) = candidate_result {
          subsystem = candidate;
          break;
        }
        if retries == 0 {
          panic!("Unable to generate a suitable subsystem.");
        }
        retries -= 1;
      }
      subsystem
    };
    trace_var!(subsystem);
    assert!(subsystem.is_habitable());
    // println!("{:#?}", subsystem);
    trace_exit!();
    Ok(())
  }

  #[named]
  #[test]
  pub fn get_random3() -> Result<(), Error> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let constraints = Constraints::habitable_p_type_binary();
    let subsystem = {
      let mut retries = 10;
      let subsystem;
      loop {
        let candidate_result = Subsystem::get_random_constrained(&mut rng, &constraints);
        if let Ok(candidate) = candidate_result {
          subsystem = candidate;
          break;
        }
        if retries == 0 {
          panic!("Unable to generate a suitable subsystem.");
        }
        retries -= 1;
      }
      subsystem
    };
    trace_var!(subsystem);
    assert!(subsystem.is_habitable());
    // println!("{:#?}", subsystem);
    trace_exit!();
    Ok(())
  }

  #[named]
  #[test]
  pub fn get_random4() -> Result<(), Error> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let constraints = Constraints::habitable_s_type_binary();
    let subsystem = {
      let mut retries = 10;
      let subsystem;
      loop {
        let candidate_result = Subsystem::get_random_constrained(&mut rng, &constraints);
        if let Ok(candidate) = candidate_result {
          subsystem = candidate;
          break;
        }
        if retries == 0 {
          panic!("Unable to generate a suitable subsystem.");
        }
        retries -= 1;
      }
      subsystem
    };
    trace_var!(subsystem);
    assert!(subsystem.is_habitable());
    // println!("{:#?}", subsystem);
    trace_exit!();
    Ok(())
  }
}
