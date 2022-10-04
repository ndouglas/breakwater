use rand::distributions::Standard;
use rand::distributions::WeightedIndex;
use rand::prelude::*;

use crate::astronomy::get_main_sequence_luminosity_from_mass;
use crate::astronomy::AstronomicalError;
use crate::astronomy::SpectralClassType;
use crate::astronomy::MAIN_SEQUENCE_STAR_MASS_LOWER_BOUND;
use crate::astronomy::MAIN_SEQUENCE_STAR_MASS_UPPER_BOUND;

/// The `SpectralClassLuminosity` type.
///
/// Luminosity indicates the actual size of the star.
///
/// This is, of course, of critical importance, since we could generate a star
/// that is simply too enormous to live near.  I'm completely clueless when it
/// comes to astronomy, but I'm _trying_ to get some basic things right.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SpectralClassLuminosity {
  /// Hypergiant
  Class0,
  /// Supergiant
  ClassI,
  /// Bright Giant
  ClassII,
  /// Giant
  ClassIII,
  /// Subgiant
  ClassIV,
  /// Main-Sequence Star
  ClassV,
  /// Subdwarf
  ClassVI,
  /// White Dwarf
  ClassVII,
}

/// Implementation of SpectralClassLuminosity.
impl SpectralClassLuminosity {
  /// From mass, for a main-sequence star.
  ///
  /// Note that `mass` is measured in solar mass equivalents.
  #[named]
  pub fn get_main_sequence_from_mass(mass: f64) -> Result<SpectralClassLuminosity, AstronomicalError> {
    trace_enter!();
    trace_var!(mass);
    if mass <= MAIN_SEQUENCE_STAR_MASS_LOWER_BOUND {
      return Err(AstronomicalError::StellarMassTooLowForMainSequence);
    }
    if mass >= MAIN_SEQUENCE_STAR_MASS_UPPER_BOUND {
      return Err(AstronomicalError::StellarMassTooHighForMainSequence);
    }
    // By definition.
    let result = SpectralClassLuminosity::ClassV;
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Implement weighted distribution.
  #[named]
  pub fn get_random<R: Rng + ?Sized>(rng: &mut R) -> Result<SpectralClassLuminosity, AstronomicalError> {
    trace_enter!();
    use SpectralClassLuminosity::*;
    // Disregard stars that would've blown up and killed us already (Class 0 - III).
    let choices = [ClassIV, ClassV, ClassVI, ClassVII];
    trace_var!(choices);
    // Subgiant stars will be very rare.  I'll try to create a nice mix.
    let weights = [1, 80, 9, 10];
    trace_var!(weights);
    let distribution = WeightedIndex::new(&weights).unwrap();
    trace_var!(distribution);
    let result = choices[distribution.sample(rng)];
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
impl Distribution<SpectralClassLuminosity> for Standard {
  #[named]
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SpectralClassLuminosity {
    trace_enter!();
    let index: u8 = rng.gen_range(0..8);
    trace_var!(index);
    use SpectralClassLuminosity::*;
    let result = match index {
      0 => Class0,
      1 => ClassI,
      2 => ClassII,
      3 => ClassIII,
      4 => ClassIV,
      5 => ClassV,
      6 => ClassVI,
      7 => ClassVII,
      _ => unreachable!(),
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
  pub fn get_main_sequence_from_mass() -> Result<(), AstronomicalError> {
    init();
    trace_enter!();
    let luminosity = SpectralClassLuminosity::get_main_sequence_from_mass(1.0)?;
    trace_var!(luminosity);
    assert_eq!(luminosity, SpectralClassLuminosity::ClassV);
    trace_exit!();
    Ok(())
  }

  #[named]
  #[test]
  pub fn get_random() {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let luminosity = SpectralClassLuminosity::get_random(&mut rng);
    trace_var!(luminosity);
    trace_exit!();
  }
}
