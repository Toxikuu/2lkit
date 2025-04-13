use anyhow::Result;
use std::fs;
use std::path::PathBuf;

use crate::msg;
use crate::shell::interactive::sesh;

pub fn newrepo(url: &str) -> Result<()> {
    let (_, repo_name) = url.rsplit_once('/').expect("Invalid url");
    let (_, repo_name) = repo_name
        .trim_end_matches(".git")
        .split_once("-")
        .expect("Invalid repo name (must start with '2-')");

    if repo_name.is_empty() {
        panic!("Failed to divine repo name :(")
    }

    let dir = PathBuf::from(format!("/var/ports/{repo_name}"));
    fs::create_dir(&dir)?;

    let command = &format!(
        r#"
        cd {dir:?}
        echo '*/.*' > .gitignore
        echo 'shell=bash' > .shellcheckrc
        echo 'disable=SC2034' >> .shellcheckrc

        echo '# {repo_name}' > README.md
        curl -fsSL -o LICENSE 'https://www.gnu.org/licenses/gpl-3.0.txt'
        echo "Licensing under GPL3 by default. You can change this if you want."

        git init
        git add .
        git commit -m "Initial commit"
        git branch -M master
        git remote add origin {url}
        git push -u origin master
        "#
    );

    sesh(command)?;

    msg!("Done!");
    Ok(())
}
