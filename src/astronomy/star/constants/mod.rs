/// Below this is too low for a main-sequence star, probably.
/// Measured in Msol, or solar mass equivalents.
pub const MINIMUM_MASS: f64 = 0.075;

/// Above this is too high for a main-sequence star, probably.
/// Measured in Msol, or solar mass equivalents.
pub const MAXIMUM_MASS: f64 = 120.0;

/// Below this is probably too low to support conventional life.
/// Measured in Msol, or solar mass equivalents.
pub const MINIMUM_HABITABLE_MASS: f64 = 0.55;

/// Above this is probably too high to support conventional life.
/// Measured in Msol, or solar mass equivalents.
pub const MAXIMUM_HABITABLE_MASS: f64 = 1.25;

/// Assume a star has to be at least this old to have interesting life.
///
/// I'm assuming that life could get started at least a little sooner than on
/// Earth, but figuring it'd take about the same amount of time to get to the
/// interesting parts.
///
/// Measured in Gyr, or billions of years.
pub const MINIMUM_HABITABLE_AGE: f64 = 4.0;
