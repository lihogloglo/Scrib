//! Finds a Morrowind installation and reads its configuration.
//!
//! Detection order, first valid hit wins:
//! 1. an explicit path from the caller (CLI flag, settings file),
//! 2. the `BEVYRROWIND_MORROWIND` environment variable,
//! 3. the Windows registry (retail/Bethesda launcher and GOG keys),
//! 4. every Steam library listed in `libraryfolders.vdf`.
//!
//! A candidate may point at the install root or at its `Data Files` folder.

mod ini;
mod steam;

pub use ini::MorrowindIni;

use std::path::{Path, PathBuf};

/// Environment variable that overrides detection.
pub const ENV_VAR: &str = "BEVYRROWIND_MORROWIND";

/// A validated Morrowind installation.
#[derive(Clone, Debug)]
pub struct MorrowindInstall {
    /// Folder containing `Morrowind.ini` (may be absent for some repacks).
    pub root: PathBuf,
    /// The `Data Files` folder; always contains `Morrowind.esm`.
    pub data_files: PathBuf,
    /// Parsed `Morrowind.ini`, if present.
    pub ini: Option<MorrowindIni>,
}

/// How an installation was found.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Origin {
    Explicit,
    Environment,
    Registry(String),
    Steam,
}

/// A candidate location considered during detection, for diagnostics.
#[derive(Clone, Debug)]
pub struct Candidate {
    pub origin: Origin,
    pub path: PathBuf,
}

#[derive(Debug, thiserror::Error)]
pub enum InstallError {
    #[error("no Morrowind installation found (checked {} locations); set {ENV_VAR} or pass the path explicitly", .checked.len())]
    NotFound { checked: Vec<Candidate> },
    #[error("{0} is not a Morrowind installation: no Data Files/Morrowind.esm")]
    Invalid(PathBuf),
    #[error("failed to read {path}: {source}")]
    Io { path: PathBuf, source: std::io::Error },
}

impl MorrowindInstall {
    /// Validates a single location (install root or `Data Files`).
    pub fn open(path: impl AsRef<Path>) -> Result<Self, InstallError> {
        let path = path.as_ref();
        let (root, data_files) = if find_child(path, "Morrowind.esm").is_some() {
            (path.parent().map_or_else(|| path.to_path_buf(), Path::to_path_buf), path.to_path_buf())
        } else if let Some(data) = find_child(path, "Data Files")
            && find_child(&data, "Morrowind.esm").is_some()
        {
            (path.to_path_buf(), data)
        } else {
            return Err(InstallError::Invalid(path.to_path_buf()));
        };

        let ini = match find_child(&root, "Morrowind.ini") {
            Some(ini_path) => {
                let bytes =
                    std::fs::read(&ini_path).map_err(|source| InstallError::Io { path: ini_path.clone(), source })?;
                Some(MorrowindIni::parse(&bytes))
            }
            None => None,
        };
        Ok(Self { root, data_files, ini })
    }

    /// Detects an installation, trying `explicit` first.
    pub fn detect(explicit: Option<&Path>) -> Result<(Self, Origin), InstallError> {
        let candidates = candidates(explicit);
        for candidate in &candidates {
            if let Ok(install) = Self::open(&candidate.path) {
                return Ok((install, candidate.origin.clone()));
            }
        }
        Err(InstallError::NotFound { checked: candidates })
    }

    /// Content files to load, in configured order. Without an ini, just
    /// `Morrowind.esm`.
    pub fn game_files(&self) -> Vec<String> {
        match &self.ini {
            Some(ini) if !ini.game_files.is_empty() => ini.game_files.clone(),
            _ => vec!["Morrowind.esm".to_owned()],
        }
    }

    /// Archives to mount, lowest priority first. `Morrowind.bsa` always
    /// comes first; the engine loads it without an ini entry.
    pub fn archives(&self) -> Vec<String> {
        let mut archives = vec!["Morrowind.bsa".to_owned()];
        if let Some(ini) = &self.ini {
            archives.extend(ini.archives.iter().filter(|a| !a.eq_ignore_ascii_case("Morrowind.bsa")).cloned());
        }
        archives
    }

    /// Finds a file in `Data Files`, case-insensitively.
    pub fn data_file(&self, name: &str) -> Option<PathBuf> {
        find_child(&self.data_files, name)
    }
}

/// Every location detection would try, in order.
pub fn candidates(explicit: Option<&Path>) -> Vec<Candidate> {
    let mut out = Vec::new();
    if let Some(path) = explicit {
        out.push(Candidate { origin: Origin::Explicit, path: path.to_path_buf() });
    }
    if let Some(path) = std::env::var_os(ENV_VAR) {
        out.push(Candidate { origin: Origin::Environment, path: path.into() });
    }
    for (key, path) in registry_installs() {
        out.push(Candidate { origin: Origin::Registry(key), path });
    }
    for library in steam::libraries() {
        out.push(Candidate { origin: Origin::Steam, path: library.join("steamapps").join("common").join("Morrowind") });
    }
    out
}

/// Case-insensitive child lookup; exact match first.
fn find_child(dir: &Path, name: &str) -> Option<PathBuf> {
    let exact = dir.join(name);
    if exact.exists() {
        return Some(exact);
    }
    std::fs::read_dir(dir)
        .ok()?
        .filter_map(Result::ok)
        .find(|e| e.file_name().to_str().is_some_and(|n| n.eq_ignore_ascii_case(name)))
        .map(|e| e.path())
}

#[cfg(windows)]
fn registry_installs() -> Vec<(String, PathBuf)> {
    use winreg::RegKey;
    use winreg::enums::HKEY_LOCAL_MACHINE;

    const KEYS: &[(&str, &str)] = &[
        (r"SOFTWARE\WOW6432Node\Bethesda Softworks\Morrowind", "Installed Path"),
        (r"SOFTWARE\Bethesda Softworks\Morrowind", "Installed Path"),
        (r"SOFTWARE\WOW6432Node\GOG.com\Games\1435828767", "path"),
        (r"SOFTWARE\GOG.com\Games\1435828767", "path"),
    ];
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    KEYS.iter()
        .filter_map(|(key, value)| {
            let path: String = hklm.open_subkey(key).ok()?.get_value(value).ok()?;
            Some(((*key).to_owned(), PathBuf::from(path)))
        })
        .collect()
}

#[cfg(not(windows))]
fn registry_installs() -> Vec<(String, PathBuf)> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fake_install(dir: &Path, ini: Option<&str>) {
        std::fs::create_dir_all(dir.join("Data Files")).unwrap();
        std::fs::write(dir.join("Data Files").join("Morrowind.esm"), b"").unwrap();
        if let Some(ini) = ini {
            std::fs::write(dir.join("Morrowind.ini"), ini).unwrap();
        }
    }

    #[test]
    fn opens_root_or_data_files() {
        let dir = tempfile::tempdir().unwrap();
        fake_install(dir.path(), Some("[Game Files]\nGameFile0=Morrowind.esm\nGameFile1=Tribunal.esm\n[Archives]\nArchive 0=Tribunal.bsa\n"));

        let from_root = MorrowindInstall::open(dir.path()).unwrap();
        let from_data = MorrowindInstall::open(dir.path().join("Data Files")).unwrap();
        assert_eq!(from_root.root, from_data.root);
        assert_eq!(from_root.data_files, from_data.data_files);
        assert_eq!(from_root.game_files(), ["Morrowind.esm", "Tribunal.esm"]);
        assert_eq!(from_root.archives(), ["Morrowind.bsa", "Tribunal.bsa"]);
        assert!(from_root.data_file("MORROWIND.ESM").is_some());
    }

    #[test]
    fn defaults_without_ini() {
        let dir = tempfile::tempdir().unwrap();
        fake_install(dir.path(), None);
        let install = MorrowindInstall::open(dir.path()).unwrap();
        assert!(install.ini.is_none());
        assert_eq!(install.game_files(), ["Morrowind.esm"]);
        assert_eq!(install.archives(), ["Morrowind.bsa"]);
    }

    #[test]
    fn rejects_non_installs_and_explicit_wins() {
        let empty = tempfile::tempdir().unwrap();
        assert!(matches!(MorrowindInstall::open(empty.path()), Err(InstallError::Invalid(_))));

        let dir = tempfile::tempdir().unwrap();
        fake_install(dir.path(), None);
        let (install, origin) = MorrowindInstall::detect(Some(dir.path())).unwrap();
        assert_eq!(origin, Origin::Explicit);
        assert_eq!(install.root, dir.path());
    }
}
