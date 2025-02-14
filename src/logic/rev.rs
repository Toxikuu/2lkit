use anyhow::{bail, Result};
use std::fs;
use std::path::Path;

use crate::shell::interactive::sesh;
use crate::shell::live::exec;
use crate::structs::package::Package;
use crate::msg;

pub fn rev(package: &Package) -> Result<bool> {
    let mut regen_needed = false;

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
    let command = &format!(r#"
        cd "{dir}"
        mkdir -p /tmp/2/m-diffs
        
        source BUILD
        declare -p > /tmp/2/m-diffs/pre

        "${{EDITOR:-/usr/bin/nvim}}" BUILD
        unset NAME VERS SOURCE UPST VCMD EXTRA

        source BUILD
        declare -p > /tmp/2/m-diffs/post

        diff /tmp/2/m-diffs/p{{re,ost}} > /tmp/2/m-diffs/diffs

        # TODO: scold the user if they change NAME or VERS

        changes=$(grep -E 'NAME|VERS|SOURCE|UPST|VCMD|EXTRA' /tmp/2/m-diffs/diffs | wc -l)
        echo "$changes" > /tmp/2/m-diffs/ct
    "#);

    sesh(command)?;

    if fs::read_to_string("/tmp/2/m-diffs/ct").unwrap_or_default().trim() != "0" {
        regen_needed = true
    }

    let contents = fs::read_to_string(&build_path)?;
    let lines = contents.lines();

    // record commit description if present
    let last = lines.clone().last().unwrap();
    let desc = last.strip_prefix("#d").unwrap_or_default().trim();

    let command = &format!(r#"
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
    "#);

    exec(command)?;
    msg!("Done!");

    Ok(regen_needed)
}
