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
//! - Any other line runs as `MWScript` (vanilla's console: `player->AddItem gold_001 100`),
//!   on the selected actor (click-select) or else the player; completion offers the
//!   functions the interpreter runs, with their short forms.

use bevy::core_pipeline::prepass::{DepthPrepass, MotionVectorPrepass};
use bevy::ecs::system::SystemState;
use bevy::pbr::{ScreenSpaceAmbientOcclusion, ScreenSpaceAmbientOcclusionQualityLevel};
use bevy::prelude::*;
use bevy_enhanced_input::EnhancedInputSystems;
use sf_character_bevy::CharacterPlacement;
use sf_console_bevy::overlay::{ConsoleInputSystems, ConsoleKeys, ConsoleOverlayPlugin, ConsoleSelection};
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
        register_mwscript(app);
        register_cheats(app);
    }
}

/// Vanilla console functions the result runner can't run (they need the world, not the RPG
/// state): `ToggleCollision` / `TCL` and `CenterOnCell` / `COC`. Console commands, so they win
/// over the `MWScript` fallback and complete like the other commands.
fn register_cheats(app: &mut App) {
    app.add_systems(Update, feed_no_clip_look);
    app.add_console_command(
        CommandSpec::new("togglecollision")
            .alias("tcl")
            .help("No-clip for the player (ToggleCollision): fly where the camera looks, through geometry, no gravity"),
        |inv| {
            let text = toggle_collision(inv.ctx)?;
            inv.log.info(text);
            Ok(())
        },
    );
    app.add_console_command(
        CommandSpec::new("centeroncell")
            .alias("coc")
            .help("Travels to an interior cell by name (CenterOnCell), at its first door's arrival marker")
            .param(Param::rest("cell")),
        |inv| {
            let cell = inv.args.str("cell").unwrap_or_default().trim().trim_matches('"').trim().to_owned();
            let text = center_on_cell(inv.ctx, &cell)?;
            inv.log.info(&text);
            Ok(())
        },
    );
}

/// Flips [`sf_action_bevy::NoClip`] on the player body; off puts the body back on its feet.
fn toggle_collision(world: &mut World) -> Result<&'static str, String> {
    let body = world
        .query_filtered::<Entity, With<crate::player::PlayerBody>>()
        .iter(world)
        .next()
        .ok_or("no player body (free camera: F11 for the player)")?;
    if world.get::<sf_action_bevy::NoClip>(body).is_some() {
        world.entity_mut(body).remove::<sf_action_bevy::NoClip>();
        if let Some(mut b) = world.get_mut::<sf_action_bevy::ActionBody>(body) {
            b.loco.mode = sf_controller::MoverMode::Ground;
        }
        Ok("collision on")
    } else {
        world.entity_mut(body).insert(sf_action_bevy::NoClip { look: Vec3::NEG_Z });
        Ok("collision off (no-clip)")
    }
}

/// No-clip climbs along the view camera's forward.
fn feed_no_clip_look(cams: Query<&GlobalTransform, With<crate::doors::DoorUser>>, mut bodies: Query<&mut sf_action_bevy::NoClip>) {
    let Some(cam) = cams.iter().next() else { return };
    for mut no_clip in &mut bodies {
        no_clip.look = cam.forward().as_vec3();
    }
}

/// The arrival of the first link end into `cell` (an interior's worldspace is its cell id).
fn cell_arrival<'a>(links: &'a [sf_world::DoorLink], cell: &str) -> Option<&'a sf_world::LinkEnd> {
    links.iter().flat_map(sf_world::DoorLink::exits).map(|(_, to)| to).find(|to| to.worldspace.as_str().eq_ignore_ascii_case(cell))
}

/// `CenterOnCell`: the door travel (`StartTravel`, fade) to `cell`'s first door arrival.
fn center_on_cell(world: &mut World, cell: &str) -> Result<String, String> {
    if cell.is_empty() {
        return Err("usage: coc \"<cell id>\"".to_owned());
    }
    if world.get_resource::<sf_world_bevy::FadeTransition>().is_some_and(sf_world_bevy::FadeTransition::is_active) {
        return Err("a travel is already under way".to_owned());
    }
    let links = world.get_resource::<sf_world_bevy::DoorLinks>().ok_or("no door links in this run")?;
    let to = cell_arrival(&links.0, cell).ok_or_else(|| {
        format!("{cell}: no interior cell by that name with a door into it (exterior names: not yet)")
    })?;
    let travel = sf_world_bevy::StartTravel {
        traveler: Entity::PLACEHOLDER,
        to: sf_world::Location::new(to.worldspace.clone(), to.arrival.pos.offset(Vec3::Y * crate::doors::ARRIVAL_EYE_M)),
        rotation: crate::doors::arrival_rotation(to.arrival.rotation),
        reload: false,
    };
    let text = format!("travel to {} at {:.1?}", to.worldspace.as_str(), to.arrival.pos.to_f64());
    let cam = world
        .query_filtered::<Entity, With<crate::doors::DoorUser>>()
        .iter(world)
        .next()
        .ok_or("no traveler camera in this run")?;
    world.write_message(sf_world_bevy::StartTravel { traveler: cam, ..travel }).ok_or("door travel isn't running")?;
    Ok(text)
}

/// Lines no command or variable claims run as `MWScript` through the dialogue's result runner
/// ([`crate::dialogue::Rpg::run_console`]); console commands always win.
fn register_mwscript(app: &mut App) {
    let names = mwscript_names();
    let mut console = app.world_mut().resource_mut::<DevConsole>();
    console.set_fallback("mwscript", |u| {
        let selected = u.ctx.get_resource::<ConsoleSelection>().and_then(|s| s.0);
        let key = selected.map(|e| crate::dialogue::key(e, u.ctx.get::<CharacterPlacement>(e)));
        let mut rpg = u.ctx.get_resource_mut::<crate::dialogue::Rpg>().ok_or("no game records in this run")?;
        for line in rpg.run_console(u.text, key)? {
            u.log.info(&line);
        }
        Ok(())
    });
    console.set_fallback_completion(move |prefix| {
        let (head, word) = prefix.rfind("->").map_or(("", prefix), |i| prefix.split_at(i + 2));
        names.iter().filter(|n| n.starts_with(word)).map(|n| format!("{head}{n}")).collect()
    });
}

/// Lowercase names and short forms of the functions the interpreter runs ([`mw_script::Builtin`]);
/// the rest of `functions.txt` parses but fails as "not runnable from the console yet".
fn mwscript_names() -> Vec<String> {
    let table = mw_script::FunctionTable::vanilla();
    let runnable: Vec<&str> = mw_script::Builtin::ALL.iter().map(|b| b.name()).collect();
    let mut names: Vec<String> = table
        .names()
        .filter(|&(_, f)| runnable.contains(&table.function(f).name.as_str()))
        .map(|(n, _)| n.to_owned())
        .collect();
    names.sort();
    names
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
        register_mwscript(&mut app);
        register_cheats(&mut app);
        app
    }

    #[test]
    fn tcl_flips_no_clip_on_the_player_body() {
        use sf_action_bevy::{ActionBody, NoClip};
        let mut app = app();
        run(&mut app, "tcl");
        assert!(log(&app).iter().any(|l| l.contains("no player body")), "{:?}", log(&app));
        let body = ActionBody::new(sf_controller::Locomotion::new(sf_coords::WorldPos::ORIGIN, 0.0), default(), sf_coords::WorldPos::ORIGIN);
        let e = app.world_mut().spawn((crate::player::PlayerBody, body)).id();
        run(&mut app, "TCL");
        assert!(app.world().get::<NoClip>(e).is_some(), "{:?}", log(&app));
        run(&mut app, "ToggleCollision");
        assert!(app.world().get::<NoClip>(e).is_none());
        assert_eq!(app.world().get::<ActionBody>(e).unwrap().loco.mode, sf_controller::MoverMode::Ground);
        assert!(app.world().resource::<DevConsole>().complete("toggleco").candidates.contains(&"togglecollision".to_owned()));
    }

    #[test]
    fn coc_requests_travel_to_the_named_cell_and_unknown_cells_say_so() {
        use sf_coords::{WorldPos, WorldTransform};
        use sf_world::{DoorLink, LinkEnd, WorldspaceId};
        let mut app = app();
        app.add_message::<sf_world_bevy::StartTravel>();
        let end = |ws: &str, y: f32| LinkEnd {
            worldspace: WorldspaceId::new(ws),
            frame: WorldTransform::IDENTITY,
            arrival: WorldTransform { pos: WorldPos::ORIGIN.offset(Vec3::Y * y), ..WorldTransform::IDENTITY },
            door: None,
            extent: Vec2::ZERO,
        };
        let link = DoorLink { a: end("", 0.0), b: end("Seyda Neen, Census and Excise Office", 3.0), two_way: true };
        app.insert_resource(sf_world_bevy::DoorLinks(vec![link]));
        let cam = app.world_mut().spawn(crate::doors::DoorUser).id();
        run(&mut app, "coc \"No Such Place\"");
        assert!(log(&app).iter().any(|l| l.contains("No Such Place: no interior cell")), "{:?}", log(&app));
        run(&mut app, "COC \"seyda neen, census and excise office\"");
        let travels = app.world().resource::<Messages<sf_world_bevy::StartTravel>>();
        let mut cursor = travels.get_cursor();
        let sent: Vec<_> = cursor.read(travels).collect();
        assert_eq!(sent.len(), 1, "{:?}", log(&app));
        assert_eq!(sent[0].traveler, cam);
        assert_eq!(sent[0].to.worldspace.as_str(), WorldspaceId::new("Seyda Neen, Census and Excise Office").as_str());
        assert!(sent[0].to.pos.delta_f32(WorldPos::ORIGIN).y < -3.0, "eye height above the arrival");
    }

    fn log(app: &App) -> Vec<String> {
        app.world().resource::<DevConsole>().log().lines().map(|l| l.text.clone()).collect()
    }

    #[test]
    fn mwscript_lines_run_on_the_player_and_commands_still_win() {
        use sf_records::{RecordDb, RecordId};
        let mut app = app();
        app.insert_resource(crate::dialogue::Rpg::new(std::sync::Arc::new(RecordDb::new())).unwrap());
        run(&mut app, "player->AddItem gold_001 5");
        run(&mut app, "AddItem \"gold_001\" 2");
        let gold = RecordId::new("gold_001");
        assert_eq!(app.world().resource::<crate::dialogue::Rpg>().state.pc.loadout.count(&gold), 7, "{:?}", log(&app));
        run(&mut app, "FooBar 1");
        // `off` is a console command: it wins over MWScript.
        run(&mut app, "off froxels");
        let log = log(&app);
        assert!(log.iter().any(|l| l.starts_with("mwscript:") && l.to_ascii_lowercase().contains("foobar")), "{log:?}");
        assert!(log.iter().any(|l| l.contains("froxels: startup only")), "{log:?}");
        let console = app.world().resource::<DevConsole>();
        assert_eq!(console.complete("player->addi").candidates, ["player->additem"]);
        assert!(console.complete("modd").candidates.contains(&"moddisposition".to_owned()));
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
