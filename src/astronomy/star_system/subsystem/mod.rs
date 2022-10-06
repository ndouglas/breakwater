use rand::prelude::*;

use crate::astronomy::AstronomicalError;
use crate::astronomy::Star;
use crate::astronomy::StarConstraints;
use crate::astronomy::MAXIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION;
use crate::astronomy::MINIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION;
use crate::astronomy::PROBABILITY_OF_BINARY_STARS;

pub mod constraints;
pub use constraints::*;
pub mod orbital_information;
pub use orbital_information::*;
pub mod r#type;
pub use r#type::*;

/// The `StarSubsystem` type.
///
/// A subsystem is either one or two subsystems.  Not three, because of the
/// 3-body problem.
#[derive(Clone, Debug, PartialEq)]
pub struct StarSubsystem {
  /// The specific configuration of this star subsystem.
  pub r#type: StarSubsystemType,
  /// Information about orbital configuration of multi-star systems.
  pub orbital_information: Option<StarSubsystemOrbitalInformation>,
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

impl StarSubsystem {
  /// Generate a random subsystem with the specified constraints.
  ///
  /// This may or may not be habitable, depending on the constraints.
  #[named]
  pub fn get_random_constrained<R: Rng + ?Sized>(
    rng: &mut R,
    constraints: &StarSubsystemConstraints,
  ) -> Result<StarSubsystem, AstronomicalError> {
    trace_enter!();
    use StarSubsystemType::*;
    let r#type = StarSubsystemType::get_random_constrained(rng, constraints)?;
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
    let orbital_information = match &r#type {
      Single(_) => None,
      Double(sub1, sub2) => {
        let mut trials = 10;
        let information;
        loop {
          match StarSubsystemOrbitalInformation::get_random_constrained(rng, &sub1, &sub2, constraints) {
            Ok(orbital_information) => {
              information = orbital_information;
              break;
            },
            Err(_) => {},
          }
          trials -= 1;
          if trials <= 0 {
            return Err(AstronomicalError::UnableToGenerateCoherentStarSubsystemOrbitalInformation);
          }
        }
        Some(information)
      },
    };
    let result = StarSubsystem {
      r#type,
      star_count,
      mass,
      luminosity,
      habitable_zone,
      satellite_bounds,
      frost_line,
      orbital_information,
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
    use StarSubsystemType::*;
    let result = self.r#type.get_luminosity();
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Retrieve or calculate the habitable zone of the subsystem.
  #[named]
  pub fn get_habitable_zone(&self) -> (f64, f64) {
    trace_enter!();
    use StarSubsystemType::*;
    let result = self.r#type.get_habitable_zone();
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Retrieve or calculate the satellite bounds of the subsystem.
  #[named]
  pub fn get_satellite_bounds(&self) -> (f64, f64) {
    trace_enter!();
    use StarSubsystemType::*;
    let result = self.r#type.get_satellite_bounds();
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Retrieve or calculate the frost line of the subsystem.
  #[named]
  pub fn get_frost_line(&self) -> f64 {
    trace_enter!();
    use StarSubsystemType::*;
    let result = self.r#type.get_frost_line();
    trace_var!(result);
    trace_exit!();
    result
  }


  /// Indicate whether this star is capable of supporting conventional life.
  #[named]
  pub fn check_habitable(&self) -> Result<(), AstronomicalError> {
    trace_enter!();
    use StarSubsystemType::*;
    let result = {
      self.r#type.check_habitable()?;
      if let Some(orbital_information) = &self.orbital_information {
        use StarSubsystemOrbitalInformationOrbitType::*;
        match orbital_information.orbit_type {
          None => return Err(AstronomicalError::NoHabitableZoneFoundInSubsystem),
          PType => {
            let habitable_zone = self.r#type.get_habitable_zone();
            let forbidden_zone = orbital_information.forbidden_zone;
            if forbidden_zone.0 <= habitable_zone.0 && forbidden_zone.1 >= habitable_zone.1 {
              return Err(AstronomicalError::HabitableZoneContainedWithinForbiddenZone);
            }
            if habitable_zone.1 <= 4.0 * orbital_information.maximum_separation {
              return Err(AstronomicalError::HabitableZoneTooCloseToBinaryHostStars);
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
  pub fn get_random() -> Result<(), AstronomicalError> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let constraints = StarSubsystemConstraints::habitable_solitary_or_p_type_binary();
    let star_subsystem = StarSubsystem::get_random_constrained(&mut rng, &constraints)?;
    trace_var!(star_subsystem);
    star_subsystem.check_habitable()?;
    assert!(star_subsystem.is_habitable());
    println!("{:#?}", star_subsystem);
    trace_exit!();
    Ok(())
  }
}
