use std::{env, sync::OnceLock};

use directories::ProjectDirs;

pub mod api;
pub mod approvals;
pub mod assets;
pub mod browser;
pub mod diff;
pub mod git;
pub mod jwt;
pub mod log_msg;
pub mod msg_store;
pub mod path;
pub mod port_file;
pub mod response;
pub mod sentry;
pub mod shell;
pub mod stream_lines;
pub mod text;
pub mod tokio;
pub mod version;

static WSL2_CACHE: OnceLock<bool> = OnceLock::new();

pub fn is_wsl2() -> bool {
    *WSL2_CACHE.get_or_init(|| {
        // Check for WSL environment variables
        if std::env::var("WSL_DISTRO_NAME").is_ok() || std::env::var("WSLENV").is_ok() {
            tracing::debug!("WSL2 detected via environment variables");
            return true;
        }

        // Check /proc/version for WSL2 signature
        if let Ok(version) = std::fs::read_to_string("/proc/version")
            && (version.contains("WSL2") || version.contains("microsoft"))
        {
            tracing::debug!("WSL2 detected via /proc/version");
            return true;
        }

        tracing::debug!("WSL2 not detected");
        false
    })
}

pub fn cache_dir() -> std::path::PathBuf {
    let proj = if cfg!(debug_assertions) {
        ProjectDirs::from("ai", "bloop-dev", env!("CARGO_PKG_NAME"))
            .expect("OS didn't give us a home directory")
    } else {
        ProjectDirs::from("ai", "bloop", env!("CARGO_PKG_NAME"))
            .expect("OS didn't give us a home directory")
    };

    proj.cache_dir().to_path_buf()
}
