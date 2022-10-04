/// The `AstronomicalError` type.
///
/// This is for errors concerning astronomy, e.g. generating a main-sequence
/// star that's too big, or a star without a habitable zone, etc.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AstronomicalError {
  /// Lower than MAIN_SEQUENCE_STAR_MASS_LOWER_BOUND.
  StellarMassTooLowForMainSequence,
  /// Higher than MAIN_SEQUENCE_STAR_MASS_UPPER_BOUND.
  StellarMassTooHighForMainSequence,
}
