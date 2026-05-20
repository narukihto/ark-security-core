// src-tauri/src/core/mod.rs

// Register sub-modules under the core engine namespace
pub mod mirror;
pub mod stones;
pub mod db_matcher;

// Re-export core structures and types for cleaner external access across the application
 

#[allow(unused_imports)]
pub use db_matcher::{HardwareDatabase, TargetDevice};

#[allow(unused_imports)]
pub use mirror::{DataBeam, MirrorSystem};

#[allow(unused_imports)]
pub use stones::{BlackStoneGate, BrownStoneSanitizer};
