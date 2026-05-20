// src-tauri/src/protocols/mod.rs


#![allow(dead_code)]
#![allow(unused_variables)]

// Register individual low-level device communication protocols
pub mod adb;
pub mod fastboot;
pub mod mtk_relay;
pub mod qualcomm_edl;
pub mod samsung_odin;
pub mod apple_pongo;

/// Unified configuration structure for hardware execution handshakes
pub struct ProtocolConfig {
    pub timeout_ms: u64,
    pub chunk_size: usize,
}

impl Default for ProtocolConfig {
    fn default() -> Self {
        Self {
            timeout_ms: 5000, // Standard 5-second timeout for hardware response channels
            chunk_size: 4096, // 4KB atomic buffer packet chunk sizing
        }
    }
}
