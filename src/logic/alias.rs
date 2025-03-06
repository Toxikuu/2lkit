use anyhow::{bail, Context, Result};
use std::path::PathBuf;

use crate::shell::live::exec;
use crate::structs::maintarg::MaintArg;
use crate::structs::package::Package;
use crate::msg;

pub fn alias(origin: &Package, alias: &MaintArg) -> Result<()> {
    let origin_repo = &origin.repo;
    let origin_name = &origin.name;
    let origin_repo_dir = PathBuf::from("/usr/ports").join(&origin.repo);
    let origin_path = origin_repo_dir.join(&origin.name);

    let alias_repo = alias.repo;
    let alias_name = alias.name;
    let alias_repo_dir = PathBuf::from("/usr/ports").join(&alias.repo);
    let alias_path = alias_repo_dir.join(&alias.name);

    if alias_path.exists() {
        bail!("Alias exists!")
    }

    if !alias_repo_dir.exists() {
        bail!("Alias repo doesn't exist!")
    }

    if !origin_path.exists() {
        bail!("Origin doesn't exist!")
    }

    if alias_path == origin_path {
        bail!("Alias and origin are identical!")
    }

    let command = &format!(r#"
        cd {alias_repo_dir:?}

        if [[ {alias_repo} == {origin_repo} ]]; then
            ln -sv {origin_name} {alias_name}
        else
            # note, this is probably ill-advised as it requires the end user to have both repos
            ln -sv ../{origin_repo}/{origin_name} {alias_name}
        fi

        MSG="Added alias {alias_repo}/{alias_name} for {origin_repo}/{origin_name}"

        git add {alias_name}
        git commit -qm "$MSG"

        TIMESTAMP=$(date -u "+%Y-%m-%d %H:%M:%S")
        COMMIT=$(git commit -qm "$MSG" && git rev-parse HEAD)

        echo "[$COMMIT] [$TIMESTAMP] | $MSG" >> {origin_path:?}/CHANGELOG

        git add {origin_path:?}/CHANGELOG
        git commit -qm "Logged $COMMIT"
    "#);

    exec(command).context("Failed to finalize alias")?;
    msg!("Done!");
    Ok(())
}
