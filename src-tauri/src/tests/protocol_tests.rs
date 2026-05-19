// tests/protocol_tests.rs

use crate::core::mirror::{MirrorSystem, DataBeam, BeamSpectrum};
use crate::core::stones::{BlackStoneGate, BrownStoneSanitizer};
use crate::core::db_matcher::{HardwareDatabase, ChipsetPlatform};
use crate::protocols::mtk_relay::MtkRelayInterface;
use crate::protocols::qualcomm_edl::QualcommEdlInterface;
use crate::protocols::samsung_odin::SamsungOdinInterface;
use crate::protocols::apple_pongo::ApplePongoInterface;

#[test]
fn test_hardware_signature_matching_to_protocol_flow() {
    // Simulate detecting a real MediaTek Bootrom interface via USB subsystem
    let mtk_vid = 0x0E8D;
    let mtk_pid = 0x0003;
    
    let device_profile = HardwareDatabase::match_signature(mtk_vid, mtk_pid);
    assert_eq!(device_profile.chipset, ChipsetPlatform::MediaTek);
    assert_eq!(device_profile.primary_exploit_target, "BROM_HANDSHAKE_BYPASS");

    // Initialize the mirror architecture to stream the matched protocol
    let mut system = MirrorSystem::new();
    let initial_beam = DataBeam::new(BeamSpectrum::Red, vec![0xAA, 0xBB, 0xCC]);
    
    system.intake_initial_beam(initial_beam).unwrap();
    assert_eq!(system.active_index, 1);

    // Bind and pass execution to the MTK specialized handler
    let mtk_interface = MtkRelayInterface::new();
    let handshake_result = mtk_interface.execute_brom_handshake(&mut system);
    
    assert!(handshake_result.is_ok());
    assert_eq!(system.active_index, 2); // Core advanced cleanly to mirror stage 2
}

#[test]
fn test_qualcomm_firehose_pipeline_to_brown_stone_rollback() {
    let mut system = MirrorSystem::new();
    let gate = BlackStoneGate::new();
    let sanitizer = BrownStoneSanitizer;
    let qualcomm = QualcommEdlInterface::new();

    // Intake initial payload into mirror 1
    let beam = DataBeam::new(BeamSpectrum::Red, vec![0x11, 0x22, 0x33]);
    system.intake_initial_beam(beam).unwrap();

    // Execute stage 1: EDL Handshake
    let res1 = qualcomm.execute_edl_handshake(&mut system);
    assert!(res1.is_ok());
    assert_eq!(system.active_index, 2);

    // Execute stage 2: Load Firehose Programmer
    let res2 = qualcomm.load_firehose_programmer(&mut system, "prog_emmc_firehose_8953.mbn");
    assert!(res2.is_ok());
    assert_eq!(system.active_index, 3);

    // Simulate an unexpected hardware detachment or transmission fault (Null Vector Panic)
    // Forcibly corrupt the pipeline stability to trigger Brown Stone emergency response
    gate.activate_lock();
    assert!(gate.is_locked());

    if gate.is_locked() {
        // Invoke absolute destructive purge and memory sanitization rollback
        let final_index = sanitizer.sanitize_and_rollback(&mut system);
        
        assert_eq!(final_index, 0, "System index must reset to absolute origin.");
        assert_eq!(system.active_index, 0);
        
        // Assert deep structural cleanup: Check that memory arrays are zeroed and dropped
        for i in 0..10 {
            assert!(
                system.mirrors[i].state_vector.is_none(),
                "Residual memory leak detected in mirror node {} during hardware failure rollback.",
                i + 1
            );
        }
    }
}

#[test]
fn test_samsung_and_apple_boundary_constraints() {
    let mut system = MirrorSystem::new();
    let samsung = SamsungOdinInterface::new();
    let apple = ApplePongoInterface::new();

    // Try executing advanced procedures prior to system alignment mapping intake
    let early_odin_res = samsung.execute_frp_bypass(&mut system);
    let early_pongo_res = apple.upload_pongo_stage(&mut system, 1024);

    assert!(early_odin_res.is_err(), "Pipeline protection breach: Allowed operation before intake initialization.");
    assert!(early_pongo_res.is_err(), "Pipeline protection breach: Allowed operation before intake initialization.");

    // Populate system to stage 1
    let beam = DataBeam::new(BeamSpectrum::White, vec![0x90, 0x90, 0x90]);
    system.intake_initial_beam(beam).unwrap();

    // Advanced shell commands must fail at shallow index alignments (active_index < 4)
    let dynamic_cmd_res = apple.execute_pongo_shell_command(&mut system, "bootx");
    assert!(dynamic_cmd_res.is_err(), "Security alignment rule violated: Shell command executed at unsafe mirror depth.");
}
