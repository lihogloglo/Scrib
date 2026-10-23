//! Bevy plugin for [`sf_console`]. Spec: `docs/specs/console.md`.
//!
//! - [`DevConsole`]: the console as a resource; its commands get `&mut World`.
//! - Plugins register through [`ConsoleAppExt`] (in any order relative to
//!   [`ConsolePlugin`]); systems read variables by [`VarId`], or gate on them
//!   with the [`console_flag`] run condition.
//! - A front end (or the command line, or a test) writes [`ConsoleSubmit`];
//!   lines run in `PreUpdate` ([`ConsoleSystems`]), then every variable change
//!   of the last frame, from a line or from code, goes out as [`ConsoleVarChanged`].
//! - Each variable remembers who set it ([`SetBy`]: default < config < command
//!   line < code < console); [`ConsoleSubmit::from`] and
//!   [`ConsoleAppExt::set_console_var`] speak for a source.
//! - Game input systems gate on [`console_closed`] while the overlay is open.
//!
//! The in-game front end (input line, scrollback, completion popup, click-select,
//! inspector) is the `overlay` feature: [`overlay::ConsoleOverlayPlugin`].

use std::ops::{Deref, DerefMut};

use bevy_app::{App, Plugin, PreUpdate};
use bevy_ecs::prelude::*;

pub use sf_console;
pub use sf_console::SetBy;
use sf_console::{CommandSpec, Console, Invocation, Value, VarId, VarSpec};

#[cfg(feature = "overlay")]
pub mod overlay;

#[cfg(test)]
mod tests;

/// Adds the console resource, its messages and the system that runs submitted lines.
pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DevConsole>()
            .init_resource::<ConsoleOpen>()
            .add_message::<ConsoleSubmit>()
            .add_message::<ConsoleVarChanged>()
            .add_systems(PreUpdate, run_console.in_set(ConsoleSystems));
    }
}

/// The dev console. Commands receive the `World` (with this resource taken out of it
/// while they run, so a command can't reach the console through the world).
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

/// Where submitted lines run, in `PreUpdate`.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ConsoleSystems;

/// Runs submitted lines, then reports variable changes.
fn run_console(world: &mut World) {
    let lines: Vec<ConsoleSubmit> = world
        .resource_mut::<Messages<ConsoleSubmit>>()
        .drain()
        .collect();
    world.resource_scope(|world, mut console: Mut<DevConsole>| {
        for s in lines {
            // Errors are already in the console's log.
            let _ = if s.typed {
                console.submit(world, &s.line)
            } else {
                console.exec_as(world, &s.line, s.source)
            };
        }
        let changed: Vec<ConsoleVarChanged> = console
            .vars_mut()
            .take_changes()
            .into_iter()
            .map(|id| {
                let var = console.vars().var(id);
                ConsoleVarChanged {
                    id,
                    name: var.name().to_owned(),
                    value: var.value().clone(),
                }
            })
            .collect();
        world.write_message_batch(changed);
    });
}

/// Registration from plugins. Creates the [`DevConsole`] resource if it isn't there yet.
/// A refused registration (bad or duplicate name, bad default) panics: it's a bug in the
/// registering plugin, like a duplicate system set.
pub trait ConsoleAppExt {
    /// Registers a variable; keep the id to read it every frame.
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
        let mut console = self.world_mut().get_resource_or_init::<DevConsole>();
        console.add_var(spec).unwrap_or_else(|e| panic!("{e}"))
    }

    fn set_console_var(&mut self, id: VarId, value: impl Into<Value>, source: SetBy) -> &mut Self {
        let mut console = self.world_mut().get_resource_or_init::<DevConsole>();
        let name = console.vars().var(id).name().to_owned();
        console
            .vars_mut()
            .set_by(id, value.into(), source)
            .unwrap_or_else(|e| panic!("console variable {name:?}: {e}"));
        self
    }

    fn add_console_command(
        &mut self,
        spec: CommandSpec,
        handler: impl FnMut(Invocation<'_, World>) -> Result<(), String> + Send + Sync + 'static,
    ) -> &mut Self {
        let mut console = self.world_mut().get_resource_or_init::<DevConsole>();
        console
            .add_command(spec, handler)
            .unwrap_or_else(|e| panic!("{e}"));
        self
    }
}

/// Run condition: the on/off variable `id` is on (false without a console).
/// `app.add_systems(Update, draw_shadows.run_if(console_flag(shadows)))`.
pub fn console_flag(id: VarId) -> impl Fn(Option<Res<DevConsole>>) -> bool + Clone {
    move |console| console.is_some_and(|c| c.vars().flag(id))
}

/// Run condition: variable `id` changed since this condition last ran (true on its first
/// run, so a system applying a switch also applies the starting value). Pair with a system
/// in `Update` to apply a switch the frame it was typed:
/// `app.add_systems(Update, apply_ssao.run_if(console_var_changed(ssao)))`.
pub fn console_var_changed(
    id: VarId,
) -> impl FnMut(Option<Res<DevConsole>>, Local<Option<u64>>) -> bool + Clone {
    move |console, mut seen| {
        let Some(console) = console else {
            return false;
        };
        let generation = console.vars().var(id).generation();
        let changed = *seen != Some(generation);
        *seen = Some(generation);
        changed
    }
}
