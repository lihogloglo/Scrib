use std::collections::HashMap;
use std::io;

use crate::{PathError, VfsPath, VfsSource};

/// In-memory files: generated content, test fixtures, runtime overrides.
#[derive(Debug, Default)]
pub struct MemSource {
    label: String,
    files: HashMap<VfsPath, Vec<u8>>,
}

impl MemSource {
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into(), files: HashMap::new() }
    }

    pub fn insert(&mut self, path: &str, data: impl Into<Vec<u8>>) -> Result<(), PathError> {
        self.files.insert(VfsPath::new(path)?, data.into());
        Ok(())
    }

    /// Builder-style [`MemSource::insert`] that panics on an invalid path.
    #[must_use]
    pub fn with(mut self, path: &str, data: impl Into<Vec<u8>>) -> Self {
        self.insert(path, data).expect("valid virtual path");
        self
    }
}

impl VfsSource for MemSource {
    fn label(&self) -> &str {
        &self.label
    }

    fn visit_files(&self, visit: &mut dyn FnMut(VfsPath)) {
        for path in self.files.keys() {
            visit(path.clone());
        }
    }

    fn read(&self, path: &VfsPath) -> io::Result<Vec<u8>> {
        self.files
            .get(path)
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, format!("{path} not in {}", self.label)))
    }
}
