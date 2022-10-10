/// The minimum separation of binary stars, in AU.
pub const MINIMUM_SEPARATION: f64 = 0.04;

/// The minimum average separation of "close" binary stars, in AU.
pub const MINIMUM_AVERAGE_SEPARATION: f64 = 0.1;

/// The maximum average separation of "close" binary stars, in AU.
pub const MAXIMUM_AVERAGE_SEPARATION: f64 = 2.0;

/// The minimum orbital eccentricity of "close" binary stars (unitless).
pub const MINIMUM_ORBITAL_ECCENTRICITY: f64 = 0.1;

/// The maximum orbital eccentricity of "close" binary stars (unitless).
pub const MAXIMUM_ORBITAL_ECCENTRICITY: f64 = 0.3;

/// The maximum maximum separation of "close" habitable binary stars, in AU.
/// I dropped this down from ~6AU because this just was not happening.
pub const MAXIMUM_HABITABLE_SEPARATION: f64 = 0.3;

/// Below this is too low for a main-sequence star, probably.
/// Measured in Msol, or solar mass equivalents.
pub const MINIMUM_MAIN_SEQUENCE_STAR_MASS: f64 = 0.075;

/// Above this is too high for a main-sequence star, probably.
/// Measured in Msol, or solar mass equivalents.
pub const MAXIMUM_MAIN_SEQUENCE_STAR_MASS: f64 = 120.0;

/// The minimum combined mass of a binary system.
/// Set it to 1 * minimum main-sequence star mass.
pub const MINIMUM_COMBINED_MASS: f64 = 1.5 * MINIMUM_MAIN_SEQUENCE_STAR_MASS;

/// The maximum combined mass of a binary system.
/// Set it to 1 * maximum main-sequence star mass.
pub const MAXIMUM_COMBINED_MASS: f64 = MAXIMUM_MAIN_SEQUENCE_STAR_MASS;

/// The minimum combined mass of a binary system.
/// Set it to 1 * minimum main-sequence star mass.
pub const MINIMUM_INDIVIDUAL_MASS: f64 = MINIMUM_MAIN_SEQUENCE_STAR_MASS;

/// The maximum combined mass of a binary system.
/// Set it to 1 * maximum main-sequence star mass.
pub const MAXIMUM_INDIVIDUAL_MASS: f64 = MAXIMUM_MAIN_SEQUENCE_STAR_MASS;

/// Assume a star has to be at least this old to have interesting life.
///
/// I'm assuming that life could get started at least a little sooner than on
/// Earth, but figuring it'd take about the same amount of time to get to the
/// interesting parts.
///
/// Measured in Gyr, or billions of years.
pub const MINIMUM_HABITABLE_AGE: f64 = 4.0;

/// Below this is probably too low to support conventional life.
/// Measured in Msol, or solar mass equivalents.
pub const MINIMUM_HABITABLE_COMBINED_MASS: f64 = 1.2;

/// Above this is probably too high to support conventional life.
/// Measured in Msol, or solar mass equivalents.
pub const MAXIMUM_HABITABLE_COMBINED_MASS: f64 = 2.0;

/// Below this is probably too low to support conventional life.
/// Measured in Msol, or solar mass equivalents.
pub const MINIMUM_HABITABLE_INDIVIDUAL_MASS: f64 = 0.0;

/// Above this is probably too high to support conventional life.
/// Measured in Msol, or solar mass equivalents.
pub const MAXIMUM_HABITABLE_INDIVIDUAL_MASS: f64 = 1.25;
