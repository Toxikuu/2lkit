// src/structs/package.rs

use std::{fs, fmt};

use anyhow::{Context, Result};
use serde::Serialize;
use crate::{shell::r#static::sex, utils::hash::linkhash};

#[derive(Debug, Serialize)]
pub struct Package {
    pub repo: String,
    pub name: String,
    pub version: String,

    pub source: Option<Url>,
    pub extra: Option<Vec<Url>>,

    upstream: Option<String>,
    version_command: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Url {
    pub url: String,
    pub hash: String,
}

impl Url {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            hash: String::new(),
        }
    }

    pub fn hash(&self, p: &Package) -> Self {
        Self {
            url: self.url.to_string(),
            hash: linkhash(&self.url, p),
        }
    }
}

impl Package {
    pub fn new(repo: &str, name: &str) -> Result<Self> {
        let command = &format!(r#"source /usr/ports/{repo}/{name}/BUILD; echo "$VERS"; echo "$SOURCE"; echo "${{EXTRA[@]}}"; echo "$UPST"; echo "$VCMD""#);
        let out = sex(command).context("Failed to source BUILD")?;

        let lines = out.lines().map(|l| l.trim()).collect::<Vec<_>>();
        let version         = lines.first().unwrap().to_string();
        let source          = lines.get(1).unwrap();
        let extra           = lines.get(2).unwrap();
        let upstream        = lines.get(3).unwrap().to_string();
        let version_command = lines.last().unwrap().to_string();

        let source = if source.is_empty() { None } else { Some(Url::new(source)) };
        let extra  = if extra.is_empty()  { None } else {
            let links = extra.split(' ').collect::<Vec<_>>();
            Some(links.iter().map(|l| Url::new(l)).collect::<Vec<_>>())
        };
        let upstream = if upstream.is_empty() { None } else { Some(upstream) };
        let version_command = if version_command.is_empty() { None } else { Some(version_command) };

        Ok(
            Self {
                repo: repo.to_string(),
                name: name.to_string(),
                version,
                source,
                extra,
                upstream,
                version_command
            }
        )
    }

    pub fn write(&self) -> Result<()> {
        let str = toml::to_string_pretty(&self)?;
        let file_path = &format!("/usr/ports/{}/{}/info.lock", self.repo, self.name);
        fs::write(file_path, str)?;

        Ok(())
    }

    pub fn dir(&self) -> String {
        format!("/usr/ports/{}/{}", self.repo, self.name)
    }
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}={}", self.repo, self.name, self.version)
    }
}
