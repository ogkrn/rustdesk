// ============================================================================
// RustDesk Agent — Lightweight Remote Access Agent
// ============================================================================
// A stripped-down, agent-only RustDesk build that runs invisibly on a secondary
// Windows PC, accepting incoming remote connections with zero UI.
//
// Usage:
//   rustdesk-agent.exe                  → First run: setup, then start service
//   rustdesk-agent.exe --service        → Run as Windows Service
//   rustdesk-agent.exe --install        → Install Windows Service
//   rustdesk-agent.exe --uninstall      → Uninstall Windows Service
//   rustdesk-agent.exe --status         → Show status window (manual only)
//   rustdesk-agent.exe --set-password X → Set permanent password
//   rustdesk-agent.exe --get-id         → Print agent ID
// ============================================================================

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use librustdesk::*;
use hbb_common::{config::Config, log};

mod agent_config;
mod agent_service;
mod agent_setup;
mod agent_status_ui;

fn main() {
    // Initialize logging with minimal output (warn/error only for low disk I/O)
    hbb_common::init_log(false, "agent");

    // Parse command-line arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "--service" => {
                // Run as Windows Service (called by SCM)
                log::info!("Starting RustDesk Agent as Windows Service");
                agent_service::run_service();
                return;
            }
            "--install" => {
                // Install the Windows Service
                println!("Installing RustDesk Agent service...");
                agent_service::install_service();
                return;
            }
            "--uninstall" => {
                // Uninstall the Windows Service
                println!("Uninstalling RustDesk Agent service...");
                agent_service::uninstall_service();
                return;
            }
            "--status" => {
                // Show manual status window
                agent_status_ui::show_status_window();
                return;
            }
            "--set-password" => {
                if args.len() > 2 {
                    agent_setup::set_password_from_cli(&args[2]);
                } else {
                    eprintln!("Usage: rustdesk-agent.exe --set-password <PASSWORD>");
                }
                return;
            }
            "--get-id" => {
                println!("{}", Config::get_id());
                return;
            }
            "--version" => {
                println!("RustDesk Agent v{}", crate::VERSION);
                return;
            }
            "--server" => {
                // Direct server mode (for debugging / non-service use)
                log::info!("Starting agent in direct server mode");
                run_agent_server();
                return;
            }
            _ => {
                eprintln!("Unknown argument: {}", args[1]);
                print_usage();
                return;
            }
        }
    }

    // Default behavior: first-run setup then start server
    if !agent_config::is_configured() {
        println!("=== RustDesk Agent — First Run Setup ===");
        agent_setup::run_first_time_setup();
    }

    // After setup, start the agent server directly
    println!("RustDesk Agent starting...");
    println!("ID: {}", Config::get_id());
    println!("Press Ctrl+C to stop.");
    run_agent_server();
}

/// Start the agent server (incoming connections only)
fn run_agent_server() {
    if !common::global_init() {
        log::error!("Failed to initialize globals");
        return;
    }
    // Start the server in "is_server" mode — this handles:
    // - Rendezvous server registration
    // - Heartbeat / keep-alive
    // - Incoming connection acceptance
    // - Screen capture, input forwarding, clipboard sharing
    crate::start_server(true, false);
    common::global_clean();
}

fn print_usage() {
    println!("RustDesk Agent — Lightweight Remote Access Agent");
    println!();
    println!("Usage:");
    println!("  rustdesk-agent.exe                    First run setup, then start");
    println!("  rustdesk-agent.exe --service           Run as Windows Service");
    println!("  rustdesk-agent.exe --server            Run in direct server mode");
    println!("  rustdesk-agent.exe --install            Install Windows Service");
    println!("  rustdesk-agent.exe --uninstall          Uninstall Windows Service");
    println!("  rustdesk-agent.exe --status             Show status window");
    println!("  rustdesk-agent.exe --set-password PWD   Set permanent password");
    println!("  rustdesk-agent.exe --get-id             Print agent ID");
    println!("  rustdesk-agent.exe --version            Show version");
}
