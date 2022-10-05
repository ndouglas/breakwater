use rand::prelude::*;

use crate::astronomy::Star;

/// The `StarSystemStars` type.
///
/// This configuration indicates how the host star(s) of the  system is/are 
/// arranged.
///
/// This is extremely simple in the case of a solitary star, but can be quite
/// complicated in the cases of binary and ternary systems. I just want to 
/// cover the basic mechanics of a few common systems here; no need to go nuts.
///
/// Multi-star systems that are stable enough to support life tend to have a
/// hierarchical structure that decomposes to a two-body problem. In other
/// words, instead of three stars that orbit each other wildly, there might be
/// two bodies locked in a close binary system with the third orbiting them at
/// a greater distance.
///
/// This means I could've made this a recursive structure, but that seemed like
/// a bigger headache.  Rather, I'll just "unroll" the possibilities here.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StarSystemStars {
  /// A single star.
  Solitary(Star),
  /// A "P-type" binary star system; the habitable planet(s) orbit both stars. 
  /// The classic example of this is Tatooine from _Star Wars_.
  BinaryP(Star, Star),
  /// An "S-type" binary star system; the habitable planet(s) orbit the primary
  /// star.
  BinaryS1(Star, Star),
  /// An "S-type" binary star system; the habitable planet(s) orbit the second
  /// star.
  BinaryS2(Star, Star),
  /// A ternary star system consisting of a close binary system and an orbiting
  /// third star, with habitable planets orbiting the binary system in a P-type
  /// configuration.
  TernaryAP(Star, Star, Star),
  /// A ternary star system consisting of a close binary system and an orbiting
  /// third star, with habitable planets orbiting the primary star of the 
  /// binary system in an S-type configuration.
  TernaryAS1(Star, Star, Star),
  /// A ternary star system consisting of a close binary system and an orbiting
  /// third star, with habitable planets orbiting the secondary star of the 
  /// binary system in an S-type configuration.
  TernaryAS2(Star, Star, Star),
  /// A ternary star system consisting of a close binary system and an orbiting
  /// third star, with habitable planets orbiting the third star.
  TernaryA3(Star, Star, Star),
  /// A ternary star system consisting of a primary star and an orbiting binary
  /// system, with habitable planets orbiting the binary system in a P-type
  /// configuration.
  TernaryBP(Star, Star, Star),
  /// A ternary star system consisting of a primary star and an orbiting binary
  /// system, with habitable planets orbiting the primary star of the binary
  /// system in an S-type configuration.
  TernaryBS1(Star, Star, Star),
  /// A ternary star system consisting of a primary star and an orbiting binary
  /// system, with habitable planets orbiting the secondary star of the binary
  /// system in an S-type configuration.
  TernaryBS2(Star, Star, Star),
  /// A ternary star system consisting of a primary star and an orbiting binary
  /// system, with habitable planets orbiting the primary star of the entire
  /// system.
  TernaryB3(Star, Star, Star),
}
