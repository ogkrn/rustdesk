// ============================================================================
// Agent Windows Service Module
// ============================================================================
// Handles registration of the agent as a Windows Service so it starts
// automatically at boot, before any user logs in.

use hbb_common::log;

/// The Windows Service name for the agent.
pub const SERVICE_NAME: &str = "RustDeskAgent";
pub const SERVICE_DISPLAY_NAME: &str = "RustDesk Agent Service";
pub const SERVICE_DESCRIPTION: &str = "RustDesk lightweight remote access agent. Allows incoming remote desktop connections.";

/// Run the agent as a Windows Service (called by the Service Control Manager).
pub fn run_service() {
    log::info!("Starting RustDesk Agent Windows Service");

    // Use RustDesk's existing service infrastructure
    // The --service flag is handled by the platform module
    crate::start_os_service();
}

/// Install the agent as a Windows Service.
pub fn install_service() {
    let exe_path = std::env::current_exe().unwrap_or_default();
    let exe_path_str = exe_path.to_string_lossy();

    log::info!("Installing service: {}", SERVICE_NAME);
    println!("Installing '{}' service...", SERVICE_DISPLAY_NAME);

    // Use sc.exe to create the service
    let output = std::process::Command::new("sc")
        .args(&[
            "create",
            SERVICE_NAME,
            &format!("binPath= \"{}\" --service", exe_path_str),
            "start=", "auto",
            &format!("DisplayName= {}", SERVICE_DISPLAY_NAME),
        ])
        .output();

    match output {
        Ok(out) => {
            if out.status.success() {
                println!("[OK] Service installed successfully.");

                // Set the service description
                let _ = std::process::Command::new("sc")
                    .args(&[
                        "description",
                        SERVICE_NAME,
                        SERVICE_DESCRIPTION,
                    ])
                    .output();

                // Configure recovery (auto-restart on failure)
                let _ = std::process::Command::new("sc")
                    .args(&[
                        "failure",
                        SERVICE_NAME,
                        "reset=", "86400",
                        "actions=", "restart/5000/restart/10000/restart/30000",
                    ])
                    .output();

                println!("[OK] Service configured for auto-restart on failure.");
                println!();
                println!("To start the service now, run:");
                println!("  sc start {}", SERVICE_NAME);
            } else {
                let stderr = String::from_utf8_lossy(&out.stderr);
                eprintln!("[ERROR] Failed to install service: {}", stderr);
                if stderr.contains("1073") {
                    eprintln!("Service already exists. Use --uninstall first.");
                }
            }
        }
        Err(e) => {
            eprintln!("[ERROR] Failed to run sc.exe: {}", e);
            eprintln!("Make sure to run as Administrator.");
        }
    }
}

/// Uninstall the agent Windows Service.
pub fn uninstall_service() {
    log::info!("Uninstalling service: {}", SERVICE_NAME);
    println!("Stopping and removing '{}' service...", SERVICE_DISPLAY_NAME);

    // Stop the service first
    let _ = std::process::Command::new("sc")
        .args(&["stop", SERVICE_NAME])
        .output();

    // Wait a moment for it to stop
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Delete the service
    let output = std::process::Command::new("sc")
        .args(&["delete", SERVICE_NAME])
        .output();

    match output {
        Ok(out) => {
            if out.status.success() {
                println!("[OK] Service uninstalled successfully.");
            } else {
                let stderr = String::from_utf8_lossy(&out.stderr);
                eprintln!("[ERROR] Failed to uninstall service: {}", stderr);
            }
        }
        Err(e) => {
            eprintln!("[ERROR] Failed to run sc.exe: {}", e);
            eprintln!("Make sure to run as Administrator.");
        }
    }
}
