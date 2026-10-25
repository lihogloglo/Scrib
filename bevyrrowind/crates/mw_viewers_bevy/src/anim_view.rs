//! `view --model <nif> --skinned --animate [--skeleton <nif>]`: a cooked
//! skinned model on an `sf_anim_bevy` rig, playing its baked graph
//! (`mw_bake::anim::AnimLibrary`: skeleton, compiled graph, clips).
//! Creatures use their own NIF as skeleton (`mw_creature` graph); body parts
//! take `--skeleton meshes/base_anim.nif` (`mw_humanoid`).
//!
//! Keys (the graph parameters; `mw_graphs::params`): arrows hold `moving`
//! with `move` = the arrow direction, R `running`, C `sneaking`, Z
//! `swimming`, N cycles `stance` 0-3, Q/E hold `turn` -1/+1, F `fidget`,
//! H `hit`, J `knockdown`, X `dead`; G skeleton gizmos, L cycles the LOD
//! tier (auto, A0-A3). Events (footsteps, attack windows) are logged.
//!
//! Until `sf_character_bevy` assembles dressed actors this is the viewer
//! path for animated actors; it replaces `skinned_view`'s NIF skeleton.

use std::collections::HashMap;
use std::sync::Arc;

use bevy::prelude::*;
use mw_assets::SharedVfs;
use mw_bake::anim::AnimLibrary;
use sf_anim_bevy::{AnimAsset, AnimEvent, AnimGizmos, AnimLod, AnimRootMotion, AnimTier, Animator, SkeletonBinding, spawn_rig};
use sf_bake::{Manifest, Store};
use sf_model_bevy::{CookedModelAsset, resolve_joints, spawn_skinned};

pub struct AnimViewPlugin;

impl Plugin for AnimViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(sf_anim_bevy::SfAnimPlugin)
            .insert_resource(AnimGizmos { enabled: true })
            .init_resource::<Drive>()
            .add_systems(Startup, gizmos_on_top)
            .add_systems(Update, (bind, keys, log_events));
    }
}

/// Skeleton lines draw over the meshes they sit inside.
fn gizmos_on_top(mut store: ResMut<GizmoConfigStore>) {
    store.config_mut::<DefaultGizmoConfigGroup>().0.depth_bias = -1.0;
}

/// The baked animation library and the assets loaded from it, by skeleton.
#[derive(Resource)]
pub struct AnimViews {
    library: AnimLibrary,
    assets: HashMap<String, Result<Arc<AnimAsset>, String>>,
}

impl AnimViews {
    pub fn new(store: Store, manifest: Manifest) -> Self {
        Self { library: AnimLibrary::new(store, manifest), assets: HashMap::new() }
    }

    /// The third-person graph baked for `skeleton` (first-person graphs and
    /// view models are left for `sf_character_bevy`).
    fn asset(&mut self, skeleton: &str) -> Result<Arc<AnimAsset>, String> {
        let library = &self.library;
        self.assets
            .entry(skeleton.to_owned())
            .or_insert_with(|| {
                let graphs = library.graphs(skeleton);
                let (graph, subject) = graphs
                    .iter()
                    .find(|(g, _)| !g.contains("first_person"))
                    .ok_or_else(|| format!("no graph baked for {skeleton} (run `bevyrrowind bake`; baked: {graphs:?})"))?;
                let a = library.load(subject, graph).map_err(|e| e.to_string())?;
                info!("anim view: {skeleton}: {graph}, {} joints, {} clips", a.skeleton.len(), a.clips.len());
                AnimAsset::new(a.skeleton, a.graph, a.clips).map(Arc::new).map_err(|e| e.to_string())
            })
            .clone()
    }
}

#[derive(Component)]
struct AnimViewRoot {
    model: Handle<CookedModelAsset>,
}

/// Spawns the root (with `bundle`), the rig of `skeleton` (default: the
/// model itself) with its graph, and starts loading the cooked model.
pub fn spawn(commands: &mut Commands, assets: &AssetServer, views: &mut AnimViews, model: &str, skeleton: Option<&str>, bundle: impl Bundle) -> Entity {
    let root = commands.spawn(bundle).id();
    let skeleton = skeleton.unwrap_or(model);
    match views.asset(skeleton) {
        Ok(asset) => {
            let model = mw_assets::cooked::load(assets, model, None);
            spawn_rig(commands, root, &asset.skeleton);
            let lod = AnimLod::new(asset.rest_height());
            commands.entity(root).insert((Animator::new(asset), lod, AnimRootMotion::default(), AnimViewRoot { model }));
        }
        Err(e) => warn!("anim view: {e}"),
    }
    root
}

#[allow(clippy::too_many_arguments)]
fn bind(
    mut commands: Commands,
    roots: Query<(Entity, &AnimViewRoot, &SkeletonBinding, &Animator)>,
    server: Res<AssetServer>,
    models: Res<Assets<CookedModelAsset>>,
    vfs: Res<SharedVfs>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (root, view, binding, animator) in &roots {
        let Some(asset) = models.get(&view.model) else { continue };
        commands.entity(root).remove::<AnimViewRoot>();
        let Some(skin) = &asset.skin else {
            warn!("anim view: the cooked product is not skinned ({:?})", asset.skipped);
            continue;
        };
        let joints = match resolve_joints(skin, |name| binding.find(&animator.asset.skeleton, name)) {
            Ok(joints) => joints,
            Err(err) => {
                warn!("anim view: {err} (body parts need --skeleton meshes/base_anim.nif)");
                continue;
            }
        };
        let mats: Vec<Handle<StandardMaterial>> =
            asset.materials.iter().map(|m| materials.add(mw_assets::cooked::cooked_material(m, &vfs.0, &server))).collect();
        let spawned = spawn_skinned(&mut commands, root, asset, 0, &joints, |entity, material| {
            if let Some(handle) = mats.get(material as usize) {
                entity.insert(MeshMaterial3d(handle.clone()));
            }
        });
        info!("anim view: {} meshes bound to {} joints", spawned.len(), joints.len());
    }
}

/// Toggled parameters and the forced tier.
#[allow(clippy::struct_excessive_bools)] // one toggle per key
#[derive(Resource, Default)]
struct Drive {
    running: bool,
    sneaking: bool,
    swimming: bool,
    dead: bool,
    stance: u8,
    lod: Option<AnimTier>,
}

fn keys(input: Res<ButtonInput<KeyCode>>, mut drive: ResMut<Drive>, mut gizmos: ResMut<AnimGizmos>, mut chars: Query<(&mut Animator, &mut AnimLod)>) {
    let axis = |pos: KeyCode, neg: KeyCode| f32::from(u8::from(input.pressed(pos))) - f32::from(u8::from(input.pressed(neg)));
    let dir = Vec2::new(axis(KeyCode::ArrowRight, KeyCode::ArrowLeft), axis(KeyCode::ArrowUp, KeyCode::ArrowDown));
    let turn = axis(KeyCode::KeyE, KeyCode::KeyQ);
    let mut changed = false;
    let d = &mut *drive;
    for (key, flag) in [(KeyCode::KeyR, &mut d.running), (KeyCode::KeyC, &mut d.sneaking), (KeyCode::KeyZ, &mut d.swimming), (KeyCode::KeyX, &mut d.dead)] {
        if input.just_pressed(key) {
            *flag = !*flag;
            changed = true;
        }
    }
    if input.just_pressed(KeyCode::KeyN) {
        drive.stance = (drive.stance + 1) % 4;
        changed = true;
    }
    if input.just_pressed(KeyCode::KeyL) {
        drive.lod = match drive.lod {
            None => Some(AnimTier::A0),
            Some(AnimTier::A3) => None,
            Some(t) => Some(t.demoted()),
        };
        changed = true;
    }
    if input.just_pressed(KeyCode::KeyG) {
        gizmos.enabled = !gizmos.enabled;
    }
    if changed {
        info!(
            "anim view: running {} sneaking {} swimming {} dead {} stance {} lod {:?}",
            drive.running, drive.sneaking, drive.swimming, drive.dead, drive.stance, drive.lod
        );
    }
    for (mut a, mut lod) in &mut chars {
        let moving = dir != Vec2::ZERO;
        a.set_bool("moving", moving);
        if moving {
            a.set_vec2("move", dir.normalize());
        }
        a.set_float("turn", turn);
        a.set_bool("running", drive.running);
        a.set_bool("sneaking", drive.sneaking);
        a.set_bool("swimming", drive.swimming);
        a.set_bool("dead", drive.dead);
        a.set_float("stance", f32::from(drive.stance));
        for (key, trigger) in [(KeyCode::KeyF, "fidget"), (KeyCode::KeyH, "hit"), (KeyCode::KeyJ, "knockdown")] {
            if input.just_pressed(key) {
                a.fire(trigger);
            }
        }
        lod.forced = drive.lod;
    }
}

fn log_events(mut events: MessageReader<AnimEvent>, names: Query<&Name>) {
    for e in events.read() {
        info!("anim event {:?} {} on {} (weight {:.2})", e.edge, e.name, names.get(e.entity).map_or("?", Name::as_str), e.weight);
    }
}
