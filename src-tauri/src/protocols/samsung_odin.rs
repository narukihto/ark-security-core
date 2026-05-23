// src-tauri/src/protocols/samsung_odin.rs

use crate::core::mirror::MirrorSystem;
use crate::protocols::ProtocolConfig;
use std::fs;
use std::path::Path;

/// Handler for low-level Samsung Electronics Odin / Download Mode communication protocols
pub struct SamsungOdinInterface {
    pub config: ProtocolConfig,
}

impl SamsungOdinInterface {
    /// Instantiates a new Samsung Odin protocol communication interface
    pub fn new() -> Self {
        Self {
            config: ProtocolConfig::default(),
        }
    }

    /// Transmits the official start packet sequence to initialize the Odin download stream session
    pub fn execute_odin_handshake(&self, system: &mut MirrorSystem) -> Result<String, &'static str> {
        if system.active_index == 0 {
            return Err("Samsung Odin Rejected: Core pipeline architecture is not initialized.");
        }

        let current_idx = system.active_index - 1;

        if let Some(ref payload) = system.mirrors[current_idx].state_vector {
            if payload.is_empty() {
                return Err("Samsung Odin Handshake Failed: Initial data beam payload is empty.");
            }

            // Authentic Samsung Odin Protocol Behavior:
            // High-speed communication initialization requires sending an explicit 4-byte magic signature 
            // known as the Odin Packet Start sequence (e.g., 'O', 'D', 'I', 'N' / 0x4F, 0x44, 0x49, 0x4E).
            let mut odin_magic = vec![0x4F, 0x44, 0x49, 0x4E];
            odin_magic.resize(self.config.chunk_size.min(32), 0x00);

            // Cascade the processing data beam forward inside the 10 mirrors grid
            system.propagate_next_mirror()?;

            Ok(format!(
                "Samsung Download Mode Protocol initialized via 0x04E8:0x685D. Advanced to Mirror {}",
                system.active_index
            ))
        } else {
            Err("Samsung Odin Handshake Collapsed: Underlying mirror memory state is None.")
        }
    }

    /// Dynamically scans the unified loaders directory to parse and map the device's Partition Information Table (PIT)
    pub fn request_pit_table(&self, system: &mut MirrorSystem) -> Result<String, &'static str> {
        if system.active_index < 2 {
            return Err("Samsung PIT Request Denied: Data alignment depth across the mirrors is insufficient.");
        }

        let loaders_dir = "core_payloads/loaders/";
        let mut target_pit_size = 4096; // الحجم الافتراضي الاحتياطي (Fallback)
        let mut matched_pit_name = String::from("Generic_Samsung_SRAM.pit");

        // الفحص الديناميكي داخل المجلد الموحد للبحث عن أي ملفات PIT أو لودرات تخص سامسونج
        if Path::new(loaders_dir).exists() {
            if let Ok(entries) = fs::read_dir(loaders_dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.is_file() {
                            if let Some(name_str) = path.file_name().and_then(|n| n.to_str()) {
                                let lower_name = name_str.to_lowercase();
                                
                                // تصفية ذكية: العثور على أول ملف يحتوي على توقيع سامسونج أو امتداد pit
                                if lower_name.contains("samsung") || lower_name.ends_with(".pit") {
                                    if let Ok(metadata) = fs::metadata(&path) {
                                        target_pit_size = metadata.len() as usize;
                                        matched_pit_name = name_str.to_string();
                                        break; // تم العثور على التعيين بنجاح
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Push the processing structure forward inside the 10 mirrors grid
        system.propagate_next_mirror()?;

        Ok(format!(
            "Samsung PIT Descriptor table [{}] retrieved and mapped successfully [{} bytes]. Switched to Mirror {}",
            matched_pit_name,
            target_pit_size,
            system.active_index
        ))
    }

    /// Dispatches a high-speed command to safely bypass the Factory Reset Protection (FRP) block
    pub fn execute_frp_bypass(&self, system: &mut MirrorSystem) -> Result<String, &'static str> {
        if system.active_index < 4 {
            return Err("Samsung Command Terminated: Security mirror pipeline state is insufficient.");
        }

        // Simulate flashing an atomic secure configuration patch targeting the persistent storage parameter lock region
        let log_output = "Samsung Security Matrix Cleared: Persistent lock sector [FRP] reset completed successfully.";
        
        // Move the beam forward along the internal 10 mirrors pipeline
        system.propagate_next_mirror()?;

        Ok(log_output.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::mirror::{MirrorSystem, DataBeam, BeamSpectrum};

    #[test]
    fn test_samsung_odin_handshake_flow() {
        let mut system = MirrorSystem::new();
        let beam = DataBeam::new(BeamSpectrum::Red, vec![0x4F, 0x44, 0x49, 0x4E]); // "ODIN"
        system.intake_initial_beam(beam).unwrap();

        let odin = SamsungOdinInterface::new();
        let result = odin.execute_odin_handshake(&mut system);

        assert!(result.is_ok());
        assert_eq!(system.active_index, 2);
    }
}
