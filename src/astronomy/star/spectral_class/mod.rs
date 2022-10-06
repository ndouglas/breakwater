use rand::distributions::Standard;
use rand::prelude::*;

use crate::astronomy::get_main_sequence_star_luminosity_from_mass;
use crate::astronomy::get_main_sequence_star_radius_from_mass;
use crate::astronomy::get_main_sequence_star_temperature_from_mass;
use crate::astronomy::AstronomicalError;
use crate::astronomy::MAIN_SEQUENCE_STAR_MASS_LOWER_BOUND;
use crate::astronomy::MAIN_SEQUENCE_STAR_MASS_UPPER_BOUND;

pub mod decile;
pub use decile::*;
pub mod luminosity;
pub use luminosity::*;
pub mod r#type;
pub use r#type::*;

/// The `SpectralClass` type.
///
/// The spectral class of a star indicates its temperature and its mass.
///
/// This is a combination of its type, a decile within that type, and its
/// luminosity.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SpectralClass {
  pub r#type: SpectralClassType,
  pub decile: Decile,
  pub luminosity: SpectralClassLuminosity,
}

/// Implementation of SpectralClass.
impl SpectralClass {
  /// From mass, for a main-sequence star.
  #[named]
  pub fn get_main_sequence_from_mass(mass: f64) -> Result<SpectralClass, AstronomicalError> {
    trace_enter!();
    trace_var!(mass);
    if mass <= MAIN_SEQUENCE_STAR_MASS_LOWER_BOUND {
      return Err(AstronomicalError::StellarMassTooLowForMainSequence);
    }
    if mass >= MAIN_SEQUENCE_STAR_MASS_UPPER_BOUND {
      return Err(AstronomicalError::StellarMassTooLowForMainSequence);
    }
    let r#type = SpectralClassType::get_main_sequence_from_mass(mass)?;
    trace_var!(r#type);
    let decile = Decile::get_main_sequence_from_mass(mass)?;
    trace_var!(decile);
    let luminosity = SpectralClassLuminosity::get_main_sequence_from_mass(mass)?;
    trace_var!(luminosity);
    let result = SpectralClass {
      r#type,
      decile,
      luminosity,
    };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Implement weighted distribution.
  #[named]
  pub fn get_random<R: Rng + ?Sized>(rng: &mut R) -> Result<SpectralClass, AstronomicalError> {
    trace_enter!();
    let r#type = SpectralClassType::get_random(rng)?;
    trace_var!(r#type);
    let decile = Decile::get_random(rng)?;
    trace_var!(decile);
    let luminosity = SpectralClassLuminosity::get_random(rng)?;
    trace_var!(luminosity);
    let result = SpectralClass {
      r#type,
      decile,
      luminosity,
    };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}

/// Implement uniform distribution.
///
/// This is not intended for general use, but for testing and debugging.
///
/// This gives a uniform distribution, with each possibility weighed the
/// same, so that we might get extremely unlikely combinations.
///
/// For actual random usage, use the ::get_random() method.
///
/// Also possible that I'll figure out a better way to do this.
impl Distribution<SpectralClass> for Standard {
  #[named]
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SpectralClass {
    trace_enter!();
    let result = SpectralClass {
      r#type: rng.gen(),
      decile: rng.gen(),
      luminosity: rng.gen(),
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
  pub fn get_random() {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let class = SpectralClass::get_random(&mut rng);
    trace_var!(class);
    trace_exit!();
  }
}
