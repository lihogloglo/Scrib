//! Script state as plain data (scripting.md "State, timers and saves").
//!
//! Everything a context persists (storage, timer payloads, event payloads) goes through
//! [`StateValue`]: booleans, numbers, strings, vectors, entity handles and tables of those. No
//! functions, foreign userdata, metatables, cycles or shared references. Conversion from Lua
//! reports the offending path (`storage.quests[3].cb is a function`), and tables are stored with
//! sorted keys so the encoding is canonical (equal states give equal bytes, which the
//! determinism hash relies on). mlua's serde bridge can't do either (it follows Lua's hash order
//! and stops at the first unsupported value without a path), hence this small walker.

use std::collections::HashSet;
use std::ffi::c_void;
use std::fmt::Write as _;

use mlua::{Lua, Table, Value};
use serde::{Deserialize, Serialize};

use crate::handle::Handle;

/// A plain-data Lua value.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum StateValue {
    Nil,
    Bool(bool),
    Number(f64),
    /// Lua strings are byte strings; usually UTF-8.
    Str(Vec<u8>),
    Vector([f32; 3]),
    Handle(Handle),
    /// Entries sorted by key (canonical order).
    Table(Vec<(StateKey, StateValue)>),
}

/// A table key. Ordering is the canonical entry order.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum StateKey {
    Bool(bool),
    /// Integral number keys (array indices included).
    Int(i64),
    Str(Vec<u8>),
}

/// Why a Lua value isn't plain data. `path` names where (`storage.quests[3].cb`).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{path} {problem}")]
pub struct StateError {
    pub path: String,
    pub problem: String,
}

impl StateValue {
    /// An empty table.
    pub fn table() -> Self {
        StateValue::Table(Vec::new())
    }

    /// Converts a Lua value; `root` names it in errors (e.g. `"storage"`).
    pub fn from_lua(value: &Value, root: &str) -> Result<Self, StateError> {
        Walker {
            root,
            path: Vec::new(),
            seen: HashSet::new(),
        }
        .value(value)
    }

    /// Builds the Lua value. Tables are fresh, so the result shares nothing with other states.
    pub fn to_lua(&self, lua: &Lua) -> mlua::Result<Value> {
        Ok(match self {
            StateValue::Nil => Value::Nil,
            StateValue::Bool(b) => Value::Boolean(*b),
            StateValue::Number(n) => Value::Number(*n),
            StateValue::Str(s) => Value::String(lua.create_string(s)?),
            StateValue::Vector([x, y, z]) => Value::Vector(mlua::Vector::new(*x, *y, *z)),
            StateValue::Handle(h) => Value::UserData(lua.create_userdata(*h)?),
            StateValue::Table(entries) => {
                let t = lua.create_table_with_capacity(0, entries.len())?;
                for (k, v) in entries {
                    let key = match k {
                        StateKey::Bool(b) => Value::Boolean(*b),
                        #[allow(clippy::cast_precision_loss)]
                        // keys beyond 2^53 don't come from Luau numbers
                        StateKey::Int(i) => Value::Number(*i as f64),
                        StateKey::Str(s) => Value::String(lua.create_string(s)?),
                    };
                    t.raw_set(key, v.to_lua(lua)?)?;
                }
                Value::Table(t)
            }
        })
    }

    /// Looks up a string key in a table value (tests and diagnostics).
    pub fn get(&self, key: &str) -> Option<&StateValue> {
        match self {
            StateValue::Table(entries) => entries
                .iter()
                .find(|(k, _)| matches!(k, StateKey::Str(s) if s == key.as_bytes()))
                .map(|(_, v)| v),
            _ => None,
        }
    }
}

struct Walker<'a> {
    root: &'a str,
    /// Keys from the root to the current value; formatted only when reporting an error.
    path: Vec<StateKey>,
    /// Tables already visited: a second visit is a cycle or a shared reference, both rejected
    /// (plain data is a tree).
    seen: HashSet<*const c_void>,
}

impl Walker<'_> {
    fn fail(&self, problem: impl Into<String>) -> StateError {
        let mut path = self.root.to_owned();
        for k in &self.path {
            let _ = match k {
                StateKey::Str(s) => write!(path, ".{}", String::from_utf8_lossy(s)),
                StateKey::Int(i) => write!(path, "[{i}]"),
                StateKey::Bool(b) => write!(path, "[{b}]"),
            };
        }
        StateError {
            path,
            problem: problem.into(),
        }
    }

    fn value(&mut self, value: &Value) -> Result<StateValue, StateError> {
        Ok(match value {
            Value::Nil => StateValue::Nil,
            Value::Boolean(b) => StateValue::Bool(*b),
            // mlua reports integral numbers as `Integer`; Luau numbers are doubles. (The
            // experimental `integer` type is stored as a number too.)
            #[allow(clippy::cast_precision_loss)]
            Value::Integer(i) => StateValue::Number(*i as f64),
            Value::Number(n) => StateValue::Number(*n),
            Value::String(s) => StateValue::Str(s.as_bytes().to_vec()),
            Value::Vector(v) => StateValue::Vector([v.x(), v.y(), v.z()]),
            Value::UserData(ud) => match ud.borrow::<Handle>() {
                Ok(h) => StateValue::Handle(*h),
                Err(_) => return Err(self.fail("is userdata other than an entity handle")),
            },
            Value::Table(t) => self.table(t)?,
            Value::Function(_) => return Err(self.fail("is a function")),
            Value::Thread(_) => return Err(self.fail("is a coroutine")),
            Value::Buffer(_) => return Err(self.fail("is a buffer")),
            Value::LightUserData(_) => return Err(self.fail("is light userdata")),
            Value::Error(_) => return Err(self.fail("is an error object")),
            Value::Other(_) => return Err(self.fail("is not plain data")),
        })
    }

    fn table(&mut self, t: &Table) -> Result<StateValue, StateError> {
        if !self.seen.insert(t.to_pointer()) {
            return Err(self.fail("is a table reached twice (cycle or shared reference)"));
        }
        if t.metatable().is_some() {
            return Err(self.fail("has a metatable"));
        }
        let mut entries = Vec::with_capacity(t.raw_len());
        let mut failure = None;
        let walked = t.for_each::<Value, Value>(|k, v| {
            let Some(key) = key(&k) else {
                failure = Some(self.fail(format!(
                    "has a key of type {} (keys must be strings, integers or booleans)",
                    k.type_name()
                )));
                return Err(mlua::Error::runtime("stop"));
            };
            self.path.push(key);
            match self.value(&v) {
                Ok(v) => {
                    let key = self.path.pop().unwrap_or(StateKey::Bool(false));
                    entries.push((key, v));
                    Ok(())
                }
                Err(e) => {
                    failure = Some(e);
                    Err(mlua::Error::runtime("stop"))
                }
            }
        });
        if let Some(e) = failure {
            return Err(e);
        }
        walked.map_err(|e| self.fail(format!("can't be read: {e}")))?;
        entries.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        Ok(StateValue::Table(entries))
    }
}

fn key(k: &Value) -> Option<StateKey> {
    match k {
        Value::Boolean(b) => Some(StateKey::Bool(*b)),
        Value::Integer(i) => Some(StateKey::Int(*i)),
        #[allow(clippy::cast_possible_truncation, clippy::float_cmp)] // exact integral check
        Value::Number(n) if n.fract() == 0.0 && n.abs() < 9.007_199_254_740_992e15 => {
            Some(StateKey::Int(*n as i64))
        }
        Value::String(s) => Some(StateKey::Str(s.as_bytes().to_vec())),
        _ => None,
    }
}
