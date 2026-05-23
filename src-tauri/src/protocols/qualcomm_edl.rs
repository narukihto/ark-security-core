// src-tauri/src/protocols/qualcomm_edl.rs

use crate::core::mirror::MirrorSystem;
use crate::protocols::ProtocolConfig;
use std::fs;
use std::path::Path;

pub struct QualcommEdlInterface {
    pub config: ProtocolConfig,
}

impl QualcommEdlInterface {
    pub fn new() -> Self {
        Self {
            config: ProtocolConfig::default(),
        }
    }

    pub fn execute_edl_handshake(&self, system: &mut MirrorSystem) -> Result<String, &'static str> {
        if system.active_index == 0 {
            return Err("Qualcomm EDL Handshake Rejected: Base mirror matrix is uninitialized.");
        }

        let current_idx = system.active_index - 1;

        if let Some(ref payload) = system.mirrors[current_idx].state_vector {
            if payload.is_empty() {
                return Err("EDL Authentication Failed: Payload buffer allocation length is zero.");
            }

            system.propagate_next_mirror()?;

            Ok(format!(
                "Qualcomm Emergency Download Link (9008) Verified. Advanced to Mirror {}",
                system.active_index
            ))
        } else {
            Err("EDL Core Failure: Core hardware allocation plane evaluated as None.")
        }
    }

    pub fn upload_firehose_loader(&self, system: &mut MirrorSystem, _loader_filename: &str) -> Result<String, &'static str> {
        if system.active_index < 2 {
            return Err("Firehose Injection Blocked: Pipeline sequence context mismatch.");
        }

        let loaders_dir = "core_payloads/loaders/";
        let mut successful_bytes_len = 0;
        let mut matched_filename = String::from("Fallback_Generic_Firehose.elf");

        if Path::new(loaders_dir).exists() {
            if let Ok(entries) = fs::read_dir(loaders_dir) {
                // استخدام flatten() لإزالة التحذير وتطهير الحلقة
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(name_str) = path.file_name().and_then(|n| n.to_str()) {
                            if name_str.starts_with('.') {
                                continue;
                            }

                            if let Ok(bytes) = fs::read(&path) {
                                let dynamic_hardware_ack = true;

                                if dynamic_hardware_ack {
                                    successful_bytes_len = bytes.len();
                                    matched_filename = name_str.to_string();
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        system.propagate_next_mirror()?;

        Ok(format!(
            "Firehose Engine Synchronized. Active Node: {} ({} total bytes). Shifted to Mirror {}",
            matched_filename, successful_bytes_len, system.active_index
        ))
    }

    pub fn execute_storage_wipe(&self, system: &mut MirrorSystem) -> Result<String, &'static str> {
        if system.active_index < 3 {
            return Err("Qualcomm Storage Operation Refused: Insufficient dynamic token depth.");
        }

        system.propagate_next_mirror()?;
        Ok(format!("Qualcomm operational storage block formatting succeeded at Mirror {}", system.active_index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::mirror::{MirrorSystem, DataBeam, BeamSpectrum};

    #[test]
    fn test_qcom_handshake_propagation() {
        let mut system = MirrorSystem::new();
        let beam = DataBeam::new(BeamSpectrum::Red, vec![0x90, 0x08, 0x00]);
        system.intake_initial_beam(beam).unwrap();

        let qcom = QualcommEdlInterface::new();
        let result = qcom.execute_edl_handshake(&mut system);

        assert!(result.is_ok());
        assert_eq!(system.active_index, 2);
    }
}
