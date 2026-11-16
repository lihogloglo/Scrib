//! Command-line and file overrides, as pure functions (the app wires them in T3). Spec:
//! `docs/specs/tunables.md` "Live in Bevy", "Command line".
//!
//! - `--set name=value` (repeatable): [`parse_set`].
//! - `--tunables <file.toml>`: [`parse_file`], a `name = value` table. Dotted names may be
//!   written bare (`shadow.lamp.fade_start_m = 15`, TOML's nested tables, flattened back to the
//!   dotted name) or quoted (`"shadow.lamp.fade_start_m" = 15`); `[shadow.lamp]` sections work too.
//! - [`apply_overrides`] sets them on one layer (normally `CommandLine`) and returns every
//!   refusal: an unknown name must fail the run, not silently measure the default.

use std::fmt;

use sf_tunables::{SetBy, Tunables};

/// One override: a tunable name and its value as text (parsed by the tunable's kind).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Override {
    pub name: String,
    pub value: String,
}

/// A bad `--set` argument or tunables file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseError(pub String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ParseError {}

/// Parses a `--set` argument, `name=value` (spaces around either side are trimmed; the value
/// may contain `=`).
pub fn parse_set(arg: &str) -> Result<Override, ParseError> {
    let Some((name, value)) = arg.split_once('=') else {
        return Err(ParseError(format!(
            "--set {arg:?}: expected name=value"
        )));
    };
    let (name, value) = (name.trim(), value.trim());
    if name.is_empty() || value.is_empty() {
        return Err(ParseError(format!(
            "--set {arg:?}: expected name=value"
        )));
    }
    Ok(Override {
        name: name.to_ascii_lowercase(),
        value: value.to_owned(),
    })
}

/// Parses a tunables file (TOML): every leaf becomes one override, sorted by name
/// (stable whatever the table layout). Booleans read
/// `on`/`off`, numbers as written, strings as is, an array of numbers as `a,b[,c]`.
pub fn parse_file(text: &str) -> Result<Vec<Override>, ParseError> {
    let table: toml::Table = text
        .parse()
        .map_err(|e: toml::de::Error| ParseError(format!("tunables file: {}", e.message())))?;
    let mut out = Vec::new();
    flatten("", &table, &mut out)?;
    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}

fn flatten(prefix: &str, table: &toml::Table, out: &mut Vec<Override>) -> Result<(), ParseError> {
    for (key, value) in table {
        let name = if prefix.is_empty() {
            key.to_ascii_lowercase()
        } else {
            format!("{prefix}.{}", key.to_ascii_lowercase())
        };
        if let toml::Value::Table(t) = value {
            flatten(&name, t, out)?;
        } else {
            let value = leaf(&name, value)?;
            out.push(Override { name, value });
        }
    }
    Ok(())
}

fn leaf(name: &str, value: &toml::Value) -> Result<String, ParseError> {
    Ok(match value {
        toml::Value::Boolean(b) => if *b { "on" } else { "off" }.to_owned(),
        toml::Value::Integer(i) => i.to_string(),
        toml::Value::Float(f) => f.to_string(),
        toml::Value::String(s) => s.clone(),
        toml::Value::Array(items) => {
            let parts: Result<Vec<String>, ParseError> = items
                .iter()
                .map(|v| match v {
                    toml::Value::Integer(i) => Ok(i.to_string()),
                    toml::Value::Float(f) => Ok(f.to_string()),
                    _ => Err(ParseError(format!(
                        "tunables file: {name}: arrays hold numbers only"
                    ))),
                })
                .collect();
            parts?.join(",")
        }
        toml::Value::Datetime(_) | toml::Value::Table(_) => {
            return Err(ParseError(format!(
                "tunables file: {name}: expected a bool, number, text or array of numbers"
            )));
        }
    })
}

/// A refused override.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refusal {
    pub name: String,
    pub reason: String,
}

impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.reason)
    }
}

/// Sets every override on `layer`. Unknown names and refused values are returned (and change
/// nothing); the rest apply. Run after every plugin registered its tunables.
pub fn apply_overrides(
    tunables: &mut Tunables,
    overrides: &[Override],
    layer: SetBy,
    note: Option<&str>,
) -> Vec<Refusal> {
    let mut refused = Vec::new();
    for o in overrides {
        let Some(id) = tunables.id(&o.name) else {
            refused.push(Refusal {
                name: o.name.clone(),
                reason: "no such tunable".into(),
            });
            continue;
        };
        let result = tunables
            .var(id)
            .kind()
            .parse(&o.value)
            .and_then(|v| tunables.set_noted(id, v, layer, note.map(str::to_owned)));
        if let Err(reason) = result {
            refused.push(Refusal {
                name: o.name.clone(),
                reason,
            });
        }
    }
    refused
}
