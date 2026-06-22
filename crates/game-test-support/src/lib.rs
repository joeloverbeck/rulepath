//! Development-only evidence helpers for Rulepath game tests.
//!
//! This crate is shared test infrastructure. It must stay out of normal and
//! build dependency graphs for production crates; `scripts/boundary-check.sh`
//! enforces that boundary.

pub mod no_leak;
pub mod profiles;
