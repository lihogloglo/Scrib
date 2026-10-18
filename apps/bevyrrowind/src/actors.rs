//! Morrowind characters for `sf_character_bevy`: `mw_actors` recipes
//! (NPC_ / CREA), cooked body parts from the bake (`mesh.cook`), the baked
//! animation (`mw_bake::anim::AnimLibrary`), NIF-path materials, the
//! smooth-body probe poses (`mw_actors::probes`; B toggles smooth ↔ vanilla
//! bodies), and the half turn vanilla placements need
//! (`mw_anim::rig::FACING_TURN`: our
//! skeletons face +Z, Morrowind's actors face -Z).

use std::sync::Arc;

use bevy::math::Affine3A;
use bevy::prelude::*;
use mw_actors::MwActors;
use mw_bake::anim::AnimLibrary;
use sf_anim_bevy::AnimAsset;
use sf_bake::{Manifest, Store};
use sf_character::{CharacterRecipe, CharacterSource, PartLookup, SlotRegistry};
use sf_character_bevy::{CharacterContent, CharacterSettings, Characters, ClothCollider, ClothSettings, ClothStats, SfCharacterPlugin};
use sf_model::Cooked;
use sf_model::steps::{MeshCookStep, MeshRestStep, MeshRigStep};
use sf_records::RecordDb;
use sf_vfs::{Vfs, VfsPath};

pub struct MwCharacters {
    actors: MwActors,
    store: Store,
    manifest: Manifest,
    animation: AnimLibrary,
    vfs: Arc<Vfs>,
    /// Robes and skirts (physics-adapter.md "Cloth").
    cloth: mw_physics::MwClothRules,
}

impl MwCharacters {
    pub fn new(db: Arc<RecordDb>, vfs: Arc<Vfs>, store: Store, manifest: Manifest) -> Self {
        let animation = AnimLibrary::new(store.clone(), manifest.clone());
        let cloth = mw_physics::MwClothRules::new(&db);
        Self { actors: MwActors::new(db, Arc::clone(&vfs)), store, manifest, animation, vfs, cloth }
    }

    pub fn actors(&self) -> &MwActors {
        &self.actors
    }

    /// A product of `step` for `model`, decoded.
    fn product(&self, step: &str, model: &VfsPath) -> Result<Cooked, String> {
        let digest = self.manifest.get(step, model.as_str()).ok_or_else(|| format!("{} is not baked (run `bevyrrowind bake`)", model.as_str()))?;
        let bytes = self.store.get(&digest).map_err(|e| format!("{}: {e}", model.as_str()))?;
        Cooked::from_bytes(&bytes).map_err(|e| format!("{}: {e}", model.as_str()))
    }
}

impl CharacterContent for MwCharacters {
    fn source(&self) -> &dyn CharacterSource {
        &self.actors
    }

    fn parts(&self) -> &dyn PartLookup {
        &self.actors
    }

    fn slots(&self) -> &SlotRegistry {
        self.actors.slots()
    }

    /// A creature body's `mesh.rig` product (rigid meshes bound to their
    /// nodes, so creatures animated per node assemble like skinned ones),
    /// else the model's `mesh.cook`.
    fn cooked(&self, model: &VfsPath) -> Result<Cooked, String> {
        let digest = self
            .manifest
            .get(MeshRigStep::ID, model.as_str())
            .or_else(|| self.manifest.get(MeshCookStep::ID, model.as_str()))
            .ok_or_else(|| format!("{} is not baked (run `bevyrrowind bake`)", model.as_str()))?;
        let bytes = self.store.get(&digest).map_err(|e| format!("{}: {e}", model.as_str()))?;
        Cooked::from_bytes(&bytes).map_err(|e| format!("{}: {e}", model.as_str()))
    }

    /// A held model's `mesh.cook`; one the cook skipped for an animated node
    /// (a torch's flame) takes its `mesh.rest` (rest pose; particles left out).
    fn held_model(&self, model: &VfsPath) -> Result<Cooked, String> {
        match self.product(MeshCookStep::ID, model)? {
            Cooked::Skipped { reason } => match self.manifest.get(MeshRestStep::ID, model.as_str()) {
                Some(_) => self.product(MeshRestStep::ID, model),
                None => Ok(Cooked::Skipped { reason: format!("{reason}; no `mesh.rest` product") }),
            },
            cooked => Ok(cooked),
        }
    }

    /// A node's model-space position, read from the NIF (the cook flattens
    /// nodes; the cooked geometry and NIF nodes leave the root transform out).
    fn node_position(&self, model: &VfsPath, node: &str) -> Option<Vec3> {
        let nif = mw_nif::load(&self.vfs.read(model).ok()?).ok()?;
        let i = nif.nodes.iter().position(|n| n.name.eq_ignore_ascii_case(node))?;
        Some(Vec3::from_array(mw_nif::world_transforms(&nif)[i].translation.to_array()))
    }

    /// Joints of a skinned held model (a bow), read from the NIF in the same
    /// model space as [`Self::node_position`].
    fn node_transforms(&self, model: &VfsPath, nodes: &[&str]) -> Vec<Option<Mat4>> {
        let Some(nif) = self.vfs.read(model).ok().and_then(|b| mw_nif::load(&b).ok()) else { return vec![None; nodes.len()] };
        let world = mw_nif::world_transforms(&nif);
        nodes
            .iter()
            .map(|name| {
                let i = nif.nodes.iter().position(|n| n.name.eq_ignore_ascii_case(name))?;
                Some(Mat4::from_cols_array(&mw_nif::glam::Mat4::from(world[i]).to_cols_array()))
            })
            .collect()
    }

    fn animation(&self, recipe: &CharacterRecipe) -> Result<AnimAsset, String> {
        let a = self.animation.load(recipe.skeleton.as_str(), recipe.graph.as_str()).map_err(|e| e.to_string())?;
        AnimAsset::new(a.skeleton, a.graph, a.clips).map_err(|e| e.to_string())
    }

    fn material(&self, material: &sf_model::Material, assets: &AssetServer) -> StandardMaterial {
        mw_assets::cooked::cooked_material(material, &self.vfs, assets)
    }

    /// Elbows, knees, shoulders and the walk/run clips (`mw_actors::probes`).
    fn smooth_probes(&self, _: &CharacterRecipe, animation: &AnimAsset) -> Vec<Vec<Affine3A>> {
        mw_actors::probes::smooth_probes(&animation.skeleton, animation.clips.iter().map(|c| &**c))
    }

    /// Robes (Chest parts) and skirts (`mw_physics::MwClothRules`).
    fn cloth_spec(&self, id: &sf_records::RecordId, part: &sf_character::BodyPart) -> Option<sf_character::ClothSpec> {
        sf_character::ClothRules::cloth(&self.cloth, id, part)
    }

    /// The skeleton's `ragdoll.cook` capsules below the line (pelvis, legs, tail) on
    /// its `rig.profile` (as the census does).
    fn cloth_colliders(&self, recipe: &CharacterRecipe, animation: &AnimAsset, below: f32) -> Vec<ClothCollider> {
        let skeleton = recipe.skeleton.as_str();
        let ragdoll = self
            .manifest
            .get(sf_ragdoll::RAGDOLL_COOK, skeleton)
            .and_then(|d| self.store.get(&d).ok())
            .and_then(|b| sf_ragdoll::RagdollDef::from_bytes(&b).ok());
        let profile = self.animation.profile(skeleton).ok().flatten();
        match (ragdoll, profile) {
            (Some(r), Some(p)) => r.cloth_colliders(&p, &animation.skeleton.rest_model(), below),
            _ => {
                warn!("cloth on {skeleton}: no ragdoll.cook or rig.profile product; no colliders");
                Vec::new()
            }
        }
    }

    fn placement_rotation(&self) -> Quat {
        Quat::from_array(mw_anim::rig::FACING_TURN.to_array())
    }
}

/// The streamed world's actors: with `db` (and no `--off actors`), the
/// character plugin with Morrowind's content.
pub fn add_world(app: &mut App, db: Option<Arc<RecordDb>>, off: bool, vfs: &Arc<Vfs>, store: &Store, manifest: &Manifest) {
    if let (Some(db), false) = (db, off) {
        add(app, db, Arc::clone(vfs), store.clone(), manifest.clone());
    }
}

/// Adds the character plugin with Morrowind's content.
pub fn add(app: &mut App, db: Arc<RecordDb>, vfs: Arc<Vfs>, store: Store, manifest: Manifest) {
    let started = std::time::Instant::now();
    let content = MwCharacters::new(db, vfs, store, manifest);
    info!("characters: mw_actors ready in {:.2} s", started.elapsed().as_secs_f32());
    add_content(app, Arc::new(content));
}

/// Adds the character plugin with prepared content.
pub fn add_content(app: &mut App, content: Arc<MwCharacters>) {
    crate::faces::add(app, Arc::clone(&content.vfs));
    crate::actor_fx::add(app, Arc::clone(&content.vfs));
    crate::held::add(app, Arc::clone(&content));
    crate::actor_particles::add(app, Arc::clone(&content.vfs));
    app.add_plugins(SfCharacterPlugin).insert_resource(Characters::new(content)).add_systems(Update, (toggle_bodies, toggle_cloth, cloth_log));
}

/// B: smooth ↔ vanilla bodies (A/B, smooth-bodies.md). Every character's
/// outfit rebuilds through the no-pop swap; both looks stay cached.
fn toggle_bodies(keys: Option<Res<ButtonInput<KeyCode>>>, mut settings: ResMut<CharacterSettings>) {
    if keys.is_some_and(|k| k.just_pressed(KeyCode::KeyB)) {
        settings.bodies = settings.bodies.toggled();
        info!("bodies: {:?} (B toggles)", settings.bodies);
    }
}

/// H: cloth simulated ↔ kinematic (A/B, physics-adapter.md "Cloth", PH-3):
/// off, robes and skirts ride the pelvis and follow the legs (the op's
/// animated targets) with no swing.
fn toggle_cloth(keys: Option<Res<ButtonInput<KeyCode>>>, mut settings: ResMut<ClothSettings>) {
    if keys.is_some_and(|k| k.just_pressed(KeyCode::KeyH)) {
        settings.simulate = !settings.simulate;
        info!("cloth: {} (H toggles)", if settings.simulate { "simulated" } else { "kinematic" });
    }
}

/// Every 5 s while cloth is bound: parts, simulated parts and the op's cost
/// per simulated part (the P10 gate: ≤ 0.03 ms per robe).
fn cloth_log(stats: Res<ClothStats>, time: Res<Time>, mut acc: Local<(f32, f32, usize, f32)>) {
    let (elapsed, us, frames, since) = &mut *acc;
    if stats.simulated > 0 {
        #[allow(clippy::cast_precision_loss)]
        let per = stats.op_us / stats.simulated as f32;
        *us += per;
        *frames += 1;
    }
    *elapsed += time.delta_secs();
    if *elapsed - *since >= 5.0 && *frames > 0 {
        #[allow(clippy::cast_precision_loss)]
        let mean = *us / *frames as f32;
        info!("cloth: {} parts, {} simulated, op {mean:.1} us per simulated part (mean of {} frames)", stats.parts, stats.simulated, *frames);
        *since = *elapsed;
        *us = 0.0;
        *frames = 0;
    }
}
