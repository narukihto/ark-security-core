// tests/mirror_tests.rs

use crate::core::mirror::{MirrorSystem, DataBeam, BeamSpectrum};
use crate::core::stones::{BlackStoneGate, BrownStoneSanitizer};

#[test]
fn test_data_beam_destructive_read_and_zero_copy() {
    // Step 1: Initialize a raw binary vector array representing an active payload
    let raw_payload = vec![0xDE, 0xAD, 0xBE, 0xEF, 0xAA, 0xBB, 0xCC, 0xDD];
    let original_ptr = raw_payload.as_ptr(); // Track the underlying physical heap address
    
    // Step 2: Encapsulate the payload inside a new high-performance Red DataBeam
    let mut beam = DataBeam::new(BeamSpectrum::Red, raw_payload);
    assert!(beam.payload.is_some(), "DataBeam must encapsulate the payload upon instantiation.");

    // Step 3: Perform the atomic destructive beam transfer execution
    let extracted_payload = beam.beam_transfer();
    
    // Assert structural integrity and absolute data removal from the source vehicle
    assert!(beam.payload.is_none(), "Source payload wrapper MUST be instantly None after transfer.");
    assert!(extracted_payload.is_some(), "Extracted target container must receive the full payload allocation.");
    
    let verified_data = extracted_payload.unwrap();
    assert_eq!(verified_data.as_ptr(), original_ptr, "CRITICAL: Memory pointer mismatch! Zero-copy optimization failed.");
    assert_eq!(verified_data[0], 0xDE);
}

#[test]
fn test_ten_mirrors_pipeline_propagation_lifecycle() {
    let mut system = MirrorSystem::new();
    let initial_data = vec![0x01, 0x02, 0x03, 0x04];
    let beam = DataBeam::new(BeamSpectrum::Red, initial_data);

    // Intake verification
    let intake_res = system.intake_initial_beam(beam);
    assert!(intake_res.is_ok(), "Initial beam intake into Mirror 1 failed.");
    assert_eq!(system.active_index, 1, "Active system alignment index must be 1 after intake.");
    assert!(system.mirrors[0].state_vector.is_some(), "Mirror 1 must hold the state vector.");

    // Sequential alignment propagation verification through the remaining mirrors
    for expected_index in 2..=10 {
        let current_source = expected_index - 2;
        let propagation_res = system.propagate_next_mirror();
        
        assert!(propagation_res.is_ok(), "Propagation collapsed during mirror transition chain.");
        assert_eq!(system.active_index, expected_index, "Active system tracker misaligned.");
        
        // Destructive verification check: The previous node MUST be left as completely None
        assert!(
            system.mirrors[current_source].state_vector.is_none(),
            "Memory leak found: Mirror node {} was not zeroed out post-propagation.",
            current_source + 1
        );
    }

    // Verify system boundary conditions (Should fail to propagate past Mirror 10)
    let boundary_res = system.propagate_next_mirror();
    assert!(boundary_res.is_err(), "Security Breach: System allowed propagation outside the 10th mirror boundary.");
}

#[test]
fn test_white_beam_dimensional_folding_transformation() {
    let mut system = MirrorSystem::new();
    let input_bytes = vec![0x10, 0x20, 0x30];
    
    // Process input data manually using the system's internal folding engine
    let transformed_bytes = system.execute_dimensional_folding(input_bytes.clone());
    
    assert_ne!(input_bytes, transformed_bytes, "Dimensional folding failed to mutate the state layout matrix.");
    assert_eq!(input_bytes.len(), transformed_bytes.len(), "Dimensional folding altered data length constraints.");
}

#[test]
fn test_emergency_interruption_matrix_and_sanitization() {
    let mut system = MirrorSystem::new();
    let gate = BlackStoneGate::new();
    let sanitizer = BrownStoneSanitizer;

    let beam = DataBeam::new(BeamSpectrum::Red, vec![0x99, 0x88, 0x77]);
    system.intake_initial_beam(beam).unwrap();
    system.propagate_next_mirror().unwrap(); // Now at Mirror 2

    // Trigger atomic Black Stone Gate lock simulation
    assert!(!gate.is_locked());
    gate.activate_lock();
    assert!(gate.is_locked(), "Black Stone Gate failed to activate security lock.");

    // If locked, simulate full brown stone destructive rollback execution
    if gate.is_locked() {
        let rolled_index = sanitizer.sanitize_and_rollback(&mut system);
        assert_eq!(rolled_index, 0, "Brown Stone failed to reset active system index tracking.");
        assert_eq!(system.active_index, 0);
        
        // Ensure every single mirror node is absolutely clean
        for i in 0..10 {
            assert!(
                system.mirrors[i].state_vector.is_none(),
                "Memory Sanitization Violation: Residual data left inside Mirror {} post Rollback.",
                i + 1
            );
        }
    }
}
