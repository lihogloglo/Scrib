//! `bevyrrowind character [ids...]`: a lineup of dressed, idling actors on
//! a lit stage (character.md "Verification": the viewer's `character`
//! mode). Ids are NPC_ or CREA record ids; `id:naked` drops the NPC's
//! equipment; `id+item+item` dresses it in exactly those items (any spec);
//! `@race,class[,f]` picks the first NPC (by id) of that race
//! and class (empty: any), `f` female only.
//!
//! Keys: O swaps every NPC's outfit (own equipment ↔ naked: the swap must
//! not pop), F fires the `fidget` trigger (cycling `Idle2`-`Idle4`), X toggles
//! `dead` (`Death1`), G skeleton gizmos, B smooth ↔
//! vanilla bodies (`--bodies`; rebuilds through the same no-pop swap),
//! 1 walk in place (`--walk`), 2 run, 3 applies root motion (actors walk off
//! at their clips' speed; 3 again puts them back), R draws/sheathes weapons (`--drawn`;
//! the graph's weapon stance follows), N night (`--night`: the stage goes
//! dark and actors carry their torches, lit).
//! The log lists each character's skinned meshes (one per material, plus
//! morphing heads) and the outfit cache.

use std::path::PathBuf;
use std::sync::Arc;

use bevy::asset::io::AssetSourceId;
use bevy::camera::Exposure;
use bevy::camera_controller::free_camera::{FreeCamera, FreeCameraPlugin};
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use mw_assets::MwAssetsPlugin;
use sf_anim_bevy::{AnimGizmos, AnimRootMotion, Animator};
use sf_character::{CharacterSource, CharacterState, Readiness};
use sf_character_bevy::{Character, CharacterBody, CharacterReadiness, CharacterContent, CharacterStats, OutfitCache};
use sf_records::{RecordDb, RecordId};
use sf_vfs::Vfs;

/// The ROADMAP M3 lineup: Imperial guard, Khajiit, Argonian, female dark
/// elf, naked Fargoth, guar, cliff racer.
pub const DEFAULT_LINEUP: [&str; 7] = ["@imperial,guard", "ajira", "huleeya", "@dark elf,,f", "fargoth:naked", "guar", "cliff racer"];

pub struct CharacterViewOptions {
    pub ids: Vec<String>,
    pub screenshot: Option<PathBuf>,
    pub settle: f32,
    pub camera: Option<(Vec3, Vec3)>,
    pub bodies: sf_character_bevy::SmoothBodies,
    /// `--cloth`: robes and skirts authored (default) or simulated (opt-in).
    pub cloth: mw_physics::ClothMode,
    /// Start walking in place (1 toggles).
    pub walk: bool,
    /// Weapons drawn from the start (R toggles).
    pub drawn: bool,
    /// Night from the start (N toggles).
    pub night: bool,
}

/// One lineup entry, resolved.
struct Entry {
    kind: &'static str,
    id: RecordId,
    /// `Some`: these items instead of the NPC's own (empty: naked).
    worn: Option<Vec<RecordId>>,
}

fn resolve(db: &RecordDb, source: &dyn CharacterSource, spec: &str) -> Option<Entry> {
    use tes3::esp::{Creature, Npc};
    // `spec+item+item`: wear exactly these items.
    let (spec, items) = match spec.split_once('+') {
        Some((base, items)) => (base, Some(items.split('+').map(|i| RecordId::new(i.trim())).collect::<Vec<_>>())),
        None => (spec, None),
    };
    // An unknown item is dropped by the recipe without a word: say so.
    for item in items.iter().flatten() {
        use tes3::esp::{Armor, Clothing, Light, Weapon};
        let known = db.get::<Armor>(item).is_some()
            || db.get::<Clothing>(item).is_some()
            || db.get::<Weapon>(item).is_some()
            || db.get::<Light>(item).is_some();
        if !known {
            eprintln!("warning: {spec}: unknown item {:?} (no armor, clothing, weapon or light record): skipped", item.as_str());
        }
    }
    if let Some(query) = spec.strip_prefix('@') {
        let mut q = query.split(',').map(str::trim);
        let (race, class, female) = (q.next().unwrap_or(""), q.next().unwrap_or(""), q.next() == Some("f"));
        let mut npcs: Vec<(&RecordId, &Npc)> = db.iter::<Npc>().collect();
        npcs.sort_by(|a, b| a.0.as_str().cmp(b.0.as_str()));
        let found = npcs.into_iter().find(|(id, n)| {
            (race.is_empty() || n.race.eq_ignore_ascii_case(race))
                && (class.is_empty() || n.class.eq_ignore_ascii_case(class))
                && (!female
                    || source
                        .recipe("NPC_", id, &CharacterState::default())
                        .is_ok_and(|r| r.skeleton.as_str() == mw_actors::skeleton::FEMALE))
        })?;
        return Some(Entry { kind: "NPC_", id: found.0.clone(), worn: items });
    }
    let (id, naked) = spec.strip_suffix(":naked").map_or((spec, false), |id| (id, true));
    let id = RecordId::new(id);
    if db.get::<Npc>(&id).is_some() {
        Some(Entry { kind: "NPC_", id, worn: if naked { Some(Vec::new()) } else { items } })
    } else if db.get::<Creature>(&id).is_some() {
        Some(Entry { kind: "CREA", id, worn: None })
    } else {
        None
    }
}

#[derive(Resource)]
struct Capture {
    path: PathBuf,
    settle: f32,
    ready_at: Option<f32>,
}

#[derive(Resource)]
struct Lineup(usize);

/// Marks lineup roots.
#[derive(Component)]
struct LineupEntry {
    creature: bool,
    reported: bool,
}

pub fn run(vfs: Arc<Vfs>, db: Arc<RecordDb>, store: sf_bake::Store, manifest: sf_bake::Manifest, options: CharacterViewOptions) {
    let content = Arc::new(crate::actors::MwCharacters::new(Arc::clone(&db), Arc::clone(&vfs), store, manifest, options.cloth));
    let specs: Vec<String> =
        if options.ids.is_empty() { DEFAULT_LINEUP.iter().map(|s| (*s).to_owned()).collect() } else { options.ids.clone() };
    let mut entries = Vec::new();
    for spec in &specs {
        match resolve(&db, content.source(), spec) {
            Some(e) => {
                println!("{spec}: {} {}", e.kind, e.id.as_str());
                entries.push(e);
            }
            None => eprintln!("{spec}: no NPC_ or CREA record"),
        }
    }

    let mut app = App::new();
    app.register_asset_source(AssetSourceId::Default, sf_vfs::bevy::asset_source(vfs.clone()));
    // DLSS builds: the project id must exist before `DefaultPlugins`.
    crate::render::RenderSettings::init(&mut app);
    crate::shipped_errors(&mut app);
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window { title: "Bevyrrowind - characters".into(), ..default() }),
        ..default()
    }))
    .add_plugins((MwAssetsPlugin { vfs }, FreeCameraPlugin, sf_anim_bevy::SfAnimPlugin))
    // Torch shadows on the stage (the exterior's High budget).
    .add_plugins(sf_sky_bevy::PointShadowPlugin)
    .insert_resource(crate::render::RenderSettings::default().point_shadow_budget(crate::render::POINT_SHADOWS_DEFAULT))
    .insert_resource(AnimGizmos { enabled: false })
    .insert_resource(mw_assets::LightCalibration::default().ambient([150, 150, 160]))
    .insert_resource(Lineup(entries.len()))
    .insert_resource(Gait { walk: options.walk, ..Gait::default() })
    .add_systems(Update, (keys, gait, report, night));
    crate::actors::add_content(&mut app, content);
    app.insert_resource(crate::held::HeldDarkness::forced(Some(options.night)));
    let drawn = if options.drawn { Readiness::Drawn } else { Readiness::Sheathed };
    app.insert_resource(sf_character_bevy::CharacterSettings { bodies: options.bodies });
    let camera = options.camera;
    app.add_systems(Startup, move |mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>| {
        stage(&mut commands, &mut meshes, &mut materials, &entries, camera, drawn);
    });
    if let Some(path) = options.screenshot {
        app.insert_resource(Capture { path, settle: options.settle, ready_at: None }).add_systems(Update, capture);
    } else {
        // Interactive: BRP for `brp_extras/send_keys` and screenshots (CLAUDE.md "Live inspection (BRP)").
        #[cfg(feature = "remote")]
        crate::remote::add(&mut app);
    }
    app.run();
}

fn stage(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    entries: &[Entry],
    camera: Option<(Vec3, Vec3)>,
    readiness: Readiness,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(40.0, 40.0))),
        MeshMaterial3d(materials.add(StandardMaterial { base_color: Color::srgb(0.35, 0.33, 0.3), perceptual_roughness: 0.9, ..default() })),
    ));
    commands.spawn((
        DirectionalLight { illuminance: 600.0, shadow_maps_enabled: true, ..default() },
        Transform::from_xyz(3.0, 6.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    // Characters face +Z (the framework convention), towards the camera.
    let mut x = 0.0;
    let mut xs = Vec::with_capacity(entries.len());
    for e in entries {
        let width = if e.kind == "CREA" { 2.6 } else { 1.1 };
        xs.push(x + width / 2.0);
        x += width;
    }
    let center = x / 2.0;
    for (e, x) in entries.iter().zip(xs) {
        let state = CharacterState { worn: e.worn.clone(), held: None };
        commands.spawn((
            Character::Record { kind: e.kind.into(), id: e.id.clone(), state },
            Transform::from_xyz(x - center, 0.0, 0.0),
            Name::new(e.id.as_str().to_owned()),
            CharacterReadiness(readiness),
            LineupEntry { creature: e.kind == "CREA", reported: false },
        ));
    }
    let (eye, target) = camera.unwrap_or((Vec3::new(0.0, 1.6, center.max(2.0) * 1.1 + 2.5), Vec3::new(0.0, 0.95, 0.0)));
    commands.spawn((Camera3d::default(), Exposure { ev100: mw_assets::SCENE_EV100 }, FreeCamera::default(), Transform::from_translation(eye).looking_at(target, Vec3::Y)));
}

fn keys(
    input: Res<ButtonInput<KeyCode>>,
    mut chars: Query<(&mut Character, &mut LineupEntry, &mut CharacterReadiness)>,
    mut animators: Query<&mut Animator>,
    mut gizmos: ResMut<AnimGizmos>,
    mut darkness: ResMut<crate::held::HeldDarkness>,
    mut fidget: Local<u8>,
    mut dead: Local<bool>,
) {
    if input.just_pressed(KeyCode::KeyR) {
        for (_, _, mut readiness) in &mut chars {
            readiness.0 = if readiness.0 == Readiness::Drawn { Readiness::Sheathed } else { Readiness::Drawn };
            info!("weapons (R): {:?}", readiness.0);
        }
    }
    if input.just_pressed(KeyCode::KeyN) {
        darkness.forced = Some(!darkness.forced.unwrap_or(false));
        info!("night (N): {:?}", darkness.forced);
    }
    if input.just_pressed(KeyCode::KeyO) {
        for (mut c, mut entry, _) in &mut chars {
            if entry.creature {
                continue;
            }
            if let Character::Record { state, .. } = &mut *c {
                state.worn = if state.worn.is_some() { None } else { Some(Vec::new()) };
                info!("outfit swap: {}", if state.worn.is_some() { "naked" } else { "own equipment" });
            }
            entry.reported = false;
        }
    }
    if input.just_pressed(KeyCode::KeyF) {
        // The idle variants a fidget plays, in turn (`Idle2`-`Idle4`).
        *fidget = if (2..4).contains(&*fidget) { *fidget + 1 } else { 2 };
        info!("fidget (F): Idle{}", *fidget);
        for mut a in &mut animators {
            a.set_float("idle_variant", f32::from(*fidget));
            a.fire("fidget");
        }
    }
    if input.just_pressed(KeyCode::KeyX) {
        *dead = !*dead;
        info!("dead (X): {}", *dead);
        for mut a in &mut animators {
            a.set_bool("dead", *dead);
        }
    }
    if input.just_pressed(KeyCode::KeyG) {
        gizmos.enabled = !gizmos.enabled;
    }
}

/// N: the stage at night (moonlight and a dim ambient) or by day.
fn night(
    darkness: Res<crate::held::HeldDarkness>,
    mut shown: Local<Option<bool>>,
    mut sun: Query<&mut DirectionalLight>,
    mut ambient: ResMut<GlobalAmbientLight>,
) {
    let dark = darkness.forced.unwrap_or(false);
    if *shown == Some(dark) {
        return;
    }
    *shown = Some(dark);
    for mut s in &mut sun {
        s.illuminance = if dark { 8.0 } else { 600.0 };
    }
    let [r, g, b] = if dark { [18, 20, 32] } else { [150, 150, 160] };
    *ambient = mw_assets::LightCalibration::default().ambient([r, g, b]);
}

/// Logs each character's meshes once it is dressed (and after each swap).
fn report(mut chars: Query<(&Name, &CharacterBody, &mut LineupEntry), Changed<CharacterBody>>, stats: Res<CharacterStats>, cache: Res<OutfitCache>) {
    for (name, body, mut entry) in &mut chars {
        if entry.reported {
            continue;
        }
        entry.reported = true;
        let morphing = body.outfit.meshes.iter().filter(|m| !m.morph_weights.is_empty()).count();
        info!(
            "{name}: {} skinned meshes ({} merged per material + {morphing} morphing), {} palette joints, skeleton {}",
            body.meshes.len(),
            body.meshes.len() - morphing,
            body.outfit.joints.len(),
            body.skeleton
        );
    }
    if stats.is_changed() {
        info!(
            "characters: {} dressed, {} swaps, {} failed, {} cached outfits (hits {}, misses {}), {} meshes, {} missing parts, {} missing joints",
            stats.spawned,
            stats.swaps,
            stats.failed,
            cache.len(),
            stats.cache_hits,
            stats.cache_misses,
            stats.meshes,
            stats.missing_parts,
            stats.missing_joints
        );
        if stats.smoothed > 0 {
            #[allow(clippy::cast_precision_loss, reason = "a log average")]
            let mean = stats.smooth_ms / stats.smoothed as f32;
            info!(
                "smooth bodies: {} outfits, {} parts rewritten, guard reverted {} seams, {mean:.1} ms per outfit on the build tasks (indicative)",
                stats.smoothed, stats.smoothed_parts, stats.smooth_reverted
            );
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn capture(
    mut commands: Commands,
    mut capture: ResMut<Capture>,
    time: Res<Time>,
    stats: Res<CharacterStats>,
    lineup: Res<Lineup>,
    server: Res<AssetServer>,
    images: Query<&MeshMaterial3d<StandardMaterial>>,
    materials: Res<Assets<StandardMaterial>>,
    mut exit: MessageWriter<AppExit>,
) {
    let now = time.elapsed_secs();
    match capture.ready_at {
        None => {
            let textures_pending = images.iter().filter_map(|m| materials.get(&m.0)).filter_map(|m| m.base_color_texture.as_ref()).any(|t| {
                !matches!(server.get_load_state(t.id()), Some(bevy::asset::LoadState::Loaded | bevy::asset::LoadState::Failed(_)) | None)
            });
            if stats.spawned + stats.failed >= lineup.0 && !textures_pending {
                capture.ready_at = Some(now);
            } else if now > 120.0 {
                warn!("characters still loading after 120 s; capturing anyway");
                capture.ready_at = Some(now);
            }
        }
        Some(t) if now - t >= capture.settle => {
            commands.spawn(Screenshot::primary_window()).observe(save_to_disk(capture.path.clone()));
            capture.settle = f32::INFINITY;
        }
        Some(_) if capture.settle.is_infinite() && capture.path.exists() => {
            exit.write(AppExit::Success);
        }
        _ => {}
    }
}

/// Walking in place (1; 2 runs): the lineup's limbs bend through the
/// locomotion cycle without leaving the stage. Every actor follows one
/// rule (animation.md "Root motion"): locomotion travel is extracted at
/// bake time onto the root-motion track (`Bip01`, else `Root Bone`), which
/// is stored on `AnimRootMotion` but not applied, so the line stays put.
/// 3 applies it (`AnimRootMotion::apply`): each actor walks off at its
/// clip's authored speed; 3 again puts everyone back on their mark.
#[derive(Resource, Clone, Copy, Default)]
struct Gait {
    walk: bool,
    run: bool,
    travel: bool,
}

/// An actor's mark on the stage, restored when root motion stops applying.
#[derive(Component, Clone, Copy)]
struct Mark(Transform);

type GaitQuery<'a> = (Entity, &'a mut Animator, Option<&'a mut AnimRootMotion>, Option<&'a mut Transform>, Option<&'a Mark>);

fn gait(input: Res<ButtonInput<KeyCode>>, mut gait: ResMut<Gait>, mut animators: Query<GaitQuery>, mut commands: Commands) {
    if input.just_pressed(KeyCode::Digit1) {
        gait.walk = !gait.walk;
        info!("walk in place (1): {}", gait.walk);
    }
    if input.just_pressed(KeyCode::Digit2) {
        gait.run = !gait.run;
        info!("run (2): {}", gait.run);
    }
    let toggled = input.just_pressed(KeyCode::Digit3);
    if toggled {
        gait.travel = !gait.travel;
        info!("apply root motion (3): {}", gait.travel);
    }
    for (entity, mut a, root, transform, mark) in &mut animators {
        a.set_bool("moving", gait.walk);
        a.set_vec2("move", Vec2::Y);
        a.set_bool("running", gait.run);
        // A treadmill: the graph is told the cycle's own authored speed, so
        // `SpeedMatch` plays it at its natural cadence (no ground samples:
        // foot placement stays out and the feet play the clip).
        let cycle = if gait.run { "RunForward" } else { "WalkForward" };
        let speed = if gait.walk { cycle_speed(&a, cycle) } else { 0.0 };
        a.drive.velocity = Vec3::Z * speed;
        if let Some(mut root) = root {
            root.apply = gait.travel;
        }
        match (transform, mark) {
            (Some(t), None) => {
                commands.entity(entity).insert(Mark(*t));
            }
            (Some(mut t), Some(mark)) if toggled && !gait.travel => *t = mark.0,
            _ => {}
        }
    }
}

/// Authored ground speed of a graph clip (its root motion), m/s.
fn cycle_speed(a: &Animator, reference: &str) -> f32 {
    let asset = &a.asset;
    asset
        .graph
        .clips
        .iter()
        .position(|c| c.reference.eq_ignore_ascii_case(reference))
        .and_then(|i| asset.clips.get(i))
        .map_or(0.0, |c| Vec2::new(c.meta.average_velocity.x, c.meta.average_velocity.z).length())
}
