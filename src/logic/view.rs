use anyhow::{Result, bail};
use std::path::Path;

use crate::shell::interactive::sesh;
use crate::structs::package::Package;

pub fn view(package: &Package) -> Result<()> {
    let dir = package.dir();
    let build_path = Path::new(&dir).join("BUILD");

    if !build_path.exists() {
        bail!("Package does not exist")
    }

    // check if any important variables were changed
    let command = &format!(
        r#"
        nvim -R "{dir}"/BUILD
        "#
    );

    sesh(command)
}
