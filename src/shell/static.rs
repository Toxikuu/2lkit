// src/shell/static.rs
use std::process::Command;
use anyhow::{Result, bail};

/// # Description
/// The conveniently named ``sex()`` is short for static execution. It takes a command and captures
/// its output without printing that output or doing any thread shenanigans.
pub fn sex(command: &str) -> Result<String> {
    let output = Command::new("bash")
        .arg("-c")
        .arg(command).output()?;

    if !output.status.success() {
        let _error = String::from_utf8_lossy(&output.stderr);
        bail!("Command failed with status: {}", output.status);
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
