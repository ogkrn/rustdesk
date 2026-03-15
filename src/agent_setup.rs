// ============================================================================
// Agent First-Run Setup Module
// ============================================================================
// Handles the one-time initial configuration on first launch:
// - Generates or confirms the agent's unique ID
// - Sets the permanent password
// - Configures server addresses
// - Marks the agent as configured

use hbb_common::{config::Config, log};
use crate::agent_config;

/// Run the first-time setup process.
/// This is called when no config file or no password is found.
pub fn run_first_time_setup() {
    println!();
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║         RustDesk Agent — First-Time Setup           ║");
    println!("╚══════════════════════════════════════════════════════╝");
    println!();

    // Step 1: Apply default server configuration
    agent_config::apply_default_server_config();

    // Step 2: Ensure ID is generated
    let id = Config::get_id();
    if id.is_empty() {
        // RustDesk auto-generates an ID when first accessed
        log::info!("ID will be generated on first server registration");
        println!("[*] Agent ID will be assigned on first connection.");
    } else {
        println!("[*] Agent ID: {}", id);
    }

    // Step 3: Show server info
    let server = Config::get_option("custom-rendezvous-server");
    if !server.is_empty() {
        println!("[*] Rendezvous Server: {}", server);
    } else {
        println!("[*] Using default public RustDesk servers");
    }

    // Step 4: Prompt for permanent password
    println!();
    println!("Set a permanent password for remote access.");
    println!("This password will be required when connecting from your main PC.");
    println!();

    loop {
        let password = prompt_password("Enter password: ");
        if password.is_empty() {
            println!("Password cannot be empty. Please try again.");
            continue;
        }
        if password.len() < 6 {
            println!("Password must be at least 6 characters. Please try again.");
            continue;
        }

        let confirm = prompt_password("Confirm password: ");
        if password != confirm {
            println!("Passwords do not match. Please try again.");
            continue;
        }

        // Set the permanent password using RustDesk's config system
        Config::set_permanent_password(&password);
        log::info!("Permanent password set successfully");
        println!("[OK] Password set successfully.");
        break;
    }

    // Step 5: Apply agent defaults (silent operation mode)
    agent_config::apply_agent_defaults();

    // Step 6: Display final info
    let id = Config::get_id();
    println!();
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║              Setup Complete!                        ║");
    println!("╠══════════════════════════════════════════════════════╣");
    if !id.is_empty() {
        println!("║  Agent ID: {:<41} ║", id);
    }
    println!("║  Password: (saved securely)                        ║");
    println!("║                                                    ║");
    println!("║  The agent will now run invisibly in the background.║");
    println!("║  Use --status to check status anytime.             ║");
    println!("║  Use --install to register as a Windows Service.   ║");
    println!("╚══════════════════════════════════════════════════════╝");
    println!();
}

/// Set the permanent password from the command line.
pub fn set_password_from_cli(password: &str) {
    if password.len() < 6 {
        eprintln!("Password must be at least 6 characters.");
        return;
    }
    Config::set_permanent_password(password);
    println!("[OK] Permanent password updated.");
}

/// Prompt the user for a password (with hidden input).
fn prompt_password(prompt: &str) -> String {
    print!("{}", prompt);
    use std::io::Write;
    std::io::stdout().flush().ok();

    // Use rpassword for hidden input
    match rpassword::read_password() {
        Ok(pw) => pw.trim().to_string(),
        Err(_) => {
            // Fallback to regular input
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).ok();
            input.trim().to_string()
        }
    }
}
