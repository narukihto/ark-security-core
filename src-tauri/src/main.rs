// src-tauri/src/main.rs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
mod protocols;

use std::sync::Mutex;
use tauri::State;

use crate::core::mirror::{MirrorSystem, DataBeam, BeamSpectrum};
use crate::core::stones::{BlackStoneGate, BrownStoneSanitizer};
use crate::core::db_matcher::{HardwareDatabase, TargetDevice};
use crate::protocols::mtk_relay::MtkRelayInterface;
use crate::protocols::qualcomm_edl::QualcommEdlInterface;
use crate::protocols::samsung_odin::SamsungOdinInterface;
use crate::protocols::apple_pongo::ApplePongoInterface;

/// Shared application state structure wrapped inside thread-safe containers
pub struct AppEngineState {
    pub mirror_system: Mutex<MirrorSystem>,
    pub black_stone: BlackStoneGate,
    pub brown_stone: BrownStoneSanitizer,
}

/// Tauri command to instantly match a physical device signature and initialize the data beam pipeline
#[tauri::command]
async fn initialize_hardware_beam(
    vid: u16,
    pid: u16,
    state: State<'_, AppEngineState>,
) -> Result<String, String> {
    // Check emergency brake state before parsing any commands
    if state.black_stone.is_locked() {
        return Err("Execution Denied: Black Stone Emergency Gate is active. System Frozen.".to_string());
    }

    // Query the hardware database signature mapping in O(1) time
    let matched_device: TargetDevice = HardwareDatabase::match_signature(vid, pid);
    
    let mut system = state.mirror_system.lock().map_err(|_| "Failed to acquire system lock matrix.")?;
    
    // Construct the localized data beam container and feed it to mirror 1
    let payload_metadata = format!("DEVICE_PROFILE::{:?}::{}", matched_device.chipset, matched_device.primary_exploit_target);
    let beam = DataBeam::new(BeamSpectrum::Red, payload_metadata.into_bytes());

    system.intake_initial_beam(beam).map_err(|e| e.to_string())?;

    Ok(format!(
        "Successfully locked onto target platform: {}. Pipeline initialized at Mirror 1.",
        matched_device.description
    ))
}

// =========================================================================
// MEDIATEK PIPELINE HANDSHAKERS
// =========================================================================

/// Stage 1: Core MediaTek Bootrom Bypass Hook
#[tauri::command]
async fn launch_mtk_bypass(
    state: State<'_, AppEngineState>,
    chip_name: String,
) -> Result<String, String> {
    if state.black_stone.is_locked() {
        return Err("Execution Denied: System is under emergency lockdown.".to_string());
    }

    let mut system = state.mirror_system.lock().map_err(|_| "System matrix lock collision.")?;
    let mtk = MtkRelayInterface::new();
    
    // ✅ التعديل المعماري: استخدام execute_brom_bypass بدلاً من القديمة
    mtk.execute_brom_bypass(&mut system, &chip_name)
        .map_err(|e| e.to_string())
}

/// Stage 2: Injects Custom Download Agents into Target Internal SRAM
#[tauri::command]
async fn upload_mtk_loader(
    state: State<'_, AppEngineState>,
    da_filename: String,
) -> Result<String, String> {
    if state.black_stone.is_locked() {
        return Err("Execution Denied: System is under emergency lockdown.".to_string());
    }

    let mut system = state.mirror_system.lock().map_err(|_| "System matrix lock collision.")?;
    let mtk = MtkRelayInterface::new();
    
    // ✅ التعديل المعماري: استخدام upload_mtk_loader بدلاً من القديمة
    mtk.upload_mtk_loader(&mut system, &da_filename)
        .map_err(|e| e.to_string())
}

/// Stage 3: Executes Storage Layout Alignment Updates and Wipe Configurations
#[tauri::command]
async fn wipe_mtk_frp(state: State<'_, AppEngineState>) -> Result<String, String> {
    if state.black_stone.is_locked() {
        return Err("Execution Denied: System is under emergency lockdown.".to_string());
    }

    let mut system = state.mirror_system.lock().map_err(|_| "System matrix lock collision.")?;
    let mtk = MtkRelayInterface::new();
    
    // ✅ التعديل المعماري: استخدام execute_frp_wipe بدلاً من القديمة
    mtk.execute_frp_wipe(&mut system)
        .map_err(|e| e.to_string())
}

// =========================================================================
// QUALCOMM EDL 9008 PIPELINE HANDSHAKERS
// =========================================================================

/// Stage 1: Dispatches the primary hardware synchronization and Firehose Hello structure
#[tauri::command]
async fn launch_qcom_bypass(state: State<'_, AppEngineState>) -> Result<String, String> {
    if state.black_stone.is_locked() {
        return Err("Execution Denied: System is under emergency lockdown.".to_string());
    }

    let mut system = state.mirror_system.lock().map_err(|_| "System matrix lock collision.")?;
    let qcom = QualcommEdlInterface::new();
    
    qcom.execute_edl_handshake(&mut system)
        .map_err(|e| e.to_string())
}

/// Stage 2: Streams and verifies any specified bkerler Firehose loader binary directly into device memory
#[tauri::command]
async fn upload_qcom_loader(
    state: State<'_, AppEngineState>,
    loader_filename: String,
) -> Result<String, String> {
    if state.black_stone.is_locked() {
        return Err("Execution Denied: System is under emergency lockdown.".to_string());
    }

    let mut system = state.mirror_system.lock().map_err(|_| "System matrix lock collision.")?;
    let qcom = QualcommEdlInterface::new();
    
    // ✅ التعديل المعماري: استخدام upload_firehose_loader بدلاً من القديمة
    qcom.upload_firehose_loader(&mut system, &loader_filename)
        .map_err(|e| e.to_string())
}

/// Stage 3: Clears persistent partition tables to bypass security and lock mechanisms
#[tauri::command]
async fn wipe_qcom_frp(state: State<'_, AppEngineState>) -> Result<String, String> {
    if state.black_stone.is_locked() {
        return Err("Execution Denied: System is under emergency lockdown.".to_string());
    }

    let mut system = state.mirror_system.lock().map_err(|_| "System matrix lock collision.")?;
    let qcom = QualcommEdlInterface::new();
    
    qcom.execute_storage_wipe(&mut system)
        .map_err(|e| e.to_string())
}

// =========================================================================
// LEGACY SINGLE-STAGE ENTRY POINTS
// =========================================================================

/// Legacy single-stage entry point preserved for concurrent protocol routines
#[tauri::command]
async fn execute_protocol_handshake(state: State<'_, AppEngineState>, platform: String) -> Result<String, String> {
    if state.black_stone.is_locked() {
        return Err("Execution Denied: System is under emergency lockdown.".to_string());
    }

    let mut system = state.mirror_system.lock().map_err(|_| "System matrix lock collision.")?;

    match platform.to_lowercase().as_str() {
        "samsung" => {
            let sam = SamsungOdinInterface::new();
            sam.execute_odin_handshake(&mut system).map_err(|e| e.to_string())
        },
        "apple" => {
            let apple = ApplePongoInterface::new();
            apple.execute_dfu_handshake(&mut system).map_err(|e| e.to_string())
        },
        _ => Err("Fallback wrapper only supports legacy single-stage interfaces.".to_string()),
    }
}

/// Omni-channel IPC trigger to lock onto the global Black Stone security gate
#[tauri::command]
fn trigger_black_stone_lock(state: State<'_, AppEngineState>) -> String {
    state.black_stone.activate_lock();
    "CRITICAL: Black Stone Atomic Lock Triggered. All hardware communication channels severed.".to_string()
}

/// Clear atomic signaling barriers and release secure lock layers
#[tauri::command]
async fn execute_system_reset(state: State<'_, AppEngineState>) -> Result<String, String> {
    let mut system = state.mirror_system.lock().map_err(|_| "System matrix lock collision.")?;
    
    state.black_stone.release_lock();
    let current_index = state.brown_stone.sanitize_and_rollback(&mut system);

    Ok(format!(
        "Sanitization complete. All mirror nodes cleared and zeroed. Active Index reset to position: {}",
        current_index
    ))
}

fn main() {
    tauri::Builder::default()
        .manage(AppEngineState {
            mirror_system: Mutex::new(MirrorSystem::new()),
            black_stone: BlackStoneGate::new(),
            brown_stone: BrownStoneSanitizer,
        })
        .invoke_handler(tauri::generate_handler![
            initialize_hardware_beam,
            launch_mtk_bypass,
            upload_mtk_loader,
            wipe_mtk_frp,
            launch_qcom_bypass,
            upload_qcom_loader,
            wipe_qcom_frp,
            execute_protocol_handshake,
            trigger_black_stone_lock,
            execute_system_reset
        ])
        .run(tauri::generate_context!())
        .expect("Fatal Error: An exception occurred while initializing the Tauri runtime orchestration platform.");
}
