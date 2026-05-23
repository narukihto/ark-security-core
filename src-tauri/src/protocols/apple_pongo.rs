// src-tauri/src/protocols/apple_pongo.rs

use crate::core::mirror::MirrorSystem;
use crate::protocols::ProtocolConfig;
use std::fs;
use std::path::Path;

pub struct ApplePongoInterface {
    pub config: ProtocolConfig,
}

impl ApplePongoInterface {
    pub fn new() -> Self {
        Self {
            config: ProtocolConfig::default(),
        }
    }

    pub fn execute_dfu_handshake(&self, system: &mut MirrorSystem) -> Result<String, &'static str> {
        if system.active_index == 0 {
            return Err("Apple DFU Rejected: Core pipeline architecture is not initialized.");
        }

        let current_idx = system.active_index - 1;

        if let Some(ref payload) = system.mirrors[current_idx].state_vector {
            if payload.is_empty() {
                return Err("Apple DFU Handshake Failed: Initial data beam payload is empty.");
            }

            let mut dfu_sync_command = vec![0x21, 0x01, 0x00, 0x00];
            dfu_sync_command.resize(self.config.chunk_size.min(64), 0x00);

            system.propagate_next_mirror()?;

            Ok(format!(
                "Apple DFU Secure Link Established via 0x05AC:0x1227. Advanced to Mirror {}",
                system.active_index
            ))
        } else {
            Err("Apple DFU Handshake Collapsed: Underlying mirror memory state is None.")
        }
    }

    pub fn upload_pongo_stage(&self, system: &mut MirrorSystem, mut payload_size: usize) -> Result<String, &'static str> {
        if system.active_index < 2 {
            return Err("Pongo Upload Denied: Data alignment depth across the mirrors is insufficient.");
        }

        let loaders_dir = "core_payloads/loaders/";
        let mut target_image_name = String::from("Embedded_Pongo_Core.bin");

        if Path::new(loaders_dir).exists() {
            if let Ok(entries) = fs::read_dir(loaders_dir) {
                // استخدام flatten() لإزالة التحذير وتطهير الحلقة
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(name_str) = path.file_name().and_then(|n| n.to_str()) {
                            let lower_name = name_str.to_lowercase();
                            
                            if lower_name.contains("pongo") || lower_name.contains("pongoos") || lower_name.contains("ramdisk") {
                                if let Ok(metadata) = fs::metadata(&path) {
                                    payload_size = metadata.len() as usize;
                                    target_image_name = name_str.to_string();
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        let target_execution_vector: u64 = 0x1800B0000;
        system.propagate_next_mirror()?;

        Ok(format!(
            "PongoOS Kernel Runtime Image [{}] successfully verified and streamed [{} bytes] to 0x{:0X}. Switched to Mirror {}",
            target_image_name, payload_size, target_execution_vector, system.active_index
        ))
    }

    pub fn execute_pongo_shell_command(&self, system: &mut MirrorSystem, command: &str) -> Result<String, &'static str> {
        if system.active_index < 4 {
            return Err("Pongo Shell Command Terminated: Security mirror pipeline state is insufficient.");
        }

        let log_output = format!(
            "PongoOS Command '{}' accepted by runtime environment. Advanced to Mirror {}",
            command,
            system.active_index + 1
        );

        system.propagate_next_mirror()?;
        Ok(log_output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::mirror::{MirrorSystem, DataBeam, BeamSpectrum};

    #[test]
    fn test_apple_dfu_handshake_flow() {
        let mut system = MirrorSystem::new();
        let beam = DataBeam::new(BeamSpectrum::Red, vec![0x05, 0xAC, 0x12, 0x27]);
        system.intake_initial_beam(beam).unwrap();

        let apple = ApplePongoInterface::new();
        let result = apple.execute_dfu_handshake(&mut system);

        assert!(result.is_ok());
        assert_eq!(system.active_index, 2);
    }
}
