use rand::prelude::*;

use crate::astronomy::math::point::get_random_point_in_sphere;
use crate::astronomy::star_system::constraints::Constraints as StarSystemConstraints;
use crate::astronomy::star_system::error::Error;
use crate::astronomy::AstronomicalError;
use crate::astronomy::Star;
use crate::astronomy::StarSystem;
use crate::astronomy::RADIUS_OF_STELLAR_NEIGHBORHOOD;

pub mod constraints;
use constraints::*;

/// The `StellarNeighbor` class.
///
/// No, not someone who brings you brownies when you move into the area.
///
/// This is just a combination of a fully-fledged star system and a set of 3-D
/// coordinates so that we can place it relative to our primary star system.
#[derive(Clone, Debug, PartialEq)]
pub struct StellarNeighbor {
  /// Each coordinate (x,y,z) is a distance (in light years) from the origin.
  pub coordinates: (f64, f64, f64),
  /// The details of this particular star system.
  pub star_system: StarSystem,
}

impl StellarNeighbor {
  /// Generate a random stellar neighborhood with the specified constraints.
  ///
  /// This may or may not be habitable.
  #[named]
  pub fn get_random_constrained<R: Rng + ?Sized>(
    rng: &mut R,
    constraints: &Constraints,
  ) -> Result<StellarNeighbor, Error> {
    trace_enter!();
    let radius = constraints.radius.unwrap_or(RADIUS_OF_STELLAR_NEIGHBORHOOD);
    trace_var!(radius);
    let raw_coordinates = get_random_point_in_sphere(rng);
    trace_var!(raw_coordinates);
    let x = raw_coordinates.0 * radius;
    trace_var!(x);
    let y = raw_coordinates.1 * radius;
    trace_var!(y);
    let z = raw_coordinates.2 * radius;
    trace_var!(z);
    let coordinates = (x, y, z);
    trace_var!(coordinates);
    let system_constraints = constraints
      .system_constraints
      .unwrap_or(StarSystemConstraints::default());
    let star_system = StarSystem::get_random_constrained(rng, &system_constraints)?;
    trace_var!(star_system);
    let result = StellarNeighbor {
      coordinates,
      star_system,
    };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Retrieve or calculate the total mass of the stars.
  ///
  /// Calculated in Msol.
  #[named]
  pub fn get_star_mass(&self) -> f64 {
    trace_enter!();
    let result = self.star_system.get_star_mass();
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Retrieve or calculate the total number of stars in the system.
  #[named]
  pub fn get_star_count(&self) -> u8 {
    trace_enter!();
    let result = self.star_system.get_star_count();
    trace_u8!(result);
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
    let constraints = Constraints::default();
    let stellar_neighbor = StellarNeighbor::get_random_constrained(&mut rng, &constraints)?;
    trace_var!(stellar_neighbor);
    // println!("{:#?}", stellar_neighbor);
    trace_exit!();
    Ok(())
  }
}
