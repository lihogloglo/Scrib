//! A synthetic biped dressed through the plugin in a headless `App`:
//! outfit cache sharing, one mesh per material, a swap that keeps the rig.

use std::collections::HashMap;
use std::sync::Arc;

use bevy::mesh::morph::MeshMorphWeights;
use bevy::prelude::*;
use sf_anim::fixtures::{biped, gait};
use sf_anim::{ClipRules, CookSettings, Looping, cook_clip};
use sf_anim_graph::{CompileSource, compile_str};
use sf_character::{
    BodyPart, CharacterError, CharacterRecipe, CharacterSource, CharacterState, Hand, HeldItem, HeldLight, Layer, Mount, PartKind, PartLookup, Readiness,
    RigMap, RigRef, SlotRegistry, TintParams, Wearable, slot,
};
use sf_model::{Cooked, Material, MeshData, Model, MorphKind, MorphTarget, Node};
use sf_records::RecordId;
use sf_vfs::VfsPath;

use crate::{
    AnimAsset, Character, CharacterBody, CharacterContent, CharacterReadiness, HeldProps, CharacterMorphs, CharacterSettings, CharacterStats, Characters, MorphCurvePlayer, MorphTrack,
    MorphTracks, OutfitCache, SfCharacterPlugin, SmoothBodies,
};

struct Idle(sf_anim::Clip);

impl CompileSource for Idle {
    fn clip(&mut self, reference: &str) -> Result<(sf_bake::ProductKey, sf_anim::Clip), String> {
        Ok((sf_bake::ProductKey::new("anim.cook", reference), self.0.clone()))
    }
    fn subgraph(&mut self, _: &str) -> Result<sf_anim_graph::asset::GraphAsset, String> {
        Err("none".into())
    }
}


fn idle_asset(morph: bool) -> AnimAsset {
    let skeleton = biped();
    let mut src = gait(&skeleton, "idle", 1.0, 0.0, 0.1, 0.6, &[("thigh_l", "foot_l", 0.0), ("thigh_r", "foot_r", 0.5)]);
    if morph {
        // The torso's morph target rises and falls once per cycle.
        let curve = sf_anim::Curve { times: vec![0.0, 0.5, 1.0], values: vec![0.0, 1.0, 0.0], interpolation: sf_anim::Interpolation::Linear };
        src.curves.push(sf_anim::SourceCurve { name: "torso#1".into(), curve });
    }
    let rules = ClipRules { looping: Looping::Whole, ..ClipRules::default() };
    let clip = cook_clip(&skeleton, &src, &rules, &CookSettings::default()).unwrap();
    let ron = r#"(nodes: {"idle": Clip(clip: "idle")}, layers: [(node: "idle")])"#;
    let graph = compile_str("idle", ron, &skeleton, &mut Idle(clip.clone())).unwrap();
    let clips = graph.clips.iter().map(|_| Arc::new(clip.clone())).collect();
    AnimAsset::new(Arc::new(skeleton), Arc::new(graph), clips).unwrap()
}

/// A 1 m quad in `material`, as a static (rigid) model.
fn quad(material: &str) -> Cooked {
    let mesh = MeshData {
        positions: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]],
        normals: Some(vec![[0.0, 0.0, 1.0]; 4]),
        uvs: Some(vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]),
        indices: vec![0, 1, 2, 0, 2, 3],
        ..MeshData::default()
    };
    let mut node = Node::new("quad");
    node.mesh = Some(0);
    let model = Model { nodes: vec![node], meshes: vec![mesh], materials: vec![Material { name: material.into(), ..Material::default() }] };
    sf_model::cook(&model, &sf_model::CookSettings::default()).unwrap()
}

/// A creature animated per node (no skin): a torso under `root` and a shin
/// under `thigh_l`, cooked for its rig (`sf_model::cook_rig`).
fn segmented(morph: bool) -> Cooked {
    let quad = |y: f32| MeshData {
        positions: vec![[0.0, y, 0.0], [0.2, y, 0.0], [0.2, y + 0.2, 0.0], [0.0, y + 0.2, 0.0]],
        normals: Some(vec![[0.0, 0.0, 1.0]; 4]),
        indices: vec![0, 1, 2, 0, 2, 3],
        ..MeshData::default()
    };
    let root = Node::new("root");
    let mut thigh = Node::new("thigh_l");
    thigh.parent = Some(0);
    thigh.translation = [0.1, 0.9, 0.0];
    let mut torso = Node::new("torso");
    torso.parent = Some(0);
    torso.mesh = Some(0);
    let mut shin = Node::new("shin");
    shin.parent = Some(1);
    shin.mesh = Some(1);
    let mut torso_mesh = quad(1.0);
    if morph {
        torso_mesh.morph_targets.push(MorphTarget {
            name: "torso#1".into(),
            positions: vec![[0.0, 0.0, 0.1]; 4],
            normals: None,
            default_weight: 0.0,
            kind: MorphKind::Expression,
        });
    }
    let model = Model { nodes: vec![root, thigh, torso, shin], meshes: vec![torso_mesh, quad(-0.4)], materials: vec![Material::default()] };
    sf_model::cook_rig(&model, &sf_model::CookSettings::default()).unwrap()
}

struct Content {
    slots: SlotRegistry,
    parts: HashMap<RecordId, BodyPart>,
    wearables: HashMap<RecordId, Wearable>,
    path: VfsPath,
}

fn path(s: &str) -> VfsPath {
    VfsPath::new(s).unwrap()
}

impl Content {
    fn new() -> Self {
        let slots = SlotRegistry::with_humanoid_defaults();
        let s = |n: &str| slots.get(n).unwrap();
        let mirror = glam::Affine3A::from_scale(Vec3::new(-1.0, 1.0, 1.0));
        let part = |model: &str, slot, kind| BodyPart { model: path(model), slot, kind, node_filter: None };
        let parts = HashMap::from([
            (RecordId::new("chest"), part("meshes/skin.nif", s(slot::CHEST), PartKind::rigid("root"))),
            (RecordId::new("leg_l"), part("meshes/skin.nif", s(slot::LEFT_UPPER_LEG), PartKind::rigid("thigh_l"))),
            (
                RecordId::new("leg_r"),
                part("meshes/skin.nif", s(slot::RIGHT_UPPER_LEG), PartKind::Rigid { joint: "thigh_r".into(), offset: mirror }),
            ),
            (RecordId::new("shirt"), part("meshes/cloth.nif", s(slot::CHEST), PartKind::rigid("root"))),
            (RecordId::new("segmented"), part("meshes/segmented.nif", s(slot::CHEST), PartKind::Skinned)),
            (RecordId::new("morphing"), part("meshes/morphing.nif", s(slot::CHEST), PartKind::Skinned)),
        ]);
        let shirt = Wearable { parts: vec![(s(slot::CHEST), RecordId::new("shirt"))], hides: vec![], layer: Layer::CLOTHING, first_person: vec![] };
        Self { slots, parts, wearables: HashMap::from([(RecordId::new("shirt"), shirt)]), path: path("meshes/biped.nif") }
    }
}

impl PartLookup for Content {
    fn part(&self, id: &RecordId) -> Option<&BodyPart> {
        self.parts.get(id)
    }
    fn wearable(&self, id: &RecordId) -> Option<&Wearable> {
        self.wearables.get(id)
    }
}

impl CharacterSource for Content {
    fn recipe(&self, kind: &str, _: &RecordId, state: &CharacterState) -> Result<CharacterRecipe, CharacterError> {
        let creature = kind == "creature" || kind == "morpher";
        let body = if kind == "morpher" { "morphing" } else { "segmented" };
        Ok(CharacterRecipe {
            skeleton: self.path.clone(),
            rig: RigRef::Profile(self.path.clone()),
            base: if creature { vec![RecordId::new(body)] } else { ["chest", "leg_l", "leg_r"].map(RecordId::new).to_vec() },
            head: None,
            hair: None,
            worn: if creature { vec![] } else { state.worn.clone().unwrap_or_else(|| vec![RecordId::new("shirt")]) },
            held: state.held.clone().unwrap_or_default(),
            scale: Vec3::ONE,
            graph: path(if kind == "morpher" { "graphs/morph.ron" } else { "graphs/idle.ron" }),
            tint: TintParams::default(),
            shape: sf_character::ShapeWeights::default(),
            first_person: None,
        })
    }
    fn rig_maps(&self) -> Vec<(VfsPath, RigMap)> {
        Vec::new()
    }
}

impl CharacterContent for Content {
    fn source(&self) -> &dyn CharacterSource {
        self
    }
    fn parts(&self) -> &dyn PartLookup {
        self
    }
    fn slots(&self) -> &SlotRegistry {
        &self.slots
    }
    fn cooked(&self, model: &VfsPath) -> Result<Cooked, String> {
        if model.as_str().contains("morphing") {
            return Ok(segmented(true));
        }
        if model.as_str().contains("segmented") {
            return Ok(segmented(false));
        }
        Ok(quad(if model.as_str().contains("cloth") { "cloth" } else { "skin" }))
    }
    fn node_position(&self, _: &VfsPath, node: &str) -> Option<Vec3> {
        (node == "AttachLight").then_some(Vec3::new(0.5, 1.0, 0.0))
    }
    fn animation(&self, recipe: &CharacterRecipe) -> Result<AnimAsset, String> {
        Ok(idle_asset(recipe.graph.as_str().contains("morph")))
    }
    /// Rest, and the left leg swung 1 rad about its hip.
    fn smooth_probes(&self, _: &CharacterRecipe, animation: &AnimAsset) -> Vec<Vec<glam::Affine3A>> {
        let skeleton = &animation.skeleton;
        let rest = skeleton.rest_model();
        let Some(thigh) = skeleton.find("thigh_l") else { return vec![rest] };
        let pivot = Vec3::from(rest[thigh].translation);
        let turn = glam::Affine3A::from_translation(pivot) * glam::Affine3A::from_rotation_x(1.0) * glam::Affine3A::from_translation(-pivot);
        let mut bent = rest.clone();
        for k in skeleton.subtree(thigh) {
            bent[k] = turn * bent[k];
        }
        vec![rest, bent]
    }
}

fn app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default(), TransformPlugin))
        .init_asset::<Mesh>()
        .init_asset::<StandardMaterial>()
        .init_asset::<bevy::mesh::skinning::SkinnedMeshInverseBindposes>()
        .add_plugins(SfCharacterPlugin)
        .insert_resource(Characters::new(Arc::new(Content::new())));
    app
}

/// Updates until `done` (build tasks run on the task pool).
fn run_until(app: &mut App, done: impl Fn(&mut World) -> bool) {
    for _ in 0..500 {
        app.update();
        if done(app.world_mut()) {
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(2));
    }
    panic!("characters never finished building");
}

fn npc(state: CharacterState) -> Character {
    Character::Record { kind: "npc".into(), id: RecordId::new("guard"), state }
}

#[test]
fn identical_characters_share_one_outfit_with_one_mesh_per_material() {
    let mut app = app();
    let a = app.world_mut().spawn(npc(CharacterState::default())).id();
    let b = app.world_mut().spawn(npc(CharacterState::default())).id();
    run_until(&mut app, |w| w.get::<CharacterBody>(a).is_some() && w.get::<CharacterBody>(b).is_some());
    let w = app.world();
    let (ba, bb) = (w.get::<CharacterBody>(a).unwrap(), w.get::<CharacterBody>(b).unwrap());
    assert!(Arc::ptr_eq(&ba.outfit, &bb.outfit), "same look, same cache entry");
    // Legs (skin, two rigid parts, one mirrored) merge; the shirt replaces the skin chest.
    assert_eq!(ba.outfit.meshes.len(), 2, "one mesh per material: skin, cloth");
    assert_eq!(ba.meshes.len(), 2);
    assert_eq!(w.resource::<OutfitCache>().len(), 1);
    let stats = w.resource::<CharacterStats>();
    assert_eq!((stats.spawned, stats.meshes, stats.failed, stats.missing_parts, stats.missing_joints), (2, 4, 0, 0, 0));
    // Every mesh is skinned to the rig's joints.
    for &m in &ba.meshes {
        let skinned = w.get::<bevy::mesh::skinning::SkinnedMesh>(m).unwrap();
        assert!(skinned.joints.iter().all(|j| w.get::<sf_anim_bevy::AnimJoint>(*j).is_some()));
    }
}

#[test]
fn outfit_swap_keeps_the_rig_and_replaces_meshes_in_one_step() {
    let mut app = app();
    let a = app.world_mut().spawn(npc(CharacterState::default())).id();
    run_until(&mut app, |w| w.get::<CharacterBody>(a).is_some());
    let (body, old_meshes) = {
        let b = app.world().get::<CharacterBody>(a).unwrap();
        (b.body, b.meshes.clone())
    };
    *app.world_mut().get_mut::<Character>(a).unwrap() = npc(CharacterState { worn: Some(vec![]), held: None });
    run_until(&mut app, |w| {
        // No pop: every frame shows a live outfit, old or new.
        let b = w.get::<CharacterBody>(a).unwrap();
        assert!(b.meshes.iter().all(|&m| w.get_entity(m).is_ok()), "a frame without the outfit");
        b.meshes != old_meshes
    });
    let w = app.world();
    let b = w.get::<CharacterBody>(a).unwrap();
    assert_eq!(b.body, body, "the rig survives a swap on the same skeleton");
    assert_eq!(b.meshes.len(), 1, "naked: the skin chest and legs merge into one mesh");
    assert!(old_meshes.iter().all(|&m| w.get_entity(m).is_err()), "old meshes despawned");
    assert!(b.meshes.iter().all(|&m| w.get_entity(m).is_ok()));
    assert_eq!(w.resource::<CharacterStats>().swaps, 1);
    assert_eq!(w.resource::<CharacterStats>().meshes, 1);
}

#[test]
fn node_animated_creature_binds_its_parts_to_joints_that_the_graph_moves() {
    let mut app = app();
    let a = app.world_mut().spawn(Character::Record { kind: "creature".into(), id: RecordId::new("scrib"), state: CharacterState::default() }).id();
    run_until(&mut app, |w| w.get::<CharacterBody>(a).is_some());
    let (joints, mesh) = {
        let b = app.world().get::<CharacterBody>(a).unwrap();
        (b.outfit.joints.clone(), b.meshes[0])
    };
    assert_eq!(joints, ["root", "thigh_l"], "each rigid part on its node's joint");
    let stats = app.world().resource::<CharacterStats>();
    assert_eq!((stats.missing_parts, stats.missing_joints, stats.meshes), (0, 0, 1), "one merged mesh, every joint on the rig");
    let thigh = app.world().get::<bevy::mesh::skinning::SkinnedMesh>(mesh).unwrap().joints[1];
    assert!(app.world().get::<sf_anim_bevy::AnimJoint>(thigh).is_some());
    // The idle graph swings the thigh: the shin's joint turns over time.
    let pose = |w: &World| w.get::<GlobalTransform>(thigh).unwrap().rotation();
    let start = pose(app.world());
    let mut turned = 0.0f32;
    for _ in 0..30 {
        std::thread::sleep(std::time::Duration::from_millis(10));
        app.update();
        turned = turned.max(start.angle_between(pose(app.world())));
    }
    assert!(turned > 0.02, "the thigh joint never moved ({turned} rad)");
}

#[test]
fn morph_curves_in_clips_animate_the_body_and_a_player_overrides_them() {
    let mut app = app();
    let a = app.world_mut().spawn(Character::Record { kind: "morpher".into(), id: RecordId::new("forager"), state: CharacterState::default() }).id();
    run_until(&mut app, |w| w.get::<CharacterBody>(a).is_some());
    let morphs = app.world().get::<CharacterMorphs>(a).expect("a morphing outfit gets CharacterMorphs");
    assert_eq!(morphs.names, ["torso#1"]);
    let mesh = app.world().get::<CharacterBody>(a).unwrap().meshes.iter().copied().find(|&m| app.world().get::<MeshMorphWeights>(m).is_some()).unwrap();
    let shown = |w: &World| match w.get::<MeshMorphWeights>(mesh).unwrap() {
        MeshMorphWeights::Value { weights } => weights[0],
        MeshMorphWeights::Reference(_) => f32::NAN,
    };
    // The clip's curve drives the weight over time (the morph-animated body loops with its idle).
    let (mut lo, mut hi) = (f32::MAX, f32::MIN);
    for _ in 0..40 {
        std::thread::sleep(std::time::Duration::from_millis(15));
        app.update();
        let w = shown(app.world());
        assert!((w - app.world().get::<CharacterMorphs>(a).unwrap().weight("torso#1").unwrap()).abs() < 1e-6, "mesh shows the character's weight");
        (lo, hi) = (lo.min(w), hi.max(w));
    }
    assert!(hi - lo > 0.1, "the morph weight never changed ({lo}..{hi})");
    // An external player (a face's blink) replaces the animated weight.
    let curve = sf_anim::Curve { times: vec![0.0, 1.0], values: vec![0.25, 0.75], interpolation: sf_anim::Interpolation::Linear };
    let tracks = Arc::new(MorphTracks { tracks: vec![MorphTrack { target: "torso#1".into(), curve }] });
    app.world_mut().entity_mut(a).insert(MorphCurvePlayer { time: 0.5, ..MorphCurvePlayer::new(tracks) });
    app.update();
    assert!((shown(app.world()) - 0.5).abs() < 1e-5, "player at t=0.5 shows 0.5, got {}", shown(app.world()));
}

#[test]
fn smooth_and_vanilla_bodies_are_separate_outfits_and_the_toggle_never_pops() {
    let mut app = app();
    assert_eq!(app.world().resource::<CharacterSettings>().bodies, SmoothBodies::Vanilla, "vanilla by default");
    app.world_mut().resource_mut::<CharacterSettings>().bodies = SmoothBodies::Smooth;
    let a = app.world_mut().spawn(npc(CharacterState::default())).id();
    run_until(&mut app, |w| w.get::<CharacterBody>(a).is_some());
    let (body, smooth_key) = {
        let b = app.world().get::<CharacterBody>(a).unwrap();
        (b.body, b.key.clone())
    };
    assert_eq!(smooth_key.bodies, SmoothBodies::Smooth, "smooth when opted in");
    assert_eq!(app.world().resource::<CharacterStats>().smoothed, 1, "the build ran the smooth step");
    for (bodies, cached) in [(SmoothBodies::Vanilla, 2), (SmoothBodies::Smooth, 2)] {
        app.world_mut().resource_mut::<CharacterSettings>().bodies = bodies;
        run_until(&mut app, |w| {
            // No pop: every frame shows a live outfit, old or new.
            let b = w.get::<CharacterBody>(a).unwrap();
            assert!(!b.meshes.is_empty() && b.meshes.iter().all(|&m| w.get_entity(m).is_ok()), "a frame without the outfit");
            b.key.bodies == bodies
        });
        let w = app.world();
        let b = w.get::<CharacterBody>(a).unwrap();
        assert_eq!(b.body, body, "the toggle keeps the rig");
        assert_eq!(w.resource::<OutfitCache>().len(), cached, "one cache entry per mode");
        let other = OutfitCacheKeyParts::of(&b.key);
        assert_eq!(other, OutfitCacheKeyParts::of(&smooth_key), "only the mode differs");
    }
    let stats = app.world().resource::<CharacterStats>();
    assert_eq!(stats.swaps, 2);
    assert!(stats.cache_hits >= 1, "back to smooth reuses the cached outfit");
    assert_eq!(stats.smoothed, 1, "the vanilla build skips the step, the second smooth build is a cache hit");
}

/// An outfit cache key without its mode.
#[derive(Debug, PartialEq)]
struct OutfitCacheKeyParts(String, sf_character::OutfitKey, u8, u64);

impl OutfitCacheKeyParts {
    fn of(k: &crate::OutfitCacheKey) -> Self {
        Self(k.skeleton.clone(), k.outfit, k.lod, k.shape)
    }
}

fn held(id: &str, socket: &str, light: bool) -> HeldItem {
    HeldItem {
        item: RecordId::new(id),
        model: path(&format!("meshes/{id}.nif")),
        hand: Hand::Main,
        drawn: Some(Mount { socket: socket.into(), offset: glam::Affine3A::from_translation(Vec3::new(0.0, -0.3, 0.0)) }),
        sheathed: None,
        light: light.then(|| HeldLight { attach_node: Some("AttachLight".into()), color: Vec3::ONE, radius: 3.0 }),
    }
}

/// The bone entity of a fixture joint on a character's rig.
fn joint(w: &World, root: Entity, name: &str) -> Entity {
    let body = w.get::<CharacterBody>(root).unwrap().body;
    w.get::<sf_anim_bevy::SkeletonBinding>(body).unwrap().joints[biped().find(name).unwrap()]
}

#[test]
fn held_items_mount_on_their_socket_follow_it_and_move_with_readiness() {
    let mut app = app();
    app.add_plugins(bevy::camera::visibility::VisibilityPlugin);
    let shield = HeldItem::always(RecordId::new("shield"), path("meshes/shield.nif"), Hand::Off, Mount::at("hand_l"));
    let state = CharacterState { worn: None, held: Some(vec![held("sword", "thigh_l", false), shield]) };
    let a = app.world_mut().spawn(npc(state)).id();
    run_until(&mut app, |w| w.get::<HeldProps>(a).is_some());
    let props = app.world().get::<HeldProps>(a).unwrap().props.clone();
    assert_eq!(props.len(), 2);
    let (sword, shield) = (&props[0], &props[1]);
    let w = app.world();
    let (thigh, hand) = (joint(w, a, "thigh_l"), joint(w, a, "hand_l"));
    // Sheathed (the default): the sword shows nowhere, the shield on its hand.
    assert_eq!(w.get::<ChildOf>(sword.entity).unwrap().parent(), thigh, "parked on its drawn socket");
    assert_eq!(w.get::<Visibility>(sword.entity), Some(&Visibility::Hidden));
    assert_eq!(w.get::<ChildOf>(shield.entity).unwrap().parent(), hand);
    assert_eq!(w.get::<Visibility>(shield.entity), Some(&Visibility::Inherited));
    assert_eq!(w.get::<Children>(sword.entity).map(Children::len), Some(1), "one mesh per cooked submesh");
    assert!(sword.light.is_none() && w.resource::<CharacterStats>().held == 2);
    assert_eq!(w.resource::<crate::HeldModelCache>().len(), 2, "one upload per model");
    // A hidden character hides its props (the socket chains inherit visibility).
    assert!(w.get::<InheritedVisibility>(shield.entity).unwrap().get());
    app.world_mut().entity_mut(a).insert(Visibility::Hidden);
    app.update();
    assert!(!app.world().get::<InheritedVisibility>(shield.entity).unwrap().get(), "the prop outlives its character's visibility");
    app.world_mut().entity_mut(a).insert(Visibility::Inherited);

    // Drawn: the sword shows at its socket, the shield stays.
    app.world_mut().entity_mut(a).insert(CharacterReadiness(Readiness::Drawn));
    app.update();
    assert_eq!(app.world().get::<Visibility>(sword.entity), Some(&Visibility::Inherited));
    assert_eq!(app.world().get::<Visibility>(shield.entity), Some(&Visibility::Inherited));
    // The prop follows the animated joint at its mount offset.
    let mut moved = 0.0f32;
    let start = app.world().get::<GlobalTransform>(sword.entity).unwrap().translation();
    for _ in 0..30 {
        std::thread::sleep(std::time::Duration::from_millis(10));
        app.update();
        let w = app.world();
        let (j, p) = (w.get::<GlobalTransform>(thigh).unwrap(), w.get::<GlobalTransform>(sword.entity).unwrap());
        let expected = j.transform_point(Vec3::new(0.0, -0.3, 0.0));
        assert!(p.translation().distance(expected) < 1e-4, "prop at {} not at its mount {expected}", p.translation());
        moved = moved.max(p.translation().distance(start));
    }
    assert!(moved > 1e-3, "the prop never moved with its joint ({moved} m)");

    // Sheathed again: hidden.
    app.world_mut().entity_mut(a).insert(CharacterReadiness(Readiness::Sheathed));
    app.update();
    assert_eq!(app.world().get::<Visibility>(sword.entity), Some(&Visibility::Hidden));

    // Held items alone change (a torch at dusk): props change, the outfit meshes stay.
    let meshes = app.world().get::<CharacterBody>(a).unwrap().meshes.clone();
    let torch = held("torch", "hand_l", true);
    *app.world_mut().get_mut::<Character>(a).unwrap() = npc(CharacterState { worn: None, held: Some(vec![torch.clone()]) });
    run_until(&mut app, |w| w.get::<HeldProps>(a).unwrap().props.iter().any(|p| p.item == torch));
    let w = app.world();
    assert_eq!(w.get::<CharacterBody>(a).unwrap().meshes, meshes, "outfit meshes kept");
    assert!(w.get_entity(sword.entity).is_err() && w.get_entity(shield.entity).is_err(), "old props despawned");
    let props = &w.get::<HeldProps>(a).unwrap().props;
    assert_eq!(props.len(), 1);
    let slot = props[0].light.expect("a light-carrying item gets a light slot");
    assert_eq!(w.get::<crate::HeldLightSlot>(slot).unwrap().item, RecordId::new("torch"));
    assert_eq!(w.get::<Transform>(slot).unwrap().translation, Vec3::new(0.5, 1.0, 0.0), "at the content's light node");
    assert_eq!(w.get::<ChildOf>(slot).unwrap().parent(), props[0].entity);
    assert_eq!(w.resource::<CharacterStats>().held, 1);
    assert_eq!(w.resource::<CharacterStats>().swaps, 0);
}

#[test]
fn anim_variety_is_seeded_by_the_stable_id_not_the_entity() {
    use crate::{AnimSeed, AnimVarietySettings, seed_variety};
    let on = AnimVarietySettings::default();
    let at = |x| Transform::from_xyz(x, 0.0, 0.0);
    // The same id, wherever it stands or whatever entity it is: the same variety.
    let id = AnimSeed::from_id(0xABCD);
    assert_eq!(seed_variety(on, Some(&id), &at(0.0)), seed_variety(on, Some(&id), &at(9.0)));
    // Neighbours without ids differ by position; no variety when disabled.
    assert_ne!(seed_variety(on, None, &at(0.0)), seed_variety(on, None, &at(3.5)));
    let off = AnimVarietySettings { enabled: false, ..on };
    assert_eq!(seed_variety(off, Some(&id), &at(0.0)), sf_anim_graph::Variety::default());
}
