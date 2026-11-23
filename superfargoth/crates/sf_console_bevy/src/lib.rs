//! Bevy plugin for [`sf_console`]. Spec: `docs/specs/console.md`.
//!
//! - [`DevConsole`]: the console as a resource; its commands get `&mut World`.
//! - Variables are tunables: they live in the shared [`Tunables`] resource (`sf_tunables_bevy`),
//!   so a tunable a plugin registers is a console variable and a console variable is a tunable
//!   (one namespace, one store). The console holds the store only while it runs lines,
//!   completes or registers ([`DevConsole::with_tunables`]); everyone else reads
//!   `Res<Tunables>`. [`ConsolePlugin`] adds `TunablesPlugin` if it isn't there yet.
//! - Plugins register through [`ConsoleAppExt`] (in any order relative to
//!   [`ConsolePlugin`]); systems read variables by [`VarId`] from [`Tunables`], or gate on them
//!   with the [`console_flag`] run condition.
//! - A front end (or the command line, or a test) writes [`ConsoleSubmit`];
//!   lines run in `PreUpdate` ([`ConsoleSystems`], before the tunables apply), then every
//!   variable change of the last frame, from a line or from code, goes out as
//!   [`ConsoleVarChanged`] (and `TunableChanged`).
//! - Each variable remembers who set it ([`SetBy`]: default < mod < preset < user config <
//!   command line < code < console); [`ConsoleSubmit::from`] and
//!   [`ConsoleAppExt::set_console_var`] speak for a source. `reset <name>` takes back the
//!   console's value, `reset <name> all` every layer's.
//! - Game input systems gate on [`console_closed`] while the overlay is open.
//! - Feature switches ([`features`]): `toggle <name>` flips one at the console layer (an on/off
//!   variable that is no switch flips as before), `features [filter]` lists them.
//!
//! The in-game front end (input line, scrollback, completion popup, click-select,
//! inspector) is the `overlay` feature: [`overlay::ConsoleOverlayPlugin`].

use std::ops::{Deref, DerefMut};

use bevy_app::{App, Plugin, PreUpdate};
use bevy_ecs::prelude::*;

pub use sf_console;
pub use sf_console::SetBy;
use sf_console::{CommandSpec, Console, Invocation, Value, VarId, VarSpec};
pub use sf_tunables_bevy;
pub use sf_tunables_bevy::Tunables;
use sf_tunables_bevy::{TunableChanged, TunableSystems, TunablesPlugin, tunable_changed};

pub mod features;
#[cfg(feature = "overlay")]
pub mod overlay;

#[cfg(test)]
mod tests;

/// Adds the console resource, its messages and the system that runs submitted lines (and
/// `TunablesPlugin` if it isn't added yet: add that one first, or not at all).
pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<TunablesPlugin>() {
            app.add_plugins(TunablesPlugin);
        }
        app.init_resource::<DevConsole>()
            .init_resource::<ConsoleOpen>()
            .add_message::<ConsoleSubmit>()
            .add_message::<ConsoleVarChanged>()
            .configure_sets(PreUpdate, ConsoleSystems.before(TunableSystems::Apply))
            .add_systems(
                PreUpdate,
                (
                    run_console.in_set(ConsoleSystems),
                    forward_changes.in_set(TunableSystems::Bind),
                ),
            );
        features::add_commands(app);
    }
}

/// The dev console. Commands receive the `World` (with this resource and [`Tunables`] taken
/// out of it while they run: a command reaches the variables through its `Invocation`).
///
/// Its variables live in [`Tunables`]: outside [`DevConsole::with_tunables`] the console's
/// own store is empty, so read variables from `Res<Tunables>`, not `console.vars()`.
#[derive(Resource, Default)]
pub struct DevConsole(pub Console<World>);

impl Deref for DevConsole {
    type Target = Console<World>;
    fn deref(&self) -> &Console<World> {
        &self.0
    }
}

impl DerefMut for DevConsole {
    fn deref_mut(&mut self) -> &mut Console<World> {
        &mut self.0
    }
}

impl DevConsole {
    /// Runs `f` with the shared store in the console: anything that looks up, sets, completes
    /// or registers variables (`exec`, `complete`, `add_var`, the prompt's completion).
    pub fn with_tunables<R>(
        &mut self,
        tunables: &mut Tunables,
        f: impl FnOnce(&mut Console<World>) -> R,
    ) -> R {
        std::mem::swap(self.0.vars_mut(), &mut tunables.0);
        let out = f(&mut self.0);
        std::mem::swap(self.0.vars_mut(), &mut tunables.0);
        out
    }
}

/// Runs `f` on the console holding the shared store, with the rest of the world (both
/// resources created if missing; both out of the world while `f` runs).
fn with_console<R>(world: &mut World, f: impl FnOnce(&mut Console<World>, &mut World) -> R) -> R {
    world.get_resource_or_init::<DevConsole>();
    world.get_resource_or_init::<Tunables>();
    world.resource_scope(|world, mut console: Mut<DevConsole>| {
        world.resource_scope(|world, mut tunables: Mut<Tunables>| {
            let before = tunables.revision();
            // The swap itself isn't a change; a changed value is.
            let out = console
                .bypass_change_detection()
                .with_tunables(tunables.bypass_change_detection(), |c| f(c, world));
            if tunables.revision() != before {
                tunables.set_changed();
            }
            out
        })
    })
}

/// A line to run next `PreUpdate`. The console drains these: they aren't for other readers.
#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct ConsoleSubmit {
    pub line: String,
    /// Typed by the user: recorded in the history and echoed. `false` for lines from
    /// config files, the command line or scripts.
    pub typed: bool,
    /// Whom the line speaks for: variables it sets keep values from higher sources.
    pub source: SetBy,
}

impl ConsoleSubmit {
    /// A line typed in the console.
    pub fn typed(line: impl Into<String>) -> Self {
        Self {
            line: line.into(),
            typed: true,
            source: SetBy::Console,
        }
    }

    /// A line from a script or a test, with console priority, not recorded.
    pub fn exec(line: impl Into<String>) -> Self {
        Self::from(SetBy::Console, line)
    }

    /// A line from a config file ([`SetBy::Config`]) or the command line
    /// ([`SetBy::CommandLine`]), not recorded.
    pub fn from(source: SetBy, line: impl Into<String>) -> Self {
        Self {
            line: line.into(),
            typed: false,
            source,
        }
    }
}

/// Whether the console overlay is open (always `false` without the overlay). While it is,
/// game input systems should not run: gate them with [`console_closed`].
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ConsoleOpen(pub bool);

/// Run condition: the console overlay is closed (true without a console).
/// `app.add_systems(Update, player_input.run_if(console_closed))`.
pub fn console_closed(open: Option<Res<ConsoleOpen>>) -> bool {
    open.is_none_or(|o| !o.0)
}

/// A variable changed (one message per variable per frame, with its new value).
#[derive(Message, Clone, Debug, PartialEq)]
pub struct ConsoleVarChanged {
    pub id: VarId,
    pub name: String,
    pub value: Value,
}

/// Where submitted lines run, in `PreUpdate`, before `TunableSystems::Apply`.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ConsoleSystems;

/// Runs submitted lines.
fn run_console(world: &mut World) {
    let lines: Vec<ConsoleSubmit> = world
        .resource_mut::<Messages<ConsoleSubmit>>()
        .drain()
        .collect();
    if lines.is_empty() {
        return;
    }
    with_console(world, |console, world| {
        for s in lines {
            // Errors are already in the console's log.
            let _ = if s.typed {
                console.submit(world, &s.line)
            } else {
                console.exec_as(world, &s.line, s.source)
            };
        }
    });
}

/// Every tunable change of the frame (the tunables plugin reports them) as a console message.
fn forward_changes(
    mut changes: MessageReader<TunableChanged>,
    tunables: Res<Tunables>,
    mut out: MessageWriter<ConsoleVarChanged>,
) {
    out.write_batch(changes.read().map(|c| ConsoleVarChanged {
        id: c.id,
        name: c.name.clone(),
        value: tunables.get(c.id).clone(),
    }));
}

/// Registration from plugins. Creates the [`DevConsole`] and [`Tunables`] resources if they
/// aren't there yet. A refused registration (bad or duplicate name, bad default) panics: it's
/// a bug in the registering plugin, like a duplicate system set.
pub trait ConsoleAppExt {
    /// Registers a variable (a tunable in the shared store); keep the id to read it every
    /// frame from `Res<Tunables>`.
    fn add_console_var(&mut self, spec: VarSpec) -> VarId;

    /// Sets a variable on behalf of `source` (typically [`SetBy::CommandLine`] right after
    /// registering, from the app's argument parser). Ignored if a higher source set it;
    /// panics on a value of the wrong type (a bug in the caller).
    fn set_console_var(&mut self, id: VarId, value: impl Into<Value>, source: SetBy) -> &mut Self;

    fn add_console_command(
        &mut self,
        spec: CommandSpec,
        handler: impl FnMut(Invocation<'_, World>) -> Result<(), String> + Send + Sync + 'static,
    ) -> &mut Self;
}

impl ConsoleAppExt for App {
    fn add_console_var(&mut self, spec: VarSpec) -> VarId {
        with_console(self.world_mut(), |console, _| {
            console.add_var(spec).unwrap_or_else(|e| panic!("{e}"))
        })
    }

    fn set_console_var(&mut self, id: VarId, value: impl Into<Value>, source: SetBy) -> &mut Self {
        let mut tunables = self.world_mut().get_resource_or_init::<Tunables>();
        let name = tunables.var(id).name().to_owned();
        tunables
            .set_by(id, value.into(), source)
            .unwrap_or_else(|e| panic!("console variable {name:?}: {e}"));
        self
    }

    fn add_console_command(
        &mut self,
        spec: CommandSpec,
        handler: impl FnMut(Invocation<'_, World>) -> Result<(), String> + Send + Sync + 'static,
    ) -> &mut Self {
        with_console(self.world_mut(), |console, _| {
            console
                .add_command(spec, handler)
                .unwrap_or_else(|e| panic!("{e}"));
        });
        self
    }
}

/// Run condition: the on/off variable `id` is on (false without the store).
/// `app.add_systems(Update, draw_shadows.run_if(console_flag(shadows)))`.
pub fn console_flag(id: VarId) -> impl Fn(Option<Res<Tunables>>) -> bool + Clone {
    move |tunables| tunables.is_some_and(|t| t.flag(id))
}

/// Run condition: variable `id` changed since this condition last ran (true on its first
/// run, so a system applying a switch also applies the starting value). Pair with a system
/// in `Update` to apply a switch the frame it was typed:
/// `app.add_systems(Update, apply_ssao.run_if(console_var_changed(ssao)))`.
/// The same as `sf_tunables_bevy::tunable_changed`.
pub fn console_var_changed(
    id: VarId,
) -> impl FnMut(Option<Res<Tunables>>, Local<Option<u64>>) -> bool + Clone {
    tunable_changed(id)
}
