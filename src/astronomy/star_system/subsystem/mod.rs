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
  /// Information about orbital configuration of multi-star systems.
  pub binary_configuration: Option<BinaryConfiguration>,
  /// The number of stars in this subsystem.
  pub star_count: u8,
  /// The total mass of this subsystem in Msol.
  pub mass: f64,
  /// The overall luminosity of this subsystem in Lsol.
  pub luminosity: f64,
  /// The habitable zone of this subsystem, in AU.
  pub habitable_zone: (f64, f64),
  /// The satellite bounds of this subsystem, in AU.
  pub satellite_bounds: (f64, f64),
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
    trace_enter!();
    use Type::*;
    let r#type = Type::get_random_constrained(rng, constraints)?;
    trace_var!(r#type);
    let star_count = r#type.get_count();
    trace_var!(star_count);
    let mass = r#type.get_mass();
    trace_var!(mass);
    let luminosity = r#type.get_luminosity();
    trace_var!(luminosity);
    let habitable_zone = r#type.get_habitable_zone();
    trace_var!(habitable_zone);
    let satellite_bounds = r#type.get_satellite_bounds();
    trace_var!(satellite_bounds);
    let frost_line = r#type.get_frost_line();
    trace_var!(frost_line);
    let binary_configuration = match &r#type {
      Single(_) => None,
      Double(sub1, sub2) => {
        let mut trials = 10;
        let information;
        loop {
          match BinaryConfiguration::get_random_constrained(rng, &sub1, &sub2, constraints) {
            Ok(binary_configuration) => {
              information = binary_configuration;
              break;
            },
            Err(_) => {},
          }
          trials -= 1;
          if trials <= 0 {
            return Err(Error::UnableToGenerateBinaryConfiguration);
          }
        }
        Some(information)
      },
    };
    let result = Subsystem {
      r#type,
      star_count,
      mass,
      luminosity,
      habitable_zone,
      satellite_bounds,
      frost_line,
      binary_configuration,
    };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Retrieve or calculate the total mass of the subsystem.
  ///
  /// Calculated in Msol.
  #[named]
  pub fn get_mass(&self) -> f64 {
    trace_enter!();
    let result = self.r#type.get_mass();
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Retrieve or calculate the total number of stars in the subsystem.
  #[named]
  pub fn get_count(&self) -> u8 {
    trace_enter!();
    let result = self.r#type.get_count();
    trace_u8!(result);
    trace_exit!();
    result
  }

  /// Retrieve or calculate the total luminosity in the subsystem.
  #[named]
  pub fn get_luminosity(&self) -> f64 {
    trace_enter!();
    let result = self.r#type.get_luminosity();
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Retrieve or calculate the habitable zone of the subsystem.
  #[named]
  pub fn get_habitable_zone(&self) -> (f64, f64) {
    trace_enter!();
    let result = self.r#type.get_habitable_zone();
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Retrieve or calculate the satellite bounds of the subsystem.
  #[named]
  pub fn get_satellite_bounds(&self) -> (f64, f64) {
    trace_enter!();
    let result = self.r#type.get_satellite_bounds();
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Retrieve or calculate the frost line of the subsystem.
  #[named]
  pub fn get_frost_line(&self) -> f64 {
    trace_enter!();
    let result = self.r#type.get_frost_line();
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Indicate whether this star is capable of supporting conventional life.
  #[named]
  pub fn check_habitable(&self) -> Result<(), Error> {
    trace_enter!();
    let result = {
      // Check habitability of the individual component subsystems.
      // This does not perform a check of habitability of binary components
      // as a unit; that will be the rest of this function.
      self.r#type.check_habitable()?;
      // If this is a binary subsystem, then check how habitable it is as a
      // unit.
      if let Some(binary_configuration) = &self.binary_configuration {
        use OrbitType::*;
        match binary_configuration.orbit_type {
          None => return Err(Error::NoHabitableZoneFound),
          PType => {
            let habitable_zone = self.r#type.get_habitable_zone();
            let forbidden_zone = binary_configuration.forbidden_zone;
            if forbidden_zone.0 <= habitable_zone.0 && forbidden_zone.1 >= habitable_zone.1 {
              return Err(Error::HabitableZoneContainedWithinForbiddenZone);
            }
            if habitable_zone.1 <= 4.0 * binary_configuration.maximum_separation {
              return Err(Error::HabitableZoneTooCloseToBinaryHostStars);
            }
          },
          STypePrimary | STypeSecondary | STypeBoth => {},
        }
      }
      Ok(())
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
    let subsystem = Subsystem::get_random_constrained(&mut rng, &constraints)?;
    trace_var!(subsystem);
    subsystem.check_habitable()?;
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
    let subsystem = Subsystem::get_random_constrained(&mut rng, &constraints)?;
    trace_var!(subsystem);
    subsystem.check_habitable()?;
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
    let subsystem = Subsystem::get_random_constrained(&mut rng, &constraints)?;
    trace_var!(subsystem);
    subsystem.check_habitable()?;
    assert!(subsystem.is_habitable());
    println!("{:#?}", subsystem);
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
    let subsystem = Subsystem::get_random_constrained(&mut rng, &constraints)?;
    trace_var!(subsystem);
    subsystem.check_habitable()?;
    assert!(subsystem.is_habitable());
    // println!("{:#?}", subsystem);
    trace_exit!();
    Ok(())
  }
}
