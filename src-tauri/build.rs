// src-tauri/build.rs

use std::io::Write;

fn main() {
    // Target OS detection via environment variables evaluated at compile time
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    if target_os == "windows" {
        // Embed the official Windows Manifest specifying forced elevation to Admin Privileges
        embed_windows_admin_manifest();
    }

    // Standard Tauri build hook initialization wrapper
    tauri_build::build();
}

/// Generates and links an application manifest file to enforce 'Run as Administrator' behavior
fn embed_windows_admin_manifest() {
    let out_dir = std::env::var("OUT_DIR").expect("Fatal: OUT_DIR environment variable is missing.");
    
    // 1. تحديد المسارات المستهدفة للمانيفست وملف الـ Resource الوسيط
    let manifest_path = std::path::Path::new(&out_dir).join("admin_privileges.manifest");
    let rc_path = std::path::Path::new(&out_dir).join("resources.rc");

    // Raw XML string definition for the execution level assembly
    let manifest_content = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
    <assemblyIdentity version="1.0.0.0" processorArchitecture="*" name="Ark.Security.Core" type="win32"/>
    <description>Ark Security Engine Low-Level Hardware Interface Subsystem</description>
    <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
        <security>
            <requestedPrivileges>
                <requestedExecutionLevel level="requireAdministrator" uiAccess="false"/>
            </requestedPrivileges>
        </security>
    </trustInfo>
</assembly>
"#;

    // Safely write the manifest file inside the transient build output directory
    let mut file = std::fs::File::create(&manifest_path)
        .expect("Fatal Build Error: Unable to create temporary application manifest file container.");
    file.write_all(manifest_content.as_bytes())
        .expect("Fatal Build Error: Failed to write embedded manifest structural payload.");

    
    let rc_content = format!(
        "1 24 \"{}\"",
        manifest_path.to_str().unwrap().replace("\\", "\\\\")
    );

    let mut rc_file = std::fs::File::create(&rc_path)
        .expect("Fatal Build Error: Unable to create temporary .rc resource file container.");
    rc_file.write_all(rc_content.as_bytes())
        .expect("Fatal Build Error: Failed to write resource layout structural payload.");

    // 3. نقوم بتمرير ملف الـ .rc المجهز لأداة embed_resource بدلاً من المانيفست مباشرة
    embed_resource::compile(&rc_path, None::<&str>);

    println!("cargo:rerun-if-changed=build.rs");
}
