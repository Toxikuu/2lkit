use anyhow::{Result, bail};
use std::fs;
use std::path::Path;

use crate::msg;
use crate::shell::interactive::sesh;
use crate::shell::r#static::sex;
use crate::structs::maintarg::MaintArg;
use crate::structs::package::Package;

pub fn upd(package: &Package, new: &MaintArg) -> Result<String> {
    let name = &package.name;
    let old_version = &package.version;
    let new_version = &new.version.expect("Provide the version argument");
    let dir = package.dir();
    let build_path = Path::new(&dir).join("BUILD");

    if !build_path.exists() {
        bail!("Package does not exist")
    }

    let mut contents = fs::read_to_string(&build_path)?;
    let mut lines = contents.lines().map(|l| l.to_string()).collect::<Vec<_>>();

    // automatically replace the version variable
    lines = lines
        .iter()
        .map(|l| {
            if l.starts_with(&format!("VERS=\"{old_version}\"")) {
                format!("VERS=\"{new_version}\"")
            } else {
                l.to_string()
            }
        })
        .collect::<Vec<_>>();

    // strip commit description if present
    lines.retain(|l| !l.starts_with("#d"));
    contents = lines.join("\n");

    fs::write(&build_path, contents)?;

    // check if any important variables were changed
    let command = &format!(
        r#"
        cd "{dir}"
        "${{EDITOR:-/usr/bin/nvim}}" BUILD
    "#
    );

    sesh(command)?;

    let contents = fs::read_to_string(&build_path)?;
    let lines = contents.lines();

    // record commit description if present
    let last = lines.clone().last().unwrap();
    let desc = last.strip_prefix("#d").unwrap_or_default().trim();

    let command = &format!(
        r#"
        cd "{dir}"
        source BUILD

        MSG="Updated {name}: {old_version} -> $VERS"

        git add .
        TIMESTAMP=$(date -u "+%Y-%m-%d %H:%M:%S")
        COMMIT=$(git commit -qm "$MSG" -m "{desc}" && git rev-parse HEAD)
        
        if [ -z "{desc}" ]; then
          echo "[$COMMIT] [$TIMESTAMP] | $MSG" >> CHANGELOG
        else
          echo "[$COMMIT] [$TIMESTAMP] | $MSG: {desc}" >> CHANGELOG
        fi

        git add CHANGELOG
        git commit -qm "Logged $COMMIT"

        echo "$VERS"
    "#
    );

    let vers = sex(command)?;
    msg!("Done!");

    Ok(vers.trim().to_string())
}
