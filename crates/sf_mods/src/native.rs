//! The framework's own record format as one more [`ContentAdapter`]: RON (and
//! TOML) files of `define` / `patch` / `delete` operations over the kinds that
//! were registered with serde (mods.md "Native record files").

use sf_records::{RecordFormat, apply_record_file};
use sf_vfs::VfsPath;

use crate::{ContentAdapter, RecordCx, RecordFileError};

/// Applies native record files (`.ron`, `.toml`). Pass it to
/// [`load_profile`](crate::load_profile) with the content pack's adapters; it
/// registers no kinds (content packs and framework crates do).
///
/// A file that doesn't parse is one error at its position and applies nothing;
/// otherwise each failing operation is an error at its own position and the
/// rest apply.
#[derive(Clone, Copy, Debug, Default)]
pub struct NativeRecords;

impl NativeRecords {
    pub const NAME: &'static str = "native";
}

impl ContentAdapter for NativeRecords {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn record_extensions(&self) -> &[&str] {
        &["ron", "toml"]
    }

    fn apply_records(&self, cx: &mut RecordCx<'_>, file: &VfsPath) -> Result<(), RecordFileError> {
        let format = file.extension().and_then(RecordFormat::from_extension);
        let format = format.ok_or_else(|| RecordFileError::new(format!("{file} is not a .ron or .toml file")))?;
        let bytes = cx.files.read(file).map_err(|e| RecordFileError::new(e.to_string()))?;
        let text = std::str::from_utf8(&bytes).map_err(|e| RecordFileError::new(format!("not UTF-8: {e}")))?;
        let text = text.strip_prefix('\u{feff}').unwrap_or(text);
        let outcome = apply_record_file(cx.db, cx.source, text, format)
            .map_err(|e| RecordFileError::new(e.message).at(e.line, e.column))?;
        for e in outcome.errors {
            cx.error(file, RecordFileError::new(e.message).at(e.line, e.column));
        }
        Ok(())
    }
}
