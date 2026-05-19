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
    // Step 1: Check emergency brake state before parsing any commands
    if state.black_stone.is_locked() {
        return Err("Execution Denied: Black Stone Emergency Gate is active. System Frozen.".to_string());
    }

    // Step 2: Query the hardware database signature mapping in O(1) time
    let matched_device: TargetDevice = HardwareDatabase::match_signature(vid, pid);
    
    let mut system = state.mirror_system.lock().map_err(|_| "Failed to acquire system lock matrix.")?;
    
    // Step 3: Construct the localized data beam container and feed it to mirror 1
    let payload_metadata = format!("DEVICE_PROFILE::{:?}::{}", matched_device.chipset, matched_device.primary_exploit_target);
    let beam = DataBeam::new(BeamSpectrum::Red, payload_metadata.into_bytes());

    system.intake_initial_beam(beam).map_err(|e| e.to_string())?;

    Ok(format!(
        "Successfully locked onto target platform: {}. Pipeline initialized at Mirror 1.",
        matched_device.description
    ))
}

/// Tauri command to stream the appropriate low-level protocol sequence based on the active mirror depth
#[tauri::command]
async fn execute_protocol_handshake(state: State<'_, AppEngineState>, platform: String) -> Result<String, String> {
    if state.black_stone.is_locked() {
        return Err("Execution Denied: System is under emergency lockdown.".to_string());
    }

    let mut system = state.mirror_system.lock().map_err(|_| "System matrix lock collision.")?;

    // Route execution dynamically to the proper protocol interface suite
    match platform.to_lowercase().as_str() {
        "mediatek" => {
            let mtk = MtkRelayInterface::new();
            mtk.execute_brom_handshake(&mut system).map_err(|e| e.to_string())
        },
        "qualcomm" => {
            let qualcomm = QualcommEdlInterface::new();
            qualcomm.execute_edl_handshake(&mut system).map_err(|e| e.to_string())
        },
        "samsung" => {
            let sam = SamsungOdinInterface::new();
            sam.execute_odin_handshake(&mut system).map_err(|e| e.to_string())
        },
        "apple" => {
            let apple = ApplePongoInterface::new();
            apple.execute_dfu_handshake(&mut system).map_err(|e| e.to_string())
        },
        _ => Err("Unsupported protocol architecture configuration requested.".to_string()),
    }
}

/// Tauri command providing an instant API bridge to trigger the global Black Stone Emergency Brake
#[tauri::command]
fn trigger_emergency_brake(state: State<'_, AppEngineState>) -> String {
    state.black_stone.activate_lock();
    "CRITICAL: Black Stone Atomic Lock Triggered. All hardware communication channels severed.".to_string()
}

/// Tauri command to release emergency locks, sanitize cache registers, and perform a full hard rollback
#[tauri::command]
async fn execute_system_reset(state: State<'_, AppEngineState>) -> Result<String, String> {
    let mut system = state.mirror_system.lock().map_err(|_| "System matrix lock collision.")?;
    
    // 1. Clear atomic signaling barriers
    state.black_stone.release_lock();

    // 2. Erase memory registers physically via the Brown Stone Sanitizer
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
            execute_protocol_handshake,
            trigger_emergency_brake,
            execute_system_reset
        ])
        .run(tauri::generate_context!())
        .expect("Fatal Error: An exception occurred while initializing the Tauri runtime orchestration platform.");
}
