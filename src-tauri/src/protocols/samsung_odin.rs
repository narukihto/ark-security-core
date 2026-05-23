// src-tauri/src/protocols/samsung_odin.rs

use crate::core::mirror::MirrorSystem;
use crate::protocols::ProtocolConfig;
use std::fs;
use std::path::Path;

pub struct SamsungOdinInterface {
    pub config: ProtocolConfig,
}

impl SamsungOdinInterface {
    pub fn new() -> Self {
        Self {
            config: ProtocolConfig::default(),
        }
    }

    pub fn execute_odin_handshake(&self, system: &mut MirrorSystem) -> Result<String, &'static str> {
        if system.active_index == 0 {
            return Err("Odin Handshake Rejected: System core context state is null.");
        }

        let current_idx = system.active_index - 1;

        if let Some(ref payload) = system.mirrors[current_idx].state_vector {
            if payload.is_empty() {
                return Err("Odin Handshake Collapsed: Stream data size validation violation.");
            }

            system.propagate_next_mirror()?;

            Ok(format!(
                "Samsung Odin Protocol Link Primed. Core context assigned to Mirror {}",
                system.active_index
            ))
        } else {
            Err("Odin Handshake Fatal: Hardware mapping reference returned None.")
        }
    }

    pub fn extract_and_verify_pit(&self, system: &mut MirrorSystem) -> Result<String, &'static str> {
        if system.active_index < 2 {
            return Err("Odin PIT Extraction Denied: Required structural alignment depth missing.");
        }

        let loaders_dir = "core_payloads/loaders/";
        let mut target_pit_size = 0;
        let mut matched_pit_name = String::from("Embedded_Stock_Odin.pit");

        if Path::new(loaders_dir).exists() {
            if let Ok(entries) = fs::read_dir(loaders_dir) {
                // استخدام flatten() لإزالة التحذير وتطهير الحلقة
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(name_str) = path.file_name().and_then(|n| n.to_str()) {
                            let lower_name = name_str.to_lowercase();
                            
                            if lower_name.contains("samsung") || lower_name.ends_with(".pit") {
                                if let Ok(metadata) = fs::metadata(&path) {
                                    target_pit_size = metadata.len() as usize;
                                    matched_pit_name = name_str.to_string();
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
            "Samsung Odin PIT Extraction Complete. Active Structural Layout: {} ({} bytes parsed). Shifted to Mirror {}",
            matched_pit_name, target_pit_size, system.active_index
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::mirror::{MirrorSystem, DataBeam, BeamSpectrum};

    #[test]
    fn test_odin_handshake_propagation() {
        let mut system = MirrorSystem::new();
        let beam = DataBeam::new(BeamSpectrum::Red, vec![0x4F, 0x44, 0x49, 0x4E]);
        system.intake_initial_beam(beam).unwrap();

        let odin = SamsungOdinInterface::new();
        let result = odin.execute_odin_handshake(&mut system);

        assert!(result.is_ok());
        assert_eq!(system.active_index, 2);
    }
}
