use anyhow::{bail, Context, Result};
use std::path::PathBuf;

use crate::shell::live::exec;
use crate::structs::maintarg::MaintArg;
use crate::structs::package::Package;
use crate::msg;

pub fn r#cp(from: &Package, to: &MaintArg) -> Result<()> {
    let from_repo = &from.repo;
    let from_name = &from.name;
    let from_dir = PathBuf::from(from.dir());

    let to_repo = to.repo;
    let to_name = to.name;
    let to_dir = PathBuf::from("/var/ports").join(to.repo).join(to.name);

    if to_dir.exists() {
        bail!("Destination exists!")
    }

    if !to_dir.parent().context("Fatherless.")?.exists() {
        bail!("Destination repo doesn't exist!")
    }

    if !from_dir.exists() {
        bail!("Origin doesn't exist!")
    }

    if to_dir == from_dir {
        bail!("Destination and origin are identical!")
    }

    let command = &format!(r#"
        cp -av {from_dir:?} {to_dir:?}
        cd {to_dir:?}
        MSG="Copied {from_repo}/{from_name} -> {to_repo}/{to_name}"

        git add .
        git commit -qm "$MSG"

        TIMESTAMP=$(date -u "+%Y-%m-%d %H:%M:%S")
        COMMIT=$(git commit -qm "$MSG" && git rev-parse HEAD)

        # should be first message in changelog, but ill keep >> anyway ¯\_(ツ)_/¯
        echo "[$COMMIT] [$TIMESTAMP] | $MSG" >> CHANGELOG

        git add CHANGELOG
        git commit -qm "Logged $COMMIT"
    "#);

    exec(command).context("Failed to finalize copy")?;
    msg!("Done!");
    Ok(())
}
