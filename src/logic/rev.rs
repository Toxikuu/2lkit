use anyhow::{Result, bail};
use std::fs;
use std::path::Path;

use crate::msg;
use crate::shell::interactive::sesh;
use crate::shell::live::exec;
use crate::structs::package::Package;

pub fn rev(package: &Package) -> Result<()> {
    let name = &package.name;
    let dir = package.dir();
    let build_path = Path::new(&dir).join("BUILD");

    if !build_path.exists() {
        bail!("Package does not exist")
    }

    let mut contents = fs::read_to_string(&build_path)?;
    let mut lines = contents.lines().map(|l| l.to_string()).collect::<Vec<_>>();

    // strip commit description if present
    lines.retain(|l| !l.starts_with("#d"));
    contents = lines.join("\n");

    fs::write(&build_path, contents)?;

    // check if any important variables were changed
    let command = &format!(
        r#"
        cd "{dir}"
        mkdir -p /tmp/2/m-diffs
        
        source BUILD
        declare -p > /tmp/2/m-diffs/pre

        "${{EDITOR:-/usr/bin/nvim}}" BUILD
        # unset in case any of these are deleted
        unset NAME VERS SOURCE UPST VCMD EXTRA DESC DEPS

        source BUILD
        declare -p > /tmp/2/m-diffs/post

        diff /tmp/2/m-diffs/p{{re,ost}} > /tmp/2/m-diffs/diffs

        if grep -E 'NAME|VERS' /tmp/2/m-diffs/diffs; then
            echo -e "\x1b[31;1mBad maintainer!!\nYou do not change \$NAME or \$VERS when revising packages.\nUse the correct flags if you want to move or update a package.\x1b[0m"
            exit 2 # bad usage
        fi
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
        [ -z "$(git status -s)" ] && exit 0 # don't log when nothing changed

        MSG="Revised {name}"

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
    "#
    );

    exec(command)?;
    msg!("Done!");

    Ok(())
}
