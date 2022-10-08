use rand::prelude::*;

use crate::astronomy::constants::MAIN_SEQUENCE_STAR_MASS_LOWER_BOUND;
use crate::astronomy::constants::MAIN_SEQUENCE_STAR_MASS_UPPER_BOUND;
use crate::astronomy::constants::MAXIMUM_STAR_MASS_TO_SUPPORT_LIFE;
use crate::astronomy::constants::MINIMUM_STAR_AGE_TO_SUPPORT_LIFE;
use crate::astronomy::constants::MINIMUM_STAR_MASS_TO_SUPPORT_LIFE;

pub mod constraints;
use constraints::*;
pub mod error;
use error::*;
pub mod math;
use math::color::ms_star_mass_to_rgb;
use math::luminosity::ms_star_mass_to_luminosity;
use math::radius::ms_star_mass_to_radius;
use math::spectral_class::ms_star_mass_to_spectral_class;
use math::temperature::ms_star_mass_to_temperature;

/// The `Star` type.
///
/// This is intended to encompass the most useful information we can generate
/// about stars, specifically main-sequence stars.
#[derive(Clone, Debug, PartialEq)]
pub struct Star {
  /// Type, Decile, Luminosity class.
  pub class: String,
  /// Measured in Msol.
  pub mass: f64,
  /// Measured in Kelvin.
  pub temperature: f64,
  /// Measured in Rsol.
  pub radius: f64,
  /// Measured in Lsol.
  pub luminosity: f64,
  /// Measured in Gyr.
  pub life_expectancy: f64,
  /// Measured in Gyr.
  pub current_age: f64,
  /// Measured in Dsol.
  pub density: f64,
  /// Habitable zone, measured in AU.
  pub habitable_zone: (f64, f64),
  /// Minimum and maximum sustainable distance for satellites, measured in AU.
  /// This is inferior to computing the Roche limit and Hill sphere, but we
  /// don't have enough information for that yet.
  pub satellite_zone: (f64, f64),
  /// The frost line, measured in AU.
  pub frost_line: f64,
  /// The absolute color of this star in SRGB.
  pub absolute_rgb: (u8, u8, u8),
}

/// Implementation of SpectralClass.
impl Star {
  /// Generate a random main-sequence star with specified constraints.
  ///
  /// This may or may not be habitable.
  #[named]
  pub fn get_random_main_sequence_constrained<R: Rng + ?Sized>(
    rng: &mut R,
    constraints: &Constraints,
  ) -> Result<Star, Error> {
    trace_enter!();
    let lower_bound_mass = constraints.minimum_mass.unwrap_or(MAIN_SEQUENCE_STAR_MASS_LOWER_BOUND);
    trace_var!(lower_bound_mass);
    let upper_bound_mass = constraints.maximum_mass.unwrap_or(MAIN_SEQUENCE_STAR_MASS_UPPER_BOUND);
    trace_var!(upper_bound_mass);
    let mass = rng.gen_range(lower_bound_mass..upper_bound_mass);
    trace_var!(mass);
    let temperature = ms_star_mass_to_temperature(mass)?;
    trace_var!(temperature);
    let luminosity = ms_star_mass_to_luminosity(mass)?;
    trace_var!(luminosity);
    let radius = ms_star_mass_to_radius(mass)?;
    trace_var!(radius);
    let class = ms_star_mass_to_spectral_class(mass)?;
    trace_var!(class);
    let life_expectancy = mass / luminosity * 10.0;
    trace_var!(life_expectancy);
    let lower_bound_age = constraints.minimum_age.unwrap_or(0.1 * life_expectancy);
    trace_var!(lower_bound_age);
    let upper_bound_age = constraints.maximum_age.unwrap_or(0.9 * life_expectancy);
    trace_var!(upper_bound_age);
    let current_age = rng.gen_range(lower_bound_age..upper_bound_age);
    trace_var!(current_age);
    let density = mass / radius.powf(3.0);
    trace_var!(density);
    let habitable_zone = ((luminosity / 1.1).sqrt(), (luminosity / 0.53).sqrt());
    trace_var!(habitable_zone);
    let approximate_satellite_inner_bound = 0.1 * mass;
    trace_var!(approximate_satellite_inner_bound);
    let approximate_satellite_outer_bound = 40.0 * mass;
    trace_var!(approximate_satellite_outer_bound);
    let satellite_zone = (approximate_satellite_inner_bound, approximate_satellite_outer_bound);
    let frost_line = 4.85 * luminosity.sqrt();
    trace_var!(frost_line);
    let absolute_rgb = ms_star_mass_to_rgb(mass)?;
    let result = Star {
      class,
      mass,
      luminosity,
      radius,
      temperature,
      life_expectancy,
      current_age,
      density,
      habitable_zone,
      satellite_zone,
      frost_line,
      absolute_rgb,
    };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Generate a random main-sequence star.
  ///
  /// This may or may not be habitable.
  #[named]
  pub fn get_random_main_sequence<R: Rng + ?Sized>(rng: &mut R) -> Result<Star, Error> {
    trace_enter!();
    let result = Star::get_random_main_sequence_constrained(rng, &Constraints::main_sequence())?;
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Generate a random habitable main-sequence star.
  #[named]
  pub fn get_random_habitable<R: Rng + ?Sized>(rng: &mut R) -> Result<Star, Error> {
    trace_enter!();
    let result = Star::get_random_main_sequence_constrained(rng, &Constraints::habitable())?;
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Indicate whether this star is capable of supporting conventional life.
  #[named]
  pub fn check_habitable(&self) -> Result<(), Error> {
    trace_enter!();
    if self.mass < MINIMUM_STAR_MASS_TO_SUPPORT_LIFE {
      return Err(Error::MassTooLowToSupportLife);
    }
    if self.mass > MAXIMUM_STAR_MASS_TO_SUPPORT_LIFE {
      return Err(Error::MassTooHighToSupportLife);
    }
    if self.current_age < MINIMUM_STAR_AGE_TO_SUPPORT_LIFE {
      return Err(Error::TooYoungToSupportLife);
    }
    trace_exit!();
    Ok(())
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
  pub fn get_random_main_sequence() -> Result<(), Error> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let star = Star::get_random_main_sequence(&mut rng)?;
    trace_var!(star);
    trace_exit!();
    Ok(())
  }

  #[named]
  #[test]
  pub fn get_random_habitable() -> Result<(), Error> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let star = Star::get_random_habitable(&mut rng)?;
    trace_var!(star);
    assert!(star.is_habitable());
    trace_exit!();
    Ok(())
  }
}
