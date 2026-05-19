// src-tauri/src/core/mirror.rs

use std::mem;

/// Represents the color/dimension spectrum of the data beam
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BeamSpectrum {
    Red,   // Normal linear 1:1 processing
    White, // Secure intensive processing (Dimensional Folding: 10 atomic ops in 1)
}

/// The atomic vehicle for transferring payload between modules using strict move semantics
#[derive(Debug)]
pub struct DataBeam {
    pub spectrum: BeamSpectrum,
    pub payload: Option<Vec<u8>>,
}

impl DataBeam {
    /// Instantiates a new data beam with an initial cryptographic or device payload
    pub fn new(spectrum: BeamSpectrum, data: Vec<u8>) -> Self {
        Self {
            spectrum,
            payload: Some(data),
        }
    }

    /// Destructively reads the payload out of the beam, leaving the source as None instantly
    pub fn beam_transfer(&mut self) -> Option<Vec<u8>> {
        mem::take(&mut self.payload)
    }
}

/// Structure representing an individual mirror node within the 10th Mirror Architecture
#[derive(Debug, Default)]
pub struct MirrorNode {
    pub state_vector: Option<Vec<u8>>,
}

/// Orchestrator for the entire 10 Mirrors alignment protocol
pub struct MirrorSystem {
    pub mirrors: [MirrorNode; 10],
    pub active_index: usize,
}

impl MirrorSystem {
    /// Initializes a clean mirror system array with the active pointer set to the first mirror
    pub fn new() -> Self {
        Self {
            mirrors: Default::default(),
            active_index: 0,
        }
    }

    /// Feeds the initial data beam into Mirror 1 to kick off the pipeline alignment
    pub fn intake_initial_beam(&mut self, mut beam: DataBeam) -> Result<(), &'static str> {
        let extracted_data = beam.beam_transfer();
        if let Some(data) = extracted_data {
            self.mirrors[0].state_vector = Some(data);
            self.active_index = 1;
            Ok(())
        } else {
            Err("Intake failure: Data beam was inherently empty (Null Radiation).")
        }
    }

    /// Propagates the data from Mirror(N) to Mirror(N+1) using State Blending and Move Semantics
    pub fn propagate_next_mirror(&mut self) -> Result<usize, &'static str> {
        if self.active_index == 0 || self.active_index >= 10 {
            return Err("Propagation error: Out of mirror boundary alignments or pipeline not initialized.");
        }

        let current_idx = self.active_index - 1;
        let next_idx = self.active_index;

        // Perform strict destructive read from current mirror node (Mirror Reset)
        let source_data = mem::take(&mut self.mirrors[current_idx].state_vector);
        
        if let Some(mut data) = source_data {
            // State Blending Logic: Instead of overwriting, blend via a mathematical resultant vector transformation
            if let Some(ref absolute_state) = self.mirrors[next_idx].state_vector {
                data = self.blend_vectors(&data, absolute_state);
            }

            // Dimensional Folding Check for White Beam optimization inside the processing matrix
            let processed_data = self.execute_dimensional_folding(data);

            // Commit to the next mirror node alignment
            self.mirrors[next_idx].state_vector = Some(processed_data);
            self.active_index += 1;

            Ok(self.active_index)
        } else {
            Err("Propagation collapsed: Current active mirror state vector was unexpectedly found None.")
        }
    }

    /// Blends two discrete vectors using Bitwise XOR state calculation to form the resultant vector
    fn blend_vectors(&self, current: &[u8], next: &[u8]) -> Vec<u8> {
        current.iter()
        .zip(next.iter().cycle())
        .map(|(&c_byte, &n_byte)| c_byte ^ n_byte)
        .collect()
    }

    /// Implements Dimensional Folding. White Beam forces 10 internal atomic mathematical transformations.
    fn execute_dimensional_folding(&self, mut data: Vec<u8>) -> Vec<u8> {
        // Mocking an intensive 10-fold atomic matrix processing block
        for iteration in 0..10 {
            for byte in data.iter_mut() {
                *byte = byte.wrapping_add(iteration as u8).rotate_left(1);
            }
        }
        data
    }

    /// Forces an absolute manual verification at Mirror 10. True means aligned, False invokes Brown Stone.
    pub fn verify_final_alignment(&self) -> bool {
        if self.active_index != 10 {
            return false;
        }
        
        if let Some(ref final_vector) = self.mirrors[9].state_vector {
            // Integrity checklist block: Validate structural correctness of the final resultant vector
            !final_vector.is_empty() && final_vector.iter().all(|&b| b != 0x00)
        } else {
            false
        }
    }
}
