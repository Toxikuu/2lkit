use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::shell::interactive::sesh;
use crate::shell::live::exec;
use crate::structs::maintarg::MaintArg;
use crate::msg;

pub fn add(package: &MaintArg) -> Result<()> {
    let repo = package.repo;
    let name = package.name;
    let version = package.version.expect("Specify version :pleading:");

    let dir = format!("/var/ports/{repo}/{name}");

    // TODO: Consider moving mkdir -pv <dots> here

    let build_path = Path::new(&dir).join("BUILD");
    fs::create_dir(&dir)?;

    let template = format!(
r#"NAME="{name}"
VERS="{version}"
DESC=""
CATG=""
UPST=""
DEPS=()

SOURCE=""
EXTRA=()

2b() {{

}}"#);

    fs::write(build_path, template)?;
    let command = &format!(r#"
        "${{EDITOR:-/usr/bin/nvim}}" {dir}/BUILD
    "#);

    sesh(command)?;

    let command = &format!(r#"
        cd "{dir}"
        MSG="Added {name}={version}"

        git add .
        TIMESTAMP=$(date -u "+%Y-%m-%d %H:%M:%S")
        COMMIT=$(git commit -qm "$MSG" && git rev-parse HEAD)

        echo "[$COMMIT] [$TIMESTAMP] | $MSG" >> CHANGELOG

        git add CHANGELOG
        git commit -qm "Logged $COMMIT"
    "#);

    exec(command).context("Failed to finalize addition")?;
    msg!("Done!");
    Ok(())
}
