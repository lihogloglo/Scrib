//! The dev console in the interactive viewer (`view` and `scene`, never in screenshot or
//! benchmark runs; ROADMAP 1.13, superfargoth `docs/specs/console.md`). The backquote key
//! (the one left of 1, physical: `²` on AZERTY) opens it; F1 stays the A/B help.
//!
//! - `render.ssao` (on/off variable): Bevy GTAO on the streaming camera, live. Starts as
//!   the run started (`--off ssao` or the Low preset: off). `toggle render.ssao`,
//!   `render.ssao off`, or `off ssao`.
//! - `render.contactshadows` (on/off variable): Bevy screen-space contact shadows
//!   (`ContactShadows` on the camera), live, same grammar (`off contactshadows`). Starts as
//!   the run started (`--off contactshadows` or the Low preset: off).
//! - `render.contactlength` / `render.contactthickness` (meters): the contact-shadow ray
//!   length and surface thickness, live (SH-16 diagnosis: `render.contactlength 3` makes any
//!   working march obvious). Start as the camera's preset values.
//! - `render.bloom` (float, 0 = off): Bevy `Bloom` intensity on the camera, live. Starts as
//!   the camera's preset value (`--off bloom` or the Low preset: 0).
//! - `render.gi` (on/off variable): the dynamic probe GI (`sf_sky_bevy::GiEnabled`), live:
//!   off stops tracing and showing the volume (today's ambient draws), on re-traces it. Starts
//!   on where the GI runs; a run without it (`--off gi`, Low/Medium, the ray-traced path, an
//!   exterior start, no ray-tracing GPU) can't turn it on.
//! - `render.gi_intensity` / `render.gi_ambient` (floats): the GI's artistic gain, live on every
//!   `GiVolume` (`intensity`: multiplier on the probe irradiance, as UE's "Indirect Lighting
//!   Intensity"; `ambient_scale`: the flat ambient kept inside the volume). They show the live
//!   volume's values; a typed value also applies to the volumes later re-fits spawn.
//! - `terrain.reflectance` (float, 0..1, Bevy scale: F0 = 0.16 x r²): the ground's reflectance
//!   (`sf_terrain_bevy::TerrainSettings::reflectance`), live (RO-16 veil fix 1). Starts as the
//!   terrain's value; wet ground still mixes toward water's from it.
//! - `on <name>` / `off <name>` / `flip <name>`: `ssao`, `contactshadows`, `gi` and the switches the A/B keys flip
//!   ([`Switch`]: `sun_shadows`, `clouds`, ...). Any other `--off` part is decided at
//!   startup: the console says so instead of doing nothing.
//! - Any other line runs as `MWScript` (vanilla's console: `player->AddItem gold_001 100`),
//!   on the selected actor (click-select) or else the player; completion offers the
//!   functions the interpreter runs, with their short forms.

use bevy_core_pipeline::prepass::{DepthPrepass, MotionVectorPrepass};
use bevy_ecs::system::SystemState;
use bevy_light::DirectionalLight;
use bevy_pbr::{ContactShadows, ScreenSpaceAmbientOcclusion, ScreenSpaceAmbientOcclusionQualityLevel};
use bevy_post_process::bloom::Bloom;
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_transform::prelude::*;
use bevy_time::prelude::*;
use bevy_input::prelude::*;
use bevy_render::prelude::*;
use bevy_enhanced_input::EnhancedInputSystems;
use sf_character_bevy::CharacterPlacement;
use sf_sky_bevy::SunLight;
use sf_console_bevy::overlay::{ConsoleInputSystems, ConsoleKeys, ConsoleOverlayPlugin, ConsoleSelection};
use sf_console_bevy::sf_console::{ArgKind, CommandSpec, Param, VarId, VarSpec, Vars};
use sf_console_bevy::{ConsoleAppExt, ConsolePlugin, DevConsole, SetBy};

use mw_game_core::exterior::ExteriorCamera;
use crate::live_ab::{LiveAb, Switch, Switches};

/// Name of the SSAO variable.
const SSAO_VAR: &str = "render.ssao";
/// Name of the contact-shadows variable.
const CONTACT_VAR: &str = "render.contactshadows";
/// Name of the contact-shadow ray length variable (meters).
const CONTACT_LENGTH_VAR: &str = "render.contactlength";
/// Name of the contact-shadow thickness variable (meters).
const CONTACT_THICKNESS_VAR: &str = "render.contactthickness";
/// Name of the bloom intensity variable (0 = off).
const BLOOM_VAR: &str = "render.bloom";
/// Name of the dynamic GI variable.
const GI_VAR: &str = "render.gi";
/// Name of the GI irradiance gain variable (`GiVolume::intensity`).
const GI_INTENSITY_VAR: &str = "render.gi_intensity";
/// Name of the GI inside-volume ambient scale variable (`GiVolume::ambient_scale`).
const GI_AMBIENT_VAR: &str = "render.gi_ambient";
/// Name of the terrain reflectance variable (`TerrainSettings::reflectance`).
const TERRAIN_REFLECTANCE_VAR: &str = "terrain.reflectance";
/// Why a run has no GI (`sf_sky_bevy::GiEnabled` absent).
const GI_NOT_IN_RUN: &str = "not in this run (--off gi, Low/Medium preset, the ray-traced path, an exterior start or no ray-tracing GPU)";

/// The console, its overlay on backquote, `render.ssao`, `render.contactshadows` and the
/// `on`/`off`/`flip` commands.
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
        if !u.ctx.contains_resource::<mw_rpg_bevy::dialogue::Rpg>() {
            return Err("no game records in this run".into());
        }
        // The runner reads the live sheets (PC filter functions in macros) beside the `Rpg`.
        let lines = u.ctx.resource_scope(|world, mut rpg: Mut<mw_rpg_bevy::dialogue::Rpg>| {
            rpg.run_console(u.text, key, &mw_rpg_bevy::dialogue::WorldSheets(world))
        })?;
        for line in lines {
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

/// `render.ssao`, `render.contactshadows`, their systems and the `on`/`off`/`flip` commands
/// (needs [`ConsolePlugin`] and [`LiveAb`]).
fn register(app: &mut App) {
    let ssao = app.add_console_var(
        VarSpec::bool(SSAO_VAR, true).help("Screen-space ambient occlusion (Bevy GTAO) on the camera; starts as the run did (--off ssao, Low preset: off)"),
    );
    let contact = app.add_console_var(VarSpec::bool(CONTACT_VAR, true).help(
        "Screen-space contact shadows (Bevy ContactShadows) on the camera; starts as the run did (--off contactshadows, Low preset: off)",
    ));
    let length = app.add_console_var(
        VarSpec::float(CONTACT_LENGTH_VAR, f64::from(ContactShadows::default().length))
            .float_range(0.01, 20.0)
            .help("Contact-shadow ray length in meters (starts as the camera's preset value; 3 makes any working march obvious)"),
    );
    let thickness = app.add_console_var(
        VarSpec::float(CONTACT_THICKNESS_VAR, f64::from(ContactShadows::default().thickness))
            .float_range(0.001, 10.0)
            .help("Contact-shadow surface thickness in meters (starts as the camera's preset value)"),
    );
    let bloom = app.add_console_var(
        VarSpec::float(BLOOM_VAR, f64::from(Bloom::NATURAL.intensity))
            .float_range(0.0, 1.0)
            .help("Bloom intensity on the camera, 0 = off (Bevy Bloom, energy-conserving; starts as the camera's preset value, --off bloom and Low: 0)"),
    );
    let gi = app.add_console_var(VarSpec::bool(GI_VAR, false).help(
        "Dynamic probe GI (sf_gi_bevy), live; starts on where the GI runs (High/Ultra raster, interior start), off and fixed without it (--off gi)",
    ));
    let gain = sf_gi_bevy::GiVolume::default();
    let gi_intensity = app.add_console_var(
        VarSpec::float(GI_INTENSITY_VAR, f64::from(gain.intensity))
            .float_range(0.0, 16.0)
            .help("GI gain: multiplier on the probe irradiance (1 = the traced bounce as is; try 1-4), live on every GI volume; starts as the live volume's"),
    );
    let gi_ambient = app.add_console_var(
        VarSpec::float(GI_AMBIENT_VAR, f64::from(gain.ambient_scale))
            .float_range(0.0, 4.0)
            .help("Flat ambient kept inside the GI volume (0 = the probes replace it, 1 = full ambient on top), live on every GI volume; starts as the live volume's"),
    );
    let reflectance = app.add_console_var(
        VarSpec::float(TERRAIN_REFLECTANCE_VAR, f64::from(sf_terrain_bevy::DEFAULT_REFLECTANCE))
            .float_range(0.0, 1.0)
            .help("Terrain reflectance, Bevy scale (F0 = 0.16 x r^2; 0.5 = F0 0.04, 0 = no specular), live; wet ground still mixes toward water; starts as the terrain's (RO-16: try 0.2-0.4)"),
    );
    app.insert_resource(Ssao { var: ssao, level: None, applied: None })
        .insert_resource(TerrainReflectance { var: reflectance, seeded: false, reported: None })
        .insert_resource(Contact { var: contact, length, thickness, settings: None, applied: None, tuned: None })
        .insert_resource(BloomVar { var: bloom, settings: None, applied: None })
        .insert_resource(GiVar { var: gi, applied: None })
        .insert_resource(GiGain { intensity: gi_intensity, ambient: gi_ambient, reported: None })
        .add_systems(Update, (apply_ssao, apply_contact_shadows, apply_bloom, apply_gi, apply_gi_gain, apply_terrain_reflectance));
    let vars = LiveVars { ssao, contact, gi };
    let live = format!("live: ssao, contactshadows, gi, ambient, viewmodel, {}; other --off parts are startup only", Switch::names());
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
                let text = set_switch(inv.ctx, inv.vars, vars, &part, on)?;
                inv.log.info(&text);
                Ok(())
            },
        );
    }
}

/// The console variables `on`/`off`/`flip` set directly (camera components, not A/B switches).
#[derive(Clone, Copy)]
struct LiveVars {
    ssao: VarId,
    contact: VarId,
    gi: VarId,
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

/// `render.contactshadows` and what the camera started with.
#[derive(Resource)]
struct Contact {
    var: VarId,
    /// `render.contactlength` (meters).
    length: VarId,
    /// `render.contactthickness` (meters).
    thickness: VarId,
    /// The camera's settings at startup, the preset's (`None`: it started without contact
    /// shadows; turning them on then uses Bevy's default, the High preset's values), then
    /// with the tuning variables applied.
    settings: Option<ContactShadows>,
    /// The variable's generation last applied (`None`: the camera hasn't been seen yet).
    applied: Option<u64>,
    /// The tuning variables' generations last applied.
    tuned: Option<(u64, u64)>,
}

impl Contact {
    /// The settings to put on the camera: the preset's (or Bevy's default) with the tuning
    /// variables' current values.
    #[expect(clippy::cast_possible_truncation, reason = "console floats are f64; the component is f32")]
    fn settings(&self, console: &DevConsole) -> ContactShadows {
        let vars = console.vars();
        ContactShadows {
            length: vars.float(self.length) as f32,
            thickness: vars.float(self.thickness) as f32,
            ..self.settings.unwrap_or_default()
        }
    }
}

/// Seeds a camera-component variable from the camera the first time it is seen (`present`:
/// the camera started with the component) and returns the generation to record as applied.
/// Command-line priority: a value typed before the camera existed wins, and then applies on
/// the next frame (`u64::MAX` is never a real generation).
fn seed_from_camera(console: &mut DevConsole, var: VarId, present: bool) -> u64 {
    let _ = console.vars_mut().set_by(var, present.into(), SetBy::CommandLine);
    if console.vars().flag(var) == present { console.vars().var(var).generation() } else { u64::MAX }
}

/// The variable's value if it changed since `applied`, which then records the new generation.
fn changed_since(console: &DevConsole, var: VarId, applied: &mut Option<u64>) -> Option<bool> {
    let generation = console.vars().var(var).generation();
    if *applied == Some(generation) {
        return None;
    }
    *applied = Some(generation);
    Some(console.vars().flag(var))
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
    if state.applied.is_none() {
        let Some((_, ssao)) = cameras.iter().next() else { return };
        state.level = ssao.map(|s| s.quality_level);
        state.applied = Some(seed_from_camera(&mut console, state.var, ssao.is_some()));
        return;
    }
    let state = &mut *state;
    let Some(on) = changed_since(&console, state.var, &mut state.applied) else { return };
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

/// Seeds `render.contactshadows` from the camera once it exists, then inserts or removes
/// Bevy's `ContactShadows` when the variable changes. The camera component gates the effect
/// per view (Bevy's `MeshPipelineKey::CONTACT_SHADOWS`); a light casts them only with its own
/// `contact_shadows_enabled`, which the sun gets only when the run started with them
/// (`exterior.rs` `sun_shadow`), so on also sets it on the sun. Off removes only the camera
/// component (the depth prepass it requires stays). The moon never casts them; lamps follow
/// `PointShadowBudget::contact_shadows`, untouched here.
#[allow(clippy::needless_pass_by_value)]
fn apply_contact_shadows(
    mut commands: Commands,
    mut console: ResMut<DevConsole>,
    mut state: ResMut<Contact>,
    cameras: Query<(Entity, Option<&ContactShadows>), With<ExteriorCamera>>,
    mut suns: Query<&mut DirectionalLight, With<SunLight>>,
) {
    if state.applied.is_none() {
        let Some((_, contact)) = cameras.iter().next() else { return };
        state.settings = contact.copied();
        state.applied = Some(seed_from_camera(&mut console, state.var, contact.is_some()));
        // The tuning variables start as the camera's values (a value typed earlier wins).
        let seed = state.settings.unwrap_or_default();
        let vars = console.vars_mut();
        let _ = vars.set_by(state.length, f64::from(seed.length).into(), SetBy::CommandLine);
        let _ = vars.set_by(state.thickness, f64::from(seed.thickness).into(), SetBy::CommandLine);
        state.tuned = Some((vars.var(state.length).generation(), vars.var(state.thickness).generation()));
        return;
    }
    let state = &mut *state;
    let tuned = (console.vars().var(state.length).generation(), console.vars().var(state.thickness).generation());
    let retune = state.tuned != Some(tuned);
    state.tuned = Some(tuned);
    let on = match changed_since(&console, state.var, &mut state.applied) {
        Some(on) => on,
        None if retune => console.vars().flag(state.var),
        None => return,
    };
    for (camera, contact) in &cameras {
        match (on, contact.is_some()) {
            (true, false) => {
                commands.entity(camera).insert(state.settings(&console));
            }
            (true, true) if retune => {
                commands.entity(camera).insert(state.settings(&console));
            }
            (false, true) => {
                commands.entity(camera).remove::<ContactShadows>();
            }
            _ => {}
        }
    }
    if on {
        for mut sun in &mut suns {
            if !sun.contact_shadows_enabled {
                sun.contact_shadows_enabled = true;
            }
        }
    }
}

/// `render.bloom` and what the camera started with.
#[derive(Resource)]
struct BloomVar {
    var: VarId,
    /// The camera's bloom at startup, the preset's (`None`: it started without bloom; turning
    /// it on then uses `Bloom::NATURAL` with the variable's intensity).
    settings: Option<Bloom>,
    /// The variable's generation last applied (`None`: the camera hasn't been seen yet).
    applied: Option<u64>,
}

/// Seeds `render.bloom` from the camera once it exists (its intensity, 0 without bloom; a
/// value typed earlier wins), then puts the preset's bloom with the variable's intensity on
/// the camera when it changes, or removes the component at 0.
#[allow(clippy::needless_pass_by_value)]
#[expect(clippy::cast_possible_truncation, reason = "console floats are f64; the component is f32")]
fn apply_bloom(mut commands: Commands, mut console: ResMut<DevConsole>, mut state: ResMut<BloomVar>, cameras: Query<(Entity, Option<&Bloom>), With<ExteriorCamera>>) {
    if state.applied.is_none() {
        let Some((_, bloom)) = cameras.iter().next() else { return };
        state.settings = bloom.cloned();
        let seed = bloom.map_or(0.0, |b| f64::from(b.intensity));
        let vars = console.vars_mut();
        let _ = vars.set_by(state.var, seed.into(), SetBy::CommandLine);
        let generation = vars.var(state.var).generation();
        // A value typed before the camera existed differs from the seed: apply it next frame.
        state.applied = Some(if (vars.float(state.var) - seed).abs() < 1e-9 { generation } else { u64::MAX });
        return;
    }
    let generation = console.vars().var(state.var).generation();
    if state.applied == Some(generation) {
        return;
    }
    state.applied = Some(generation);
    let intensity = console.vars().float(state.var) as f32;
    for (camera, _) in &cameras {
        if intensity > 0.0 {
            let base = state.settings.clone().unwrap_or(Bloom::NATURAL);
            commands.entity(camera).insert(Bloom { intensity, ..base });
        } else {
            commands.entity(camera).remove::<Bloom>();
        }
    }
}

/// `render.gi` and whether the GI is in this run.
#[derive(Resource)]
struct GiVar {
    var: VarId,
    /// The variable's generation last applied (`None`: not seeded yet).
    applied: Option<u64>,
}

/// Seeds `render.gi` from `sf_sky_bevy::GiEnabled` (absent: the GI isn't in this run, the
/// variable stays off), then sets the resource when the variable changes: `sf_gi_bevy` drops
/// the volumes' textures (nothing traced or shown) and re-traces them when it comes back.
/// A value typed before the seed wins, as for the camera variables.
#[allow(clippy::needless_pass_by_value)]
fn apply_gi(mut console: ResMut<DevConsole>, mut state: ResMut<GiVar>, enabled: Option<ResMut<sf_sky_bevy::GiEnabled>>) {
    let Some(mut enabled) = enabled else {
        if state.applied.is_none() {
            let _ = console.vars_mut().set_by(state.var, false.into(), SetBy::CommandLine);
            state.applied = Some(console.vars().var(state.var).generation());
        }
        return;
    };
    if state.applied.is_none() {
        state.applied = Some(seed_from_camera(&mut console, state.var, enabled.0));
        return;
    }
    let Some(on) = changed_since(&console, state.var, &mut state.applied) else { return };
    if enabled.0 != on {
        enabled.0 = on;
    }
}

/// `render.gi_intensity` and `render.gi_ambient`.
#[derive(Resource)]
struct GiGain {
    intensity: VarId,
    ambient: VarId,
    /// Both variables' generations when last checked in a run without GI (`None`: not yet).
    reported: Option<(u64, u64)>,
}

/// Keeps the GI gain variables and the `GiVolume`s in step. A variable not typed in the
/// console follows the volumes the game spawns (seeded from each new one, as the camera
/// variables are); one typed in the console ([`SetBy::Console`]) wins on every volume, the
/// ones a re-fit or door travel spawns later included. `sf_gi_bevy` rewrites the volume's
/// uniform when the values change (no re-trace). A run without the GI says so when one is typed.
#[allow(clippy::needless_pass_by_value)]
#[expect(clippy::cast_possible_truncation, reason = "console floats are f64; the component is f32")]
fn apply_gi_gain(
    mut console: ResMut<DevConsole>,
    mut state: ResMut<GiGain>,
    enabled: Option<Res<sf_sky_bevy::GiEnabled>>,
    mut volumes: Query<&mut sf_gi_bevy::GiVolume>,
) {
    let (intensity, ambient) = (state.intensity, state.ambient);
    if enabled.is_none() {
        let vars = console.vars();
        let generations = (vars.var(intensity).generation(), vars.var(ambient).generation());
        if state.reported.is_some_and(|seen| seen != generations) {
            console.log_mut().warn(&format!("gi gain: {GI_NOT_IN_RUN}"));
        }
        state.reported = Some(generations);
        return;
    }
    // Read through `Mut` (for `is_added`) without writing: no change is flagged.
    for volume in &mut volumes {
        if !volume.is_added() {
            continue;
        }
        let vars = console.vars_mut();
        // Outranked (and ignored) once a value was typed in the console.
        let _ = vars.set_by(intensity, f64::from(volume.intensity).into(), SetBy::CommandLine);
        let _ = vars.set_by(ambient, f64::from(volume.ambient_scale).into(), SetBy::CommandLine);
    }
    let vars = console.vars();
    let typed = |var: VarId| (vars.var(var).set_by() == SetBy::Console).then(|| vars.float(var) as f32);
    let (intensity, ambient) = (typed(intensity), typed(ambient));
    if intensity.is_none() && ambient.is_none() {
        return;
    }
    for mut volume in &mut volumes {
        // Compared before writing: an unchanged volume keeps its change ticks.
        if let Some(v) = intensity.filter(|v| v.to_bits() != volume.intensity.to_bits()) {
            volume.intensity = v;
        }
        if let Some(v) = ambient.filter(|v| v.to_bits() != volume.ambient_scale.to_bits()) {
            volume.ambient_scale = v;
        }
    }
}

/// `terrain.reflectance` and whether it was seeded from the terrain.
#[derive(Resource)]
struct TerrainReflectance {
    var: VarId,
    /// Seeded from `TerrainSettings` (the first frame it exists).
    seeded: bool,
    /// The variable's generation when last checked in a run without terrain (`None`: not yet).
    reported: Option<u64>,
}

/// Seeds `terrain.reflectance` from `sf_terrain_bevy::TerrainSettings` once it exists (a value
/// typed earlier wins), then writes a typed value into the settings, which the terrain uploads
/// to its shader every frame (RO-16 veil fix 1). Without terrain, a typed value says so.
#[allow(clippy::needless_pass_by_value)]
#[expect(clippy::cast_possible_truncation, reason = "console floats are f64; the setting is f32")]
fn apply_terrain_reflectance(mut console: ResMut<DevConsole>, mut state: ResMut<TerrainReflectance>, settings: Option<ResMut<sf_terrain_bevy::TerrainSettings>>) {
    let Some(mut settings) = settings else {
        let generation = console.vars().var(state.var).generation();
        if state.reported.is_some_and(|seen| seen != generation) {
            console.log_mut().warn("terrain reflectance: no terrain here");
        }
        state.reported = Some(generation);
        return;
    };
    if !state.seeded {
        state.seeded = true;
        // Outranked (and ignored) once a value was typed in the console.
        let _ = console.vars_mut().set_by(state.var, f64::from(settings.reflectance).into(), SetBy::CommandLine);
    }
    let vars = console.vars();
    if vars.var(state.var).set_by() != SetBy::Console {
        return;
    }
    let value = vars.float(state.var) as f32;
    // Compared before writing: unchanged settings keep their change ticks.
    if value.to_bits() != settings.reflectance.to_bits() {
        settings.reflectance = value;
    }
}

/// `on`/`off`/`flip ambient`, RO-16 veil diagnosis (scaffold: remove with the veil fix,
/// `docs/visual-checks/rendering.md` RO-16): the sky's flat ambient floor on the streaming camera
/// (`SkyCamera::ambient`: 4 nits at night, 0 once the sun is above -4°), for every object. Off
/// keeps the camera's `AmbientLight` at 0 rather than removing it (without it the interior's
/// global ambient would apply); on hands it back to the sky, which rewrites it the next frame.
fn set_sky_ambient(world: &mut World, on: Option<bool>) -> Result<String, String> {
    let mut cameras = world.query_filtered::<(&mut sf_sky_bevy::SkyCamera, Option<&mut bevy_light::AmbientLight>), With<ExteriorCamera>>();
    let mut text = None;
    for (mut sky, ambient) in cameras.iter_mut(world) {
        let v = on.unwrap_or(!sky.ambient);
        sky.ambient = v;
        if !v && let Some(mut ambient) = ambient {
            ambient.brightness = 0.0;
        }
        text = Some(format!("sky ambient {}", if v { "on" } else { "off" }));
    }
    text.ok_or_else(|| "ambient: no sky camera in this run (interior or --off sky)".to_owned())
}

/// `rain_slant_cap` (WE-8): the precipitation's capped, smoothed lean vs the frame's raw wind.
fn set_rain_slant_cap(world: &mut World, on: Option<bool>) -> Result<String, String> {
    let mut lean = world
        .get_resource_mut::<sf_weather_bevy::PrecipWind>()
        .ok_or_else(|| "rain_slant_cap: no weather in this run (interior)".to_owned())?;
    let v = on.unwrap_or(!lean.slant_cap);
    lean.slant_cap = v;
    Ok(format!("rain slant cap {}", if v { "on" } else { "off (raw wind)" }))
}

/// `viewmodel` (first-person.md FP-2): the player camera's `ComfortSettings::view_model`, so the
/// owner can A/B the first-person view model in one run (arms and held items redrawn at the body
/// FOV in the near depth slice vs drawn with the body).
fn set_view_model(world: &mut World, on: Option<bool>) -> Result<String, String> {
    let mut cameras = world.query::<&mut sf_camera_bevy::PlayerCamera>();
    let mut text = None;
    for mut cam in cameras.iter_mut(world) {
        let v = on.unwrap_or(!cam.settings.view_model);
        cam.settings.view_model = v;
        text = Some(format!("view model {} (first person)", if v { "on" } else { "off" }));
    }
    text.ok_or_else(|| "viewmodel: no player camera in this run".to_owned())
}

/// `on`/`off`/`flip <name>`: what happened, or why nothing did.
fn set_switch(world: &mut World, vars: &mut Vars, live: LiveVars, name: &str, on: Option<bool>) -> Result<String, String> {
    let var = match name {
        "ssao" => Some(live.ssao),
        "contactshadows" => Some(live.contact),
        _ => None,
    };
    if name == "gi" {
        if !world.contains_resource::<sf_sky_bevy::GiEnabled>() {
            return Err(format!("gi: {GI_NOT_IN_RUN}"));
        }
        let v = on.unwrap_or(!vars.flag(live.gi));
        vars.set_by(live.gi, v.into(), SetBy::Console)?;
        return Ok(format!("gi {}", if v { "on" } else { "off" }));
    }
    if name == "ambient" {
        return set_sky_ambient(world, on);
    }
    if name == "rain_slant_cap" {
        return set_rain_slant_cap(world, on);
    }
    if name == "viewmodel" {
        return set_view_model(world, on);
    }
    if let Some(var) = var {
        if world.query_filtered::<(), With<ExteriorCamera>>().iter(world).next().is_none() {
            return Err(format!("{name}: no streaming camera in this run"));
        }
        let v = on.unwrap_or(!vars.flag(var));
        vars.set_by(var, v.into(), SetBy::Console)?;
        return Ok(format!("{name} {}", if v { "on" } else { "off" }));
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
        Err(unknown) => Err(format!("{unknown}; live: ssao, contactshadows, gi, ambient, viewmodel, {}", Switch::names())),
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

    #[test]
    fn contact_shadows_flip_live_restore_the_preset_and_light_the_sun() {
        let mut app = app();
        let medium = ContactShadows { linear_steps: 8, thickness: 0.1, length: 0.3 };
        let camera = app.world_mut().spawn((ExteriorCamera, medium)).id();
        // A run started with them off leaves the sun without them (`exterior.rs` `sun_shadow`).
        let sun = app.world_mut().spawn((SunLight, DirectionalLight { contact_shadows_enabled: false, ..Default::default() })).id();
        app.update();
        app.update();
        assert!(app.world().get::<ContactShadows>(camera).is_some());
        run(&mut app, "off contactshadows");
        assert!(app.world().get::<ContactShadows>(camera).is_none(), "{:?}", log(&app));
        run(&mut app, "toggle render.contactshadows");
        let back = app.world().get::<ContactShadows>(camera).expect("contact shadows back on");
        assert_eq!(back.linear_steps, 8, "the preset's settings, not the default 16");
        assert!(app.world().get::<DirectionalLight>(sun).unwrap().contact_shadows_enabled);
        run(&mut app, "flip contactshadows");
        assert!(app.world().get::<ContactShadows>(camera).is_none());
        assert!(log(&app).iter().any(|l| l == "contactshadows off"), "{:?}", log(&app));
    }

    #[test]
    fn gi_flips_live_and_says_when_the_run_has_none() {
        let mut none = app();
        run(&mut none, "on gi");
        assert!(log(&none).iter().any(|l| l.contains("gi: not in this run")), "{:?}", log(&none));
        let var = none.world().resource::<GiVar>().var;
        assert!(!none.world().resource::<DevConsole>().vars().flag(var), "no GI: seeded off");

        let mut app = app();
        app.insert_resource(sf_sky_bevy::GiEnabled(true));
        app.update();
        app.update();
        let var = app.world().resource::<GiVar>().var;
        assert!(app.world().resource::<DevConsole>().vars().flag(var), "seeded from GiEnabled");
        run(&mut app, "off gi");
        assert!(!app.world().resource::<sf_sky_bevy::GiEnabled>().0, "{:?}", log(&app));
        run(&mut app, "flip gi");
        assert!(app.world().resource::<sf_sky_bevy::GiEnabled>().0);
        run(&mut app, "toggle render.gi");
        assert!(!app.world().resource::<sf_sky_bevy::GiEnabled>().0);
        assert!(log(&app).iter().any(|l| l == "gi on"), "{:?}", log(&app));
    }

    #[test]
    #[allow(clippy::float_cmp)] // exact copies, no arithmetic
    fn gi_gain_tunes_every_volume_live_survives_a_refit_and_says_when_the_run_has_none() {
        use sf_gi_bevy::GiVolume;
        let mut none = app();
        none.update();
        run(&mut none, "render.gi_intensity 2");
        assert!(log(&none).iter().any(|l| l.contains("gi gain: not in this run")), "{:?}", log(&none));

        let mut app = app();
        app.insert_resource(sf_sky_bevy::GiEnabled(true));
        let first = app.world_mut().spawn(GiVolume { intensity: 1.25, ..GiVolume::default() }).id();
        app.update();
        app.update();
        let gain = app.world().resource::<GiGain>();
        let (intensity, ambient) = (gain.intensity, gain.ambient);
        let vars = app.world().resource::<DevConsole>().vars();
        assert_eq!(vars.float(intensity), 1.25, "seeded from the live volume");
        assert_eq!(vars.float(ambient), 0.0);
        run(&mut app, "render.gi_intensity 2");
        run(&mut app, "render.gi_ambient 0.1");
        let tuned = app.world().get::<GiVolume>(first).expect("volume");
        assert_eq!((tuned.intensity, tuned.ambient_scale), (2.0, 0.1), "{:?}", log(&app));
        // A re-fit (door travel): the game spawns a new volume with its own values.
        app.world_mut().despawn(first);
        let second = app.world_mut().spawn(GiVolume::default()).id();
        app.update();
        let refit = app.world().get::<GiVolume>(second).expect("volume");
        assert_eq!((refit.intensity, refit.ambient_scale), (2.0, 0.1), "typed values win on the new volume");
        assert_eq!(app.world().resource::<DevConsole>().vars().float(intensity), 2.0);
    }

    #[test]
    fn contact_length_and_thickness_retune_the_camera_live_and_survive_a_flip() {
        let mut app = app();
        let medium = ContactShadows { linear_steps: 8, thickness: 0.1, length: 0.3 };
        let camera = app.world_mut().spawn((ExteriorCamera, medium)).id();
        app.update();
        app.update();
        let state = app.world().resource::<Contact>();
        let length = state.length;
        let vars = app.world().resource::<DevConsole>().vars();
        assert!((vars.float(length) - 0.3).abs() < 1e-6, "seeded from the camera, not Bevy's default");
        run(&mut app, "render.contactlength 3");
        run(&mut app, "render.contactthickness 1");
        let tuned = app.world().get::<ContactShadows>(camera).expect("still on");
        assert!((tuned.length - 3.0).abs() < 1e-6 && (tuned.thickness - 1.0).abs() < 1e-6, "length {} thickness {}", tuned.length, tuned.thickness);
        assert_eq!(tuned.linear_steps, 8, "the preset's steps stay");
        run(&mut app, "off contactshadows");
        assert!(app.world().get::<ContactShadows>(camera).is_none());
        run(&mut app, "on contactshadows");
        let back = app.world().get::<ContactShadows>(camera).expect("back on");
        assert!((back.length - 3.0).abs() < 1e-6, "the tuned length survives a flip: {}", back.length);
    }

    #[test]
    fn bloom_intensity_seeds_from_the_camera_retunes_live_and_zero_turns_it_off() {
        let mut app = app();
        let preset = Bloom { intensity: 0.1, ..Bloom::NATURAL };
        let camera = app.world_mut().spawn((ExteriorCamera, preset)).id();
        app.update();
        app.update();
        let var = app.world().resource::<BloomVar>().var;
        assert!((app.world().resource::<DevConsole>().vars().float(var) - 0.1).abs() < 1e-6, "seeded from the camera, not NATURAL's 0.15");
        run(&mut app, "render.bloom 0.3");
        let tuned = app.world().get::<Bloom>(camera).expect("still on");
        assert!((tuned.intensity - 0.3).abs() < 1e-6, "intensity {}", tuned.intensity);
        run(&mut app, "render.bloom 0");
        assert!(app.world().get::<Bloom>(camera).is_none(), "{:?}", log(&app));
        run(&mut app, "render.bloom 0.2");
        let back = app.world().get::<Bloom>(camera).expect("back on");
        assert!((back.intensity - 0.2).abs() < 1e-6, "intensity {}", back.intensity);
    }

    #[test]
    fn camera_without_bloom_seeds_zero_and_turns_on_natural() {
        let mut app = app();
        let camera = app.world_mut().spawn(ExteriorCamera).id();
        app.update();
        app.update();
        let var = app.world().resource::<BloomVar>().var;
        assert!(app.world().resource::<DevConsole>().vars().float(var).abs() < 1e-9);
        assert!(app.world().get::<Bloom>(camera).is_none());
        run(&mut app, "render.bloom 0.1");
        assert!(app.world().get::<Bloom>(camera).is_some(), "{:?}", log(&app));
    }

    #[test]
    fn camera_without_contact_shadows_seeds_off_and_turns_on_with_defaults() {
        let mut app = app();
        let camera = app.world_mut().spawn(ExteriorCamera).id();
        app.update();
        app.update();
        let var = app.world().resource::<Contact>().var;
        assert!(!app.world().resource::<DevConsole>().vars().flag(var));
        run(&mut app, "on contactshadows");
        let on = app.world().get::<ContactShadows>(camera).expect("contact shadows on");
        assert_eq!(on.linear_steps, ContactShadows::default().linear_steps);
    }

    /// WE-10 fix 1: `flip cloud_history` swaps the accumulating resolve for the legacy one.
    #[test]
    fn cloud_history_flips_the_cloud_resolve() {
        use sf_atmos_bevy::sf_atmos::CloudTemporal;
        let mut app = app();
        run(&mut app, "flip cloud_history");
        assert!(log(&app).iter().any(|l| l.contains("cloud temporal accumulation: not in this run")), "{:?}", log(&app));
        app.insert_resource(sf_atmos_bevy::Clouds {
            settings: sf_atmos_bevy::sf_atmos::CloudSettings::default(),
            enabled: true,
            shadows: false,
            glows: [Vec4::ZERO; sf_atmos_bevy::GLOWS],
            temporal: CloudTemporal::default(),
        });
        run(&mut app, "flip cloud_history");
        assert_eq!(app.world().resource::<sf_atmos_bevy::Clouds>().temporal, CloudTemporal::LEGACY);
        run(&mut app, "on cloud_history");
        assert_eq!(app.world().resource::<sf_atmos_bevy::Clouds>().temporal, CloudTemporal::ACCUMULATE);
    }

    /// RO-16 veil diagnosis switches (scaffold: remove with the veil fix).
    #[test]
    fn veil_switches_flip_terrain_bits_surface_fog_and_the_sky_ambient_live() {
        use sf_terrain_bevy::TerrainDebug;
        let mut app = app();
        run(&mut app, "off terrain_specular");
        run(&mut app, "off ambient");
        let lines = log(&app);
        assert!(lines.iter().any(|l| l == "terrain specular: no terrain here"), "{lines:?}");
        assert!(lines.iter().any(|l| l.contains("ambient: no sky camera")), "{lines:?}");

        app.init_resource::<TerrainDebug>().init_resource::<sf_surface_bevy::SurfaceSettings>().insert_resource(
            sf_atmos_bevy::Fog { settings: sf_atmos_bevy::sf_atmos::FogSettings::default(), enabled: true, froxels: None, view_ambient: sf_atmos_bevy::sf_atmos::VIEW_AMBIENT_IN_FOG },
        );
        for line in ["off terrain_specular", "off terrain_env_specular", "flip terrain_indirect", "off terrain_tint", "off surface", "flip fog", "flip fog_night_light"] {
            run(&mut app, line);
        }
        let d = *app.world().resource::<TerrainDebug>();
        assert!(d.no_specular && d.no_env_specular && d.no_indirect && d.raw_layers && !d.lod_colors && !d.no_far_shadows, "{d:?}");
        assert!(!app.world().resource::<sf_surface_bevy::SurfaceSettings>().layer);
        assert!(!app.world().resource::<sf_atmos_bevy::Fog>().enabled);
        assert!(app.world().resource::<sf_atmos_bevy::Fog>().view_ambient <= 0.0, "flip fog_night_light: sky-only fill");
        run(&mut app, "on terrain_specular");
        assert!(!app.world().resource::<TerrainDebug>().no_specular);

        let camera = app
            .world_mut()
            .spawn((ExteriorCamera, sf_sky_bevy::SkyCamera::default(), bevy_light::AmbientLight { brightness: 4.0, ..Default::default() }))
            .id();
        run(&mut app, "off ambient");
        assert_eq!(app.world().get::<bevy_light::AmbientLight>(camera).map(|a| a.brightness), Some(0.0));
        assert!(!app.world().get::<sf_sky_bevy::SkyCamera>(camera).expect("sky camera").ambient);
        run(&mut app, "flip ambient");
        assert!(app.world().get::<sf_sky_bevy::SkyCamera>(camera).expect("sky camera").ambient);
        assert!(log(&app).iter().any(|l| l == "sky ambient on"), "{:?}", log(&app));
    }

    /// RO-16 fix 2: `flip weather_ambient` swaps the weather's ambient tint for the old fixed blue.
    #[test]
    fn view_model_switch_flips_the_player_camera_setting() {
        let mut app = app();
        run(&mut app, "flip viewmodel");
        assert!(log(&app).iter().any(|l| l.ends_with("viewmodel: no player camera in this run")), "{:?}", log(&app));
        let cam = app.world_mut().spawn(sf_camera_bevy::PlayerCamera::new(Entity::PLACEHOLDER)).id();
        assert!(!app.world().get::<sf_camera_bevy::PlayerCamera>(cam).unwrap().settings.view_model, "off by default");
        run(&mut app, "flip viewmodel");
        assert!(app.world().get::<sf_camera_bevy::PlayerCamera>(cam).unwrap().settings.view_model);
        assert!(log(&app).iter().any(|l| l == "view model on (first person)"), "{:?}", log(&app));
        run(&mut app, "off viewmodel");
        assert!(!app.world().get::<sf_camera_bevy::PlayerCamera>(cam).unwrap().settings.view_model);
    }

    #[test]
    fn weather_ambient_switch_flips_the_sky_option() {
        let mut app = app();
        run(&mut app, "flip weather_ambient");
        assert!(log(&app).iter().any(|l| l == "weather ambient tint: not in this run (--off sky)"), "{:?}", log(&app));
        app.init_resource::<sf_sky_bevy::SkyRenderOptions>();
        run(&mut app, "flip weather_ambient");
        assert!(!app.world().resource::<sf_sky_bevy::SkyRenderOptions>().weather_ambient_tint);
        run(&mut app, "on weather_ambient");
        assert!(app.world().resource::<sf_sky_bevy::SkyRenderOptions>().weather_ambient_tint);
        assert!(log(&app).iter().any(|l| l == "weather ambient tint on"), "{:?}", log(&app));
    }

    /// N1: `flip ambient_specular` brings back Bevy's flat-ambient specular (the game runs
    /// diffuse only), live on the vendored `bevy_pbr::AmbientLightSpecular`.
    #[test]
    fn ambient_specular_switch_flips_the_pbr_resource() {
        let mut app = app();
        run(&mut app, "flip ambient_specular");
        assert!(log(&app).iter().any(|l| l == "flat ambient specular: not in this run (no PBR renderer)"), "{:?}", log(&app));
        app.insert_resource(bevy_pbr::AmbientLightSpecular(false));
        run(&mut app, "flip ambient_specular");
        assert!(app.world().resource::<bevy_pbr::AmbientLightSpecular>().0);
        assert!(log(&app).iter().any(|l| l == "flat ambient specular on"), "{:?}", log(&app));
        run(&mut app, "off ambient_specular");
        assert!(!app.world().resource::<bevy_pbr::AmbientLightSpecular>().0);
    }

    /// N2: `flip night_sky_glow` turns the night sky's own glow off (a black sky behind the stars).
    #[test]
    fn night_sky_glow_switch_flips_the_sky_option() {
        let mut app = app();
        run(&mut app, "flip night_sky_glow");
        assert!(log(&app).iter().any(|l| l == "night sky glow: not in this run (--off sky)"), "{:?}", log(&app));
        app.init_resource::<sf_sky_bevy::SkyRenderOptions>();
        run(&mut app, "flip night_sky_glow");
        assert!(!app.world().resource::<sf_sky_bevy::SkyRenderOptions>().night_sky_glow);
        run(&mut app, "on night_sky_glow");
        assert!(app.world().resource::<sf_sky_bevy::SkyRenderOptions>().night_sky_glow);
        assert!(log(&app).iter().any(|l| l == "night sky glow on"), "{:?}", log(&app));
    }

    /// N3: `flip moonlight` turns the artistic moonlight off (the moons' own dim light) and back.
    #[test]
    fn moonlight_switch_flips_the_sky_settings() {
        let mut app = app();
        run(&mut app, "flip moonlight");
        assert!(log(&app).iter().any(|l| l == "artistic moonlight: not in this run (--off sky)"), "{:?}", log(&app));
        let mut settings = sf_sky_bevy::sf_sky::SkySettings::default();
        settings.moonlight.artistic = true;
        app.insert_resource(sf_sky_bevy::SkyConfig(settings));
        let artistic = |app: &App| app.world().resource::<sf_sky_bevy::SkyConfig>().0.moonlight.artistic;
        run(&mut app, "flip moonlight");
        assert!(!artistic(&app));
        run(&mut app, "on moonlight");
        assert!(artistic(&app));
        assert!(log(&app).iter().any(|l| l == "artistic moonlight on"), "{:?}", log(&app));
    }
    /// RO-16 veil fix 1: `terrain.reflectance` seeds from the terrain, retunes it live, and says
    /// so in a run without terrain.
    #[test]
    fn terrain_reflectance_seeds_from_the_terrain_and_retunes_it_live() {
        use sf_terrain_bevy::TerrainSettings;
        let mut none = app();
        none.update();
        run(&mut none, "terrain.reflectance 0.2");
        assert!(log(&none).iter().any(|l| l.contains("terrain reflectance: no terrain here")), "{:?}", log(&none));

        let mut app = app();
        app.insert_resource(TerrainSettings { reflectance: 0.35, ..TerrainSettings::default() });
        app.update();
        let var = app.world().resource::<TerrainReflectance>().var;
        let seeded = app.world().resource::<DevConsole>().vars().float(var);
        assert!((seeded - 0.35).abs() < 1e-6, "seeded from the terrain, not the default: {seeded}");
        run(&mut app, "terrain.reflectance 0.2");
        let tuned = app.world().resource::<TerrainSettings>().reflectance;
        assert!((tuned - 0.2).abs() < 1e-6, "{tuned} {:?}", log(&app));
        run(&mut app, "terrain.reflectance 0");
        assert!(app.world().resource::<TerrainSettings>().reflectance.abs() < 1e-9);
    }
}
