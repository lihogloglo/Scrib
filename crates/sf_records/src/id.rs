use std::fmt;
use std::sync::Arc;

/// Case-insensitive record identifier.
///
/// IDs are folded to ASCII lowercase on construction. ASCII-only folding is
/// deliberate: it is locale-independent and matches how Bethesda-era content
/// compares IDs, so mods that differ only in casing still address the same
/// record. The record value keeps its original spelling for display.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RecordId(Arc<str>);

impl RecordId {
    pub fn new(id: &str) -> Self {
        if id.bytes().any(|b| b.is_ascii_uppercase()) {
            Self(id.to_ascii_lowercase().into())
        } else {
            Self(id.into())
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for RecordId {
    fn from(id: &str) -> Self {
        Self::new(id)
    }
}

impl From<String> for RecordId {
    fn from(mut id: String) -> Self {
        id.make_ascii_lowercase();
        Self(id.into())
    }
}

impl fmt::Debug for RecordId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&*self.0, f)
    }
}

impl fmt::Display for RecordId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
