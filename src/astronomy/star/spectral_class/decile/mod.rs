use rand::distributions::Standard;
use rand::distributions::Uniform;
use rand::prelude::*;

use crate::astronomy::get_main_sequence_star_temperature_from_mass;
use crate::astronomy::AstronomicalError;
use crate::astronomy::MAIN_SEQUENCE_STAR_MASS_LOWER_BOUND;
use crate::astronomy::MAIN_SEQUENCE_STAR_MASS_UPPER_BOUND;

/// The `Decile` type.
///
/// This rather annoying type is to indicate which of the ten subdivisions
/// of a spectral class type a star might fall.
///
/// This unfortunately has a rather complex effect on which stars are and are
/// not habitable. An F6 star? Sure, we could probably live there. F4? Eh...
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Decile {
  D0,
  D1,
  D2,
  D3,
  D4,
  D5,
  D6,
  D7,
  D8,
  D9,
}

/// Implementation of Decile.
impl Decile {
  /// From mass, for a main-sequence star.
  ///
  /// Note that `mass` is measured in solar mass equivalents.
  #[named]
  pub fn get_main_sequence_from_mass(mass: f64) -> Result<Decile, AstronomicalError> {
    trace_enter!();
    use Decile::*;
    trace_var!(mass);
    if mass <= MAIN_SEQUENCE_STAR_MASS_LOWER_BOUND {
      return Err(AstronomicalError::StellarMassTooLowForMainSequence);
    }
    if mass >= MAIN_SEQUENCE_STAR_MASS_UPPER_BOUND {
      return Err(AstronomicalError::StellarMassTooHighForMainSequence);
    }
    let options = [D0, D1, D2, D3, D4, D5, D6, D7, D8, D9];
    let temperature = get_main_sequence_star_temperature_from_mass(mass)?;
    let result = match temperature {
      temperature if temperature < 3_700.0 => options[(10.0 * (1.0 - ((temperature - 2_000.0) / 1_700.0))) as usize],
      temperature if temperature < 5_200.0 => options[(10.0 * (1.0 - ((temperature - 3_700.0) / 1_500.0))) as usize],
      temperature if temperature < 6_000.0 => options[(10.0 * (1.0 - ((temperature - 5_200.0) / 800.0))) as usize],
      temperature if temperature < 7_500.0 => options[(10.0 * (1.0 - ((temperature - 6_000.0) / 1_500.0))) as usize],
      temperature if temperature < 10_000.0 => options[(10.0 * (1.0 - ((temperature - 7_500.0) / 2_500.0))) as usize],
      temperature if temperature < 33_000.0 => options[(10.0 * (1.0 - ((temperature - 10_000.0) / 23_000.0))) as usize],
      temperature if temperature < 95_000.0 => options[(10.0 * (1.0 - ((temperature - 33_000.0) / 62_000.0))) as usize],
      _ => unreachable!(),
    };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Implement weighted distribution.
  #[named]
  pub fn get_random<R: Rng + ?Sized>(rng: &mut R) -> Result<Decile, AstronomicalError> {
    trace_enter!();
    let result = rng.gen();
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
impl Distribution<Decile> for Standard {
  #[named]
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Decile {
    trace_enter!();
    let index: u8 = rng.gen_range(0..10);
    trace_var!(index);
    use Decile::*;
    let result = match index {
      0 => D0,
      1 => D1,
      2 => D2,
      3 => D3,
      4 => D4,
      5 => D5,
      6 => D6,
      7 => D7,
      8 => D8,
      9 => D9,
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
  pub fn get_random() {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let decile = Decile::get_random(&mut rng);
    trace_var!(decile);
    trace_exit!();
  }
}
