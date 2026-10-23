//! `sf/records`: the runtime record overlay (save.md "What is saved"): records created in play
//! (spells, potions, enchantments: Bethesda's "created objects") under the `save:` id namespace,
//! and runtime patches to existing records as field-path operations, the same ops mods use
//! (mods.md "Native record files").
//!
//! Values are stored as [`SavedValue`], a frozen copy of `sf_records::Value`'s shape, so the save
//! format doesn't follow `sf_records`' in-memory enum: a change there is a conversion here, not a
//! silent format change.

use ron::extensions::Extensions;
use serde::{Deserialize, Serialize};
use sf_records::{PatchOp, RecordDb, RecordError, RecordId, SourceId, Value, ValueError};

/// Prefix of ids created in play.
pub const SAVE_NAMESPACE: &str = "save:";

/// A record value in the save (mirrors `sf_records::Value`; RON text keeps only its text).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SavedValue {
    Unit,
    Bool(bool),
    I64(i64),
    U64(u64),
    F64(f64),
    Char(char),
    String(String),
    Bytes(Vec<u8>),
    None,
    Some(Box<SavedValue>),
    Seq(Vec<SavedValue>),
    Map(Vec<(SavedValue, SavedValue)>),
    Struct(Vec<(String, SavedValue)>),
    Enum { variant: String, payload: Option<Box<SavedValue>> },
    /// RON text (values that came from a record file and were never typed) with the file's RON
    /// extension bits.
    Ron { text: String, extensions: u64 },
}

impl From<&Value> for SavedValue {
    fn from(v: &Value) -> Self {
        match v {
            Value::Unit => Self::Unit,
            Value::Bool(b) => Self::Bool(*b),
            Value::I64(i) => Self::I64(*i),
            Value::U64(u) => Self::U64(*u),
            Value::F64(f) => Self::F64(*f),
            Value::Char(c) => Self::Char(*c),
            Value::String(s) => Self::String(s.clone()),
            Value::Bytes(b) => Self::Bytes(b.clone()),
            Value::None => Self::None,
            Value::Some(v) => Self::Some(Box::new(Self::from(&**v))),
            Value::Seq(s) => Self::Seq(s.iter().map(Self::from).collect()),
            Value::Map(m) => Self::Map(m.iter().map(|(k, v)| (Self::from(k), Self::from(v))).collect()),
            Value::Struct(f) => Self::Struct(f.iter().map(|(k, v)| (k.clone(), Self::from(v))).collect()),
            Value::Enum { variant, payload } => {
                Self::Enum { variant: variant.clone(), payload: payload.as_ref().map(|p| Box::new(Self::from(&**p))) }
            }
            Value::Raw(raw) => Self::Ron { text: raw.text.clone(), extensions: raw.extensions.bits() as u64 },
        }
    }
}

impl SavedValue {
    /// Back to a record value.
    pub fn to_value(&self) -> Value {
        match self {
            Self::Unit => Value::Unit,
            Self::Bool(b) => Value::Bool(*b),
            Self::I64(i) => Value::I64(*i),
            Self::U64(u) => Value::U64(*u),
            Self::F64(f) => Value::F64(*f),
            Self::Char(c) => Value::Char(*c),
            Self::String(s) => Value::String(s.clone()),
            Self::Bytes(b) => Value::Bytes(b.clone()),
            Self::None => Value::None,
            Self::Some(v) => Value::Some(Box::new(v.to_value())),
            Self::Seq(s) => Value::Seq(s.iter().map(Self::to_value).collect()),
            Self::Map(m) => Value::Map(m.iter().map(|(k, v)| (k.to_value(), v.to_value())).collect()),
            Self::Struct(f) => Value::Struct(f.iter().map(|(k, v)| (k.clone(), v.to_value())).collect()),
            Self::Enum { variant, payload } => {
                Value::Enum { variant: variant.clone(), payload: payload.as_ref().map(|p| Box::new(p.to_value())) }
            }
            Self::Ron { text, extensions } => Value::Raw(sf_records::Raw {
                text: text.clone(),
                offset: 0,
                extensions: Extensions::from_bits_truncate(usize::try_from(*extensions).unwrap_or(0)),
            }),
        }
    }
}

/// One field-path operation (mirrors `sf_records::PatchOp`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SavedOp {
    Set { path: String, value: SavedValue },
    Merge { path: String, value: SavedValue },
    Remove { path: String },
    Append { path: String, value: SavedValue },
}

impl From<&PatchOp> for SavedOp {
    fn from(op: &PatchOp) -> Self {
        match op {
            PatchOp::Set { path, value } => Self::Set { path: path.clone(), value: value.into() },
            PatchOp::Merge { path, value } => Self::Merge { path: path.clone(), value: value.into() },
            PatchOp::Remove { path } => Self::Remove { path: path.clone() },
            PatchOp::Append { path, value } => Self::Append { path: path.clone(), value: value.into() },
        }
    }
}

impl SavedOp {
    pub fn to_op(&self) -> PatchOp {
        match self {
            Self::Set { path, value } => PatchOp::Set { path: path.clone(), value: value.to_value() },
            Self::Merge { path, value } => PatchOp::Merge { path: path.clone(), value: value.to_value() },
            Self::Remove { path } => PatchOp::Remove { path: path.clone() },
            Self::Append { path, value } => PatchOp::Append { path: path.clone(), value: value.to_value() },
        }
    }
}

/// A record created in play.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreatedRecord {
    pub kind: String,
    pub id: RecordId,
    pub value: SavedValue,
}

/// Runtime patches to one record, in the order they were made.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecordPatch {
    pub kind: String,
    pub id: RecordId,
    pub ops: Vec<SavedOp>,
}

/// Changes to records made in play. Applied over the mods' records on load, as one more source
/// after every mod.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RecordOverlay {
    pub created: Vec<CreatedRecord>,
    pub patches: Vec<RecordPatch>,
    /// Counter behind fresh `save:` ids.
    pub next_id: u64,
}

/// Why an overlay entry was not applied (kept in the overlay: a returning mod brings it back).
#[derive(Clone, Debug, PartialEq)]
pub struct Skipped {
    pub kind: String,
    pub id: RecordId,
    pub error: String,
}

impl RecordOverlay {
    /// Records `value` as a new record of `kind` with a fresh `save:<stem>.<n>` id, returned.
    /// Apply it to the running database with [`RecordOverlay::apply`] or `define_value`.
    pub fn create<T: Serialize>(&mut self, kind: &str, stem: &str, value: &T) -> Result<RecordId, ValueError> {
        let value = Value::from_serialize(value)?;
        let id = RecordId::new(&format!("{SAVE_NAMESPACE}{stem}.{}", self.next_id));
        self.next_id += 1;
        self.created.push(CreatedRecord { kind: kind.to_owned(), id: id.clone(), value: (&value).into() });
        Ok(id)
    }

    /// Records runtime patches to an existing record.
    pub fn patch(&mut self, kind: &str, id: &RecordId, ops: &[PatchOp]) {
        let ops: Vec<SavedOp> = ops.iter().map(SavedOp::from).collect();
        match self.patches.last_mut() {
            Some(last) if last.kind == kind && last.id == *id => last.ops.extend(ops),
            _ => self.patches.push(RecordPatch { kind: kind.to_owned(), id: id.clone(), ops }),
        }
    }

    /// Defines the created records, then applies the patches, as `source` (a source added after
    /// every mod). Entries whose kind or record no longer exists, or whose field path is gone,
    /// are skipped and reported, not dropped (save.md "Field-path patches survive struct
    /// changes").
    pub fn apply(&self, db: &mut RecordDb, source: SourceId) -> Vec<Skipped> {
        let mut skipped = Vec::new();
        let mut skip = |kind: &str, id: &RecordId, e: RecordError| {
            skipped.push(Skipped { kind: kind.to_owned(), id: id.clone(), error: e.to_string() });
        };
        for c in &self.created {
            if let Err(e) = db.define_value(source, &c.kind, c.id.clone(), &c.value.to_value()) {
                skip(&c.kind, &c.id, e);
            }
        }
        for p in &self.patches {
            for op in &p.ops {
                if let Err(e) = db.patch_value(source, &p.kind, &p.id, std::slice::from_ref(&op.to_op())) {
                    skip(&p.kind, &p.id, e);
                }
            }
        }
        skipped
    }
}
