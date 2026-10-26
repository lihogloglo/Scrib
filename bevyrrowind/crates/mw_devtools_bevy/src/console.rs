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

use bevy_core_pipeline::prepass::{DepthPrepass, MotionVectorPrepass};
use bevy_ecs::system::SystemState;
use bevy_pbr::{ScreenSpaceAmbientOcclusion, ScreenSpaceAmbientOcclusionQualityLevel};
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_transform::prelude::*;
use bevy_time::prelude::*;
use bevy_input::prelude::*;
use bevy_render::prelude::*;
use bevy_enhanced_input::EnhancedInputSystems;
use sf_character_bevy::CharacterPlacement;
use sf_console_bevy::overlay::{ConsoleInputSystems, ConsoleKeys, ConsoleOverlayPlugin, ConsoleSelection};
use sf_console_bevy::sf_console::{ArgKind, CommandSpec, Param, VarId, VarSpec, Vars};
use sf_console_bevy::{ConsoleAppExt, ConsolePlugin, DevConsole, SetBy};

use mw_game_core::exterior::ExteriorCamera;
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
            .help(
                "Travels to a cell (CenterOnCell): an interior by name (its first door's arrival marker), \
                 an exterior by cell or region name, or grid `x y` (the square's centre, on the ground)",
            )
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
        .query_filtered::<Entity, With<mw_game_core::player::PlayerBody>>()
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
fn feed_no_clip_look(cams: Query<&GlobalTransform, With<mw_game_core::doors::DoorUser>>, mut bodies: Query<&mut sf_action_bevy::NoClip>) {
    let Some(cam) = cams.iter().next() else { return };
    for mut no_clip in &mut bodies {
        no_clip.look = cam.forward().as_vec3();
    }
}

/// The arrival of the first link end into `cell` (an interior's worldspace is its cell id).
fn cell_arrival<'a>(links: &'a [sf_world::DoorLink], cell: &str) -> Option<&'a sf_world::LinkEnd> {
    links.iter().flat_map(sf_world::DoorLink::exits).map(|(_, to)| to).find(|to| to.worldspace.as_str().eq_ignore_ascii_case(cell))
}

/// The vanilla `sDefaultCellname` (nameless exterior cells without a region), when the GMST is missing.
const DEFAULT_CELL_NAME: &str = "Wilderness";

/// The exterior grid square `coc <name>` goes to. Port of `OpenMW` `WorldModel::findCell(std::string_view)`
/// (`apps/openmw/mwworld/worldmodel.cpp:312`) and `World::findExteriorPosition`
/// (`apps/openmw/mwworld/worldimp.cpp:2683`), commit 4e718b50: the first exterior CELL in load order
/// whose name matches (case-insensitive); a nameless cell answers to its region's name (the region
/// id when that is empty), else to `sDefaultCellname`. Then grid coordinates: `x,y` as `OpenMW`,
/// also `x y` here. The caller has already tried interiors (`OpenMW` tries them first too).
fn exterior_grid(db: &sf_records::RecordDb, name: &str) -> Option<(i32, i32)> {
    use sf_records::RecordId;
    use tes3::esp::{Cell, GameSetting, GameSettingValue, Region};
    let default_name = match db.get::<GameSetting>(&RecordId::new("sDefaultCellname")).map(|g| &g.value) {
        Some(GameSettingValue::String(s)) => s.as_str(),
        _ => DEFAULT_CELL_NAME,
    };
    let resolved = |cell: &Cell| -> String {
        if !cell.name.is_empty() {
            return cell.name.clone();
        }
        match cell.region.as_deref().and_then(|id| db.get::<Region>(&RecordId::new(id)).map(|r| (id, r))) {
            Some((id, region)) if region.name.is_empty() => id.to_owned(),
            Some((_, region)) => region.name.clone(),
            None => default_name.to_owned(),
        }
    };
    let named = db.iter::<Cell>().find(|(_, c)| !c.is_interior() && resolved(c).eq_ignore_ascii_case(name)).map(|(_, c)| c.data.grid);
    named.or_else(|| {
        let mut parts = name.split(|c: char| c == ',' || c.is_whitespace()).filter(|p| !p.is_empty());
        match (parts.next()?.parse().ok()?, parts.next()?.parse().ok()?, parts.next()) {
            (x, y, None) => Some((x, y)),
            _ => None,
        }
    })
}

/// Where the player arrives in exterior square `grid`, Morrowind units: the centre
/// (`ESM::indexToPosition(.., centre = true)`), at the land's height or the water level (0),
/// whichever is higher (`World::adjustPosition`, `worldimp.cpp:1237`, commit 4e718b50). The arrival
/// gate (`mw_player_bevy::await_ground`) then puts the feet on what's there once collision is in.
#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)] // half a cell; small grid indices
fn exterior_arrival(db: &sf_records::RecordDb, grid: (i32, i32)) -> [f32; 3] {
    let half = (mw_world::convert::CELL_UNITS / 2.0) as f32;
    let land = db.get::<tes3::esp::Landscape>(&mw_records::keys::exterior(grid)).map(mw_world::terrain::LandData::decode);
    let ground = land.map_or(0.0, |l| l.height_at(half, half)).max(0.0);
    let corner = |i: i32| i as f32 * half * 2.0;
    [corner(grid.0) + half, corner(grid.1) + half, ground]
}

/// `CenterOnCell`: the door travel (`StartTravel`, fade) to `cell`'s first door arrival, or to an
/// exterior square ([`exterior_grid`], [`exterior_arrival`]).
fn center_on_cell(world: &mut World, cell: &str) -> Result<String, String> {
    if cell.is_empty() {
        return Err("usage: coc \"<cell name>\" or coc <x> <y>".to_owned());
    }
    if world.get_resource::<sf_world_bevy::FadeTransition>().is_some_and(sf_world_bevy::FadeTransition::is_active) {
        return Err("a travel is already under way".to_owned());
    }
    let eye = Vec3::Y * mw_game_core::doors::ARRIVAL_EYE_M;
    let interior = world.get_resource::<sf_world_bevy::DoorLinks>().and_then(|links| {
        let to = cell_arrival(&links.0, cell)?;
        let text = format!("travel to {} at {:.1?}", to.worldspace.as_str(), to.arrival.pos.to_f64());
        let at = sf_world::Location::new(to.worldspace.clone(), to.arrival.pos.offset(eye));
        Some((at, mw_doors_bevy::arrival_rotation(to.arrival.rotation), text))
    });
    let exterior = || {
        let db = world.get_resource::<mw_rpg_bevy::dialogue::Rpg>()?.records();
        let grid = exterior_grid(db, cell)?;
        let feet = exterior_arrival(db, grid);
        let text = format!("travel to exterior {},{} at {:.0?} (units)", grid.0, grid.1, feet);
        // Rotation zero faces north, as `OpenMW`'s.
        Some((sf_world::Location::new(mw_world::ids::exterior(), mw_world::convert::world_pos(feet).offset(eye)), Quat::IDENTITY, text))
    };
    let (to, rotation, text) = interior.or_else(exterior).ok_or_else(|| {
        format!("{cell}: no cell by that name (an interior with a door into it, an exterior cell or region, or grid x y)")
    })?;
    let travel = sf_world_bevy::StartTravel { traveler: Entity::PLACEHOLDER, to, rotation, reload: false };
    let cam = world
        .query_filtered::<Entity, With<mw_game_core::doors::DoorUser>>()
        .iter(world)
        .next()
        .ok_or("no traveler camera in this run")?;
    world.write_message(sf_world_bevy::StartTravel { traveler: cam, ..travel }).ok_or("door travel isn't running")?;
    Ok(text)
}

/// Lines no command or variable claims run as `MWScript` through the dialogue's result runner
/// ([`mw_rpg_bevy::dialogue::Rpg::run_console`]); console commands always win.
fn register_mwscript(app: &mut App) {
    let names = mwscript_names();
    let mut console = app.world_mut().resource_mut::<DevConsole>();
    console.set_fallback("mwscript", |u| {
        let selected = u.ctx.get_resource::<ConsoleSelection>().and_then(|s| s.0);
        let key = selected.map(|e| mw_rpg_bevy::dialogue::key(e, u.ctx.get::<CharacterPlacement>(e)));
        let mut rpg = u.ctx.get_resource_mut::<mw_rpg_bevy::dialogue::Rpg>().ok_or("no game records in this run")?;
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
                    .insert((ScreenSpaceAmbientOcclusion { quality_level, ..Default::default() }, DepthPrepass, MotionVectorPrepass, Msaa::Off));
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
    match mw_game_core::perf::Off::from_flags(&[name.to_owned()]) {
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
        app.add_plugins((TaskPoolPlugin::default(), bevy_time::TimePlugin)).add_plugins(ConsolePlugin).init_resource::<LiveAb>();
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
        let body = ActionBody::new(sf_controller::Locomotion::new(sf_coords::WorldPos::ORIGIN, 0.0), sf_controller::MoverParams::default(), sf_coords::WorldPos::ORIGIN);
        let e = app.world_mut().spawn((mw_game_core::player::PlayerBody, body)).id();
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
        let cam = app.world_mut().spawn(mw_game_core::doors::DoorUser).id();
        run(&mut app, "coc \"No Such Place\"");
        assert!(log(&app).iter().any(|l| l.contains("No Such Place: no cell by that name")), "{:?}", log(&app));
        run(&mut app, "COC \"seyda neen, census and excise office\"");
        let travels = app.world().resource::<Messages<sf_world_bevy::StartTravel>>();
        let mut cursor = travels.get_cursor();
        let sent: Vec<_> = cursor.read(travels).collect();
        assert_eq!(sent.len(), 1, "{:?}", log(&app));
        assert_eq!(sent[0].traveler, cam);
        assert_eq!(sent[0].to.worldspace.as_str(), WorldspaceId::new("Seyda Neen, Census and Excise Office").as_str());
        assert!(sent[0].to.pos.delta_f32(WorldPos::ORIGIN).y < -3.0, "eye height above the arrival");
    }

    #[test]
    fn coc_finds_exterior_cells_by_name_region_and_grid() {
        use sf_records::{RecordDb, RecordId};
        use tes3::esp::{Cell, Region};
        let mut db = RecordDb::new();
        db.register::<Cell>("CELL").unwrap();
        db.register::<Region>("REGN").unwrap();
        let esm = db.add_source("Test.esm", &[]);
        let region = Region { id: "Bitter Coast Region".into(), name: "Bitter Coast Region".into(), ..Region::default() };
        db.define(esm, RecordId::new("Bitter Coast Region"), region).unwrap();
        let ext = |name: &str, region: Option<&str>, grid: (i32, i32)| Cell {
            name: name.into(),
            region: region.map(Into::into),
            data: tes3::esp::CellData { grid, ..Default::default() },
            ..Cell::default()
        };
        for c in [ext("Balmora", None, (-3, -2)), ext("Balmora", None, (-4, -2)), ext("", Some("Bitter Coast Region"), (-5, -6)), ext("", None, (7, 7))] {
            db.define(esm, mw_records::keys::cell(&c), c).unwrap();
        }
        assert_eq!(exterior_grid(&db, "balmora"), Some((-3, -2)), "the first in load order");
        assert_eq!(exterior_grid(&db, "bitter coast region"), Some((-5, -6)), "a nameless cell answers to its region");
        assert_eq!(exterior_grid(&db, "Wilderness"), Some((7, 7)), "no region: sDefaultCellname");
        assert_eq!(exterior_grid(&db, "-2 -9"), Some((-2, -9)));
        assert_eq!(exterior_grid(&db, "-2,-9"), Some((-2, -9)), "OpenMW's form");
        assert_eq!(exterior_grid(&db, "nowhere"), None);
        assert_eq!(exterior_grid(&db, "1 2 3"), None);

        // Through the console: an exterior travel to the square's centre, eye above the ground.
        let mut app = app();
        app.add_message::<sf_world_bevy::StartTravel>();
        app.insert_resource(mw_rpg_bevy::dialogue::Rpg::new(std::sync::Arc::new(db)).unwrap());
        let cam = app.world_mut().spawn(mw_game_core::doors::DoorUser).id();
        let travel = |app: &mut App, line: &str| {
            run(app, line);
            let travels = app.world().resource::<Messages<sf_world_bevy::StartTravel>>();
            travels.get_cursor().read(travels).last().cloned().unwrap_or_else(|| panic!("{line}: {:?}", log(app)))
        };
        for (line, (x, y)) in [("coc balmora", (-3.0, -2.0)), ("COC -2 -9", (-2.0, -9.0))] {
            let sent = travel(&mut app, line);
            assert_eq!((sent.traveler, sent.to.worldspace.as_str()), (cam, mw_world::ids::EXTERIOR), "{line}");
            let centre = mw_world::convert::world_pos([x * 8192.0 + 4096.0, y * 8192.0 + 4096.0, 0.0]);
            let eye = centre.delta_f32(sent.to.pos);
            assert!(eye.xz().length() < 1e-2 && (eye.y - mw_game_core::doors::ARRIVAL_EYE_M).abs() < 1e-2, "{line}: {eye:?}");
        }
    }

    fn log(app: &App) -> Vec<String> {
        app.world().resource::<DevConsole>().log().lines().map(|l| l.text.clone()).collect()
    }

    #[test]
    fn mwscript_lines_run_on_the_player_and_commands_still_win() {
        use sf_records::{RecordDb, RecordId};
        let mut app = app();
        app.insert_resource(mw_rpg_bevy::dialogue::Rpg::new(std::sync::Arc::new(RecordDb::new())).unwrap());
        run(&mut app, "player->AddItem gold_001 5");
        run(&mut app, "AddItem \"gold_001\" 2");
        let gold = RecordId::new("gold_001");
        assert_eq!(app.world().resource::<mw_rpg_bevy::dialogue::Rpg>().state.pc.loadout.count(&gold), 7, "{:?}", log(&app));
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
            .spawn((ExteriorCamera, ScreenSpaceAmbientOcclusion { quality_level: ScreenSpaceAmbientOcclusionQualityLevel::Low, ..Default::default() }))
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
