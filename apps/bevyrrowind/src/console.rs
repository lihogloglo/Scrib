//! The dev console in the interactive viewer (`view` and `scene`, never in screenshot or
//! benchmark runs; ROADMAP 1.13, superfargoth `docs/specs/console.md`). The backquote key
//! (the one left of 1, physical: `²` on AZERTY) opens it; F1 stays the A/B help.
//!
//! - `render.ssao` (on/off variable): Bevy GTAO on the streaming camera, live. Starts as
//!   the run started (`--off ssao` or the Low preset: off). `toggle render.ssao`,
//!   `render.ssao off`, or `off ssao`.
//! - `on <name>` / `off <name>` / `flip <name>`: `ssao` and the switches the A/B keys flip
//!   ([`Switch`]: `sun_shadows`, `clouds`, ...). Any other `--off` part is decided at
//!   startup: the console says so instead of doing nothing.

use bevy::core_pipeline::prepass::{DepthPrepass, MotionVectorPrepass};
use bevy::ecs::system::SystemState;
use bevy::pbr::{ScreenSpaceAmbientOcclusion, ScreenSpaceAmbientOcclusionQualityLevel};
use bevy::prelude::*;
use bevy_enhanced_input::EnhancedInputSystems;
use sf_console_bevy::overlay::{ConsoleInputSystems, ConsoleKeys, ConsoleOverlayPlugin};
use sf_console_bevy::sf_console::{ArgKind, CommandSpec, Param, VarId, VarSpec, Vars};
use sf_console_bevy::{ConsoleAppExt, ConsolePlugin, DevConsole, SetBy};

use crate::exterior::ExteriorCamera;
use crate::live_ab::{LiveAb, Switch, Switches};

/// Name of the SSAO variable.
const SSAO_VAR: &str = "render.ssao";

/// The console, its overlay on backquote, `render.ssao` and the `on`/`off`/`flip` commands.
pub struct ConsoleWiring;

impl Plugin for ConsoleWiring {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<ConsolePlugin>() {
            app.add_plugins(ConsolePlugin);
        }
        // F1 is the A/B help (`live_ab.rs`): backquote only.
        app.add_plugins(ConsoleOverlayPlugin).insert_resource(ConsoleKeys { toggle: vec![KeyCode::Backquote] });
        // The open console clears the keys before the player's actions read them (console.md "Overlay").
        app.configure_sets(
            PreUpdate,
            ConsoleInputSystems.before(EnhancedInputSystems::Prepare).before(EnhancedInputSystems::Update),
        );
        register(app);
    }
}

/// `render.ssao`, its system and the `on`/`off`/`flip` commands (needs [`ConsolePlugin`] and [`LiveAb`]).
fn register(app: &mut App) {
    let ssao = app.add_console_var(
        VarSpec::bool(SSAO_VAR, true).help("Screen-space ambient occlusion (Bevy GTAO) on the camera; starts as the run did (--off ssao, Low preset: off)"),
    );
    app.insert_resource(Ssao { var: ssao, level: None, applied: None }).add_systems(Update, apply_ssao);
    let live = format!("live: ssao, {}; other --off parts are startup only", Switch::names());
    for (name, on) in [("on", Some(true)), ("off", Some(false)), ("flip", None)] {
        let verb = match on {
            Some(true) => "Turns on",
            Some(false) => "Turns off",
            None => "Toggles",
        };
        app.add_console_command(
            CommandSpec::new(name).help(&format!("{verb} a switch ({live})")).param(Param::new("switch", ArgKind::Str)),
            move |inv| {
                let part = inv.args.str("switch").unwrap_or_default().to_ascii_lowercase();
                let text = set_switch(inv.ctx, inv.vars, ssao, &part, on)?;
                inv.log.info(&text);
                Ok(())
            },
        );
    }
}

/// `render.ssao` and what the camera started with.
#[derive(Resource)]
struct Ssao {
    var: VarId,
    /// The camera's quality level at startup (`None`: it started without SSAO; turning it
    /// on then uses Bevy's default level).
    level: Option<ScreenSpaceAmbientOcclusionQualityLevel>,
    /// The variable's generation last applied (`None`: the camera hasn't been seen yet).
    applied: Option<u64>,
}

/// Seeds `render.ssao` from the camera once it exists, then inserts or removes Bevy's
/// SSAO component when the variable changes. On: the depth + motion-vector prepass and
/// MSAA off, as at startup (`exterior.rs`). Off: only the SSAO component goes; the
/// prepasses stay (the image is the A/B; for the cost, compare with a `--off ssao` run).
#[allow(clippy::needless_pass_by_value)]
fn apply_ssao(
    mut commands: Commands,
    mut console: ResMut<DevConsole>,
    mut state: ResMut<Ssao>,
    cameras: Query<(Entity, Option<&ScreenSpaceAmbientOcclusion>), With<ExteriorCamera>>,
) {
    let Some(applied) = state.applied else {
        let Some((_, ssao)) = cameras.iter().next() else { return };
        state.level = ssao.map(|s| s.quality_level);
        let var = state.var;
        // Command-line priority: a value typed before the camera existed wins.
        let _ = console.vars_mut().set_by(var, ssao.is_some().into(), SetBy::CommandLine);
        // A different value (typed before) applies on the next frame: `u64::MAX` is never a
        // real generation.
        let matches = console.vars().flag(var) == ssao.is_some();
        state.applied = Some(if matches { console.vars().var(var).generation() } else { u64::MAX });
        return;
    };
    let generation = console.vars().var(state.var).generation();
    if generation == applied {
        return;
    }
    state.applied = Some(generation);
    let on = console.vars().flag(state.var);
    for (camera, ssao) in &cameras {
        match (on, ssao.is_some()) {
            (true, false) => {
                let quality_level = state.level.unwrap_or_default();
                commands
                    .entity(camera)
                    .insert((ScreenSpaceAmbientOcclusion { quality_level, ..default() }, DepthPrepass, MotionVectorPrepass, Msaa::Off));
            }
            (false, true) => {
                commands.entity(camera).remove::<ScreenSpaceAmbientOcclusion>();
            }
            _ => {}
        }
    }
}

/// `on`/`off`/`flip <name>`: what happened, or why nothing did.
fn set_switch(world: &mut World, vars: &mut Vars, ssao: VarId, name: &str, on: Option<bool>) -> Result<String, String> {
    if name == "ssao" {
        if world.query_filtered::<(), With<ExteriorCamera>>().iter(world).next().is_none() {
            return Err("ssao: no streaming camera in this run".to_owned());
        }
        let v = on.unwrap_or(!vars.flag(ssao));
        vars.set_by(ssao, v.into(), SetBy::Console)?;
        return Ok(format!("ssao {}", if v { "on" } else { "off" }));
    }
    if let Some(switch) = Switch::from_name(name) {
        let mut state: SystemState<(ResMut<LiveAb>, Res<Time<Real>>, Switches)> = SystemState::new(world);
        let (mut ab, real, mut s) = state.get_mut(world).map_err(|e| format!("{name}: {e}"))?;
        let text = switch.set(&mut ab, &mut s, on);
        ab.note(text.clone(), real.elapsed_secs());
        state.apply(world);
        return Ok(text);
    }
    match crate::perf::Off::from_flags(&[name.to_owned()]) {
        Ok(_) => Err(format!("{name}: startup only, relaunch with --off {name}")),
        Err(unknown) => Err(format!("{unknown}; live: ssao, {}", Switch::names())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sf_console_bevy::ConsoleSubmit;

    fn app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(ConsolePlugin).init_resource::<LiveAb>();
        // No overlay: it needs a window.
        register(&mut app);
        app
    }

    fn run(app: &mut App, line: &str) {
        app.world_mut().write_message(ConsoleSubmit::exec(line));
        app.update();
    }

    #[test]
    fn ssao_flips_live_and_starts_as_the_camera_did() {
        let mut app = app();
        let camera = app
            .world_mut()
            .spawn((ExteriorCamera, ScreenSpaceAmbientOcclusion { quality_level: ScreenSpaceAmbientOcclusionQualityLevel::Low, ..default() }))
            .id();
        app.update();
        app.update();
        assert!(app.world().get::<ScreenSpaceAmbientOcclusion>(camera).is_some());
        run(&mut app, "off ssao");
        assert!(app.world().get::<ScreenSpaceAmbientOcclusion>(camera).is_none());
        run(&mut app, "toggle render.ssao");
        let back = app.world().get::<ScreenSpaceAmbientOcclusion>(camera).expect("ssao back on");
        assert_eq!(back.quality_level, ScreenSpaceAmbientOcclusionQualityLevel::Low);
        assert_eq!(app.world().get::<Msaa>(camera), Some(&Msaa::Off));
    }

    #[test]
    fn camera_without_ssao_seeds_off_and_startup_parts_say_so() {
        let mut app = app();
        let camera = app.world_mut().spawn(ExteriorCamera).id();
        app.update();
        app.update();
        assert!(app.world().get::<ScreenSpaceAmbientOcclusion>(camera).is_none());
        let var = app.world().resource::<Ssao>().var;
        assert!(!app.world().resource::<DevConsole>().vars().flag(var));
        run(&mut app, "on ssao");
        assert!(app.world().get::<ScreenSpaceAmbientOcclusion>(camera).is_some());
        run(&mut app, "off froxels");
        run(&mut app, "off wireframe");
        let log: Vec<String> = app.world().resource::<DevConsole>().log().lines().map(|l| l.text.clone()).collect();
        assert!(log.iter().any(|l| l.contains("froxels: startup only, relaunch with --off froxels")), "{log:?}");
        assert!(log.iter().any(|l| l == "wireframe off"), "{log:?}");
    }
}
