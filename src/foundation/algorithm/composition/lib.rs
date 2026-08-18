//! Generic source-bound reconstruction algorithm foundation.

#[path = "adapters.rs"]
pub mod adapters;
#[path = "application/mod.rs"]
mod application;
#[path = "../domain/mod.rs"]
pub mod domain;
pub mod document;

pub use application::{create_algorithm, replay_algorithm};
pub use domain::{AlgorithmError, Settings};
