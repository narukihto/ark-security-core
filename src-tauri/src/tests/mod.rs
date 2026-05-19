// tests/mod.rs

// Register individual integration test suites for core and protocols
pub mod integration_core;
pub mod integration_protocols;

/// Global helper function to initialize a standard clean sandbox matrix for testing environments
pub fn setup_test_sandbox() {
    // Standard initialization of environmental test variables if required,
    // ensuring thread logs are collected properly during low-level execution simulations.
    std::env::set_var("RUST_BACKTRACE", "1");
}
