// src/structs/package.rs

use std::{fmt, fs};

use crate::{
    shell::r#static::sex,
    utils::{hash::linkhash, time::timestamp},
};
use anyhow::{Context, Result};
use serde::Serialize;

use super::maintarg::MaintArg;

#[derive(Debug, Serialize)]
pub struct Package {
    pub repo: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>, // Else "No description provided"
    pub categories: Option<Vec<String>>,

    pub timestamp: String, // Generation timestamp
    pub dependencies: Option<Vec<String>>,

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
        // Note: A space-delimited list is interpreted no different from an array for package
        // formation through bash

        let command = &format!(
            r#"source /var/ports/{repo}/{name}/BUILD; echo "$VERS"; echo "$DESC"; echo "${{CATG[@]}}"; echo "${{DEPS[@]}}"; echo "$SOURCE"; echo "${{EXTRA[@]}}"; echo "$UPST"; echo "$VCMD""#
        );
        let out = sex(command).context("Failed to source BUILD")?;

        let lines = out.lines().map(str::trim).collect::<Vec<_>>();
        let [vers, desc, catg, deps, source, extra, upstream, vcmd] = &lines[..] else {
            panic!("Shouldn't happen lol")
        };

        let description = if desc.is_empty() {
            None
        } else {
            Some(desc.to_string())
        };
        let categories = if catg.is_empty() {
            None
        } else {
            Some(catg.split_whitespace().map(str::to_string).collect())
        };
        let dependencies = if deps.is_empty() {
            None
        } else {
            Some(deps.split_whitespace().map(str::to_string).collect())
        };

        let source = if source.is_empty() {
            None
        } else {
            Some(Url::new(source))
        };
        let extra = if extra.is_empty() {
            None
        } else {
            let links = extra.split_whitespace().collect::<Vec<_>>();
            Some(links.iter().map(|l| Url::new(l)).collect::<Vec<_>>())
        };
        let upstream = if upstream.is_empty() {
            None
        } else {
            Some(upstream.to_string())
        };
        let version_command = if vcmd.is_empty() {
            None
        } else {
            Some(vcmd.to_string())
        };

        Ok(Self {
            repo: repo.to_string(),
            name: name.to_string(),
            version: vers.to_string(),
            description,
            categories,
            timestamp: timestamp(),
            dependencies,
            source,
            extra,
            upstream,
            version_command,
        })
    }

    pub fn write(&self) -> Result<()> {
        let str = toml::to_string_pretty(&self)?;
        let file_path = &format!("/var/ports/{}/{}/LOCK", self.repo, self.name);
        fs::write(file_path, str)?;

        Ok(())
    }

    pub fn dir(&self) -> String {
        format!("/var/ports/{}/{}", self.repo, self.name)
    }

    pub fn reform(&self) -> Self {
        Self::new(&self.repo, &self.name).expect("Failed to reform package")
    }
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}={}", self.repo, self.name, self.version)
    }
}

impl<'a> From<MaintArg<'a>> for Package {
    fn from(ma: MaintArg) -> Self {
        Self::new(ma.repo, ma.name).expect("Failed to form package")
    }
}
