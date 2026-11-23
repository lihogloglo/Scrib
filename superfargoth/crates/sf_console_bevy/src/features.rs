//! Feature-switch console commands (slice FS-2a; spec `docs/specs/feature-switches.md` "Console
//! and command line").
//!
//! - `toggle <name>` flips a feature switch at the console layer, through a [`SetTunable`]
//!   request (applied the same frame, `TunableSystems::Apply` runs after the console). It
//!   replaces the console's built-in `toggle`: a plain on/off variable that is no switch still
//!   flips as before. A switch not built this run answers why and changes nothing; a blocked one
//!   is set and says what blocks it.
//! - `features [filter]` lists the switches by category: state, layer, effect, lifetime (when not
//!   live), availability, label. The filter is a substring of the name, label or category.
//! - [`set_feature`] is the same flip for a game's own `on`/`off`/`flip` commands.

use bevy_app::App;
use bevy_ecs::prelude::*;
use sf_console::{ArgKind, CommandSpec, NameFilter, Param, SetOutcome, Value, Vars, closest_names};
use sf_tunables_bevy::sf_tunables::{self, Lifetime};
use sf_tunables_bevy::{
    Availability, Category, Effect, FeatureSwitch, FeatureSwitches, OffValue, SetBy, SetTunable,
    SwitchState,
};

use crate::with_console;

/// Registers `toggle` (in place of the built-in) and `features`.
pub(crate) fn add_commands(app: &mut App) {
    with_console(app.world_mut(), |console, _| {
        console
            .replace_command(
                CommandSpec::new("toggle")
                    .help(
                        "Flip a feature switch (console layer; `features` lists them) or an on/off \
                         variable",
                    )
                    .param(Param::new("name", ArgKind::Name(NameFilter::Vars))),
                |inv| {
                    let name = inv.args.str("name").unwrap_or_default().to_ascii_lowercase();
                    let text = match set_feature(inv.ctx, inv.vars, &name, None) {
                        Some(answer) => answer?,
                        None => flip_variable(inv.vars, &name)?,
                    };
                    inv.log.info(&text);
                    Ok(())
                },
            )
            .unwrap_or_else(|e| panic!("{e}"));
        console
            .add_command(
                CommandSpec::new("features")
                    .help(
                        "List the feature switches by category: state, layer, effect, lifetime, \
                         availability; optionally those whose name, label or category contains text",
                    )
                    .param(Param::new("filter", ArgKind::Str).optional()),
                |inv| {
                    let Some(switches) = inv.ctx.get_resource::<FeatureSwitches>() else {
                        return Err("no feature switches in this app".to_owned());
                    };
                    for line in feature_lines(switches, inv.vars, inv.args.str("filter")) {
                        inv.log.info(&line);
                    }
                    Ok(())
                },
            )
            .unwrap_or_else(|e| panic!("{e}"));
    });
}

/// Sets feature switch `name` at the console layer: `on` = `Some(true)` / `Some(false)`, or
/// `None` to flip its effective value. Writes a [`SetTunable`] (applied in this frame's
/// `TunableSystems::Apply` when called from a console line) and returns the answer line, e.g.
/// `sky.froxels.enabled: off (console; skips)`. `None` when `name` is no registered switch
/// (or the app has no [`FeatureSwitches`]); `Some(Err)` when the feature was not built this run
/// or the switch is startup only ([`FeatureSwitch::startup_only`]; nothing is set).
pub fn set_feature(
    world: &mut World,
    vars: &Vars,
    name: &str,
    on: Option<bool>,
) -> Option<Result<String, String>> {
    let switches = world.get_resource::<FeatureSwitches>()?;
    let switch = switches.get(name)?;
    let id = switches.tunable(name)?;
    let state = switches.state(vars, name)?;
    if let Availability::NotBuilt { reason } = state.availability {
        return Some(Err(format!(
            "{}: not built this run ({reason}); unchanged, it comes back on a restart without it",
            switch.name()
        )));
    }
    if let Some(hint) = switch.startup_hint() {
        return Some(Err(format!("{}: startup only: {hint}; unchanged", switch.name())));
    }
    let blocked = match state.availability {
        Availability::Blocked { by } => Some(by),
        _ => None,
    };
    let turn_on = on.unwrap_or(!state.on);
    let tunable = vars.var(id);
    let value = if turn_on {
        on_value(switch.off_value(), tunable)
    } else {
        switch.off_value().to_value()
    };
    let lifetime = tunable.meta().lifetime;
    let text = value.to_string();
    if world
        .write_message(SetTunable::new(switch.name(), text, SetBy::Console).note("toggle"))
        .is_none()
    {
        return Some(Err(format!("{}: no tunables plugin to apply it", switch.name())));
    }
    // The console layer is the strongest, so the value written is the one that applies.
    let mut line = format!(
        "{}: {} (console; {})",
        switch.name(),
        state_word(switch, turn_on, &value),
        effect_name(switch.effect_of())
    );
    if let Some(notice) = lifetime.notice() {
        line.push_str("; ");
        line.push_str(notice);
    }
    if let Some(by) = blocked {
        line.push_str("; blocked by ");
        line.push_str(by);
    }
    Some(Ok(line))
}

/// Up to three registered switch names close to `name` (the console's suggestion rule), for a
/// game command's "unknown name" answer. Empty without [`FeatureSwitches`].
pub fn feature_suggestions(world: &World, name: &str) -> Vec<String> {
    world
        .get_resource::<FeatureSwitches>()
        .map(|s| closest_names(name, s.iter().map(FeatureSwitch::name)))
        .unwrap_or_default()
}

/// One header line per category that has a matching switch (`sky:`), then one line per switch
/// (`  sky.froxels.enabled: on (default; skips) - Volumetric fog`); a line saying so when none
/// match.
pub fn feature_lines(
    switches: &sf_tunables::FeatureSwitches,
    vars: &Vars,
    filter: Option<&str>,
) -> Vec<String> {
    let filter = filter.map(str::to_ascii_lowercase);
    let matches = |s: &FeatureSwitch| {
        filter.as_deref().is_none_or(|f| {
            s.name().to_ascii_lowercase().contains(f)
                || s.label_text().to_ascii_lowercase().contains(f)
                || s.category_of().name().contains(f)
        })
    };
    let mut lines = Vec::new();
    for category in Category::ALL {
        let mut header = false;
        for switch in switches.by_category(category).filter(|s| matches(s)) {
            let Some(state) = switches.state(vars, switch.name()) else {
                continue;
            };
            if !header {
                lines.push(format!("{}:", category.name()));
                header = true;
            }
            lines.push(format!("  {}", switch_line(switch, &state, vars, switches)));
        }
    }
    if lines.is_empty() {
        lines.push(match filter {
            Some(f) => format!("no feature switch matches \"{f}\""),
            None => "no feature switches registered".to_owned(),
        });
    }
    lines
}

/// `name: on (layer; effect[; lifetime][; not built: reason | blocked by x])[ - label]`.
fn switch_line(
    switch: &FeatureSwitch,
    state: &SwitchState<'_>,
    vars: &Vars,
    switches: &sf_tunables::FeatureSwitches,
) -> String {
    let tunable = switches.tunable(switch.name()).map(|id| vars.var(id));
    let value = tunable.map_or(Value::Bool(state.on), |t| t.value().clone());
    let mut line = format!(
        "{}: {} ({}; {}",
        switch.name(),
        state_word(switch, state.on, &value),
        state.layer.describe(),
        effect_name(switch.effect_of())
    );
    let lifetime = tunable.map_or(Lifetime::Live, |t| t.meta().lifetime);
    if lifetime != Lifetime::Live {
        line.push_str("; ");
        line.push_str(lifetime.name());
    }
    match state.availability {
        Availability::Available => {}
        Availability::NotBuilt { reason } => {
            line.push_str("; not built: ");
            line.push_str(reason);
        }
        Availability::Blocked { by } => {
            line.push_str("; blocked by ");
            line.push_str(by);
        }
    }
    line.push(')');
    if switch.label_text() != switch.name() {
        line.push_str(" - ");
        line.push_str(switch.label_text());
    }
    line
}

/// `on`/`off` for a bool switch; the choice itself for a choice switch that is on.
fn state_word(switch: &FeatureSwitch, on: bool, value: &Value) -> String {
    match (on, switch.off_value()) {
        (false, _) => "off".to_owned(),
        (true, OffValue::Bool(_)) => "on".to_owned(),
        (true, OffValue::Choice(_)) => value.to_string(),
    }
}

const fn effect_name(effect: Effect) -> &'static str {
    match effect {
        Effect::Skips => "skips",
        Effect::Hides => "hides",
    }
}

/// The value that turns a switch on: the opposite bool; for a choice, the tunable's default
/// when that isn't the off value, else its first option that isn't.
fn on_value(off: OffValue, tunable: &sf_tunables::Tunable) -> Value {
    match off {
        OffValue::Bool(b) => Value::Bool(!b),
        OffValue::Choice(_) if !off.matches(tunable.default()) => tunable.default().clone(),
        OffValue::Choice(_) => match tunable.kind() {
            ArgKind::Choice(options) => options
                .iter()
                .find(|o| !off.matches(&Value::Str((*o).clone())))
                .map_or_else(|| tunable.default().clone(), |o| Value::Str(o.clone())),
            _ => tunable.default().clone(),
        },
    }
}

/// The built-in `toggle` for a variable that is no feature switch: flips an on/off variable at
/// the console layer.
fn flip_variable(vars: &mut Vars, name: &str) -> Result<String, String> {
    let id = vars
        .id(name)
        .ok_or_else(|| format!("no feature switch or variable \"{name}\""))?;
    if vars.var(id).kind() != &ArgKind::Bool {
        return Err(format!(
            "{name} is not a feature switch or an on/off variable (`features` lists the switches)"
        ));
    }
    let on = !vars.flag(id);
    Ok(match vars.set_by(id, on.into(), SetBy::Console)? {
        SetOutcome::Outranked(by) => format!("{name} kept: set by {}", by.describe()),
        _ => vars.var(id).describe(),
    })
}
