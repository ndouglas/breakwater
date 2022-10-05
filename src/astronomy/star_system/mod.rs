pub mod stars;
pub use stars::*;

/// The `StarSystem` type.
///
/// This is probably a good place to include some notes on terminology.
///
/// For ease of programming, I'm conflating the concepts of "star" or "stellar"
/// systems and "planetary" systems.
///
/// Here, a "star system" means one or more stars gravitationally connected in
/// some interesting way, along with all of the planets and other satellites
/// bound to those stars in some interesting way.
///
/// And I use "solar system" only to refer to our (your and my) star system.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StarSystem {
  /// The basic configuration of the host star(s).
  pub stars: StarSystemStars,
}
