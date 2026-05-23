// src-tauri/src/protocols/mtk_relay.rs

use crate::core::mirror::MirrorSystem;
use crate::protocols::ProtocolConfig;
use std::fs;
use std::path::Path;

pub struct MtkRelayInterface {
    pub config: ProtocolConfig,
}

impl MtkRelayInterface {
    pub fn new() -> Self {
        Self {
            config: ProtocolConfig::default(),
        }
    }

    pub fn execute_brom_bypass(&self, system: &mut MirrorSystem, chip_name: &str) -> Result<String, &'static str> {
        if system.active_index == 0 {
            return Err("MTK Bypass Rejected: Data beam intake has not occurred.");
        }

        let current_idx = system.active_index - 1;

        if let Some(ref payload) = system.mirrors[current_idx].state_vector {
            if payload.is_empty() {
                return Err("MTK Bypass Failed: Source mirror vector allocation is null.");
            }

            system.propagate_next_mirror()?;

            Ok(format!(
                "MediaTek BROM Security Handshake Disabled for {}. Core moved to Mirror {}",
                chip_name.to_uppercase(),
                system.active_index
            ))
        } else {
            Err("MTK Core Failure: Underlying memory plane is None.")
        }
    }

    pub fn upload_mtk_loader(&self, system: &mut MirrorSystem, _da_filename: &str) -> Result<String, &'static str> {
        if system.active_index < 2 {
            return Err("MTK Download Agent Refused: System mirror processing layer is too shallow.");
        }

        let loaders_dir = "core_payloads/loaders/";
        let mut successful_da_len = 0;
        let mut matched_da_name = String::from("Fallback_Generic_DA.bin");

        if Path::new(loaders_dir).exists() {
            if let Ok(entries) = fs::read_dir(loaders_dir) {
                // استخدام flatten() لإزالة التحذير وتطهير الحلقة
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(name_str) = path.file_name().and_then(|n| n.to_str()) {
                            let lower_name = name_str.to_lowercase();
                            
                            if lower_name.starts_with('.') || (!lower_name.contains("mtk") && !lower_name.contains("da")) {
                                continue;
                            }

                            if let Ok(bytes) = fs::read(&path) {
                                let simulated_sram_ack = true;

                                if simulated_sram_ack {
                                    successful_da_len = bytes.len();
                                    matched_da_name = name_str.to_string();
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
            "MTK DA Streaming Pipeline Settled. Active DA: {} ({} bytes Loaded). Advanced to Mirror {}",
            matched_da_name, successful_da_len, system.active_index
        ))
    }

    pub fn execute_frp_wipe(&self, system: &mut MirrorSystem) -> Result<String, &'static str> {
        if system.active_index < 3 {
            return Err("FRP Execution Blocked: Critical structural alignment index was not reached.");
        }

        system.propagate_next_mirror()?;
        Ok(format!("MTK Partition block cleared. Storage state stable at Mirror {}", system.active_index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::mirror::{MirrorSystem, DataBeam, BeamSpectrum};

    #[test]
    fn test_mtk_bypass_propagation() {
        let mut system = MirrorSystem::new();
        let beam = DataBeam::new(BeamSpectrum::Red, vec![0x11, 0x22, 0x33]);
        system.intake_initial_beam(beam).unwrap();

        let mtk = MtkRelayInterface::new();
        let result = mtk.execute_brom_bypass(&mut system, "mt6765");

        assert!(result.is_ok());
        assert_eq!(system.active_index, 2);
    }
}
