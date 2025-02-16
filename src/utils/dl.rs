// src/utils/dl.rs

use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    fs::{self, File}, io::{Read, Write}, path::{Path, PathBuf}
};
use ureq::http::header::CONTENT_LENGTH;

use crate::structs::package::Package;

const BAR: &str = "{prefix:.red} {msg:32.red} [{elapsed_precise}] [{bar:32.red/black}] {bytes}/{total_bytes}";

/// # Description
///
/// Includes progress bar
/// Overwrites existing files
pub fn download(url: &str, out: &str) -> Result<PathBuf> {
    let out = out.trim();
    let file_name = out.rsplit_once('/').map(|(_, name)| name.to_string()).expect("Invalid output path");
    let file_path = Path::new(&out);

    if file_path.exists() { return Ok(file_path.to_path_buf()) }

    let r = ureq::get(url).call()?;

    let length: u64 = r.headers()
        .get(CONTENT_LENGTH)
        .and_then(|hv| hv.to_str().ok())
        .and_then(|s| s.parse().ok())
        .unwrap_or(8192);

    let sty = ProgressStyle::with_template(BAR)
        .expect("Invalid bar template")
        .progress_chars("=>-");

    let pb = ProgressBar::new(length);
    pb.set_style(sty);
    pb.set_length(length);
    pb.set_prefix("󱑤 ");
    pb.set_message(file_name.clone());

    let mut f = File::create(file_path)?;

    let body = r.into_body();
    let reader = body.into_reader();

    let mut downloaded = 0;
    let mut reader = pb.wrap_read(reader);
    let mut buffer = vec![0; 8192];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 { break }

        f.write_all(&buffer[..bytes_read])?;
        downloaded += bytes_read as u64;

        pb.set_position(downloaded);

        if length < downloaded {
            pb.set_length(downloaded);
        }
    }

    pb.set_position(length);
    pb.set_prefix("󰗠 ");
    pb.finish_with_message(file_name);

    Ok(file_path.to_path_buf())
}

pub fn normalize_tarball(p: &Package, tb: &str) -> String {
    let ext = tb.rsplit_once(".t")
        .map(|(_, ext)| format!(".t{ext}"))
        .expect("Unsupported tarball format");

    let pp = format!("{}={}", p.name, p.version);
    let to = match ext.as_str() {
        ".tar.bz2"  | ".tbz" | ".tb2" | ".tbz2" | ".tz2" => format!("{pp}.tar.bz2" ),
        ".tar.gz"   | ".tgz" | ".taz"                    => format!("{pp}.tar.gz"  ),
        ".tar.lz"                                        => format!("{pp}.tar.lz"  ),
        ".tar.lzma" | ".tlz"                             => format!("{pp}.tar.lzma"),
        ".tar.lzo"                                       => format!("{pp}.tar.lzo" ),
        ".tar.xz"   | ".txz"                             => format!("{pp}.tar.xz"  ),
        ".tar.zst"  | ".tzst"                            => format!("{pp}.tar.zst" ),
        _ => panic!("Unsupported tarball extension: {}", ext),
    };

    to
}

fn download_tarball(p: &Package) -> Result<Option<String>> {
    let src = match &p.source {
        Some(src) => src,
        None => return Ok(None),
    };

    let url = &src.url;
    if url.is_empty() { return Ok(None) }

    let file_name = url.split('/').next_back().expect("Invalid url");
    let file_name = normalize_tarball(p, file_name);

    let srcpath = format!("{}/.sources", p.dir());
    fs::create_dir_all(&srcpath).expect("Failed to create source path");

    let out = format!("{srcpath}/{file_name}");
    download(url, &out).unwrap_or_else(|_| panic!("Failed to download tarball from '{url}'"));
    Ok(Some(out))
}

pub fn dl_url(url: &str, p: &Package) -> Result<String> {
    if let Some(src) = &p.source {
        if src.url == url {
            if let Some(file_path) = download_tarball(p)? {
                return Ok(file_path)
            }
        }
    }

    let f = url.rsplit_once('/').map(|(_, name)| name.to_string()).unwrap();
    let out = format!("{}/.sources/{f}", p.dir());
    download(url, &out)?;
    Ok(out)
}
