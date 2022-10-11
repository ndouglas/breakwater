use rand::prelude::*;

use crate::astronomy::star_system::constraints::Constraints as StarSystemConstraints;
use crate::astronomy::star_system::error::Error;
use crate::astronomy::star_system::StarSystem;
use crate::astronomy::stellar_neighborhood::constants::STELLAR_NEIGHBORHOOD_RADIUS;

pub mod constraints;
use constraints::*;
pub mod math;
use math::point::get_random_point_in_sphere;

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
  /// The distance from the origin.
  pub distance: f64,
  /// The name of the primary star.
  pub name: String,
}

impl StellarNeighbor {
  /// Generate a random stellar neighborhood with the specified constraints.
  ///
  /// This may or may not be habitable.
  #[named]
  pub fn from_constraints<R: Rng + ?Sized>(rng: &mut R, constraints: &Constraints) -> Result<StellarNeighbor, Error> {
    trace_enter!();
    // @todo: move this into stellar neighborhood, probably.
    let radius = constraints.radius.unwrap_or(STELLAR_NEIGHBORHOOD_RADIUS);
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
    let distance = (x.powf(2.0) + y.powf(2.0) + z.powf(2.0)).sqrt();
    let system_constraints = constraints
      .system_constraints
      .unwrap_or(StarSystemConstraints::default());
    let star_system = system_constraints.generate(rng)?;
    trace_var!(star_system);
    let name = star_system.name.clone();
    let result = StellarNeighbor {
      coordinates,
      star_system,
      distance,
      name,
    };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Retrieve or calculate the total mass of the stars.
  ///
  /// Calculated in Msol.
  #[named]
  pub fn get_stellar_mass(&self) -> f64 {
    trace_enter!();
    let result = self.star_system.get_stellar_mass();
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Retrieve or calculate the total number of stars in the system.
  #[named]
  pub fn get_stellar_count(&self) -> u8 {
    trace_enter!();
    let result = self.star_system.get_stellar_count();
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
    let stellar_neighbor = StellarNeighbor::from_constraints(&mut rng, &constraints)?;
    info_var!(stellar_neighbor);
    print_var!(stellar_neighbor);
    trace_exit!();
    Ok(())
  }
}
