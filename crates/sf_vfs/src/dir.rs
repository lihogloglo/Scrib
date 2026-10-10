use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

use crate::{VfsPath, VfsSource};

/// A directory of loose files, indexed once at construction.
#[derive(Debug)]
pub struct DirSource {
    label: String,
    root: PathBuf,
    files: HashMap<VfsPath, PathBuf>,
    skipped: usize,
}

impl DirSource {
    /// Recursively indexes `root`. Files whose names are not valid UTF-8 are
    /// skipped and counted in [`DirSource::skipped`].
    pub fn scan(label: impl Into<String>, root: impl Into<PathBuf>) -> io::Result<Self> {
        let root = root.into();
        let mut source = Self { label: label.into(), root: root.clone(), files: HashMap::new(), skipped: 0 };
        source.walk(&root, &root)?;
        Ok(source)
    }

    fn walk(&mut self, root: &Path, dir: &Path) -> io::Result<()> {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let file_type = entry.file_type()?;
            if file_type.is_dir() {
                self.walk(root, &path)?;
            } else if file_type.is_file() {
                let relative = path.strip_prefix(root).expect("walked paths stay under root");
                match relative.to_str().map(VfsPath::new) {
                    // Two files differing only in case on a case-sensitive
                    // filesystem: the first one found wins, deterministically
                    // enough for a malformed mod.
                    Some(Ok(vpath)) => {
                        self.files.entry(vpath).or_insert(path);
                    }
                    _ => self.skipped += 1,
                }
            }
        }
        Ok(())
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Files skipped because their names could not be represented.
    pub fn skipped(&self) -> usize {
        self.skipped
    }
}

impl VfsSource for DirSource {
    fn label(&self) -> &str {
        &self.label
    }

    fn visit_files(&self, visit: &mut dyn FnMut(VfsPath)) {
        for path in self.files.keys() {
            visit(path.clone());
        }
    }

    fn read(&self, path: &VfsPath) -> io::Result<Vec<u8>> {
        match self.files.get(path) {
            Some(real) => std::fs::read(real),
            None => Err(io::Error::new(io::ErrorKind::NotFound, format!("{path} not in {}", self.label))),
        }
    }

    fn real_path(&self, path: &VfsPath) -> Option<PathBuf> {
        self.files.get(path).cloned()
    }
}
