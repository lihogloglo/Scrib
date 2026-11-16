//! Bevy Remote Protocol methods for tunables. Spec: `docs/specs/tunables.md` "Live in Bevy", BRP.
//!
//! | Method | Params | Result |
//! |---|---|---|
//! | `sf.tunables/list` | `{filter?: text (name contains), changed?: bool (only non-default)}` | `{tunables: [row]}` |
//! | `sf.tunables/get` | `{name}` | row |
//! | `sf.tunables/set` | `{name, value}` (bool, number, text or `[a, b]`) | `{outcome, tunable: row}` |
//! | `sf.tunables/reset` | `{name, all?: bool}` | row |
//!
//! A row is `sf_tunables::Row` (value, layer, every set layer, unit, range in `kind`, source,
//! lifetime). `set` writes the `Console` layer, validated by kind and range (a refused value
//! changes nothing and is an invalid-params error); its outcome is `changed`, `unchanged` or
//! `outranked` (stored under a stronger layer). `reset` peels the `Console` layer, or every layer
//! but the default with `all`. The handlers are pure functions on the store, so they are tested
//! without a server; the `remote` feature registers them with Bevy Remote.

use serde_json::{Map, Value, json};
use sf_tunables::{SetBy, SetOutcome, TunableId, Tunables};

pub const LIST: &str = "sf.tunables/list";
pub const GET: &str = "sf.tunables/get";
pub const SET: &str = "sf.tunables/set";
pub const RESET: &str = "sf.tunables/reset";

/// JSON-RPC "invalid params" (`bevy_remote::error_codes::INVALID_PARAMS`).
// tunables: const (JSON-RPC protocol error code)
pub const INVALID_PARAMS: i16 = -32602;
/// JSON-RPC "internal error".
// tunables: const (JSON-RPC protocol error code)
pub const INTERNAL_ERROR: i16 = -32603;

/// A refused call: JSON-RPC error code and message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Failure {
    pub code: i16,
    pub message: String,
}

fn invalid(message: impl Into<String>) -> Failure {
    Failure {
        code: INVALID_PARAMS,
        message: message.into(),
    }
}

fn object(params: Option<&Value>) -> Result<Map<String, Value>, Failure> {
    match params {
        None | Some(Value::Null) => Ok(Map::new()),
        Some(Value::Object(m)) => Ok(m.clone()),
        Some(other) => Err(invalid(format!("expected an object of params, got {other}"))),
    }
}

fn name_param(params: &Map<String, Value>, tunables: &Tunables) -> Result<TunableId, Failure> {
    let name = params
        .get("name")
        .and_then(Value::as_str)
        .ok_or_else(|| invalid("missing \"name\" (text)"))?;
    tunables
        .id(name)
        .ok_or_else(|| invalid(format!("no such tunable: {name}")))
}

fn row(tunables: &Tunables, id: TunableId) -> Result<Value, Failure> {
    let name = tunables.var(id).name();
    let row = tunables
        .rows()
        .into_iter()
        .find(|r| r.name == name)
        .expect("a registered tunable has a row");
    serde_json::to_value(row).map_err(|e| Failure {
        code: INTERNAL_ERROR,
        message: e.to_string(),
    })
}

/// `sf.tunables/list`.
pub fn list(tunables: &Tunables, params: Option<&Value>) -> Result<Value, Failure> {
    let params = object(params)?;
    let filter = match params.get("filter") {
        None | Some(Value::Null) => None,
        Some(Value::String(f)) => Some(f.to_ascii_lowercase()),
        Some(_) => return Err(invalid("\"filter\" must be text")),
    };
    let changed = match params.get("changed") {
        None | Some(Value::Null) => false,
        Some(Value::Bool(b)) => *b,
        Some(_) => return Err(invalid("\"changed\" must be a bool")),
    };
    let rows: Vec<_> = tunables
        .rows()
        .into_iter()
        .filter(|r| filter.as_ref().is_none_or(|f| r.name.contains(f.as_str())))
        .filter(|r| !changed || !r.layers.is_empty())
        .collect();
    Ok(json!({ "tunables": rows }))
}

/// `sf.tunables/get`.
pub fn get(tunables: &Tunables, params: Option<&Value>) -> Result<Value, Failure> {
    let params = object(params)?;
    let id = name_param(&params, tunables)?;
    row(tunables, id)
}

/// `sf.tunables/set`: the `Console` layer, validated.
pub fn set(tunables: &mut Tunables, params: Option<&Value>) -> Result<Value, Failure> {
    let params = object(params)?;
    let id = name_param(&params, tunables)?;
    let text = value_text(params.get("value").ok_or_else(|| invalid("missing \"value\""))?)?;
    let name = tunables.var(id).name().to_owned();
    let outcome = tunables
        .set_text_by(id, &text, SetBy::Console)
        .map_err(|e| invalid(format!("{name}: {e}")))?;
    let outcome = match outcome {
        SetOutcome::Changed => "changed",
        SetOutcome::Unchanged => "unchanged",
        SetOutcome::Outranked(_) => "outranked",
    };
    Ok(json!({ "outcome": outcome, "tunable": row(tunables, id)? }))
}

/// `sf.tunables/reset`: peels the `Console` layer, or every layer with `all`.
pub fn reset(tunables: &mut Tunables, params: Option<&Value>) -> Result<Value, Failure> {
    let params = object(params)?;
    let id = name_param(&params, tunables)?;
    let all = match params.get("all") {
        None | Some(Value::Null) => false,
        Some(Value::Bool(b)) => *b,
        Some(_) => return Err(invalid("\"all\" must be a bool")),
    };
    if all {
        tunables.reset(id);
    } else {
        tunables.clear(id, SetBy::Console);
    }
    row(tunables, id)
}

/// A JSON value as the text a kind parses: `on`/`off`, numbers as written, text as is, an array
/// of numbers as `a,b[,c]`.
fn value_text(value: &Value) -> Result<String, Failure> {
    Ok(match value {
        Value::Bool(b) => if *b { "on" } else { "off" }.to_owned(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.clone(),
        Value::Array(items) => {
            let parts: Option<Vec<String>> = items
                .iter()
                .map(|v| v.as_number().map(ToString::to_string))
                .collect();
            parts
                .ok_or_else(|| invalid("an array value holds numbers only"))?
                .join(",")
        }
        Value::Null | Value::Object(_) => {
            return Err(invalid("\"value\" must be a bool, number, text or array of numbers"));
        }
    })
}

/// Registers the four methods into Bevy Remote's method table (after `RemotePlugin` built it;
/// called from `TunablesPlugin::finish`). Without `RemotePlugin` in the app, does nothing.
#[cfg(feature = "remote")]
pub(crate) fn register_remote_methods(app: &mut bevy_app::App) {
    use bevy_ecs::prelude::*;
    use bevy_remote::{BrpError, BrpResult, RemoteMethodSystemId, RemoteMethods};

    use crate::Tunables as Store;

    fn brp(result: Result<Value, Failure>) -> BrpResult {
        result.map_err(|f| BrpError {
            code: f.code,
            message: f.message,
            data: None,
        })
    }
    fn list_system(In(p): In<Option<Value>>, t: Res<Store>) -> BrpResult {
        brp(list(&t, p.as_ref()))
    }
    fn get_system(In(p): In<Option<Value>>, t: Res<Store>) -> BrpResult {
        brp(get(&t, p.as_ref()))
    }
    fn set_system(In(p): In<Option<Value>>, mut t: ResMut<Store>) -> BrpResult {
        brp(set(&mut t, p.as_ref()))
    }
    fn reset_system(In(p): In<Option<Value>>, mut t: ResMut<Store>) -> BrpResult {
        brp(reset(&mut t, p.as_ref()))
    }

    let world = app.world_mut();
    if !world.contains_resource::<RemoteMethods>() {
        return;
    }
    let ids = [
        (LIST, world.register_system(list_system)),
        (GET, world.register_system(get_system)),
        (SET, world.register_system(set_system)),
        (RESET, world.register_system(reset_system)),
    ];
    let mut methods = world.resource_mut::<RemoteMethods>();
    for (name, id) in ids {
        methods.insert(name, RemoteMethodSystemId::Instant(id));
    }
}
