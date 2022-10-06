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
  /// Lower than MINIMUM_STAR_AGE_TO_SUPPORT_LIFE.
  StarTooYoungToSupportLife,
  /// Lower than MINIMUM_STAR_MASS_TO_SUPPORT_LIFE.
  StellarMassTooLowToSupportLife,
  /// Higher than MAXIMUM_STAR_MASS_TO_SUPPORT_LIFE.
  StellarMassTooHighToSupportLife,
  /// Lower than MINIMUM_BINARY_STAR_SEPARATION.
  BinaryStarsTooCloseForComfort,
  /// Unable to generate subsystem after a number of tries.
  UnableToGenerateCoherentStarSubsystemOrbitalInformation,
  /// The habitable zone is contained within the forbidden zone.
  HabitableZoneContainedWithinForbiddenZone,
  /// The habitable zone isn't sufficiently far from the host stars.
  HabitableZoneTooCloseToBinaryHostStars,
  /// No habitable conditions found anywhere in subsystem.
  NoHabitableZoneFoundInSubsystem,
}
