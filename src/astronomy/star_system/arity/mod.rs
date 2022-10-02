use rand::distributions::Standard;
use rand::distributions::WeightedIndex;
use rand::prelude::*;

/// The `StarSystemArity` type.
///
/// This is a broad categorization; we'll use this further later.
///
/// Solitary star systems are the most common.  Trinary and quaternary might
/// seem very exotic and unlikely (and they're certainly less common), but
/// for them to be stable long enough to produce a lifebearing planet, they
/// _probably_ need to be in some configuration that decomposes to a classic
/// two-body problem.  
///
/// So weird systems with stars weaving in and out between other planets are
/// not considered here.  We'll have one star, one star and another orbiting
/// around a shared center of gravity, one star and two stars orbiting around
/// a shared center of gravity, or one star and three stars or two stars and
/// two stars orbiting around a shared center of gravity.
///
/// We'll contemplate the actual configuration elsewhere.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum StarSystemArity {
  /// A single star.
  Solitary,
  /// A binary star system is comprised of two stars.
  Binary,
  /// A trinary star system is comprised of three stars.
  Trinary,
  /// A quaternary star system is comprised of four stars.
  Quaternary,
}

/// Implement standard distribution.
impl Distribution<StarSystemArity> for Standard {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> StarSystemArity {
    let index: u8 = rng.gen_range(0..4);
    use StarSystemArity::*;
    match index {
      0 => Solitary,
      1 => Binary,
      2 => Trinary,
      3 => Quaternary,
      _ => unreachable!(),
    }
  }
}

impl StarSystemArity {
  /// Implement weighted distribution.
  pub fn get_random<R: Rng + ?Sized>(rng: &mut R) -> StarSystemArity {
    use StarSystemArity::*;
    let choices = [Solitary, Binary, Trinary, Quaternary];
    // These don't match actual distributions of star systems.
    let weights = [130, 60, 9, 1];
    let distribution = WeightedIndex::new(&weights).unwrap();
    return choices[distribution.sample(rng)];
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
    let mut rng = thread_rng();
    let _ = StarSystemArity::get_random(&mut rng);
  }
}
