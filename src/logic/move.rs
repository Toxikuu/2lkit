use anyhow::{Context, Result, bail};
use std::path::PathBuf;

use crate::msg;
use crate::shell::live::exec;
use crate::structs::maintarg::MaintArg;
use crate::structs::package::Package;

pub fn r#move(from: &Package, to: &MaintArg) -> Result<()> {
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

    let command = &format!(
        r#"
        mv -v {from_dir:?} {to_dir:?}
        cd /var/ports/{from_repo}
        MSG="Moved {from_repo}/{from_name} -> {to_repo}/{to_name}"

        git rm -r "{from_name}"
        git commit -qm "$MSG"

        cd {to_dir:?}
        git add .
        TIMESTAMP=$(date -u "+%Y-%m-%d %H:%M:%S")
        COMMIT=$(git commit -qm "$MSG" && git rev-parse HEAD)

        echo "[$COMMIT] [$TIMESTAMP] | $MSG" >> CHANGELOG

        git add CHANGELOG
        git commit -qm "Logged $COMMIT"
    "#
    );

    exec(command).context("Failed to finalize move")?;
    msg!("Done!");
    Ok(())
}
