use rand::distributions::Standard;
use rand::distributions::WeightedIndex;
use rand::prelude::*;

use crate::astronomy::constants::MAIN_SEQUENCE_STAR_MASS_LOWER_BOUND;
use crate::astronomy::constants::MAIN_SEQUENCE_STAR_MASS_UPPER_BOUND;
use crate::astronomy::star::error::Error;
use crate::astronomy::star::math::temperature::get_main_sequence_star_temperature_from_mass;

/// The `Type` type.
///
/// The spectral class of a star indicates its temperature and its mass.
///
/// The broadest classification here is the Morgan-Keenan Type, which is a
/// single letter (O, B, A, F, G, K, or M).
///
/// Because we can, we'll include some of the more recent, less conventional
/// classes, including:
/// - L, a dark red dwarf (oooooh!)
/// - T, a cool brown dwarf (might be invisible to locals, except in eclipse)
/// - Y, a very cool brown dwarf
/// - D, a white dwarf
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Type {
  /// very hot and extremely luminous
  O,
  /// very luminous and blue
  B,
  /// white or bluish-white
  A,
  /// a yellow-white hue
  F,
  /// white, for more luminous types, to only very slightly yellowish
  G,
  /// yellowish to orangish
  K,
  /// orangish to red
  M,
  /// reddish
  L,
  /// dimmer red
  T,
  /// brown
  Y,
  /// dim white
  D,
}

/// Implementation of Type.
impl Type {
  /// From mass, for a main-sequence star.
  #[named]
  pub fn get_main_sequence_from_mass(mass: f64) -> Result<Type, Error> {
    trace_enter!();
    trace_var!(mass);
    if mass <= MAIN_SEQUENCE_STAR_MASS_LOWER_BOUND {
      return Err(Error::MassTooLowForMainSequence);
    }
    if mass >= MAIN_SEQUENCE_STAR_MASS_UPPER_BOUND {
      return Err(Error::MassTooHighForMainSequence);
    }
    let temperature = get_main_sequence_star_temperature_from_mass(mass)?;
    use Type::*;
    let result = match temperature {
      temperature if temperature < 3_700.0 => M,
      temperature if temperature < 5_200.0 => K,
      temperature if temperature < 6_000.0 => G,
      temperature if temperature < 7_500.0 => F,
      temperature if temperature < 10_000.0 => A,
      temperature if temperature < 33_000.0 => B,
      temperature if temperature < 95_000.0 => O,
      _ => unreachable!(),
    };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Implement weighted distribution.
  #[named]
  pub fn get_random<R: Rng + ?Sized>(rng: &mut R) -> Result<Type, Error> {
    use Type::*;
    // Just assume that we're calculating based on main-sequence stars and
    // things that won't kill everyone.
    let choices = [B, A, F, G, K, M, L, T, Y, D];
    // Bump up a couple probabilities for some interesting but rare variants,
    // without drastically affecting the probabilities of main-sequence stars.
    // It might be nice for this to be refined by someone who isn't an idiot.
    let weights = [1, 6, 30, 76, 121, 725, 10, 10, 10, 10];
    let distribution = WeightedIndex::new(&weights).unwrap();
    trace_var!(distribution);
    let result = choices[distribution.sample(rng)];
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Get char equivalent.
  #[named]
  pub fn get_char(&self) -> char {
    trace_enter!();
    use Type::*;
    let result = match self {
      O => 'O',
      B => 'B',
      A => 'A',
      F => 'F',
      G => 'G',
      K => 'K',
      M => 'M',
      L => 'L',
      T => 'T',
      Y => 'Y',
      D => 'D',
    };
    trace_var!(result);
    trace_exit!();
    result
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
impl Distribution<Type> for Standard {
  #[named]
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Type {
    trace_enter!();
    use Type::*;
    let index: u8 = rng.gen_range(0..11);
    trace_var!(index);
    let result = match index {
      0 => O,
      1 => B,
      2 => A,
      3 => F,
      4 => G,
      5 => K,
      6 => M,
      7 => L,
      8 => T,
      9 => Y,
      10 => D,
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
    let r#type = Type::get_random(&mut rng);
    trace_var!(r#type);
    trace_exit!();
  }
}
