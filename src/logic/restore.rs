use anyhow::{bail, Result};
use std::path::PathBuf;

use crate::shell::r#static::sex;
use crate::structs::maintarg::MaintArg;
use crate::msg;

pub fn restore(package: &MaintArg, commit: &str) -> Result<()> {
    let name = &package.name;
    let dir = PathBuf::from("/usr/ports").join(package.repo).join(name);

    if !dir.exists() {
        bail!("Package directory does not exist")
    }

    let command = &format!(r#"
        cd "{dir:?}"

        git restore --staged --worktree --source={commit} -- $(git ls-tree -r --name-only {commit} | grep -v CHANGELOG)

        MSG="Restored {name}: Back to {commit}"

        git add .
        TIMESTAMP=$(date -u "+%Y-%m-%d %H:%M:%S")
        COMMIT=$(git commit -qm "$MSG" && git rev-parse HEAD)
        
        echo "[$COMMIT] [$TIMESTAMP] | $MSG" >> CHANGELOG

        git add CHANGELOG
        git commit -qm "Logged $COMMIT"
    "#);

    sex(command)?;
    msg!("Done!");

    Ok(())
}
