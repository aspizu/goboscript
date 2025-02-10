use core::str;
use std::{
    fs::{
        self,
        File,
    },
    io,
    path::{
        Path,
        PathBuf,
    },
    process::Command,
};

use anyhow::{
    bail,
    Context,
};
use semver::Version;

pub struct StandardLibrary {
    version: Version,
    path: PathBuf,
}

impl StandardLibrary {
    pub fn new(version: Version, cache_path: &Path) -> Self {
        Self {
            path: cache_path.join(format!("v{}", version)),
            version,
        }
    }

    pub fn from_latest(cache_path: &Path) -> anyhow::Result<Self> {
        let path = cache_path.join("main");
        fs::create_dir_all(&path).with_context(|| {
            format!(
                "Failed to create standard library version directory {}",
                path.display()
            )
        })?;
        if path.exists() {
            if !Command::new("git")
                .current_dir(&path)
                .args(["pull"])
                .status()
                .with_context(|| format!("Failed to fetch standard library updates"))?
                .success()
            {
                bail!("Failed to fetch standard library updates");
            }
        } else {
            if !Command::new("git")
                .args([
                    "clone",
                    "https://github.com/goboscript/std",
                    "--branch",
                    "main",
                    path.to_str().unwrap(),
                ])
                .status()
                .with_context(|| format!("Failed to clone standard library"))?
                .success()
            {
                bail!("Failed to clone standard library");
            }
        }
        let output = Command::new("git")
            .current_dir(&path)
            .args(["describe", "--tags", "--abbrev=0"])
            .output()
            .with_context(|| format!("Failed to get standard library version"))?;
        if !output.status.success() {
            bail!("Failed to get latest standard library version");
        }
        let version = str::from_utf8(output.stdout.as_slice()).unwrap();
        let version = version.strip_prefix('v').unwrap_or(version);
        Ok(Self {
            path,
            version: version
                .parse()
                .context("Latest tag on standard library is not a valid semver version")?,
        })
    }

    pub fn fetch(&self) -> anyhow::Result<()> {
        if self.path.exists() {
            return Ok(());
        }
        fs::create_dir_all(&self.path).with_context(|| {
            format!(
                "Failed to create standard library version directory {}",
                self.version
            )
        })?;
        if !Command::new("git")
            .args([
                "clone",
                "--depth=1",
                "https://github.com/goboscript/std",
                "--branch",
                &format!("v{}", self.version),
                self.path.to_str().unwrap(),
            ])
            .status()
            .with_context(|| format!("Failed to clone standard library version {}", self.version))?
            .success()
        {
            bail!("Failed to clone standard library version {}", self.version);
        }
        fs::remove_dir_all(self.path.join(".git")).with_context(|| {
            format!(
                "Failed to remove .git directory from standard library version {}",
                self.version
            )
        })?;
        Ok(())
    }

    pub fn open_file(&self, path: &str) -> io::Result<File> {
        File::open(self.path.join(path))
    }
}
