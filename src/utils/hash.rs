// src/utils/hash.rs

use sha2::{Sha256, Digest};
use anyhow::Result;
use base64::{engine::general_purpose::STANDARD, Engine};
use std::{
    fs::{create_dir, File}, io::Read
};

use crate::structs::package::Package;

use super::dl;

fn twohash(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path)?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 8192];

    while let Ok(n) = file.read(&mut buf) {
        if n == 0 { break }
        hasher.update(&buf[..n])
    }

    let hash = hasher.finalize();
    let b64_hash = STANDARD.encode(hash);

    Ok(
        b64_hash.replace('+', "-").replace('/', "_").trim_end_matches("=").to_string()
    )
}

pub fn linkhash(url: &str, p: &Package) -> String {
    let _ = create_dir(format!("/usr/ports/{}/{}/.sources/", p.repo, p.name));
    let out = dl::dl_url(url, p).expect("Failed to download url");
    twohash(&out).expect("Failed to hash downloaded file")
}
