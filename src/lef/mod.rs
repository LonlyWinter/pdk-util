//! LEF file

pub mod macros;

#[cfg(feature = "lef")]
pub mod meta;

#[cfg(feature = "lef")]
pub use meta::{LibLef, TechLef};

#[cfg(all(feature = "lef", test))]
mod test;
