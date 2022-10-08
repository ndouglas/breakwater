#![allow(unused_imports)]
use breakwater::astronomy::star_system::constraints::Constraints;
use breakwater::astronomy::star_system::error::Error;
use breakwater::astronomy::star_system::StarSystem;
///! Generates a star system and prints a little report on it.
use breakwater::*;
use rand::prelude::*;

pub struct StarSystemReporter {}

impl StarSystemReporter {
  #[named]
  pub fn new() -> Self {
    Self {}
  }

  #[named]
  pub fn report(&self, star_system: &StarSystem) {
    println!("{:#?}", star_system);
  }
}

#[named]
fn main() -> Result<(), Error> {
  init_pretty_env_logger();
  trace_enter!();
  let mut rng = rand::thread_rng();
  let constraints = Constraints::main_sequence();
  let star_system = StarSystem::get_random_constrained(&mut rng, &constraints)?;
  let reporter = StarSystemReporter::new();
  reporter.report(&star_system);
  trace_exit!();
  Ok(())
}
