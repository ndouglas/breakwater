use crate::astronomy::moon::Moon;

/// The `Moons` object is a wrapper around a list of `Moon` objects.
#[derive(Clone, Debug, PartialEq)]
pub struct Moons {
  pub moons: Vec<Moon>,
}
