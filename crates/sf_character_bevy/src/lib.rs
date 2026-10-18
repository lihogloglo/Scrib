//! Bevy glue for `sf_character` (character.md "Assembly at runtime"):
//! characters spawned from recipes, dressed with per-material merged
//! outfits on an `sf_anim_bevy` rig.
//!
//! A [`Character`] component (a base record plus its runtime state, or a
//! ready recipe) is all a caller spawns. Then, per character:
//!
//! 1. **Off the main thread** (`AsyncComputeTaskPool`): the adapter's
//!    [`CharacterContent`] resolves the recipe, the outfit (slots, layers,
//!    hides: `sf_character::resolve_recipe`) and its key. If the outfit is
//!    resident in the [`OutfitCache`] the task stops there; otherwise it
//!    loads each part's cooked model (rigid parts become one-joint skins,
//!    mirrored ones already flipped in the data), smooths them as one
//!    segmented set unless [`CharacterSettings::bodies`] is
//!    [`SmoothBodies::Vanilla`] (`sf_character::smooth_outfit`: seams
//!    welded, weights blended, a per-seam guard over the content's
//!    [`CharacterContent::smooth_probes`]; heads with morphs and hair stay
//!    as authored), and merges them per
//!    material with `sf_character::assemble` (morph-bearing parts kept
//!    apart, shape morphs baked). The animation asset (skeleton, compiled
//!    graph, clips) loads there too.
//! 2. **On the main thread**: a cache miss uploads the meshes (one `Mesh`
//!    per merged material, one per morph-bearing part; one shared inverse
//!    bindpose asset) and materials; the first build spawns the body: a
//!    child entity with the race scale, the rig (`spawn_rig`), the
//!    `Animator` playing the recipe's graph, then one `SkinnedMesh` entity
//!    per outfit mesh sharing the rig's joint entities, with
//!    `DynamicSkinnedMeshBounds`.
//! 3. **Outfit swaps** (the component changes): the same steps; the old
//!    mesh entities stay until the new outfit is uploaded and settled, then
//!    both change in one command flush, so nothing pops. The rig and the
//!    animator survive a swap on the same skeleton.
//! 4. **Held items** (module `held`): each recipe item becomes a rigid prop
//!    on its socket joint, mounted by [`CharacterReadiness`]. A change of
//!    held items alone (same outfit) keeps the outfit meshes.
//!
//! Identical looks share one cache entry (key: skeleton, outfit key, LOD,
//! shape weights), so a hundred guards are one set of meshes. Entries no
//! character uses are evicted past [`OutfitCache::capacity`].
//!
//! Game-neutral: everything the adapter knows (records, cooked products,
//! baked animation, materials, its facing convention) comes through
//! [`CharacterContent`]. With feature `world`, [`CharacterSpawner`] spawns
//! actor placements of `sf_world_bevy` streaming as characters.

mod cloth;
mod held;
mod mesh;
mod morph;
#[cfg(test)]
mod tests;
#[cfg(feature = "world")]
mod world;

pub use cloth::{ClothJoint, ClothSettings, ClothStats};
pub use sf_anim::cloth::ClothCollider;
pub use held::{CharacterReadiness, HeldLightSlot, HeldModelAssets, HeldModelCache, HeldProp, HeldProps, HeldSkinAssets};
pub use mesh::{OutfitMeshError, assembled_mesh_to_mesh, outfit_inverse_bindposes};
#[cfg(feature = "world")]
pub use world::CharacterSpawner;
pub use morph::{CharacterMorphs, MorphCurvePlayer, MorphTrack, MorphTracks, drive_character_morphs};

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, PoisonError, RwLock};

use bevy_app::{App, Plugin, PostUpdate, Update};
use bevy_asset::{AssetServer, Assets, Handle, RenderAssetUsages};
use bevy_camera::visibility::{DynamicSkinnedMeshBounds, RenderLayers, Visibility};
use bevy_ecs::prelude::*;
use bevy_math::{Affine3A, Mat4, Quat, Vec3};
use bevy_mesh::morph::MeshMorphWeights;
use bevy_mesh::skinning::{SkinnedMesh, SkinnedMeshInverseBindposes};
use bevy_mesh::{Mesh, Mesh3d};
use bevy_pbr::{MeshMaterial3d, StandardMaterial};
use bevy_tasks::{AsyncComputeTaskPool, Task};
use bevy_transform::components::Transform;
use futures_lite::future;
use sf_anim_bevy::{AnimAsset, AnimLod, AnimRootMotion, AnimSystems, Animator, SkeletonBinding, spawn_rig};
use sf_character::{
    AssembledOutfit, AssemblyPart, BodyPart, CharacterRecipe, CharacterSource, CharacterState, OutfitKey, OutfitSmoothReport, OutfitSmoothing, PartKind,
    PartLookup, SlotRegistry, SmoothRig, View,
    assemble, resolve_recipe, smooth_outfit,
};
use sf_model::{Cooked, CookedSkinned};
use sf_records::RecordId;
use sf_vfs::VfsPath;

/// What the content pack provides (Bevy-free data, plus the material
/// mapping). Implemented by adapters next to their `CharacterSource`.
pub trait CharacterContent: Send + Sync + 'static {
    /// Base records → recipes.
    fn source(&self) -> &dyn CharacterSource;
    /// The part and wearable records recipes name.
    fn parts(&self) -> &dyn PartLookup;
    fn slots(&self) -> &SlotRegistry;
    /// A model's cooked product (`mesh.cook`), decoded.
    fn cooked(&self, model: &VfsPath) -> Result<Cooked, String>;
    /// The recipe's third-person animation: skeleton, compiled graph, clips.
    fn animation(&self, recipe: &CharacterRecipe) -> Result<AnimAsset, String>;
    /// Model-space rest transforms of named nodes, in order (`None`: no such
    /// node): the joints of a skinned held model (a bow and its string).
    /// Default: [`Self::node_position`] as translations.
    fn node_transforms(&self, model: &VfsPath, nodes: &[&str]) -> Vec<Option<Mat4>> {
        nodes.iter().map(|n| self.node_position(model, n).map(Mat4::from_translation)).collect()
    }
    /// A held item's model (`HeldItem::model`): `Cooked::Static` (a rigid
    /// prop) or `Cooked::Skinned` (a prop on its own small skeleton, posed
    /// at rest: a bow). Default: [`Self::cooked`].
    fn held_model(&self, model: &VfsPath) -> Result<Cooked, String> {
        self.cooked(model)
    }
    /// Model-space position of a named node (cooks flatten nodes away):
    /// where a held item's light sits (`HeldLight::attach_node`). `None`
    /// (the default): the model's bounds center.
    fn node_position(&self, model: &VfsPath, node: &str) -> Option<Vec3> {
        let _ = (model, node);
        None
    }
    /// A cooked material as a Bevy material (textures through `assets`).
    fn material(&self, material: &sf_model::Material, assets: &AssetServer) -> StandardMaterial {
        let _ = (material, assets);
        StandardMaterial::default()
    }
    /// Turn added to a placement's rotation by [`CharacterSpawner`] when
    /// the content's placed actors face another way than its skeletons.
    fn placement_rotation(&self) -> Quat {
        Quat::IDENTITY
    }
    /// Smooth bodies' probe poses for a skeleton: model-space joint
    /// transforms (bent elbows and knees, raised shoulders, locomotion
    /// frames), one `Vec` per pose, in `animation.skeleton`'s joint order.
    /// The seam guard checks every smoothed seam in them (never wider than
    /// the authored parts). Empty (the default): outfits stay as authored.
    fn smooth_probes(&self, recipe: &CharacterRecipe, animation: &AnimAsset) -> Vec<Vec<Affine3A>> {
        let _ = (recipe, animation);
        Vec::new()
    }
    /// Cloth (physics.md "Cloth"): the setup of an outfit part that hangs
    /// as cloth (robes, skirts; the adapter's `ClothRules`), `None` (the
    /// default) for the rest.
    fn cloth_spec(&self, id: &RecordId, part: &BodyPart) -> Option<sf_character::ClothSpec> {
        let _ = (id, part);
        None
    }
    /// The body capsules a cloth hanging from height `below` (rest pose,
    /// model space) collides with on the recipe's skeleton (from its
    /// ragdoll: `RagdollDef::cloth_colliders`).
    fn cloth_colliders(&self, recipe: &CharacterRecipe, animation: &AnimAsset, below: f32) -> Vec<ClothCollider> {
        let _ = (recipe, animation, below);
        Vec::new()
    }
    /// How far a cloth's hem follows the authored leg skin instead of
    /// riding the girdle (`sf_anim_graph::ClothBinding::new`).
    fn cloth_leg_follow(&self) -> f32 {
        sf_anim_bevy::CLOTH_LEG_FOLLOW
    }
}

/// How segmented bodies are drawn (smooth-bodies.md; owner decision
/// 2026-10-12: smooth by default, vanilla as an option).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SmoothBodies {
    /// Seams welded and weights blended across joints (`sf_character::smooth_outfit`).
    #[default]
    Smooth,
    /// The parts as authored: rigid pieces, seams open when joints bend.
    Vanilla,
}

impl SmoothBodies {
    #[must_use]
    pub fn toggled(self) -> Self {
        match self {
            Self::Smooth => Self::Vanilla,
            Self::Vanilla => Self::Smooth,
        }
    }
}

/// Character look settings. Changing them rebuilds every character's
/// outfit through the no-pop swap (the rig and animator stay).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct CharacterSettings {
    pub bodies: SmoothBodies,
}

/// The merged-mesh cache key (character.md): equal keys look identical.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OutfitCacheKey {
    pub skeleton: String,
    pub outfit: OutfitKey,
    pub lod: u8,
    /// `ShapeWeights::key()`: 0 without sliders.
    pub shape: u64,
    /// Smooth and vanilla builds of one outfit are different meshes.
    pub bodies: SmoothBodies,
}

/// A character to spawn and dress. Changing it rebuilds the outfit (and
/// the rig only when the skeleton changes).
#[derive(Component, Clone, Debug, PartialEq)]
#[require(Transform, Visibility)]
pub enum Character {
    /// A base record (kind and id, as the `CharacterSource` takes them) and
    /// its runtime state (equipment overlay).
    Record { kind: Arc<str>, id: RecordId, state: CharacterState },
    /// A ready recipe.
    Recipe(Box<CharacterRecipe>),
}

impl Character {
    pub fn record(kind: impl Into<Arc<str>>, id: RecordId) -> Self {
        Self::Record { kind: kind.into(), id, state: CharacterState::default() }
    }
}

/// On a dressed character's root: its body (rig and animator) and the
/// outfit it shows.
#[derive(Component, Clone, Debug)]
pub struct CharacterBody {
    /// Child of the root: race scale, `SkeletonBinding`, `Animator`.
    pub body: Entity,
    pub skeleton: String,
    pub key: OutfitCacheKey,
    pub outfit: Arc<OutfitAssets>,
    /// The `SkinnedMesh` entities (children of `body`).
    pub meshes: Vec<Entity>,
    /// The cloth lattice joint entities (children of `body`).
    pub cloth_joints: Vec<Entity>,
}

/// One cached outfit's GPU assets.
#[derive(Debug)]
pub struct OutfitAssets {
    /// Palette joint names, `SkinnedMesh::joints` order.
    pub joints: Vec<String>,
    pub inverse_bindposes: Handle<SkinnedMeshInverseBindposes>,
    pub meshes: Vec<OutfitMesh>,
    /// The outfit's cloth parts (lattice joints `cloth<k>#<i>` in `joints`).
    pub cloth: Vec<Arc<sf_anim_bevy::ClothBinding>>,
    /// First frame the outfit may be shown (uploads settle first).
    ready_frame: u32,
}

#[derive(Debug)]
pub struct OutfitMesh {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    /// The cooked material's name (`sf_model::Material::name`): adapters
    /// find the meshes their material effects drive by it.
    pub material_name: String,
    /// Runtime morph target names (`MorphTarget::name`); empty for merged meshes.
    pub morph_targets: Vec<String>,
    /// Default morph weights, one per target.
    pub morph_weights: Vec<f32>,
}

/// Merged outfits by key, and materials by value.
#[derive(Resource)]
pub struct OutfitCache {
    entries: HashMap<OutfitCacheKey, Arc<OutfitAssets>>,
    materials: HashMap<String, Handle<StandardMaterial>>,
    /// Entries kept with no character using them.
    pub capacity: usize,
}

impl Default for OutfitCache {
    fn default() -> Self {
        Self { entries: HashMap::new(), materials: HashMap::new(), capacity: 256 }
    }
}

impl OutfitCache {
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// Counters (read by viewers and benches).
#[derive(Resource, Clone, Debug, Default)]
pub struct CharacterStats {
    /// Characters dressed (first build), and outfit swaps.
    pub spawned: usize,
    pub swaps: usize,
    /// Builds that failed (no recipe, no animation, assembly error).
    pub failed: usize,
    pub cache_hits: usize,
    pub cache_misses: usize,
    /// Parts skipped (no record, not cooked, not skinnable).
    pub missing_parts: usize,
    /// Palette joints the skeleton lacks (bound to its root joint).
    pub missing_joints: usize,
    /// Outfits assembled with smooth bodies, the parts the step rewrote,
    /// the seams its guard reverted, and the step's total time on the build
    /// tasks (ms, indicative: it shares the cores with the frame).
    pub smoothed: usize,
    pub smoothed_parts: usize,
    pub smooth_reverted: usize,
    pub smooth_ms: f32,
    /// `SkinnedMesh` entities alive.
    pub meshes: usize,
    /// Held item props alive, and held items skipped (model not cooked
    /// static, or no mount socket in the skeleton).
    pub held: usize,
    pub missing_held: usize,
}

/// Uploads settle this many frames before a new outfit replaces an old one
/// (Bevy prepares a new mesh in a later render frame than its spawn when the
/// per-frame upload budget is spent).
const SETTLE_FRAMES: u32 = 2;

/// Loaded animation assets by (skeleton, graph); errors are cached too.
type AnimationCache = HashMap<(String, String), Result<Arc<AnimAsset>, String>>;

/// The content and the Bevy-free caches the build tasks share.
struct Shared {
    content: Arc<dyn CharacterContent>,
    /// Cooked parts as skins, by part record (a rigid part's conversion
    /// depends on its joint and offset, so not by model).
    parts: Mutex<HashMap<RecordId, Option<Arc<CookedSkinned>>>>,
    /// Animation assets by (skeleton, graph).
    animations: Mutex<AnimationCache>,
    /// Smooth-body rigs (rest pose, parents, probe poses) by skeleton.
    rigs: Mutex<HashMap<String, Arc<SmoothRig>>>,
    /// Keys the main thread has in its [`OutfitCache`].
    resident: RwLock<HashSet<OutfitCacheKey>>,
    held: Mutex<HeldModels>,
    /// Cooked cloth parts by (skeleton, part record); `None`: the cook failed.
    cloth: Mutex<HashMap<(String, RecordId), Option<Arc<cloth::ClothPart>>>>,
}

/// Loaded held models by (model, light node); `None`: not usable.
type HeldModels = HashMap<(VfsPath, Option<String>), Option<Arc<held::HeldCooked>>>;

/// The content pack's characters: insert before spawning [`Character`]s.
#[derive(Resource, Clone)]
pub struct Characters {
    shared: Arc<Shared>,
}

impl Characters {
    pub fn new(content: Arc<dyn CharacterContent>) -> Self {
        Self {
            shared: Arc::new(Shared {
                content,
                parts: Mutex::default(),
                animations: Mutex::default(),
                rigs: Mutex::default(),
                resident: RwLock::default(),
                held: Mutex::default(),
                cloth: Mutex::default(),
            }),
        }
    }

    pub fn content(&self) -> &Arc<dyn CharacterContent> {
        &self.shared.content
    }
}

/// Adds the build and swap systems (and `SfAnimPlugin` when missing).
pub struct SfCharacterPlugin;

impl Plugin for SfCharacterPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<sf_anim_bevy::SfAnimPlugin>() {
            app.add_plugins(sf_anim_bevy::SfAnimPlugin);
        }
        app.init_resource::<OutfitCache>()
            .init_resource::<CharacterStats>()
            .init_resource::<Frame>()
            .init_resource::<CharacterSettings>()
            .init_resource::<HeldModelCache>()
            .init_resource::<ClothSettings>()
            .init_resource::<ClothStats>()
            .add_systems(PostUpdate, (cloth::apply_cloth_settings, cloth::attach_cloth).chain().before(AnimSystems::Evaluate))
            .add_systems(PostUpdate, cloth::write_cloth_joints.in_set(AnimSystems::Apply))
            .add_systems(
                Update,
                (apply_settings, request_builds, finish_builds, held::sync_held, held::apply_readiness).chain().in_set(CharacterSystems),
            )
            .add_systems(PostUpdate, drive_character_morphs.after(AnimSystems::Apply));
    }
}

/// The build and swap systems (`Update`).
#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CharacterSystems;

#[derive(Resource, Default)]
struct Frame(u32);

/// A build running for this character (dropping it cancels the build).
#[derive(Component)]
pub struct CharacterBuild(Task<Result<Built, String>>);

/// A finished build waiting for its outfit's uploads to settle.
#[derive(Component)]
struct PendingSwap(Built, Arc<OutfitAssets>);

struct Built {
    recipe: CharacterRecipe,
    animation: Arc<AnimAsset>,
    key: OutfitCacheKey,
    /// `None`: the outfit was resident when the task looked.
    outfit: Option<AssembledOutfit>,
    missing_parts: usize,
    /// The smooth-body step's report and time (ms), when it ran.
    smooth: Option<(OutfitSmoothReport, f32)>,
    /// `recipe.held`'s models, loaded.
    held: Vec<Option<Arc<held::HeldCooked>>>,
    /// The outfit's cloth parts, in palette order (`cloth<k>#<i>`).
    cloth: Vec<Arc<sf_anim_bevy::ClothBinding>>,
}

fn lock<T>(m: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    m.lock().unwrap_or_else(PoisonError::into_inner)
}

impl Shared {
    fn animation(&self, recipe: &CharacterRecipe) -> Result<Arc<AnimAsset>, String> {
        let key = (recipe.skeleton.as_str().to_owned(), recipe.graph.as_str().to_owned());
        if let Some(found) = lock(&self.animations).get(&key) {
            return found.clone();
        }
        // Loaded unlocked: two tasks may load the same asset once each.
        let loaded = self.content.animation(recipe).map(Arc::new);
        lock(&self.animations).entry(key).or_insert(loaded).clone()
    }

    fn part(&self, id: &RecordId, part: &BodyPart) -> Option<Arc<CookedSkinned>> {
        if let Some(found) = lock(&self.parts).get(id) {
            return found.clone();
        }
        let skinned = match self.content.cooked(&part.model) {
            Ok(cooked) => match (&part.kind, cooked) {
                (PartKind::Rigid { joint, offset }, c) => CookedSkinned::rigid(&c, joint, *offset).unwrap_or_else(|e| {
                    log::warn!("character part {id}: {e}");
                    None
                }),
                (PartKind::Skinned, Cooked::Skinned(s)) => Some(s),
                (PartKind::Skinned, other) => {
                    // A body animated per node needs its rig cook (`sf_model::cook_rig`).
                    let why = match other {
                        Cooked::Skipped { reason } => format!("skipped by the cook: {reason}"),
                        _ => "cooked static (no skin; the adapter should hand over its `cook_rig` product)".to_owned(),
                    };
                    log::warn!("character part {id}: {} is not a skin: {why}", part.model.as_str());
                    None
                }
            },
            Err(e) => {
                log::warn!("character part {id}: {e}");
                None
            }
        }
        .map(Arc::new);
        lock(&self.parts).entry(id.clone()).or_insert(skinned).clone()
    }

    /// A held item's model (cached by model and light node).
    fn held(&self, item: &sf_character::HeldItem) -> Option<Arc<held::HeldCooked>> {
        let key = (item.model.clone(), item.light.as_ref().and_then(|l| l.attach_node.clone()));
        if let Some(found) = lock(&self.held).get(&key) {
            return found.clone();
        }
        let loaded = held::load_held(self.content.as_ref(), item).map(Arc::new);
        lock(&self.held).entry(key).or_insert(loaded).clone()
    }

    /// The smooth-body rig of the recipe's skeleton (probe poses from the
    /// content), built once per skeleton.
    fn smooth_rig(&self, recipe: &CharacterRecipe, animation: &AnimAsset) -> Arc<SmoothRig> {
        let key = recipe.skeleton.as_str();
        if let Some(found) = lock(&self.rigs).get(key) {
            return Arc::clone(found);
        }
        let skeleton = &animation.skeleton;
        let posed = self.content.smooth_probes(recipe, animation);
        if posed.is_empty() {
            // Creatures and other rigs the adapter has no probes for.
            log::debug!("smooth bodies: no probe poses for {key}; its outfits stay as authored");
        }
        let rig = SmoothRig::new(skeleton.names().to_vec(), (0..skeleton.len()).map(|j| skeleton.parent(j)).collect(), skeleton.rest_model(), &posed);
        Arc::clone(lock(&self.rigs).entry(key.to_owned()).or_insert_with(|| Arc::new(rig)))
    }

    /// Step 1 of the module docs (runs on a task).
    fn build(&self, spec: &Character, bodies: SmoothBodies, force: bool) -> Result<Built, String> {
        let recipe = match spec {
            Character::Record { kind, id, state } => self.content.source().recipe(kind, id, state).map_err(|e| e.to_string())?,
            Character::Recipe(recipe) => (**recipe).clone(),
        };
        let animation = self.animation(&recipe)?;
        let parts = self.content.parts();
        let outfit = resolve_recipe(parts, self.content.slots(), &recipe, View::ThirdPerson).map_err(|e| e.to_string())?;
        let key = OutfitCacheKey { skeleton: recipe.skeleton.as_str().to_owned(), outfit: outfit.key, lod: 0, shape: recipe.shape.key(), bodies };
        let held = recipe.held.iter().map(|item| self.held(item)).collect();
        let resident = self.resident.read().unwrap_or_else(PoisonError::into_inner).contains(&key);
        if resident && !force {
            return Ok(Built { recipe, animation, key, outfit: None, missing_parts: 0, smooth: None, held, cloth: Vec::new() });
        }
        let mut missing_parts = 0;
        let mut owned: Vec<(Arc<CookedSkinned>, &BodyPart, bool, &RecordId)> = Vec::with_capacity(outfit.parts.len());
        for (_, id) in &outfit.parts {
            match parts.part(id).and_then(|part| Some((self.part(id, part)?, part))) {
                // Hair stays on its joint as authored (smooth-bodies.md).
                Some((cooked, part)) => owned.push((cooked, part, recipe.hair.as_ref() == Some(id), id)),
                None => missing_parts += 1,
            }
        }
        // Cloth parts (module `cloth`): rebound onto their lattice joints.
        let cloth_parts: Vec<Option<Arc<cloth::ClothPart>>> =
            owned.iter().map(|(cooked, part, _, id)| self.cloth_part(&recipe, &animation, id, cooked, part)).collect();
        let mut assembly: Vec<AssemblyPart<'_>> = owned.iter().map(|(cooked, part, _, _)| AssemblyPart::new(cooked, Some(*part))).collect();
        let lod = usize::from(key.lod);
        let mut smooth = None;
        let mut rewritten = if bodies == SmoothBodies::Smooth {
            let started = std::time::Instant::now();
            let rig = self.smooth_rig(&recipe, &animation);
            // Cloth parts keep their lattice skin (not smoothed yet: physics-adapter.md "Order with smooth bodies").
            let exclude: Vec<bool> = owned.iter().zip(&cloth_parts).map(|((_, _, hair, _), c)| *hair || c.is_some()).collect();
            let (rewritten, report) = smooth_outfit(&assembly, &exclude, lod, &rig, &OutfitSmoothing::default()).map_err(|e| e.to_string())?;
            smooth = Some((report, started.elapsed().as_secs_f32() * 1000.0));
            rewritten
        } else {
            vec![None; assembly.len()]
        };
        let mut cloth = Vec::new();
        for (slot, part) in rewritten.iter_mut().zip(&cloth_parts) {
            if let Some(part) = part {
                *slot = Some(cloth::renamed(part, cloth.len()));
                cloth.push(Arc::clone(&part.binding));
            }
        }
        for (part, decoded) in assembly.iter_mut().zip(&rewritten) {
            part.decoded = decoded.as_ref();
        }
        let assembled = assemble(&assembly, lod, &recipe.shape).map_err(|e| e.to_string())?;
        Ok(Built { recipe, animation, key, outfit: Some(assembled), missing_parts, smooth, held, cloth })
    }

    /// A cloth part's cook on the recipe's skeleton (cached; `None`: not cloth or the cook failed).
    fn cloth_part(&self, recipe: &CharacterRecipe, animation: &AnimAsset, id: &RecordId, cooked: &CookedSkinned, part: &BodyPart) -> Option<Arc<cloth::ClothPart>> {
        let spec = self.content.cloth_spec(id, part)?;
        let key = (recipe.skeleton.as_str().to_owned(), id.clone());
        if let Some(found) = lock(&self.cloth).get(&key) {
            return found.clone();
        }
        let made = match cloth::cook_part(self.content.as_ref(), recipe, animation, id, cooked, part, &spec) {
            Ok(p) => Some(Arc::new(p)),
            Err(e) => {
                log::warn!("cloth {id} on {}: {e}; drawn as authored", key.0);
                None
            }
        };
        lock(&self.cloth).entry(key).or_insert(made).clone()
    }
}

fn spawn_build(shared: &Arc<Shared>, spec: &Character, bodies: SmoothBodies, force: bool) -> CharacterBuild {
    let shared = Arc::clone(shared);
    let spec = spec.clone();
    CharacterBuild(AsyncComputeTaskPool::get().spawn(async move { shared.build(&spec, bodies, force) }))
}

/// A settings change rebuilds every character (through the no-pop swap).
fn apply_settings(settings: Res<CharacterSettings>, mut characters: Query<&mut Character>) {
    if settings.is_changed() && !settings.is_added() {
        for mut c in &mut characters {
            c.set_changed();
        }
    }
}

fn request_builds(
    mut commands: Commands,
    characters: Option<Res<Characters>>,
    settings: Res<CharacterSettings>,
    changed: Query<(Entity, &Character), Changed<Character>>,
) {
    let Some(characters) = characters else { return };
    for (entity, spec) in &changed {
        // Replaces (cancels) a running build and a pending swap.
        commands.entity(entity).remove::<PendingSwap>().insert(spawn_build(&characters.shared, spec, settings.bodies, false));
    }
}

/// Uploads an assembled outfit (step 2 of the module docs).
#[allow(clippy::too_many_arguments)]
fn upload(
    built: &AssembledOutfit,
    content: &dyn CharacterContent,
    cache: &mut OutfitCache,
    server: &AssetServer,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    inverse: &mut Assets<SkinnedMeshInverseBindposes>,
    frame: u32,
) -> Result<OutfitAssets, OutfitMeshError> {
    let inverse_bindposes = inverse.add(outfit_inverse_bindposes(built)?);
    let mut out = Vec::with_capacity(built.merged.len() + built.morphed.len());
    for m in built.meshes() {
        let mesh = meshes.add(assembled_mesh_to_mesh(m, built.has_colors, RenderAssetUsages::RENDER_WORLD, true)?);
        let material = cache
            .materials
            .entry(format!("{:?}", m.material))
            .or_insert_with(|| materials.add(content.material(&m.material, server)))
            .clone();
        out.push(OutfitMesh {
            mesh,
            material,
            material_name: m.material.name.clone(),
            morph_targets: m.morph_targets.iter().map(|t| t.name.clone()).collect(),
            morph_weights: m.morph_targets.iter().map(|t| t.default_weight).collect(),
        });
    }
    Ok(OutfitAssets {
        joints: built.joints.iter().map(|j| j.name.clone()).collect(),
        inverse_bindposes,
        meshes: out,
        cloth: Vec::new(),
        ready_frame: frame + SETTLE_FRAMES,
    })
}

/// Drops cache entries no character holds, beyond the capacity.
fn evict(cache: &mut OutfitCache, shared: &Shared) {
    if cache.entries.len() <= cache.capacity {
        return;
    }
    let unused: Vec<OutfitCacheKey> = cache.entries.iter().filter(|(_, a)| Arc::strong_count(a) == 1).map(|(k, _)| k.clone()).collect();
    let excess = cache.entries.len() - cache.capacity;
    let mut resident = shared.resident.write().unwrap_or_else(PoisonError::into_inner);
    for key in unused.into_iter().take(excess) {
        cache.entries.remove(&key);
        resident.remove(&key);
    }
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines, clippy::type_complexity)]
fn finish_builds(
    mut commands: Commands,
    characters: Option<Res<Characters>>,
    mut cache: ResMut<OutfitCache>,
    mut stats: ResMut<CharacterStats>,
    mut frame: ResMut<Frame>,
    server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut inverse: ResMut<Assets<SkinnedMeshInverseBindposes>>,
    mut building: Query<(Entity, &Character, &mut CharacterBuild)>,
    pending: Query<(Entity, &PendingSwap)>,
    mut roots: Query<(Option<&mut CharacterBody>, Option<&RenderLayers>)>,
    bindings: Query<&SkeletonBinding>,
) {
    frame.0 = frame.0.wrapping_add(1);
    let Some(characters) = characters else { return };
    let shared = &characters.shared;
    let mut ready: Vec<(Entity, Built, Arc<OutfitAssets>)> = Vec::new();
    for (entity, spec, mut build) in &mut building {
        let Some(result) = future::block_on(future::poll_once(&mut build.0)) else { continue };
        commands.entity(entity).remove::<CharacterBuild>();
        let built = match result {
            Ok(built) => built,
            Err(e) => {
                log::warn!("character {spec:?}: {e}");
                stats.failed += 1;
                continue;
            }
        };
        stats.missing_parts += built.missing_parts;
        if let Some((report, ms)) = &built.smooth {
            stats.smoothed += 1;
            stats.smoothed_parts += report.rewritten;
            stats.smooth_reverted += report.guard.reverted.len();
            stats.smooth_ms += ms;
            log::debug!(
                "smooth bodies {:?}: {} of {} parts rewritten, {} seams, {} welded, guard reverted {} seams, {ms:.1} ms",
                built.key.outfit,
                report.rewritten,
                report.parts,
                report.seams,
                report.welded,
                report.guard.reverted.len()
            );
        }
        let assets = if let Some(found) = cache.entries.get(&built.key) {
            stats.cache_hits += 1;
            Arc::clone(found)
        } else if let Some(outfit) = &built.outfit {
            stats.cache_misses += 1;
            match upload(outfit, shared.content.as_ref(), &mut cache, &server, &mut meshes, &mut materials, &mut inverse, frame.0) {
                Ok(assets) => {
                    let mut assets = assets;
                    assets.cloth.clone_from(&built.cloth);
                    let assets = Arc::new(assets);
                    cache.entries.insert(built.key.clone(), Arc::clone(&assets));
                    shared.resident.write().unwrap_or_else(PoisonError::into_inner).insert(built.key.clone());
                    assets
                }
                Err(e) => {
                    log::warn!("character {spec:?}: {e}");
                    stats.failed += 1;
                    continue;
                }
            }
        } else {
            // Evicted between the task's look and now: assemble after all.
            commands.entity(entity).insert(spawn_build(shared, spec, built.key.bodies, true));
            continue;
        };
        ready.push((entity, built, assets));
    }
    for (entity, swap) in &pending {
        ready.push((entity, swap.0.clone_for_swap(), Arc::clone(&swap.1)));
        commands.entity(entity).remove::<PendingSwap>();
    }
    for (entity, built, assets) in ready {
        let Ok((body, layers)) = roots.get_mut(entity) else { continue };
        let has_body = body.is_some();
        if has_body && frame.0 < assets.ready_frame {
            // The old outfit stays until the new one has settled.
            commands.entity(entity).insert(PendingSwap(built, assets));
            continue;
        }
        commands.entity(entity).insert(held::HeldPending {
            items: built.recipe.held.clone(),
            cooked: built.held.clone(),
            animation: Arc::clone(&built.animation),
        });
        if body.as_deref().is_some_and(|b| b.key == built.key && Arc::ptr_eq(&b.outfit, &assets)) {
            // Same outfit on the same rig (a held items change): keep the meshes.
            continue;
        }
        let layers = layers.cloned();
        let (body_entity, binding) = match body.as_deref() {
            Some(b) if b.skeleton == built.key.skeleton => match bindings.get(b.body) {
                Ok(binding) => (b.body, binding.clone()),
                Err(_) => continue,
            },
            old => {
                if let Some(old) = old {
                    commands.entity(old.body).despawn();
                    stats.meshes -= old.meshes.len();
                }
                spawn_body(&mut commands, entity, &built)
            }
        };
        // Cloth lattice joints (module `cloth`): entities under the body, posed by the cloth op.
        let mut cloth_joints = Vec::new();
        let joints: Vec<Entity> = assets
            .joints
            .iter()
            .map(|name| {
                if let Some((k, i)) = cloth::parse_joint(name)
                    && let Some(b) = assets.cloth.get(k)
                    && let (Ok(cloth), Ok(particle)) = (u16::try_from(k), u16::try_from(i))
                {
                    let e = commands.spawn((ClothJoint { body: body_entity, cloth, particle }, cloth::rest_transform(b, i), ChildOf(body_entity))).id();
                    cloth_joints.push(e);
                    return e;
                }
                binding.find(&built.animation.skeleton, name).unwrap_or_else(|| {
                    stats.missing_joints += 1;
                    binding.joints[0]
                })
            })
            .collect();
        commands.entity(body_entity).insert(cloth::ClothPending(assets.cloth.clone()));
        let mesh_entities: Vec<Entity> = assets
            .meshes
            .iter()
            .map(|m| {
                let mut e = commands.spawn((
                    Mesh3d(m.mesh.clone()),
                    MeshMaterial3d(m.material.clone()),
                    SkinnedMesh { inverse_bindposes: assets.inverse_bindposes.clone(), joints: joints.clone() },
                    DynamicSkinnedMeshBounds,
                    Transform::IDENTITY,
                    Visibility::default(),
                    ChildOf(body_entity),
                ));
                if !m.morph_weights.is_empty() {
                    e.insert(MeshMorphWeights::Value { weights: m.morph_weights.clone() });
                }
                if let Some(layers) = &layers {
                    e.insert(layers.clone());
                }
                e.id()
            })
            .collect();
        stats.meshes += mesh_entities.len();
        match CharacterMorphs::of_outfit(&assets, &mesh_entities) {
            Some(morphs) => commands.entity(entity).insert(morphs),
            None => commands.entity(entity).remove::<CharacterMorphs>(),
        };
        let next = CharacterBody { body: body_entity, skeleton: built.key.skeleton.clone(), key: built.key, outfit: assets, meshes: mesh_entities, cloth_joints };
        match body {
            Some(mut old) if old.body == body_entity => {
                // Same flush as the new meshes' spawn: no frame without either.
                for &m in &old.meshes {
                    commands.entity(m).despawn();
                }
                for &j in &old.cloth_joints {
                    commands.entity(j).despawn();
                }
                stats.meshes -= old.meshes.len();
                stats.swaps += 1;
                *old = next;
            }
            _ => {
                stats.spawned += usize::from(!has_body);
                commands.entity(entity).insert(next);
            }
        }
    }
    evict(&mut cache, shared);
}

impl Built {
    /// Pending swaps keep their build; the query hands out a reference.
    fn clone_for_swap(&self) -> Self {
        Self {
            recipe: self.recipe.clone(),
            animation: Arc::clone(&self.animation),
            key: self.key.clone(),
            outfit: None,
            missing_parts: 0,
            smooth: None,
            held: self.held.clone(),
            cloth: self.cloth.clone(),
        }
    }
}

/// The body under `root`: race scale, rig, animator.
fn spawn_body(commands: &mut Commands, root: Entity, built: &Built) -> (Entity, SkeletonBinding) {
    let scale = built.recipe.scale;
    let scale = if scale.cmpgt(Vec3::ZERO).all() { scale } else { Vec3::ONE };
    let body = commands.spawn((Name::new("body"), Transform::from_scale(scale), Visibility::default(), ChildOf(root))).id();
    let binding = spawn_rig(commands, body, &built.animation.skeleton);
    let asset = Arc::clone(&built.animation);
    let lod = AnimLod::new(asset.rest_height() * scale.y);
    commands.entity(body).insert((Animator::new(asset), lod, AnimRootMotion::default()));
    (body, binding)
}
