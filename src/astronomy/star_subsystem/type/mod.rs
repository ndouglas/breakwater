use rand::prelude::*;

use crate::astronomy::constants::PROBABILITY_OF_BINARY_STARS;
use crate::astronomy::star::constraints::Constraints as StarConstraints;
use crate::astronomy::star::Star;
use crate::astronomy::star_subsystem::binary_configuration::BinaryConfiguration;
use crate::astronomy::star_subsystem::constraints::Constraints;
use crate::astronomy::star_subsystem::error::Error;

/// The `Type` type.
///
/// A subsystem is either one or two subsystems.  Not three, because of the
/// 3-body problem.
#[derive(Clone, Debug, PartialEq)]
pub enum Type {
  /// A single star.  All subsystems ultimately decompose to this.
  Single(Star),
  /// Two subsystems.  Each can be a star or a subsystem.
  Double(Box<BinaryConfiguration>),
}

impl Type {
  /// Generate a random subsystem type with the specified constraints.
  ///
  /// This may or may not be habitable, depending on the constraints.
  #[named]
  pub fn from_constraints<R: Rng + ?Sized>(rng: &mut R, constraints: &Constraints) -> Result<Type, Error> {
    trace_enter!();
    let mut maximum_depth = constraints.maximum_depth;
    maximum_depth -= 1;
    let Constraints {
      enable_close_binaries,
      enable_distant_binaries,
      force_solitary,
      force_close_binary,
      force_distant_binary,
      ..
    } = *constraints;
    let binary_probability = constraints.binary_probability.unwrap_or(PROBABILITY_OF_BINARY_STARS);
    let is_binary = {
      if force_solitary || (!enable_close_binaries && !enable_distant_binaries) {
        false
      } else if force_close_binary || force_distant_binary {
        true
      } else {
        rng.gen_range(0.0..1.0) <= binary_probability
      }
    };
    let star_constraints = constraints.star_constraints.unwrap_or(StarConstraints::default());
    let result = match is_binary && maximum_depth >= 1 {
      true => {
        let mut new_constraints = constraints.clone();
        new_constraints.maximum_depth = maximum_depth;
        let binary_configuration = BinaryConfiguration::from_constraints(rng, &new_constraints)?;
        Type::Double(Box::new(binary_configuration))
      },
      false => Type::Single(Star::from_constraints(rng, &star_constraints)?),
    };
    trace_var!(result);
    trace_exit!();
    Ok(result)
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
    let constraints = Constraints::default();
    let r#type = Type::from_constraints(&mut rng, &constraints)?;
    info_var!(r#type);
    print_var!(r#type);
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
    let constraints = Constraints::habitable_solitary();
    let r#type = Type::from_constraints(&mut rng, &constraints)?;
    info_var!(r#type);
    print_var!(r#type);
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
    let constraints = Constraints::habitable_close_binary();
    let r#type = Type::from_constraints(&mut rng, &constraints)?;
    info_var!(r#type);
    print_var!(r#type);
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
    let constraints = Constraints::habitable_distant_binary();
    let r#type = Type::from_constraints(&mut rng, &constraints)?;
    info_var!(r#type);
    print_var!(r#type);
    trace_exit!();
    Ok(())
  }
}
