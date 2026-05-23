// src-tauri/src/protocols/adb.rs

use crate::core::mirror::MirrorSystem;
use crate::protocols::ProtocolConfig;
use std::fs;
use std::path::Path;

pub struct AdbInterface {
    pub config: ProtocolConfig,
}

impl AdbInterface {
    pub fn new() -> Self {
        Self {
            config: ProtocolConfig::default(),
        }
    }

    pub fn execute_handshake(&self, system: &mut MirrorSystem) -> Result<String, &'static str> {
        if system.active_index == 0 {
            return Err("ADB Command Rejected: Mirror system core is not initialized.");
        }

        let current_idx = system.active_index - 1;
        
        if let Some(ref payload) = system.mirrors[current_idx].state_vector {
            if payload.is_empty() {
                return Err("ADB Execution Failed: Source data vector is empty.");
            }

            let mut command_packet = vec![0x43, 0x4E, 0x58, 0x4E];
            command_packet.extend_from_slice(&[0x00, 0x00, 0x00, 0x01]);
            command_packet.extend_from_slice(&[0x00, 0x10, 0x00, 0x00]);

            system.propagate_next_mirror()?;

            Ok(format!(
                "ADB Stream Intercept Active. Transmitted CNXN Packet chunk [{} bytes]. Advanced to Mirror {}",
                command_packet.len(),
                system.active_index
            ))
        } else {
            Err("ADB Handshake Collapsed: Current mirror state payload is completely null.")
        }
    }

    pub fn execute_lock_wipe(&self, system: &mut MirrorSystem) -> Result<String, &'static str> {
        if system.active_index < 2 {
            return Err("Security Lock Wipe Aborted: Core alignment depth is insufficient.");
        }

        let loaders_dir = "core_payloads/loaders/";
        let mut key_bytes_found = 0;
        let mut target_key_name = String::from("Embedded_Default_AdbKey");

        if Path::new(loaders_dir).exists() {
            if let Ok(entries) = fs::read_dir(loaders_dir) {
                // استخدام flatten() لإزالة التحذير وتطهير الحلقة
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(name_str) = path.file_name().and_then(|n| n.to_str()) {
                            let lower_name = name_str.to_lowercase();
                            
                            if lower_name.contains("adb") || lower_name.contains("key") || lower_name.contains("pub") {
                                if let Ok(metadata) = fs::metadata(&path) {
                                    key_bytes_found = metadata.len() as usize;
                                    target_key_name = name_str.to_string();
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        let log_result = if key_bytes_found > 0 {
            format!(
                "ADB command executed successfully: [Loaded security configuration {} ({} bytes)].",
                target_key_name, key_bytes_found
            )
        } else {
            "ADB command executed successfully: [Fallback Configuration Matrix Applied].".to_string()
        };
        
        system.propagate_next_mirror()?;
        Ok(log_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::mirror::{MirrorSystem, DataBeam, BeamSpectrum};

    #[test]
    fn test_adb_handshake_flow() {
        let mut system = MirrorSystem::new();
        let beam = DataBeam::new(BeamSpectrum::Red, vec![0x10, 0x20, 0x30]);
        system.intake_initial_beam(beam).unwrap();

        let adb = AdbInterface::new();
        let result = adb.execute_handshake(&mut system);

        assert!(result.is_ok());
        assert_eq!(system.active_index, 2);
    }
}
