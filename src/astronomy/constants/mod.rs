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
pub const MINIMUM_BINARY_STAR_SEPARATION: f64 = 0.04;

/// The minimum average separation of "close" binary stars, in AU.
pub const MINIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION: f64 = 0.1;

/// The maximum average separation of "close" binary stars, in AU.
/// I've adjusted this down from about 2AU to its present number because the
/// failure rate is ridiculously high otherwise.
pub const MAXIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION: f64 = 0.20;

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
pub const MAXIMUM_STAR_SUBSYSTEM_RECURSION: u8 = 3;
