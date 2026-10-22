//! # `chalkline`
//!
//! Interactive specification planner, ambiguity resolver, and topological task DAG compiler.

#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]

pub use chalkline_core::*;

/// Crate version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
