// ============================================================================
// Agent Status UI Module
// ============================================================================
// Provides a minimal status window that is ONLY shown when the user manually
// runs "rustdesk-agent.exe --status". This window communicates with the
// background service via IPC to show the current status and agent ID.
//
// The window is NOT launched at boot, NOT minimized to tray, and fully exits
// when the user closes it.

use hbb_common::{config::Config, log};

/// Show the manual status window.
/// This creates a small console-based status display.
/// For a GUI version, this could use Win32 API or a lightweight GUI library.
pub fn show_status_window() {
    println!();
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║               RustDesk Agent Status                 ║");
    println!("╠══════════════════════════════════════════════════════╣");

    // Get the agent ID
    let id = Config::get_id();
    if id.is_empty() {
        println!("║  ID: (not yet assigned)                            ║");
    } else {
        println!("║  ID: {:<47} ║", id);
    }

    // Check if the service is running by trying to connect via IPC
    let status = check_service_status();
    let status_indicator = if status { "● Online" } else { "○ Offline" };
    println!("║  Status: {:<43} ║", status_indicator);

    // Show server info
    let server = Config::get_option("custom-rendezvous-server");
    if !server.is_empty() {
        println!("║  Server: {:<43} ║", server);
    } else {
        println!("║  Server: (default public servers)                  ║");
    }

    // Check if password is set
    let has_password = !Config::get_permanent_password().is_empty();
    let pw_status = if has_password { "Set" } else { "Not set" };
    println!("║  Password: {:<41} ║", pw_status);

    println!("╠══════════════════════════════════════════════════════╣");
    println!("║  Commands:                                         ║");
    println!("║    --set-password <PWD>  Change password            ║");
    println!("║    --install             Install as service         ║");
    println!("║    --uninstall           Remove service             ║");
    println!("╚══════════════════════════════════════════════════════╝");
    println!();

    // If running interactively, show the GUI status window
    #[cfg(windows)]
    {
        show_win32_status_window(&id, status, &server, has_password);
    }
}

/// Check if the agent service is running.
/// Tries to connect to the agent's IPC pipe.
fn check_service_status() -> bool {
    // Try to see if the service process is running
    #[cfg(windows)]
    {
        let output = std::process::Command::new("sc")
            .args(&["query", "RustDeskAgent"])
            .output();

        if let Ok(out) = output {
            let stdout = String::from_utf8_lossy(&out.stdout);
            return stdout.contains("RUNNING");
        }
    }

    false
}

/// Show a Win32 MessageBox-based status window on Windows.
#[cfg(windows)]
fn show_win32_status_window(id: &str, online: bool, server: &str, has_password: bool) {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    let status_text = if online { "Online ●" } else { "Offline ○" };
    let server_text = if server.is_empty() {
        "Default public servers"
    } else {
        server
    };
    let pw_text = if has_password { "Set ✓" } else { "NOT SET ✗" };
    let id_text = if id.is_empty() {
        "(not yet assigned)".to_string()
    } else {
        id.to_string()
    };

    let message = format!(
        "RustDesk Agent Status\n\
         ─────────────────────\n\
         \n\
         Status:    {}\n\
         ID:        {}\n\
         Server:    {}\n\
         Password:  {}\n\
         \n\
         ─────────────────────\n\
         Use --set-password to change password.\n\
         Use --install to register as service.",
        status_text, id_text, server_text, pw_text
    );

    // Use Win32 MessageBox for a proper GUI popup
    let title: Vec<u16> = OsStr::new("RustDesk Agent")
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let msg: Vec<u16> = OsStr::new(&message)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        winapi::um::winuser::MessageBoxW(
            std::ptr::null_mut(),
            msg.as_ptr(),
            title.as_ptr(),
            winapi::um::winuser::MB_OK | winapi::um::winuser::MB_ICONINFORMATION,
        );
    }
}
