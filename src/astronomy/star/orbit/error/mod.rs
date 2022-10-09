/// Star orbit-related errors.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Unknown error.
  UnknownError,
}

honeyholt_define_brief!(Error, |error: &Error| {
  use Error::*;
  match error {
    UnknownError => "we encountered an unknown error",
  }
});
