// src/structs/maintarg.rs

#[derive(Debug, Copy, Clone)]
pub struct MaintArg<'a> {
    pub repo: &'a str,
    pub name: &'a str,
    pub version: Option<&'a str>,
}

impl<'a> MaintArg<'a> {
    pub fn new(str: &'a str) -> Self {
        let (repo, rest) = str.trim().split_once('/').expect("Invalid argument syntax");
        if let Some((name, version)) = rest.trim().split_once('=') {
            Self {
                repo,
                name,
                version: Some(version),
            }
        } else {
            Self {
                repo,
                name: rest,
                version: None,
            }
        }
    }
}
