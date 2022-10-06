use rand::prelude::*;

use crate::astronomy::AstronomicalError;
use crate::astronomy::Star;
use crate::astronomy::star::constraints::Constraints as StarConstraints;
use crate::astronomy::star_system::subsystem::constraints::Constraints;
use crate::astronomy::star_system::subsystem::Subsystem;
use crate::astronomy::star_system::subsystem::constraints::Constraints as SubsystemConstraints;
use crate::astronomy::PROBABILITY_OF_BINARY_STARS;

/// The `Type` type.
///
/// A subsystem is either one or two subsystems.  Not three, because of the
/// 3-body problem.
#[derive(Clone, Debug, PartialEq)]
pub enum Type {
  /// A single star.  All subsystems ultimately decompose to this.
  Single(Star),
  /// Two subsystems.  Each can be a star or a subsystem.
  Double(Box<Subsystem>, Box<Subsystem>),
}

impl Type {
  /// Generate a random subsystem type with the specified constraints.
  ///
  /// This may or may not be habitable, depending on the constraints.
  #[named]
  pub fn get_random_constrained<R: Rng + ?Sized>(
    rng: &mut R,
    constraints: &Constraints,
  ) -> Result<Type, AstronomicalError> {
    trace_enter!();
    let mut maximum_depth = constraints.maximum_depth;
    maximum_depth -= 1;
    let binary_probability = constraints.binary_probability.unwrap_or(PROBABILITY_OF_BINARY_STARS);
    let is_binary = rng.gen_range(0.0..1.0) <= binary_probability;
    let star_constraints = constraints.star_constraints.unwrap_or(StarConstraints::default());
    let result = match is_binary && maximum_depth >= 1 {
      true => {
        let mut new_constraints = constraints.clone();
        new_constraints.maximum_depth = maximum_depth;
        let sub_a = Subsystem::get_random_constrained(rng, &new_constraints)?;
        let sub_b = Subsystem::get_random_constrained(rng, &new_constraints)?;
        let sub_a_mass = sub_a.get_mass();
        let sub_b_mass = sub_b.get_mass();
        let first = Box::new(if sub_a_mass > sub_b_mass {
          sub_a.clone()
        } else {
          sub_b.clone()
        });
        let second = Box::new(if sub_a_mass > sub_b_mass {
          sub_b.clone()
        } else {
          sub_a.clone()
        });
        Type::Double(first, second)
      },
      false => Type::Single(Star::get_random_main_sequence_constrained(rng, &star_constraints)?),
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
    use Type::*;
    let result = match self {
      Single(star) => star.mass,
      Double(sub1, sub2) => sub1.get_mass() + sub2.get_mass(),
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Retrieve or calculate the total number of stars in the subsystem.
  #[named]
  pub fn get_count(&self) -> u8 {
    trace_enter!();
    use Type::*;
    let result = match self {
      Single(_) => 1,
      Double(sub1, sub2) => sub1.get_count() + sub2.get_count(),
    };
    trace_u8!(result);
    trace_exit!();
    result
  }

  /// Retrieve or calculate the total luminosity in the subsystem.
  #[named]
  pub fn get_luminosity(&self) -> f64 {
    trace_enter!();
    use Type::*;
    let result = match self {
      Single(star) => star.luminosity,
      Double(sub1, sub2) => sub1.get_luminosity() + sub2.get_luminosity(),
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Retrieve or calculate the habitable zone of the subsystem.
  #[named]
  pub fn get_habitable_zone(&self) -> (f64, f64) {
    trace_enter!();
    use Type::*;
    let result = match self {
      Single(star) => star.habitable_zone,
      Double(sub1, sub2) => {
        let sub1_lum = sub1.get_luminosity();
        let sub2_lum = sub2.get_luminosity();
        let base = (sub1_lum + sub2_lum).sqrt();
        let habitable_zone = (0.95 * base, 1.37 * base);
        habitable_zone
      },
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Retrieve or calculate the satellite bounds of the subsystem.
  #[named]
  pub fn get_satellite_bounds(&self) -> (f64, f64) {
    trace_enter!();
    use Type::*;
    let result = match self {
      Single(star) => star.satellite_bounds,
      Double(sub1, sub2) => {
        let sub1_mass = sub1.get_mass();
        let sub2_mass = sub2.get_mass();
        let total_mass = sub1_mass + sub2_mass;
        let satellite_inner_bound = 0.1 * total_mass;
        let satellite_outer_bound = 40.0 * total_mass;
        (satellite_inner_bound, satellite_outer_bound)
      },
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Retrieve or calculate the frost line of the subsystem.
  #[named]
  pub fn get_frost_line(&self) -> f64 {
    trace_enter!();
    use Type::*;
    let result = match self {
      Single(star) => star.frost_line,
      Double(sub1, sub2) => {
        let sub1_lum = sub1.get_luminosity();
        let sub2_lum = sub2.get_luminosity();
        let frost_line = 4.85 * (sub1_lum + sub2_lum).sqrt();
        frost_line
      },
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Indicate whether this star is capable of supporting conventional life.
  #[named]
  pub fn check_habitable(&self) -> Result<(), AstronomicalError> {
    trace_enter!();
    use Type::*;
    let result = match self {
      Single(star) => star.check_habitable(),
      Double(sub1, sub2) => {
        if !sub1.is_habitable() && !sub2.is_habitable() {
          return Err(AstronomicalError::NoHabitableZoneFoundInSubsystem);
        }
        Ok(())
      },
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
    let constraints = Constraints::habitable_solitary_or_p_type_binary();
    let r#type = Type::get_random_constrained(&mut rng, &constraints)?;
    trace_var!(r#type);
    // println!("{:#?}", r#type);
    trace_exit!();
    Ok(())
  }

  #[named]
  #[test]
  pub fn get_random2() -> Result<(), AstronomicalError> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let constraints = Constraints::habitable_solitary_or_s_type_binary();
    let r#type = Type::get_random_constrained(&mut rng, &constraints)?;
    trace_var!(r#type);
    // println!("{:#?}", r#type);
    trace_exit!();
    Ok(())
  }

  #[named]
  #[test]
  pub fn get_random3() -> Result<(), AstronomicalError> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let constraints = Constraints::habitable_p_type_binary();
    let r#type = Type::get_random_constrained(&mut rng, &constraints)?;
    trace_var!(r#type);
    // println!("{:#?}", r#type);
    trace_exit!();
    Ok(())
  }


  #[named]
  #[test]
  pub fn get_random4() -> Result<(), AstronomicalError> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let constraints = Constraints::habitable_s_type_binary();
    let r#type = Type::get_random_constrained(&mut rng, &constraints)?;
    trace_var!(r#type);
    // println!("{:#?}", r#type);
    trace_exit!();
    Ok(())
  }

}
