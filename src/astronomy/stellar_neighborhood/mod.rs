use rand::prelude::*;
use std::f64::consts::PI;

use crate::astronomy::constants::DENSITY_OF_STELLAR_NEIGHBORHOOD;
use crate::astronomy::constants::RADIUS_OF_STELLAR_NEIGHBORHOOD;
use crate::astronomy::star_system::constraints::Constraints as StarSystemConstraints;
use crate::astronomy::star_system::error::Error;

pub mod constraints;
use constraints::*;
pub mod stellar_neighbor;
use stellar_neighbor::constraints::Constraints as StellarNeighborConstraints;
use stellar_neighbor::*;

/// The `StellarNeighborhood` type.
///
/// This is mostly a container for star systems.
///
/// We carve out a spherical section, a few light years or so in radius, and
/// generate some companion star systems.  These are likely to be other class V
/// stars, possibly with planets of their own.
///
/// Why?  Well, just to add a little color to the night sky.
#[derive(Clone, Debug, PartialEq)]
pub struct StellarNeighborhood {
  /// The radius of this neighborhood, measured in light years.
  pub radius: f64,
  /// The stellar density of this neighborhood, measured in stars per cubic
  /// light year.  This is not terribly useful once the neighborhood has
  /// been generated, but we keep it around for posterity.
  pub density: f64,
  /// Stellar "neighbors", which is a glorified tuple of three-dimensional
  /// coordinates and a star system.
  pub neighbors: Vec<StellarNeighbor>,
  /// The number of stars in this stellar neighborhood.
  pub star_count: usize,
}

impl StellarNeighborhood {
  /// Generate a random stellar neighborhood with the specified constraints.
  ///
  /// This may or may not be habitable.
  #[named]
  pub fn get_random_constrained<R: Rng + ?Sized>(
    rng: &mut R,
    constraints: &Constraints,
  ) -> Result<StellarNeighborhood, Error> {
    trace_enter!();
    let radius = constraints.radius.unwrap_or(RADIUS_OF_STELLAR_NEIGHBORHOOD);
    trace_var!(radius);
    let density = constraints.density.unwrap_or(DENSITY_OF_STELLAR_NEIGHBORHOOD);
    trace_var!(density);
    let volume = (4.0 / 3.0) * PI * radius.powf(3.0);
    trace_var!(volume);
    let average_stars = density * volume;
    trace_var!(average_stars);
    let number_of_stars = rng.gen_range((0.875 * average_stars)..(1.125 * average_stars)) as usize;
    trace_var!(number_of_stars);
    let mut neighbors = vec![];
    trace_var!(neighbors);
    let mut star_count = 0;
    let neighbor_constraints = constraints.neighbor_constraints.unwrap_or(StellarNeighborConstraints {
      radius: Some(radius),
      system_constraints: Some(StarSystemConstraints::default()),
    });
    trace_var!(neighbor_constraints);
    loop {
      let neighbor = StellarNeighbor::get_random_constrained(rng, &neighbor_constraints)?;
      star_count += neighbor.get_star_count() as usize;
      neighbors.push(neighbor);
      if star_count >= number_of_stars {
        break;
      }
    }
    trace_var!(neighbors);
    trace_var!(star_count);
    let result = StellarNeighborhood {
      radius,
      density,
      neighbors,
      star_count,
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
    let constraints = Constraints::habitable();
    let stellar_neighborhood = StellarNeighborhood::get_random_constrained(&mut rng, &constraints)?;
    info_var!(stellar_neighborhood);
    // println!("{:#?}", stellar_neighborhood);
    trace_exit!();
    Ok(())
  }
}
