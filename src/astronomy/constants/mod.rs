/// Below this is too low for a main-sequence star, probably.
/// Measured in Msol, or solar mass equivalents.
pub const MAIN_SEQUENCE_STAR_MASS_LOWER_BOUND: f64 = 0.075;

/// Above this is too high for a main-sequence star, probably.
/// Measured in Msol, or solar mass equivalents.
pub const MAIN_SEQUENCE_STAR_MASS_UPPER_BOUND: f64 = 120.0;

/// Below this is probably too low to support conventional life.
/// Measured in Msol, or solar mass equivalents.
pub const MINIMUM_STAR_MASS_TO_SUPPORT_LIFE: f64 = 0.55;

/// Above this is probably too high to support conventional life.
/// Measured in Msol, or solar mass equivalents.
pub const MAXIMUM_STAR_MASS_TO_SUPPORT_LIFE: f64 = 1.31;

/// Assume a star has to be at least this old to have interesting life.
///
/// I'm assuming that life could get started at least a little sooner than on
/// Earth, but figuring it'd take about the same amount of time to get to the
/// interesting parts.
///
/// Measured in Gyr, or billions of years.
pub const MINIMUM_STAR_AGE_TO_SUPPORT_LIFE: f64 = 4.0;
