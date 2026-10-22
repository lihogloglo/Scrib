//! Bevy plugin for [`sf_console`]. Spec: `docs/specs/console.md`.
//!
//! - [`DevConsole`]: the console as a resource; its commands get `&mut World`.
//! - Plugins register through [`ConsoleAppExt`] (in any order relative to
//!   [`ConsolePlugin`]); systems read variables by [`VarId`], or gate on them
//!   with the [`console_flag`] run condition.
//! - A front end (or the command line, or a test) writes [`ConsoleSubmit`];
//!   lines run in `PreUpdate` ([`ConsoleSystems`]), then every variable change
//!   of the last frame, from a line or from code, goes out as [`ConsoleVarChanged`].
//!
//! No UI here: the overlay is a later front end over [`sf_console::Log`],
//! [`sf_console::History`] and [`sf_console::Console::complete`].

use std::ops::{Deref, DerefMut};

use bevy_app::{App, Plugin, PreUpdate};
use bevy_ecs::prelude::*;

pub use sf_console;
use sf_console::{CommandSpec, Console, Invocation, Value, VarId, VarSpec};

#[cfg(test)]
mod tests;

/// Adds the console resource, its messages and the system that runs submitted lines.
pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DevConsole>()
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
}

impl ConsoleSubmit {
    pub fn typed(line: impl Into<String>) -> Self {
        Self {
            line: line.into(),
            typed: true,
        }
    }

    pub fn exec(line: impl Into<String>) -> Self {
        Self {
            line: line.into(),
            typed: false,
        }
    }
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
                console.exec(world, &s.line)
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
