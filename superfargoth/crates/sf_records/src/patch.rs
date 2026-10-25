//! Patch operations on a record's value tree (mods.md "Native record files").
//!
//! A field path is a dot-separated list of segments: struct field names, map
//! keys (strings, numbers, chars, bools, unit enum variants as written), and
//! list indices. `Option`s and enum payloads are transparent: `light.radius`
//! reaches into `light: Some(Light { radius })` or `kind: Torch(radius: ..)`.

use std::fmt;

use crate::value::Value;

/// One operation of a `patch`.
#[derive(Clone, Debug, PartialEq)]
pub enum PatchOp {
    /// Replaces the value at the path (a map entry or struct field may be new).
    Set { path: String, value: Value },
    /// Deep-merges a map or struct into the value at the path; other values replace.
    Merge { path: String, value: Value },
    /// Drops a map entry or list element, or resets a struct field to its
    /// serde default (`#[serde(default)]`, `Option` fields become `None`).
    Remove { path: String },
    /// Appends a list's elements to the list at the path.
    Append { path: String, value: Value },
}

impl PatchOp {
    pub fn path(&self) -> &str {
        match self {
            Self::Set { path, .. } | Self::Merge { path, .. } | Self::Remove { path } | Self::Append { path, .. } => path,
        }
    }

    /// The op's value, if it has one (for error positions).
    pub fn value(&self) -> Option<&Value> {
        match self {
            Self::Set { value, .. } | Self::Merge { value, .. } | Self::Append { value, .. } => Some(value),
            Self::Remove { .. } => None,
        }
    }
}

/// A field path that can't be applied.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PathError {
    pub path: String,
    pub message: String,
}

impl fmt::Display for PathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "field path {:?}: {}", self.path, self.message)
    }
}

fn err(path: &str, message: impl Into<String>) -> PathError {
    PathError { path: path.to_owned(), message: message.into() }
}

/// Whether two field paths overlap: equal, or one contains the other.
pub fn paths_overlap(a: &str, b: &str) -> bool {
    let (short, long) = if a.len() <= b.len() { (a, b) } else { (b, a) };
    long.strip_prefix(short).is_some_and(|rest| rest.is_empty() || rest.starts_with('.'))
}

fn segments(path: &str) -> Result<Vec<&str>, PathError> {
    let segs: Vec<&str> = path.split('.').collect();
    if segs.iter().any(|s| s.is_empty()) {
        return Err(err(path, "empty segment"));
    }
    Ok(segs)
}

/// Steps through `Some` and enum payloads.
fn transparent_mut(mut node: &mut Value) -> &mut Value {
    loop {
        match node {
            Value::Some(inner) | Value::Enum { payload: Some(inner), .. } => node = inner,
            other => return other,
        }
    }
}

fn transparent(mut node: &Value) -> &Value {
    loop {
        match node {
            Value::Some(inner) | Value::Enum { payload: Some(inner), .. } => node = inner,
            other => return other,
        }
    }
}

fn describe(node: &Value) -> &'static str {
    match node {
        Value::None => "None",
        Value::Raw(_) => "a value set earlier in this patch (patch its fields in a separate patch)",
        Value::Seq(_) => "a list",
        Value::Map(_) => "a map",
        Value::Struct(_) => "a struct",
        _ => "a plain value",
    }
}

fn child_mut<'v>(node: &'v mut Value, seg: &str, path: &str) -> Result<&'v mut Value, PathError> {
    let node = transparent_mut(node);
    let kind = describe(node);
    let found = match node {
        Value::Struct(fields) => fields.iter_mut().find(|(k, _)| k == seg).map(|(_, v)| v),
        Value::Map(entries) => entries.iter_mut().find(|(k, _)| k.key_text().as_deref() == Some(seg)).map(|(_, v)| v),
        Value::Seq(items) => seg.parse::<usize>().ok().and_then(|i| items.get_mut(i)),
        _ => return Err(err(path, format!("{seg:?} is inside {kind}"))),
    };
    found.ok_or_else(|| err(path, format!("{seg:?} does not exist")))
}

fn child<'v>(node: &'v Value, seg: &str) -> Option<&'v Value> {
    match transparent(node) {
        Value::Struct(fields) => fields.iter().find(|(k, _)| k == seg).map(|(_, v)| v),
        Value::Map(entries) => entries.iter().find(|(k, _)| k.key_text().as_deref() == Some(seg)).map(|(_, v)| v),
        Value::Seq(items) => seg.parse::<usize>().ok().and_then(|i| items.get(i)),
        _ => None,
    }
}

/// Whether `path` exists in `root`.
pub fn path_exists(root: &Value, path: &str) -> bool {
    let mut node = root;
    for seg in path.split('.') {
        match child(node, seg) {
            Some(next) => node = next,
            None => return false,
        }
    }
    true
}

fn parent_mut<'v>(root: &'v mut Value, segs: &[&str], path: &str) -> Result<&'v mut Value, PathError> {
    let mut node = root;
    for seg in &segs[..segs.len() - 1] {
        node = child_mut(node, seg, path)?;
    }
    Ok(transparent_mut(node))
}

/// A key for a new map entry: a string when the map's keys are strings (or it
/// is empty), else the segment as RON text, typed by the map's key type.
fn new_key(entries: &[(Value, Value)], seg: &str, template: &Value) -> Value {
    match entries.first() {
        None | Some((Value::String(_), _)) => Value::String(seg.to_owned()),
        Some((Value::I64(_), _)) if seg.parse::<i64>().is_ok() => Value::I64(seg.parse().unwrap_or_default()),
        Some((Value::U64(_), _)) if seg.parse::<u64>().is_ok() => Value::U64(seg.parse().unwrap_or_default()),
        Some(_) => match template {
            Value::Raw(raw) => Value::Raw(crate::value::Raw { text: seg.to_owned(), ..raw.clone() }),
            _ => Value::String(seg.to_owned()),
        },
    }
}

/// Writes `value` at `segs` (the last segment may be new in a map or struct).
fn set_at(root: &mut Value, path: &str, value: Value) -> Result<(), PathError> {
    let segs = segments(path)?;
    let last = segs[segs.len() - 1];
    let parent = parent_mut(root, &segs, path)?;
    let kind = describe(parent);
    match parent {
        Value::Struct(fields) => match fields.iter_mut().find(|(k, _)| k == last) {
            Some((_, slot)) => *slot = value,
            None => fields.push((last.to_owned(), value)),
        },
        Value::Map(entries) => {
            if let Some((_, slot)) = entries.iter_mut().find(|(k, _)| k.key_text().as_deref() == Some(last)) {
                *slot = value;
            } else {
                let key = new_key(entries, last, &value);
                entries.push((key, value));
            }
        }
        Value::Seq(items) => {
            let len = items.len();
            let slot = last.parse::<usize>().ok().and_then(|i| items.get_mut(i));
            *slot.ok_or_else(|| err(path, format!("index {last:?} is out of range (len {len}; use append)")))? = value;
        }
        _ => return Err(err(path, format!("{last:?} is inside {kind}"))),
    }
    Ok(())
}

/// The entries of a merge value, if it is a map or struct.
fn merge_entries(value: &Value) -> Option<Vec<(String, Value)>> {
    match value {
        Value::Raw(raw) => Some(raw.entries()?.into_iter().map(|(k, v)| (k, Value::Raw(v))).collect()),
        Value::Map(entries) => entries.iter().map(|(k, v)| Some((k.key_text()?, v.clone()))).collect(),
        Value::Struct(fields) => Some(fields.clone()),
        _ => None,
    }
}

/// Deep merge; records every leaf path written into `written`.
fn merge_into(target: &mut Value, path: &str, value: Value, written: &mut Vec<String>) -> Result<(), PathError> {
    let node = transparent_mut(target);
    let mergeable = matches!(node, Value::Struct(_) | Value::Map(_));
    let entries = if mergeable { merge_entries(&value) } else { None };
    let Some(entries) = entries else {
        *node = value;
        written.push(path.to_owned());
        return Ok(());
    };
    for (key, child) in entries {
        if key.is_empty() || key.contains('.') {
            return Err(err(path, format!("merge key {key:?} must be a single field name or map key")));
        }
        let child_path = format!("{path}.{key}");
        let existing = match &mut *node {
            Value::Struct(fields) => fields.iter_mut().find(|(k, _)| *k == key).map(|(_, v)| v),
            Value::Map(entries) => entries.iter_mut().find(|(k, _)| k.key_text().as_deref() == Some(&key)).map(|(_, v)| v),
            _ => None,
        };
        if let Some(slot) = existing {
            merge_into(slot, &child_path, child, written)?;
        } else {
            set_at(node, &key, child)?;
            written.push(child_path);
        }
    }
    Ok(())
}

fn append_at(root: &mut Value, path: &str, value: &Value) -> Result<(), PathError> {
    let segs = segments(path)?;
    let mut node = &mut *root;
    for seg in &segs {
        node = child_mut(node, seg, path)?;
    }
    let node = transparent_mut(node);
    let kind = describe(node);
    let Value::Seq(items) = node else { return Err(err(path, format!("append needs a list, this is {kind}"))) };
    match value {
        Value::Raw(raw) => {
            let elements = raw.elements().ok_or_else(|| err(path, "append value must be a list"))?;
            items.extend(elements.into_iter().map(Value::Raw));
        }
        Value::Seq(more) => items.extend(more.iter().cloned()),
        _ => return Err(err(path, "append value must be a list")),
    }
    Ok(())
}

fn remove_at(root: &mut Value, path: &str) -> Result<(), PathError> {
    let segs = segments(path)?;
    let last = segs[segs.len() - 1];
    let parent = parent_mut(root, &segs, path)?;
    let kind = describe(parent);
    let removed = match parent {
        Value::Struct(fields) => fields.iter().position(|(k, _)| k == last).map(|i| drop(fields.remove(i))),
        Value::Map(entries) => {
            entries.iter().position(|(k, _)| k.key_text().as_deref() == Some(last)).map(|i| drop(entries.remove(i)))
        }
        Value::Seq(items) => last.parse::<usize>().ok().filter(|i| *i < items.len()).map(|i| drop(items.remove(i))),
        _ => return Err(err(path, format!("{last:?} is inside {kind}"))),
    };
    removed.ok_or_else(|| err(path, format!("{last:?} does not exist")))
}

/// Field paths a patch wrote.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Written {
    /// Set, appended, and merged leaves: these must exist in the result.
    pub values: Vec<String>,
    /// Removed paths (may be absent from the result: defaults can be skipped).
    pub removed: Vec<String>,
}

impl Written {
    /// Every path touched, sorted, without duplicates.
    pub fn all(&self) -> Vec<String> {
        let mut all: Vec<String> = self.values.iter().chain(&self.removed).cloned().collect();
        all.sort();
        all.dedup();
        all
    }
}

/// Applies `ops` in order to `root` and returns the field paths written.
pub fn apply_ops(root: &mut Value, ops: &[PatchOp]) -> Result<Written, PathError> {
    let mut written = Vec::with_capacity(ops.len());
    let mut removed = Vec::new();
    for op in ops {
        match op {
            PatchOp::Set { path, value } => {
                set_at(root, path, value.clone())?;
                written.push(path.clone());
            }
            PatchOp::Merge { path, value } => {
                let segs = segments(path)?;
                let mut node = &mut *root;
                for seg in &segs {
                    node = child_mut(node, seg, path)?;
                }
                merge_into(node, path, value.clone(), &mut written)?;
            }
            PatchOp::Remove { path } => {
                remove_at(root, path)?;
                removed.push(path.clone());
            }
            PatchOp::Append { path, value } => {
                append_at(root, path, value)?;
                written.push(path.clone());
            }
        }
    }
    Ok(Written { values: written, removed })
}
