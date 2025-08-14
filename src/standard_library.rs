use std::{
    fs::{
        self,
        File,
    },
    io::{
        self,
        Cursor,
    },
    path::{
        Path,
        PathBuf,
    },
    time::{
        SystemTime,
        UNIX_EPOCH,
    },
};

use anyhow::Context;
use semver::Version;
use serde::Deserialize;
use zip::ZipArchive;

#[derive(Deserialize)]
struct Tag {
    name: String,
}

fn create_client() -> reqwest::Result<reqwest::blocking::Client> {
    reqwest::blocking::Client::builder()
        .user_agent("goboscript")
        .build()
}

fn get_latest_version() -> anyhow::Result<Version> {
    let client = create_client()?;
    let response = client
        .get("https://api.github.com/repos/goboscript/std/tags")
        .send()
        .context("Failed to fetch tags from GitHub")?;
    let tags = response.json::<Vec<Tag>>()?;
    let mut tags: Vec<_> = tags
        .into_iter()
        .map(|tag| Version::parse(tag.name.strip_prefix("v").unwrap()).unwrap())
        .collect();
    tags.sort();
    Ok(tags.last().unwrap().clone())
}

fn fetch_package(version: &Version, path: &Path) -> anyhow::Result<()> {
    if path.exists() {
        return Ok(());
    }
    fs::create_dir_all(path)?;
    let client = create_client()?;
    let response = client
        .get(format!(
            "https://api.github.com/repos/goboscript/std/zipball/refs/tags/v{version}"
        ))
        .send()
        .context("Failed to fetch standard library package")?;
    let mut zipfile = Cursor::new(response.bytes()?);
    let mut zipfile = ZipArchive::new(&mut zipfile)?;
    for i in 0..zipfile.len() {
        let mut file = zipfile.by_index(i)?;
        let Some(outpath) = file.enclosed_name() else {
            continue;
        };
        if file.is_dir() {
            continue;
        }
        let outpath = outpath.components().skip(1).collect::<PathBuf>();
        let outpath = path.join(outpath);
        if let Some(parent) = outpath.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut outfile = File::create(&outpath)?;
        io::copy(&mut file, &mut outfile)?;
    }
    Ok(())
}

pub struct StandardLibrary {
    pub version: Version,
    pub path: PathBuf,
}

pub fn new_standard_library(version: Version, cache_path: &Path) -> StandardLibrary {
    StandardLibrary {
        path: cache_path.join(format!("v{}", version)),
        version,
    }
}

pub fn standard_library_from_latest(cache_path: &Path) -> anyhow::Result<StandardLibrary> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let verinfo = cache_path.join("verinfo.txt");

    if verinfo.exists() {
        let verinfo_content = fs::read_to_string(&verinfo).unwrap();
        let (version_str, last_updated_str) = verinfo_content.split_once('/').unwrap();
        let version = version_str
            .trim()
            .parse::<Version>()
            .context("verinfo.txt contains invalid semver version")?;
        let last_updated = last_updated_str.trim().parse::<u64>().unwrap();

        // If cache is less than a week old, use cached version
        if now - last_updated < 60 * 60 * 24 * 7 {
            return Ok(StandardLibrary {
                path: cache_path.join(format!("v{}", version)),
                version,
            });
        }
    }

    // Fetch latest version from GitHub API
    let version = get_latest_version()?;

    // Update cache with new version info
    fs::create_dir_all(cache_path)?;
    fs::write(verinfo, format!("{}/{}", version, now)).unwrap();

    Ok(StandardLibrary {
        path: cache_path.join(format!("v{}", version)),
        version,
    })
}

pub fn fetch_standard_library(stdlib: &StandardLibrary) -> anyhow::Result<()> {
    fetch_package(&stdlib.version, &stdlib.path)
}
