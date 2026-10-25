//! Morrowind's file layers as superfargoth VFS sources.
//!
//! Mount order (lowest priority first): `Morrowind.bsa`, then the archives
//! listed in `Morrowind.ini`, then loose files in `Data Files`. Loose files
//! always beat archives. The original engine compared file dates instead;
//! loose-wins is what every modern tool and mod author expects.

use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

use mw_install::MorrowindInstall;
use sf_vfs::{DirSource, Vfs, VfsBuilder, VfsPath, VfsSource};
use tes3::bsa::Archive;

/// A memory-mapped BSA archive.
pub struct BsaSource {
    label: String,
    archive: Archive<'static>,
    /// Normalized path -> name exactly as stored (needed for hash lookup).
    names: HashMap<VfsPath, Vec<u8>>,
    skipped: usize,
}

impl BsaSource {
    /// Opens and indexes an archive. Entries whose names are not valid UTF-8
    /// or not valid virtual paths are skipped and counted.
    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref();
        let archive = Archive::from_path(path)?;
        let label = path.file_name().map_or_else(|| path.display().to_string(), |n| n.to_string_lossy().into_owned());
        if !archive.has_names() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("{label} stores no file names")));
        }
        let mut names = HashMap::with_capacity(archive.len());
        let mut skipped = 0;
        for entry in archive.entries() {
            let raw = entry.name().expect("archive has names");
            match std::str::from_utf8(raw).ok().map(VfsPath::new) {
                Some(Ok(vpath)) => {
                    names.insert(vpath, raw.to_vec());
                }
                _ => skipped += 1,
            }
        }
        Ok(Self { label, archive, names, skipped })
    }

    /// Entries skipped because their names could not be represented.
    pub fn skipped(&self) -> usize {
        self.skipped
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }
}

impl VfsSource for BsaSource {
    fn label(&self) -> &str {
        &self.label
    }

    fn visit_files(&self, visit: &mut dyn FnMut(VfsPath)) {
        for path in self.names.keys() {
            visit(path.clone());
        }
    }

    fn read(&self, path: &VfsPath) -> io::Result<Vec<u8>> {
        self.names
            .get(path)
            .and_then(|raw| self.archive.get(raw))
            .map(|entry| entry.as_bytes().to_vec())
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, format!("{path} not in {}", self.label)))
    }
}

/// A non-fatal problem found while mounting.
#[derive(Debug)]
pub enum MountWarning {
    MissingArchive(String),
    UnreadableArchive { name: String, error: io::Error },
    SkippedNames { source: String, count: usize },
}

/// Mount result: the VFS plus anything worth telling the user.
pub struct Mounted {
    pub vfs: Vfs,
    pub warnings: Vec<MountWarning>,
}

#[derive(Debug, thiserror::Error)]
#[error("failed to index {path}: {source}")]
pub struct MountError {
    pub path: PathBuf,
    pub source: io::Error,
}

/// Mounts an installation's archives and loose files onto `builder`.
///
/// Missing or broken archives are warnings, not errors: a stale ini entry
/// should not stop the game from starting.
pub fn mount_install(install: &MorrowindInstall, builder: VfsBuilder) -> Result<Mounted, MountError> {
    let mut builder = builder;
    let mut warnings = Vec::new();

    for name in install.archives() {
        let Some(path) = install.data_file(&name) else {
            warnings.push(MountWarning::MissingArchive(name));
            continue;
        };
        match BsaSource::open(&path) {
            Ok(bsa) => {
                if bsa.skipped() > 0 {
                    warnings.push(MountWarning::SkippedNames { source: name.clone(), count: bsa.skipped() });
                }
                builder = builder.mount(bsa);
            }
            Err(error) => warnings.push(MountWarning::UnreadableArchive { name, error }),
        }
    }

    let loose = DirSource::scan("Data Files", &install.data_files)
        .map_err(|source| MountError { path: install.data_files.clone(), source })?;
    if loose.skipped() > 0 {
        warnings.push(MountWarning::SkippedNames { source: "Data Files".into(), count: loose.skipped() });
    }
    builder = builder.mount(loose);

    Ok(Mounted { vfs: builder.build(), warnings })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tes3::bsa::Builder;

    fn p(s: &str) -> VfsPath {
        VfsPath::new(s).unwrap()
    }

    fn write_bsa(path: &Path, files: &[(&str, &str)]) {
        let mut builder = Builder::new();
        for (name, data) in files {
            builder.insert(name.as_bytes().to_vec(), data.as_bytes()).unwrap();
        }
        builder.save_path(path).unwrap();
    }

    #[test]
    fn bsa_source_reads_by_normalized_path() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("Test.bsa");
        write_bsa(&path, &[("meshes\\x\\Rock.nif", "rock"), ("textures\\a.dds", "dds")]);

        let bsa = BsaSource::open(&path).unwrap();
        assert_eq!(bsa.len(), 2);
        assert_eq!(bsa.read(&p("meshes/x/rock.nif")).unwrap(), b"rock");
        assert_eq!(bsa.read(&p("nope")).unwrap_err().kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn mounts_archives_then_loose_files() {
        let dir = tempfile::tempdir().unwrap();
        let data = dir.path().join("Data Files");
        std::fs::create_dir_all(data.join("Meshes")).unwrap();
        std::fs::write(data.join("Morrowind.esm"), b"").unwrap();
        std::fs::write(dir.path().join("Morrowind.ini"), "[Archives]\nArchive 0=Tribunal.bsa\nArchive 1=Missing.bsa\n")
            .unwrap();
        write_bsa(&data.join("Morrowind.bsa"), &[("meshes\\a.nif", "base"), ("meshes\\b.nif", "base")]);
        write_bsa(&data.join("Tribunal.bsa"), &[("meshes\\b.nif", "tribunal")]);
        std::fs::write(data.join("Meshes").join("A.nif"), "loose").unwrap();

        let install = MorrowindInstall::open(dir.path()).unwrap();
        let mounted = mount_install(&install, Vfs::builder()).unwrap();
        let vfs = &mounted.vfs;

        assert_eq!(vfs.read(&p("meshes/a.nif")).unwrap(), b"loose");
        assert_eq!(vfs.read(&p("meshes/b.nif")).unwrap(), b"tribunal");
        assert!(vfs.contains(&p("morrowind.esm")));
        let labels: Vec<String> = vfs.stats().into_iter().map(|s| s.label).collect();
        assert_eq!(labels, ["Morrowind.bsa", "Tribunal.bsa", "Data Files"]);
        assert!(matches!(&mounted.warnings[..], [MountWarning::MissingArchive(name)] if name == "Missing.bsa"));
    }
}
