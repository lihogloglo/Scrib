//! Native record files: RON (and TOML for simple cases) lists of `define`,
//! `patch` and `delete` operations over serde kinds (mods.md "Native record files").
//!
//! A syntax error rejects the whole file. Otherwise each operation applies on
//! its own: an operation that fails (unknown kind, missing record, bad field
//! path, value of the wrong type) is reported with its line and column and
//! skipped, and a patch applies all of its field operations or none.

use std::fmt;

use ron::value::RawValue;
use serde::Deserialize;

use crate::value::{Entries, Raw, subslice_offset};
use crate::{PatchOp, RecordDb, RecordId, SourceId, Value};

/// A record file format.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecordFormat {
    Ron,
    Toml,
}

impl RecordFormat {
    /// The format of a file extension (lowercase, no dot).
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext {
            "ron" => Some(Self::Ron),
            "toml" => Some(Self::Toml),
            _ => None,
        }
    }
}

/// An error at a position in a record file (1-based line and column).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileError {
    pub message: String,
    pub line: u32,
    pub column: u32,
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}: {}", self.line, self.column, self.message)
    }
}

/// What a record file did.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FileOutcome {
    pub defined: usize,
    pub patched: usize,
    pub deleted: usize,
    /// Operations that failed and were skipped.
    pub errors: Vec<FileError>,
}

/// The RON extensions record files get by default (`radius: 5.0` fills an
/// `Option<f32>`); a file can enable more with `#![enable(...)]`.
pub const DEFAULT_EXTENSIONS: ron::extensions::Extensions = ron::extensions::Extensions::IMPLICIT_SOME;

/// Applies a record file's operations into `source`, in file order (TOML:
/// all defines, then patches, then deletes). `Err` when the file doesn't parse.
pub fn apply_record_file(
    db: &mut RecordDb,
    source: SourceId,
    text: &str,
    format: RecordFormat,
) -> Result<FileOutcome, FileError> {
    let ops = match format {
        RecordFormat::Ron => parse_ron(text)?,
        RecordFormat::Toml => parse_toml(text)?,
    };
    let mut outcome = FileOutcome::default();
    for op in ops {
        let at = op.at;
        let result = match op.op {
            Err(e) => Err(e),
            Ok(Action::Define { kind, id, value }) => {
                db.define_value(source, &kind, RecordId::new(&id), &value).map(|()| outcome.defined += 1).map_err(|e| {
                    let offset = e.offset();
                    (e.to_string(), offset)
                })
            }
            Ok(Action::Patch { kind, id, ops }) => db
                .patch_value(source, &kind, &RecordId::new(&id), &ops)
                .map(|()| outcome.patched += 1)
                .map_err(|e| {
                    let offset = e.offset().or_else(|| error_value_offset(&e, &ops));
                    (e.to_string(), offset)
                }),
            Ok(Action::Delete { kind, id }) => db
                .delete_by_name(source, &kind, RecordId::new(&id))
                .map(|()| outcome.deleted += 1)
                .map_err(|e| (e.to_string(), None)),
        };
        if let Err((message, offset)) = result {
            let (line, column) = line_col(text, offset.unwrap_or(at));
            outcome.errors.push(FileError { message, line, column });
        }
    }
    Ok(outcome)
}

/// For a field path error, the position of the op that names the path.
fn error_value_offset(error: &crate::RecordError, ops: &[PatchOp]) -> Option<usize> {
    let crate::RecordError::Path { error, .. } = error else { return None };
    let op = ops.iter().find(|op| op.path() == error.path || error.path.starts_with(&format!("{}.", op.path())))?;
    match op.value()? {
        Value::Raw(raw) => Some(raw.offset),
        _ => None,
    }
}

/// 1-based line and column (in characters) of a byte offset.
fn line_col(text: &str, offset: usize) -> (u32, u32) {
    let offset = offset.min(text.len());
    let before = text.get(..offset).unwrap_or(text);
    let line = before.matches('\n').count() + 1;
    let line_start = before.rfind('\n').map_or(0, |i| i + 1);
    let column = before[line_start..].chars().count() + 1;
    (u32::try_from(line).unwrap_or(u32::MAX), u32::try_from(column).unwrap_or(u32::MAX))
}

enum Action {
    Define { kind: String, id: String, value: Value },
    Patch { kind: String, id: String, ops: Vec<PatchOp> },
    Delete { kind: String, id: String },
}

/// One operation: its byte offset, and the action or why it doesn't parse.
struct FileOp {
    at: usize,
    op: Result<Action, (String, Option<usize>)>,
}

fn patch_ops(
    set: Vec<(String, Value)>,
    merge: Vec<(String, Value)>,
    remove: Vec<String>,
    append: Vec<(String, Value)>,
) -> Result<Vec<PatchOp>, String> {
    let mut ops: Vec<PatchOp> = set.into_iter().map(|(path, value)| PatchOp::Set { path, value }).collect();
    ops.extend(merge.into_iter().map(|(path, value)| PatchOp::Merge { path, value }));
    ops.extend(remove.into_iter().map(|path| PatchOp::Remove { path }));
    ops.extend(append.into_iter().map(|(path, value)| PatchOp::Append { path, value }));
    if ops.is_empty() {
        return Err("patch has no set, merge, remove or append".into());
    }
    Ok(ops)
}

// ---------------------------------------------------------------------------
// RON

#[derive(Deserialize)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
enum RonOp<'a> {
    Define {
        kind: String,
        id: String,
        #[serde(borrow)]
        value: &'a RawValue,
    },
    Patch {
        kind: String,
        id: String,
        #[serde(default, borrow)]
        set: Entries<'a>,
        #[serde(default, borrow)]
        merge: Entries<'a>,
        #[serde(default)]
        remove: Vec<String>,
        #[serde(default, borrow)]
        append: Entries<'a>,
    },
    Delete {
        kind: String,
        id: String,
    },
}

fn parse_ron(text: &str) -> Result<Vec<FileOp>, FileError> {
    let spanned = |e: &ron::error::SpannedError| FileError {
        message: e.code.to_string(),
        line: u32::try_from(e.span.start.line).unwrap_or(u32::MAX),
        column: u32::try_from(e.span.start.col).unwrap_or(u32::MAX),
    };
    let options = ron::Options::default().with_default_extension(DEFAULT_EXTENSIONS);
    let mut de = ron::Deserializer::from_str_with_options(text, &options).map_err(|e| spanned(&e))?;
    let extensions = de.extensions();
    let items: Vec<&RawValue> =
        Vec::deserialize(&mut de).map_err(|e| spanned(&de.span_error(e)))?;
    de.end().map_err(|e| spanned(&de.span_error(e)))?;

    Ok(items
        .into_iter()
        .map(|item| {
            let item = item.trim().get_ron();
            let raw = Raw { text: item.to_owned(), offset: subslice_offset(text, item), extensions };
            FileOp { at: raw.offset, op: ron_action(&raw) }
        })
        .collect())
}

fn ron_action(raw: &Raw) -> Result<Action, (String, Option<usize>)> {
    let op: RonOp<'_> = raw.deserialize_ron().map_err(|e| (e.message, e.at))?;
    let values = |entries: Entries<'_>| -> Vec<(String, Value)> {
        entries.0.into_iter().map(|(path, v)| (path, Value::Raw(raw.sub(v.trim().get_ron())))).collect()
    };
    Ok(match op {
        RonOp::Define { kind, id, value } => Action::Define { kind, id, value: Value::Raw(raw.sub(value.trim().get_ron())) },
        RonOp::Patch { kind, id, set, merge, remove, append } => {
            let ops = patch_ops(values(set), values(merge), remove, values(append)).map_err(|e| (e, None))?;
            Action::Patch { kind, id, ops }
        }
        RonOp::Delete { kind, id } => Action::Delete { kind, id },
    })
}

// ---------------------------------------------------------------------------
// TOML

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TomlFile {
    #[serde(default)]
    define: Vec<toml::Spanned<TomlDefine>>,
    #[serde(default)]
    patch: Vec<toml::Spanned<TomlPatch>>,
    #[serde(default)]
    delete: Vec<toml::Spanned<TomlDelete>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TomlDefine {
    kind: String,
    id: String,
    value: toml::Value,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TomlPatch {
    kind: String,
    id: String,
    #[serde(default)]
    set: toml::Table,
    #[serde(default)]
    merge: toml::Table,
    #[serde(default)]
    remove: Vec<String>,
    #[serde(default)]
    append: toml::Table,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TomlDelete {
    kind: String,
    id: String,
}

fn from_toml(value: toml::Value) -> Value {
    match value {
        toml::Value::String(s) => Value::String(s),
        toml::Value::Integer(n) => Value::I64(n),
        toml::Value::Float(f) => Value::F64(f),
        toml::Value::Boolean(b) => Value::Bool(b),
        toml::Value::Datetime(d) => Value::String(d.to_string()),
        toml::Value::Array(items) => Value::Seq(items.into_iter().map(from_toml).collect()),
        toml::Value::Table(table) => Value::Map(table_entries(table).into_iter().map(|(k, v)| (Value::String(k), v)).collect()),
    }
}

fn table_entries(table: toml::Table) -> Vec<(String, Value)> {
    table.into_iter().map(|(k, v)| (k, from_toml(v))).collect()
}

fn parse_toml(text: &str) -> Result<Vec<FileOp>, FileError> {
    let file: TomlFile = toml::from_str(text).map_err(|e| {
        let (line, column) = line_col(text, e.span().map_or(0, |s| s.start));
        FileError { message: e.message().trim().to_owned(), line, column }
    })?;
    let mut ops = Vec::new();
    for d in file.define {
        let at = d.span().start;
        let d = d.into_inner();
        ops.push(FileOp { at, op: Ok(Action::Define { kind: d.kind, id: d.id, value: from_toml(d.value) }) });
    }
    for p in file.patch {
        let at = p.span().start;
        let p = p.into_inner();
        let op = patch_ops(table_entries(p.set), table_entries(p.merge), p.remove, table_entries(p.append))
            .map(|ops| Action::Patch { kind: p.kind, id: p.id, ops })
            .map_err(|e| (e, None));
        ops.push(FileOp { at, op });
    }
    for d in file.delete {
        let at = d.span().start;
        let d = d.into_inner();
        ops.push(FileOp { at, op: Ok(Action::Delete { kind: d.kind, id: d.id }) });
    }
    Ok(ops)
}
