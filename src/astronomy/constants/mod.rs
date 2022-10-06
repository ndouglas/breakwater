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
pub const MAXIMUM_STAR_MASS_TO_SUPPORT_LIFE: f64 = 1.25;

/// Assume a star has to be at least this old to have interesting life.
///
/// I'm assuming that life could get started at least a little sooner than on
/// Earth, but figuring it'd take about the same amount of time to get to the
/// interesting parts.
///
/// Measured in Gyr, or billions of years.
pub const MINIMUM_STAR_AGE_TO_SUPPORT_LIFE: f64 = 4.0;

/// The radius of our stellar neighborhood.
///
/// This may be flexible or changed at some point, but for the time being I'm
/// thinking about fairly conventional fantasy systems where interstellar
/// travel isn't a thing.
///
/// Measured in Ly, or light years.
pub const RADIUS_OF_STELLAR_NEIGHBORHOOD: f64 = 10.0;

/// The stellar density of our (stellar) neighborhood.
///
/// As above, this is currently set to be fairly conventional.
///
/// Measured in s/ly^3, or stars per cubic light year.
pub const DENSITY_OF_STELLAR_NEIGHBORHOOD: f64 = 0.004;

/// The probability that a given star subsystem will be binary.
///
/// This probability might be slightly lower than actual.
pub const PROBABILITY_OF_BINARY_STARS: f64 = 0.25;

/// The minimum separation of binary stars, in AU.
pub const MINIMUM_BINARY_STAR_SEPARATION: f64 = 0.1;

/// The minimum average separation of "close" binary stars, in AU.
pub const MINIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION: f64 = 0.15;

/// The maximum average separation of "close" binary stars, in AU.
pub const MAXIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION: f64 = 2.0;

/// The minimum orbital eccentricity of "close" binary stars (unitless).
pub const MINIMUM_CLOSE_BINARY_STAR_ORBITAL_ECCENTRICITY: f64 = 0.4;

/// The maximum orbital eccentricity of "close" binary stars (unitless).
pub const MAXIMUM_CLOSE_BINARY_STAR_ORBITAL_ECCENTRICITY: f64 = 0.7;

/// The maximum maximum separation of "close" habitable binary stars, in AU.
pub const MAXIMUM_HABITABLE_CLOSE_BINARY_STAR_SEPARATION: f64 = 6.0;

/// The minimum average separation of "distant" binary stars, in AU.
pub const MINIMUM_DISTANT_BINARY_STAR_AVERAGE_SEPARATION: f64 = 120.0;

/// The maximum average separation of "distant" binary stars, in AU.
pub const MAXIMUM_DISTANT_BINARY_STAR_AVERAGE_SEPARATION: f64 = 600.0;

/// The minimum orbital eccentricity of "distant" binary stars (unitless).
pub const MINIMUM_DISTANT_BINARY_STAR_ORBITAL_ECCENTRICITY: f64 = 0.4;

/// The maximum orbital eccentricity of "distant" binary stars (unitless).
pub const MAXIMUM_DISTANT_BINARY_STAR_ORBITAL_ECCENTRICITY: f64 = 0.7;

/// Maximum subsystem nesting.
pub const MAXIMUM_STAR_SUBSYSTEM_RECURSION: u8 = 2;
