//! API v0.1 (L3) headless: `@sf/world` (spawn/despawn queue, nearby, handle methods, entity
//! scripts in the activation pipeline), `on_load` world access, the example mod
//! `examples/mods/api_tour`, and the golden `sf.d.luau`.

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_time::Time;
use sf_script::{ContextConfig, Handle, ScriptRuntime, ScriptSource, StateValue, storage_value};

use crate::{
    Activated, ActivationOutcome, PLAYER, Placed, ScriptDiagnostics, ScriptPlacements, ScriptSlot, Scripts, ScriptsPlugin,
    WorldCommand,
};

struct MemSource(HashMap<String, String>);

impl ScriptSource for MemSource {
    fn read(&self, path: &str) -> Option<Vec<u8>> {
        self.0.get(path).map(|s| s.as_bytes().to_vec())
    }
}

/// Every file under `<mod>/assets/scripts`, keyed by VFS path (`scripts/<id>/...`).
fn example_mod(id: &str) -> HashMap<String, String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/mods").join(id).join("assets");
    let mut out = HashMap::new();
    let mut dirs = vec![root.join("scripts")];
    while let Some(dir) = dirs.pop() {
        for entry in std::fs::read_dir(&dir).unwrap_or_else(|e| panic!("{}: {e}", dir.display())) {
            let path = entry.unwrap().path();
            if path.is_dir() {
                dirs.push(path);
            } else {
                let rel = path.strip_prefix(&root).unwrap().to_string_lossy().replace('\\', "/");
                out.insert(rel, std::fs::read_to_string(&path).unwrap());
            }
        }
    }
    out
}

const PEEK: &str = r#"
local interfaces = require("@sf/interfaces")
local core = require("@sf/core")
local events = require("@sf/events")
events.on("update", function()
    local tour = interfaces.get("api_tour", "tour")
    core.storage().tour_spawned = tour and tour.spawned()
end)
"#;

const TAPESTRY: Handle = Handle(10);
const BARREL: Handle = Handle(11);

#[derive(Resource, Default)]
struct Pending(Vec<Activated>);

#[derive(Resource, Default)]
struct Seen {
    outcomes: Vec<ActivationOutcome>,
    commands: Vec<WorldCommand>,
    diagnostics: Vec<String>,
}

fn send(mut pending: ResMut<Pending>, mut out: MessageWriter<Activated>) {
    for a in pending.0.drain(..) {
        out.write(a);
    }
}

fn collect(
    mut seen: ResMut<Seen>,
    mut outcomes: MessageReader<ActivationOutcome>,
    mut commands: MessageReader<WorldCommand>,
    mut diags: MessageReader<ScriptDiagnostics>,
) {
    seen.outcomes.extend(outcomes.read().copied());
    seen.commands.extend(commands.read().cloned());
    seen.diagnostics.extend(diags.read().flat_map(|d| d.0.iter().map(|d| format!("{}: {}", d.context, d.message))));
}

fn app(files: HashMap<String, String>, configs: Vec<ContextConfig>) -> App {
    let mut scripts = Scripts::new(ScriptRuntime::default());
    scripts.start(configs, Arc::new(MemSource(files)));
    assert!(scripts.stopped().next().is_none(), "{:?}", scripts.recent().collect::<Vec<_>>());
    let mut app = App::new();
    app.insert_resource(scripts)
        .insert_resource(Time::<()>::default())
        .add_plugins(ScriptsPlugin)
        .init_resource::<Pending>()
        .init_resource::<Seen>()
        .add_systems(Update, send.before(ScriptSlot::Update))
        .add_systems(Update, collect.after(ScriptSlot::Update));
    let placed = |record: &str, x: f64| Placed { record: record.into(), worldspace: "ws".into(), position: [x, 0.0, 0.0], radius: 0.5 };
    let mut p = app.world_mut().resource_mut::<ScriptPlacements>();
    p.loaded.insert(TAPESTRY, placed("tapestry", 3.0));
    p.loaded.insert(BARREL, placed("barrel", 4.0));
    p.player = Some(placed("player", 1.5));
    app
}

fn frame(app: &mut App) {
    app.world_mut().resource_mut::<Time>().advance_by(Duration::from_millis(100));
    app.update();
}

fn activate(app: &mut App, target: Handle) {
    app.world_mut().resource_mut::<Pending>().0.push(Activated { target, actor: PLAYER });
    frame(app);
}

fn value(app: &App, ctx: &str, key: &str) -> Option<StateValue> {
    storage_value(app.world().resource::<Scripts>().runtime.context(ctx)?, key)
}

fn eval(app: &mut App, ctx: &str, code: &str) -> Option<bool> {
    let mut scripts = app.world_mut().resource_mut::<Scripts>();
    let policy = *scripts.runtime.policy();
    scripts.runtime.context_mut(ctx)?.eval::<bool>(code, &policy)
}

#[test]
fn example_mod_api_tour_runs_headless() {
    let mut files = example_mod("api_tour");
    files.insert("scripts/peek/main.luau".into(), PEEK.into());
    let mut app = app(files, vec![ContextConfig::for_mod("api_tour"), ContextConfig::for_mod("peek")]);

    // on_load ran in the first Update slot, with world access.
    frame(&mut app);
    assert_eq!(value(&app, "api_tour", "player_x"), Some(StateValue::Number(1.5)));
    assert_eq!(value(&app, "api_tour", "last_load"), Some(StateValue::Str(b"start".to_vec())));

    // First use: entity script attached by the global handler, a copy spawned 1 m above.
    activate(&mut app, TAPESTRY);
    let spawned = {
        let seen = app.world().resource::<Seen>();
        assert_eq!(seen.outcomes.len(), 1);
        assert!(!seen.outcomes[0].handled);
        match seen.commands.as_slice() {
            [WorldCommand::Spawn { handle, record, worldspace, position, context, .. }] => {
                assert_eq!((record.as_str(), worldspace.as_str(), *position, context.as_str()), ("tapestry", "ws", [3.0, 1.0, 0.0], "api_tour"));
                *handle
            }
            other => panic!("{other:?} / {:?}", seen.diagnostics),
        }
    };
    // The handle is readable at once (scripts read their own writes).
    assert!(app.world().resource::<ScriptPlacements>().loaded.contains_key(&spawned));
    // Later uses reach the entity script first; it returns "handled" on its second use.
    activate(&mut app, TAPESTRY);
    assert!(!app.world().resource::<Seen>().outcomes[1].handled);
    activate(&mut app, TAPESTRY);
    assert!(app.world().resource::<Seen>().outcomes[2].handled);
    // Interfaces: another mod reads the count through the export.
    assert_eq!(value(&app, "peek", "tour_spawned"), Some(StateValue::Number(3.0)));
    // Ticks accumulate while loaded; the timer despawns the first copy after 3 s.
    for _ in 0..30 {
        frame(&mut app);
    }
    let seen = app.world().resource::<Seen>();
    assert!(seen.commands.iter().any(|c| matches!(c, WorldCommand::Despawn { handle, .. } if *handle == spawned)), "{:?}", seen.commands);
    assert!(!app.world().resource::<ScriptPlacements>().loaded.contains_key(&spawned));
    assert!(seen.diagnostics.is_empty(), "{:?}", seen.diagnostics);
    // The entity state is plain data in the mod's snapshot (saves, hot reload).
    let bytes = app.world().resource::<Scripts>().runtime.snapshot("api_tour").unwrap();
    let snap = sf_script::ContextSnapshot::from_bytes(&bytes).unwrap();
    let [lamp] = snap.entities.as_slice() else { panic!("{:?}", snap.entities) };
    assert_eq!((lamp.handle, lamp.script.as_str(), lamp.state.get("uses")), (TAPESTRY, "lamp", Some(&StateValue::Number(2.0))));
    assert!(matches!(lamp.state.get("age"), Some(StateValue::Number(a)) if *a > 2.9));
}

#[test]
#[allow(clippy::float_cmp, reason = "exact values passed through")]
fn world_api_spawn_ids_nearby_order_and_errors() {
    let main = r#"
local world = require("@sf/world")
local core = require("@sf/core")
-- Top level runs with the slot closed: world reads fail here.
core.storage().top_level = pcall(world.position, world.player())
"#;
    let mut app = app(HashMap::from([("scripts/m/main.luau".into(), main.into())]), vec![ContextConfig::for_mod("m")]);
    assert_eq!(value(&app, "m", "top_level"), Some(StateValue::Bool(false)));
    frame(&mut app);
    // Outside a slot (eval runs between frames) world calls fail.
    assert_eq!(eval(&mut app, "m", r#"return not pcall(require("@sf/world").spawn, "x", { x = 0, y = 0, z = 0 })"#), Some(true));
    // Inside one they work. Nearby: distance order, then id; spawn ids differ per call.
    let code = r#"
local world = require("@sf/world")
local events = require("@sf/events")
local s = require("@sf/core").storage()
local near = world.nearby({ x = 3.4, y = 0, z = 0 }, 1)
s.near = near[1]:record() .. "," .. near[2]:record()
local a = world.spawn("barrel", world.player())
local b = world.spawn("barrel", world.player(), 1.5)
s.distinct = a ~= b and a:is_loaded() and b:record() == "barrel"
s.despawn_player = pcall(world.despawn, world.player())
"#;
    // Open the slot by hand around one guarded call (as the slot systems do).
    let world = app.world_mut();
    let movers = world.resource::<Scripts>().movers.clone();
    world.resource_scope(|world, mut scripts: Mut<Scripts>| {
        scripts.access.open(world, &movers);
        let policy = *scripts.runtime.policy();
        let ctx = scripts.runtime.context_mut("m").unwrap();
        let f = ctx.lua().load(code).set_name("=console:m").into_function().unwrap();
        assert!(ctx.call::<()>(&f, (), "snippet", &policy).is_some(), "{:?}", ctx.last_error());
        scripts.access.close(world, &movers);
    });
    assert_eq!(value(&app, "m", "near"), Some(StateValue::Str(b"tapestry,barrel".to_vec())));
    assert_eq!(value(&app, "m", "distinct"), Some(StateValue::Bool(true)));
    assert_eq!(value(&app, "m", "despawn_player"), Some(StateValue::Bool(false)));
    let w = app.world_mut().resource_mut::<crate::ScriptWorld>().take_commands();
    assert!(matches!(w.as_slice(), [WorldCommand::Spawn { yaw: y0, .. }, WorldCommand::Spawn { yaw: y1, .. }] if *y0 == 0.0 && *y1 == 1.5));
}

/// The golden definitions file for every API module `Scripts` registers. Regenerate with
/// `SF_BLESS=1 cargo test -p sf_script_bevy golden`.
#[test]
fn golden_type_definitions() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/api/sf.d.luau");
    let generated = Scripts::default().runtime.type_definitions();
    if std::env::var_os("SF_BLESS").is_some() {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, &generated).unwrap();
    }
    let golden = std::fs::read_to_string(&path).unwrap_or_default().replace("\r\n", "\n");
    assert!(golden == generated, "{} is stale: run `SF_BLESS=1 cargo test -p sf_script_bevy golden`", path.display());
}
