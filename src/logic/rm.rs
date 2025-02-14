use anyhow::{bail, Context, Result};
use std::fs;
use std::path::Path;

use crate::shell::live::exec;
use crate::structs::package::Package;
use crate::msg;

pub fn rm(package: &Package) -> Result<()> {
    let repo = &package.repo;
    let name = &package.name;
    let version = &package.version;
    let dir = package.dir();

    if !Path::new(&dir).exists() {
        bail!(format!("Package '{package}' does not exist!"))
    }

    fs::remove_dir_all(&dir).expect("Nothing was removed");

    let command = &format!(r#"
        cd "/usr/ports/{repo}"

        git rm -r "{name}"
        git commit -qm "Removed {name}={version}"
    "#);

    exec(command).context("Failed to finalize removal")?;
    msg!("Done!");
    Ok(())
}
