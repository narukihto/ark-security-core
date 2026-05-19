// src-tauri/src/core/stones.rs

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use crate::core::mirror::MirrorSystem;

/// The Black Stone Gate: Global Emergency Brake using high-speed atomic signaling
#[derive(Debug, Clone)]
pub struct BlackStoneGate {
    // Atomic synchronization primitive to instantly broadcast lock state across background execution threads
    lock_signal: Arc<AtomicBool>,
}

impl BlackStoneGate {
    /// Instantiates a clear, unlocked Black Stone security gate
    pub fn new() -> Self {
        Self {
            lock_signal: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Triggers the emergency lock state, instantly freezing all data beam routing modules
    pub fn activate_lock(&self) {
        self.lock_signal.store(true, Ordering::SeqCst);
    }

    /// Releases the global freeze state to allow pipeline re-initialization
    pub fn release_lock(&self) {
        self.lock_signal.store(false, Ordering::SeqCst);
    }

    /// Returns whether the system is under an active emergency lock condition
    pub fn is_locked(&self) -> bool {
        self.lock_signal.load(Ordering::SeqCst)
    }
}

/// The Brown Stone Sanitizer: Controls hard automated rollback operations and data sanitization
pub struct BrownStoneSanitizer;

impl BrownStoneSanitizer {
    /// Sanitizes all active mirror state vectors via destructive overwriting, then drops the memory allocation
    pub fn sanitize_and_rollback(&self, system: &mut MirrorSystem) -> usize {
        // Step 1: Iterate over every single mirror block to scrub memory traces
        for mirror in system.mirrors.iter_mut() {
            if let Some(ref mut vector) = mirror.state_vector {
                // Cryptographic Sanitization: Overwrite the contents with standard zeroed memory buffers
                // This acts as a manual fallback to prevent data leakages in lower level registers
                for byte in vector.iter_mut() {
                    *byte = 0x00;
                }
                
                // Forcibly clear and free the allocated heap structure capacity
                vector.clear();
                vector.shrink_to_fit();
            }
            // Destructively reset the Option block back to absolute None
            mirror.state_vector = None;
        }

        // Step 2: Forcibly reset the operational execution index back to Mirror 1 layout position
        system.active_index = 0;
        
        // Return the fresh state position (0 indicating completely rolled back and empty)
        system.active_index
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::mirror::{MirrorSystem, DataBeam, BeamSpectrum};

    #[test]
    fn test_black_stone_atomic_lock() {
        let gate = BlackStoneGate::new();
        assert!(!gate.is_locked());
        
        gate.activate_lock();
        assert!(gate.is_locked());
        
        gate.release_lock();
        assert!(!gate.is_locked());
    }

    #[test]
    fn test_brown_stone_sanitization_flow() {
        let mut system = MirrorSystem::new();
        let beam = DataBeam::new(BeamSpectrum::Red, vec![0xAA, 0xBB, 0xCC]);
        
        system.intake_initial_beam(beam).unwrap();
        assert_eq!(system.active_index, 1);
        assert!(system.mirrors[0].state_vector.is_some());

        // Trigger manual Brown Stone Rollback execution
        let sanitizer = BrownStoneSanitizer;
        let fresh_index = sanitizer.sanitize_and_rollback(&mut system);

        assert_eq!(fresh_index, 0);
        assert_eq!(system.active_index, 0);
        assert!(system.mirrors[0].state_vector.is_none());
    }
}
