// src-tauri/src/protocols/mod.rs

#![allow(dead_code)]
#![allow(unused_variables)]

use crate::core::mirror::MirrorSystem;
use crate::protocols::mtk_relay::MtkRelayInterface;
use std::sync::Mutex;

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

// =========================================================================
// TAURI COMMANDS BRIDGE 
// =========================================================================

#[tauri::command]
pub async fn launch_mtk_bypass(
    state: tauri::State<'_, Mutex<MirrorSystem>>,
    chip_name: String,
) -> Result<String, String> {
    let mut system = state.lock().map_err(|_| "Failed to lock MirrorSystem state.")?;
    let mtk = MtkRelayInterface::new();
    
    mtk.execute_brom_handshake(&mut system, &chip_name)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn upload_mtk_loader(
    state: tauri::State<'_, Mutex<MirrorSystem>>,
    da_filename: String,
) -> Result<String, String> {
    let mut system = state.lock().map_err(|_| "Failed to lock MirrorSystem state.")?;
    let mtk = MtkRelayInterface::new();
    
    mtk.inject_download_agent(&mut system, &da_filename)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn wipe_mtk_frp(
    state: tauri::State<'_, Mutex<MirrorSystem>>,
) -> Result<String, String> {
    let mut system = state.lock().map_err(|_| "Failed to lock MirrorSystem state.")?;
    let mtk = MtkRelayInterface::new();
    
    mtk.execute_frp_clear_sequence(&mut system)
        .map_err(|e| e.to_string())
}
