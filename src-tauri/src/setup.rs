// src-tauri/src/setup.rs

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr;

// Conditionally import Windows API development crates when compiling on Windows platforms
#[cfg(target_os = "windows")]
use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken, TOKEN_QUERY};
#[cfg(target_os = "windows")]
use windows_sys::Win32::Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION};

/// Architecture configuration wrapper for host operating system configurations
pub struct HostEnvironmentManager;

impl HostEnvironmentManager {
    /// Verifies whether the current process instance possesses elevated Administrator privileges
    /// This is an absolute hardware access prerequisite for raw Windows WinUSB/LibUSB kernel routing.
    #[cfg(target_os = "windows")]
    pub fn is_running_as_admin() -> bool {
        unsafe {
            let mut token_handle = ptr::null_mut();
            if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle) == 0 {
                return false;
            }

            let mut elevation: TOKEN_ELEVATION = std::mem::zeroed();
            let mut return_length = 0;
            
            let status = GetTokenInformation(
                token_handle,
                TokenElevation,
                &mut elevation as *mut _ as *mut _,
                std::mem::size_of::<TOKEN_ELEVATION>() as u32,
                &mut return_length,
            );

            // Strictly close the unmanaged handle to avoid kernel memory leaks
            windows_sys::Win32::Foundation::CloseHandle(token_handle);

            status != 0 && elevation.TokenIsElevated != 0
        }
    }

    /// Fallback implementation for non-Windows compilation targets during development simulations
    #[cfg(not(target_os = "windows"))]
    pub fn is_running_as_admin() -> bool {
        // Mock non-windows environments as authorized to allow pipeline compilation pass flags
        true
    }

    /// Configures essential system-level environment variables for the embedded USB driver backends
    pub fn configure_low_level_backends() -> Result<(), &'static str> {
        // Enforce optimized execution flags for multi-threaded hardware event loops
        std::env::set_var("RE_USB_BACKEND", "WINUSB");
        std::env::set_var("LIBUSB_DEBUG", "1"); // Pipe warning logs into internal telemetry
        
        Ok(())
    }

    /// Encodes a standard Rust string block into a null-terminated UTF-16 wide string pointer vector
    /// required by Win32 API kernel subroutines.
    pub fn to_wide_string(string: &str) -> Vec<u16> {
        OsStr::new(string)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_variable_configuration() {
        let setup_res = HostEnvironmentManager::configure_low_level_backends();
        assert!(setup_res.is_ok());
        
        assert_eq!(std::env::var("RE_USB_BACKEND").unwrap(), "WINUSB");
        assert_eq!(std::env::var("LIBUSB_DEBUG").unwrap(), "1");
    }

    #[test]
    fn test_wide_string_null_termination() {
        let wide_str = HostEnvironmentManager::to_wide_string("ARK_PROTOCOL");
        assert_mut_eq!(*wide_str.last().unwrap(), 0x0000); // Assert correct termination
    }
}
