#![allow(dead_code)]
// src-tauri/src/core/db_matcher.rs

use std::collections::HashMap;
use std::sync::OnceLock;

/// Supported Hardware Chipset categories within the Universal Unlocker architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChipsetPlatform {
    MediaTek,
    Qualcomm,
    Samsung,
    Apple,
    Unknown,
}

impl ChipsetPlatform {
    /// Helper to convert enum values into standardized uppercase string representations
    pub fn as_str(&self) -> &'static str {
        match self {
            ChipsetPlatform::MediaTek => "MediaTek",
            ChipsetPlatform::Qualcomm => "Qualcomm",
            ChipsetPlatform::Samsung => "Samsung",
            ChipsetPlatform::Apple => "Apple",
            ChipsetPlatform::Unknown => "NONE",
        }
    }
}

/// Structure representing metadata profile associated with matched real-world hardware
#[derive(Debug, Clone)]
pub struct TargetDevice {
    pub chipset: ChipsetPlatform,
    pub description: &'static str,
    pub primary_exploit_target: &'static str,
}

/// Structural manager of the global lookup engine
pub struct HardwareDatabase;

// Global immutable hardware matrix initialized only once at first access
static GLOBAL_HW_REGISTRY: OnceLock<HashMap<(u16, u16), TargetDevice>> = OnceLock::new();

impl HardwareDatabase {
    /// Populates and returns the synchronized global hardware lookup table
    fn init_registry() -> HashMap<(u16, u16), TargetDevice> {
        let mut registry = HashMap::new();

        // 1. MediaTek (MTK) Real-World Identifier Signatures
        registry.insert(
            (0x0E8D, 0x0003), // Standard MediaTek High-Speed Bootrom (BROM Mode)
            TargetDevice {
                chipset: ChipsetPlatform::MediaTek,
                description: "MediaTek MTK BootROM Mode (BROM Bypass Target)",
                primary_exploit_target: "BROM_HANDSHAKE_BYPASS",
            },
        );
        registry.insert(
            (0x0E8D, 0x2000), // MediaTek Preloader Mode
            TargetDevice {
                chipset: ChipsetPlatform::MediaTek,
                description: "MediaTek MTK Preloader Mode VCOM",
                primary_exploit_target: "PRELOADER_DA_INJECTION",
            },
        );

        // 2. Qualcomm Real-World Identifier Signatures
        registry.insert(
            (0x05C6, 0x9008), // Qualcomm Emergency Download Mode (EDL 9008)
            TargetDevice {
                chipset: ChipsetPlatform::Qualcomm,
                description: "Qualcomm Snapdragon Emergency Download (EDL 9008)",
                primary_exploit_target: "FIREHOSE_STREAM_INJECTION",
            },
        );

        // 3. Samsung Real-World Identifier Signatures
        registry.insert(
            (0x04E8, 0x685D), // Samsung official Download Mode (Odin Interface)
            TargetDevice {
                chipset: ChipsetPlatform::Samsung,
                description: "Samsung Electronics Mobile Download Mode (Odin Mode)",
                primary_exploit_target: "ODIN_PIT_FRP_RESET",
            },
        );
        registry.insert(
            (0x04E8, 0x6860), // Samsung MTP Mode
            TargetDevice {
                chipset: ChipsetPlatform::Samsung,
                description: "Samsung Electronics Mobile MTP Command Interface",
                primary_exploit_target: "MTP_AT_COMMAND_BYPASS",
            },
        );

        // 4. Apple Real-World Identifier Signatures (Checkm8 Compatible DFU targets)
        registry.insert(
            (0x05AC, 0x1227), // Apple Inc. Mobile Device DFU Mode (A7 to A11 Core Targets)
            TargetDevice {
                chipset: ChipsetPlatform::Apple,
                description: "Apple Inc. DFU Mode Secure Bootrom (Checkm8 Vulnerable)",
                primary_exploit_target: "CHECKM8_PONGO_STAGE1",
            },
        );

        registry
    }

    /// Matches a given raw VID and PID signature instantly using O(1) memory lookup
    pub fn match_signature(vid: u16, pid: u16) -> TargetDevice {
        let registry = GLOBAL_HW_REGISTRY.get_or_init(Self::init_registry);
        
        if let Some(matched_profile) = registry.get(&(vid, pid)) {
            matched_profile.clone()
        } else {
            TargetDevice {
                chipset: ChipsetPlatform::Unknown,
                description: "Generic/Unsupported Hardware Signature",
                primary_exploit_target: "GENERIC_USB_STREAM",
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mtk_brom_instant_matching() {
        let target = HardwareDatabase::match_signature(0x0E8D, 0x0003);
        assert_eq!(target.chipset, ChipsetPlatform::MediaTek);
        assert_eq!(target.primary_exploit_target, "BROM_HANDSHAKE_BYPASS");
    }

    #[test]
    fn test_qualcomm_edl_instant_matching() {
        let target = HardwareDatabase::match_signature(0x05C6, 0x9008);
        assert_eq!(target.chipset, ChipsetPlatform::Qualcomm);
        assert_eq!(target.primary_exploit_target, "FIREHOSE_STREAM_INJECTION");
    }

    #[test]
    fn test_unknown_signature_handling() {
        let target = HardwareDatabase::match_signature(0x1111, 0x2222);
        assert_eq!(target.chipset, ChipsetPlatform::Unknown);
        assert_eq!(target.primary_exploit_target, "GENERIC_USB_STREAM");
    }
}
