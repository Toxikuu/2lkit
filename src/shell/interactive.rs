// src/shell/interactive.rs

use anyhow::{Context, Result, bail};
use std::process::Command;

pub fn sesh(command: &str) -> Result<()> {
    let mut child = Command::new("bash")
        .arg("-c")
        .arg(command)
        .spawn()
        .context("Failed to spawn bash")?;

    let status = child.wait()?;
    if !status.success() {
        bail!("Command failed with exit code {}", status.code().unwrap_or(-1));
    }

    Ok(())
}
