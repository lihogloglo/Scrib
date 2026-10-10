use std::fmt;
use std::sync::Arc;

/// A normalized virtual path.
///
/// Normalization: ASCII-lowercased, `/` separators, no leading or trailing
/// `/`, no empty or `.` segments. `..` segments are rejected, so a virtual
/// path can never escape its source. Case-insensitivity is deliberate: mod
/// content authored on Windows must resolve identically on every OS.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VfsPath(Arc<str>);

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PathError {
    #[error("path {0:?} contains a '..' segment")]
    ParentSegment(String),
    #[error("path is empty")]
    Empty,
}

impl VfsPath {
    /// Normalizes `raw`, accepting both `/` and `\` separators.
    pub fn new(raw: &str) -> Result<Self, PathError> {
        let mut out = String::with_capacity(raw.len());
        for segment in raw.split(['/', '\\']) {
            match segment {
                "" | "." => {}
                ".." => return Err(PathError::ParentSegment(raw.to_owned())),
                s => {
                    if !out.is_empty() {
                        out.push('/');
                    }
                    out.push_str(s);
                }
            }
        }
        if out.is_empty() {
            return Err(PathError::Empty);
        }
        out.make_ascii_lowercase();
        Ok(Self(out.into()))
    }

    /// Wraps an already-normalized string (or a range bound) without checks.
    pub(crate) fn raw(normalized: String) -> Self {
        Self(normalized.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The final segment's extension, without the dot.
    pub fn extension(&self) -> Option<&str> {
        let file = self.file_name();
        let dot = file.rfind('.')?;
        (dot > 0).then(|| &file[dot + 1..])
    }

    pub fn file_name(&self) -> &str {
        self.0.rsplit('/').next().unwrap_or(&self.0)
    }

    /// The parent directory, or `None` at the root.
    pub fn parent(&self) -> Option<&str> {
        self.0.rfind('/').map(|i| &self.0[..i])
    }

    /// The same path with its extension replaced (or added).
    #[must_use]
    pub fn with_extension(&self, ext: &str) -> Self {
        let file_start = self.0.rfind('/').map_or(0, |i| i + 1);
        let stem_end = match self.0[file_start..].rfind('.') {
            Some(dot) if dot > 0 => file_start + dot,
            _ => self.0.len(),
        };
        let mut out = String::with_capacity(stem_end + 1 + ext.len());
        out.push_str(&self.0[..stem_end]);
        out.push('.');
        out.push_str(&ext.to_ascii_lowercase());
        Self(out.into())
    }
}

impl fmt::Debug for VfsPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&*self.0, f)
    }
}

impl fmt::Display for VfsPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl TryFrom<&str> for VfsPath {
    type Error = PathError;
    fn try_from(raw: &str) -> Result<Self, Self::Error> {
        Self::new(raw)
    }
}
