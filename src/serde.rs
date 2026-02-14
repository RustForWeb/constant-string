//! Serde integration.

use std::fmt;

use serde::de::{Error, Unexpected, Visitor};

/// Serde visitor for a static string.
///
/// Based on `MustBeStrVisitor` from [`monostate`](https://crates.io/crates/monostate).
pub struct MustBeStrVisitor(pub &'static str);

impl<'de> Visitor<'de> for MustBeStrVisitor {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{:?}", self.0)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if v == self.0 {
            Ok(())
        } else {
            Err(E::invalid_value(Unexpected::Str(v), &self))
        }
    }
}
