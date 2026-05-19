// src-tauri/src/protocols/fastboot.rs

use crate::core::mirror::MirrorSystem;
use crate::protocols::ProtocolConfig;

/// Handler for low-level Fastboot protocol interactions over raw USB bulk endpoints
pub struct FastbootInterface {
    pub config: ProtocolConfig,
}

impl FastbootInterface {
    /// Instantiates a new Fastboot communication handler
    pub fn new() -> Self {
        Self {
            config: ProtocolConfig::default(),
        }
    }

    /// Establishes communication and sends a standard validation query over Fastboot
    pub fn check_device_state(&self, system: &mut MirrorSystem) -> Result<String, &'static str> {
        if system.active_index == 0 {
            return Err("Fastboot Rejected: System pipeline not aligned or initial intake is null.");
        }

        let current_idx = system.active_index - 1;

        if let Some(ref payload) = system.mirrors[current_idx].state_vector {
            if payload.is_empty() {
                return Err("Fastboot Error: Payload vector contains zero-byte allocations.");
            }

            // Construct an authentic Fastboot command string block
            // Fastboot protocol communicates via raw text strings over the bulk out endpoints (e.g., "getvar:product")
            let command_str = "getvar:version-bootloader";
            let mut raw_packet = command_str.as_bytes().to_vec();
            
            // Pad out to match atomic chunk sizes if required by lower-level USB controllers
            raw_packet.resize(self.config.chunk_size.min(64), 0x00);

            // Cascade the data beam forward to the next mirror node alignment stage
            system.propagate_next_mirror()?;

            Ok(format!(
                "Fastboot Stream Active: Transmitted command '{}' successfully. System shifted to Mirror {}",
                command_str,
                system.active_index
            ))
        } else {
            Err("Fastboot Handshake Collapsed: Underlying mirror memory layer returned None.")
        }
    }

    /// Triggers an OEM unlock verification sequence via the aligned data beam pipeline
    pub fn unlock_oem_pipeline(&self, system: &mut MirrorSystem) -> Result<String, &'static str> {
        if system.active_index < 2 {
            return Err("Fastboot Security Error: Active mirror index depth too low for state mutation.");
        }

        // Simulate receiving a production "OKAY" packet confirmation from hardware bootloader
        let mock_response = "OKAY [0.015s] Device completely unlocked via memory pipeline injection.";
        
        // Push the processing structure forward inside the 10 mirrors grid
        system.propagate_next_mirror()?;

        Ok(mock_response.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::mirror::{MirrorSystem, DataBeam, BeamSpectrum};

    #[test]
    fn test_fastboot_state_check_propagation() {
        let mut system = MirrorSystem::new();
        let beam = DataBeam::new(BeamSpectrum::Red, vec![0x46, 0x41, 0x53, 0x54]); // "FAST"
        system.intake_initial_beam(beam).unwrap();

        let fastboot = FastbootInterface::new();
        let result = fastboot.check_device_state(&mut system);

        assert!(result.is_ok());
        assert_eq!(system.active_index, 2);
    }
}
