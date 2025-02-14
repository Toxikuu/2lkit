// src/logic/gen.rs

use anyhow::{Context, Result};

use crate::{msg, shell::live::exec, structs::package::Package};

pub fn r#gen(package: &mut Package) -> Result<()> {
    let name = &package.name;
    let version = &package.version;

    package.source = package.source.as_ref().map(|src| src.hash(package));
    package.extra = package.extra.as_ref().map(|extras| extras.iter().map(|s| s.hash(package)).collect::<Vec<_>>());

    msg!("Generating info.lock for {package}");
    package.write()?;

    let dir = package.dir();
    let command = format!(r#"
        set -x

        cd "{dir}"
        mkdir -pv .{{build,data,dist,logs,sources}}

        MSG="Generated {name}={version}"

        # Bail if nothing changed
        [ -z "$(git status -s .)" ] && exit 0

        git add info.lock
        TIMESTAMP=$(date -u "+%Y-%m-%d %H:%M:%S")
        COMMIT=$(git commit -qm "$MSG" && git rev-parse HEAD)

        echo "[$COMMIT] [$TIMESTAMP] | $MSG" >> CHANGELOG

        git add CHANGELOG
        git commit -qm "Logged $COMMIT"
    "#);

    exec(&command).context("Failed to finalize generation")?;
    msg!("Done!");

    Ok(())
}
