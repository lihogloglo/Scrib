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
            (RecordId::new("head"), part("meshes/skin.nif", s(slot::HEAD), PartKind::rigid("root"))),
            (RecordId::new("arm_l"), part("meshes/skin.nif", s(slot::LEFT_UPPER_ARM), PartKind::rigid("root"))),
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
            base: match kind {
                _ if creature => vec![RecordId::new(body)],
                "player" => ["chest", "leg_l", "leg_r", "arm_l"].map(RecordId::new).to_vec(),
                _ => ["chest", "leg_l", "leg_r"].map(RecordId::new).to_vec(),
            },
            head: (kind == "player").then(|| RecordId::new("head")),
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

/// A character, its built body, its held prop and torch slot, and parts the
/// game hangs on it (a mesh, a light) all take the render layer of the
/// worldspace it is in, and follow when an ancestor moves it to another
/// worldspace (a player avatar under a body that follows the camera through
/// a door).
#[cfg(feature = "world")]
#[test]
fn characters_and_their_parts_follow_their_worldspace_render_layer() {
    use crate::ShadowLayers;
    use bevy::camera::visibility::RenderLayers;
    use sf_world_bevy::WorldspaceRoot;
    let mut app = app();
    let worldspace = |w: &mut World, id: &str, layer: usize| {
        w.spawn((WorldspaceRoot { id: sf_world::WorldspaceId::new(id), layer }, Transform::IDENTITY, Visibility::default())).id()
    };
    let three = worldspace(app.world_mut(), "outside", 3);
    let five = worldspace(app.world_mut(), "inside", 5);
    let carrier = app.world_mut().spawn((Transform::IDENTITY, Visibility::default(), ChildOf(three))).id();
    let state = CharacterState { worn: None, held: Some(vec![held("torch", "hand_l", true)]) };
    let a = app.world_mut().spawn((npc(state), Transform::IDENTITY, Visibility::default(), ChildOf(carrier))).id();
    let mesh = app.world_mut().spawn((Mesh3d(Handle::default()), ChildOf(a))).id();
    let light = app.world_mut().spawn((PointLight::default(), ChildOf(a))).id();
    run_until(&mut app, |w| {
        w.get::<CharacterBody>(a).is_some() && w.get::<HeldProps>(a).is_some_and(|h| h.props.iter().any(|p| p.light.is_some()))
    });
    app.update();
    let parts = |w: &World| -> Vec<Entity> {
        let props = &w.get::<HeldProps>(a).unwrap().props;
        let prop_meshes = props.iter().flat_map(|p| w.get::<Children>(p.entity).map(|c| c.to_vec()).unwrap_or_default());
        let mut parts: Vec<Entity> = vec![a, mesh, light];
        parts.extend(w.get::<CharacterBody>(a).unwrap().meshes.iter().copied());
        parts.extend(prop_meshes.filter(|e| w.get::<Mesh3d>(*e).is_some()));
        parts.extend(props.iter().filter_map(|p| p.light));
        parts
    };
    let layers_of = |app: &App| -> Vec<Option<RenderLayers>> {
        let w = app.world();
        parts(w).into_iter().map(|e| w.get::<RenderLayers>(e).cloned()).collect()
    };
    let n = parts(app.world()).len();
    assert!(n >= 6, "root, mesh, light, body meshes, prop mesh, light slot: {n}");
    // The held light slot, and the light a host hangs under it, take the worldspace's light
    // layers (content plus shadow-only, FP-1b); everything else its content layer.
    let slot = app.world().get::<HeldProps>(a).unwrap().props.iter().find_map(|p| p.light).unwrap();
    // The host's light copies the slot's layers (`mw_actors_bevy` `light_held_slots`).
    let slot_layers = app.world().get::<RenderLayers>(slot).cloned().unwrap();
    let torch = app.world_mut().spawn((PointLight::default(), slot_layers, ChildOf(slot))).id();
    let expect = |world: usize| -> (Vec<Option<RenderLayers>>, [Option<RenderLayers>; 2], Option<ShadowLayers>) {
        let ws = sf_world_bevy::WorldspaceLayers::of(world);
        let mut v = vec![Some(ws.content()); n];
        v[n - 1] = Some(ws.light());
        (v, [Some(ws.light()), Some(ws.light())], Some(ShadowLayers(ws.shadow_only())))
    };
    let got = |app: &App| {
        let w = app.world();
        (layers_of(app), [slot, torch].map(|e| w.get::<RenderLayers>(e).cloned()), w.get::<ShadowLayers>(a).cloned())
    };
    assert_eq!(got(&app), expect(3));
    assert_eq!(expect(3).1[0], Some(RenderLayers::from_layers(&[3, 4])));
    // The carrier moves to the other worldspace: everything follows.
    app.world_mut().entity_mut(carrier).insert(ChildOf(five));
    app.update();
    assert_eq!(got(&app), expect(5));
    // The character itself reparented back: back to layer 3.
    app.world_mut().entity_mut(a).insert(ChildOf(three));
    app.update();
    assert_eq!(got(&app), expect(3));
}

/// FP-1b through the real systems: a first-person root in a worldspace casts its head on that
/// worldspace's shadow-only layer only, body and held items on its content layer, its torch on
/// both; travel moves the shadow layer with the worldspace.
#[cfg(feature = "world")]
#[test]
fn first_person_head_is_shadow_only_in_its_worldspace_and_follows_travel() {
    use crate::{PartRole, RoleLayers, ViewRole};
    use bevy::camera::visibility::RenderLayers;
    use sf_world_bevy::{WorldspaceLayers, WorldspaceRoot};
    let mut app = app();
    let worldspace = |w: &mut World, id: &str, layer: usize| {
        w.spawn((WorldspaceRoot { id: sf_world::WorldspaceId::new(id), layer }, Transform::IDENTITY, Visibility::default())).id()
    };
    let outside = worldspace(app.world_mut(), "outside", 1);
    let inside = worldspace(app.world_mut(), "inside", 3);
    let player = Character::Record {
        kind: "player".into(),
        id: RecordId::new("player"),
        state: CharacterState { worn: None, held: Some(vec![held("torch", "hand_l", true)]) },
    };
    let a = app.world_mut().spawn((player, RoleLayers::first_person(false), Transform::IDENTITY, Visibility::default(), ChildOf(outside))).id();
    run_until(&mut app, |w| w.get::<CharacterBody>(a).is_some() && w.get::<HeldProps>(a).is_some_and(|h| h.props.iter().any(|p| p.light.is_some())));
    app.update();
    let by_role = |app: &App| -> (Vec<(ViewRole, RenderLayers)>, Option<RenderLayers>) {
        let w = app.world();
        let props = &w.get::<HeldProps>(a).unwrap().props;
        let prop_meshes = props.iter().flat_map(|p| w.get::<Children>(p.entity).map(|c| c.to_vec()).unwrap_or_default());
        let body = w.get::<CharacterBody>(a).unwrap().meshes.iter().copied();
        let mut v: Vec<_> =
            body.chain(prop_meshes).filter_map(|e| Some((w.get::<PartRole>(e)?.0, w.get::<RenderLayers>(e).cloned().unwrap_or_default()))).collect();
        v.sort_by_key(|(r, _)| *r);
        v.dedup();
        (v, props.iter().find_map(|p| p.light).and_then(|l| w.get::<RenderLayers>(l).cloned()))
    };
    let expect = |ws: WorldspaceLayers| {
        let v = vec![(ViewRole::Body, ws.content()), (ViewRole::Head, ws.shadow_only()), (ViewRole::Arms, ws.content()), (ViewRole::Held, ws.content())];
        (v, Some(ws.light()))
    };
    assert_eq!(by_role(&app), expect(WorldspaceLayers::of(1)));
    assert_eq!(by_role(&app).0[1], (ViewRole::Head, RenderLayers::layer(2)), "the head: shadow-only layer alone");
    app.world_mut().entity_mut(a).insert(ChildOf(inside));
    app.update();
    assert_eq!(by_role(&app), expect(WorldspaceLayers::of(3)));
    // Back to third person: the head back on the content layer, off the shadow layer.
    app.world_mut().entity_mut(a).insert(RoleLayers::world());
    app.update();
    assert!(by_role(&app).0.iter().all(|(_, l)| *l == RenderLayers::layer(3)));
}

/// First-person roles (first-person.md): a root with `RoleLayers` gets one mesh per material and
/// role, each tagged with its `PartRole` (held props `Held`), and re-layering by role changes
/// only layers: first person takes the head off the root's layers, keeps the rest, and leaves
/// the torch's light slot alone. A character without `RoleLayers` merges as before.
#[test]
fn role_layers_split_the_outfit_by_role_and_relayer_on_a_view_switch() {
    use crate::{PartRole, RoleLayer, RoleLayers, ViewRole};
    use bevy::camera::visibility::RenderLayers;
    let mut app = app();
    let player = || Character::Record {
        kind: "player".into(),
        id: RecordId::new("player"),
        state: CharacterState { worn: None, held: Some(vec![held("torch", "hand_l", true)]) },
    };
    let ws = RenderLayers::layer(3);
    let shadow = RenderLayers::layer(4);
    let a = app.world_mut().spawn((player(), RoleLayers::world(), ws.clone(), crate::ShadowLayers(shadow.clone()))).id();
    let plain = app.world_mut().spawn(player()).id();
    run_until(&mut app, |w| [a, plain].iter().all(|&e| w.get::<CharacterBody>(e).is_some() && w.get::<HeldProps>(e).is_some_and(|h| !h.props.is_empty())));
    app.update();
    let roles = |w: &World, e: Entity| -> Vec<(String, Option<ViewRole>)> {
        let mut v: Vec<_> = w.get::<CharacterBody>(e).unwrap().outfit.meshes.iter().map(|m| (m.material_name.clone(), m.role)).collect();
        v.sort();
        v
    };
    // The shirt (cloth) replaces the skin chest; skin splits into legs (body), head and arm.
    let split = vec![
        ("cloth".to_owned(), Some(ViewRole::Body)),
        ("skin".to_owned(), Some(ViewRole::Body)),
        ("skin".to_owned(), Some(ViewRole::Head)),
        ("skin".to_owned(), Some(ViewRole::Arms)),
    ];
    let mut sorted = split.clone();
    sorted.sort();
    assert_eq!(roles(app.world(), a), sorted);
    assert_eq!(roles(app.world(), plain), [("cloth".to_owned(), None), ("skin".to_owned(), None)], "no RoleLayers: merged per material");
    assert!(!Arc::ptr_eq(&app.world().get::<CharacterBody>(a).unwrap().outfit, &app.world().get::<CharacterBody>(plain).unwrap().outfit));

    // Every role part with its layers; the torch's light slot apart.
    let parts = |w: &World| -> (Vec<(ViewRole, Option<RenderLayers>)>, Option<RenderLayers>) {
        let body = w.get::<CharacterBody>(a).unwrap();
        let props = &w.get::<HeldProps>(a).unwrap().props;
        let prop_meshes = props.iter().flat_map(|p| w.get::<Children>(p.entity).map(|c| c.to_vec()).unwrap_or_default());
        let mut v: Vec<_> =
            body.meshes.iter().copied().chain(prop_meshes).filter_map(|e| Some((w.get::<PartRole>(e)?.0, w.get::<RenderLayers>(e).cloned()))).collect();
        v.sort_by_key(|(r, _)| *r);
        let light = props.iter().find_map(|p| p.light).and_then(|l| w.get::<RenderLayers>(l).cloned());
        (v, light)
    };
    let all = |head: Option<RenderLayers>| {
        let mut v = vec![
            (ViewRole::Body, Some(ws.clone())),
            (ViewRole::Body, Some(ws.clone())),
            (ViewRole::Head, head),
            (ViewRole::Arms, Some(ws.clone())),
            (ViewRole::Held, Some(ws.clone())),
        ];
        v.sort_by_key(|(r, _)| *r);
        v
    };
    let light = ws.union(&shadow);
    assert_eq!(parts(app.world()), (all(Some(ws.clone())), Some(light.clone())), "third person: every role on the root's layers");

    app.world_mut().entity_mut(a).insert(RoleLayers::first_person(false));
    app.update();
    assert_eq!(parts(app.world()), (all(Some(shadow.clone())), Some(light.clone())), "first person: the head on the shadow-only layer alone");
    let meshes = app.world().get::<CharacterBody>(a).unwrap().meshes.clone();

    // A host's extra layer on the arms and held items adds to the world layers.
    let vm = RoleLayer { extra: RenderLayers::layer(9), ..RoleLayer::WORLD };
    app.world_mut().entity_mut(a).insert(RoleLayers::first_person(false).with(ViewRole::Arms, vm.clone()).with(ViewRole::Held, vm));
    app.update();
    let (got, light) = parts(app.world());
    let with_vm = ws.clone().with(9);
    assert_eq!(
        got.iter().filter(|(r, _)| matches!(r, ViewRole::Arms | ViewRole::Held)).map(|(_, l)| l.clone()).collect::<Vec<_>>(),
        [Some(with_vm.clone()), Some(with_vm)]
    );
    assert_eq!(light, Some(ws.union(&shadow)));

    // Back to third person: all on the root's layers again, same mesh entities.
    app.world_mut().entity_mut(a).insert(RoleLayers::world());
    app.update();
    assert_eq!(parts(app.world()), (all(Some(ws.clone())), Some(ws.union(&shadow))));
    assert_eq!(app.world().get::<CharacterBody>(a).unwrap().meshes, meshes, "a view switch rebuilds nothing");
}

/// The per-role layer rule, exactly (first-person.md "Layer plan").
#[test]
fn role_layer_rule_per_role() {
    use crate::{RoleLayer, RoleLayers, ShadowLayers, ViewRole};
    use bevy::camera::visibility::RenderLayers;
    let ws = RenderLayers::from_layers(&[3]);
    let s = RenderLayers::layer(4);
    let world = RoleLayers::world();
    let fp = RoleLayers::first_person(false);
    for role in ViewRole::ALL {
        assert_eq!(world.layers(role, &ws, &s), ws, "{role:?}");
    }
    assert_eq!(ViewRole::ALL.map(|r| fp.layers(r, &ws, &s)), [ws.clone(), s.clone(), ws.clone(), ws.clone()]);
    assert_eq!(fp.layers(ViewRole::Head, &ws, &RenderLayers::none()), RenderLayers::none(), "no shadow layer: casts nothing");
    let extra = RoleLayers::world().with(ViewRole::Arms, RoleLayer { extra: RenderLayers::layer(40), ..RoleLayer::SHADOW });
    assert_eq!(extra.layers(ViewRole::Arms, &ws, &s), RenderLayers::from_layers(&[4, 40]), "shadow + a host's extra layer");
    // FP-2: arms and held items shadow only, their view copies on the root's layers.
    let vm = RoleLayers::first_person(true);
    assert_eq!(ViewRole::ALL.map(|r| vm.layers(r, &ws, &s)), [ws.clone(), s.clone(), s.clone(), s.clone()]);
    assert_eq!(ViewRole::ALL.map(|r| vm.copy_layers(r, &ws)), [None, None, Some(ws.clone()), Some(ws.clone())]);
    assert!(ViewRole::ALL.iter().all(|&r| fp.copy_layers(r, &ws).is_none() && world.copy_layers(r, &ws).is_none()), "no copies off the view model");
    assert_eq!(RoleLayer::OFF.extra, RenderLayers::none());
    assert_eq!(RoleLayers::world().with(ViewRole::Head, RoleLayer::OFF).layers(ViewRole::Head, &ws, &s), RenderLayers::none());
    assert_eq!(RoleLayers::world().layers(ViewRole::Body, &RenderLayers::default(), &s), RenderLayers::layer(0));
    assert_eq!(ShadowLayers::light(Some(&ShadowLayers(s.clone())), &ws), RenderLayers::from_layers(&[3, 4]));
    assert_eq!(ShadowLayers::light(None, &ws), ws);
}

/// Every view copy in the world: (copy, its part, the part's role).
fn view_copies(app: &mut App) -> Vec<(Entity, Entity, crate::ViewRole)> {
    let mut q = app.world_mut().query::<(Entity, &crate::ViewCopy)>();
    let w = app.world();
    let mut v: Vec<_> = q.iter(w).map(|(e, c)| (e, c.source, w.get::<crate::PartRole>(c.source).expect("a copy's source is a role part").0)).collect();
    v.sort_by_key(|(_, _, r)| *r);
    v
}

/// FP-2a (first-person.md "View model"): view copies exist only with the view model on, for arms
/// and held items alone, each a child of its part on the root's layers, sharing its mesh,
/// material and joints, casting nothing and out of the ray-traced scene, while the part goes
/// shadow only. Copies follow a material swap, a held change and an outfit swap, and go with the
/// rule.
#[test]
fn view_copies_redraw_arms_and_held_items_only_with_the_view_model() {
    use crate::{PartRole, RoleLayers, ViewRole};
    use bevy::camera::visibility::RenderLayers;
    use bevy::light::NotShadowCaster;
    use bevy::mesh::skinning::SkinnedMesh;
    use sf_model_bevy::NoRayTracing;
    let mut app = app();
    let player = |held_item: HeldItem, worn: Option<Vec<RecordId>>| Character::Record {
        kind: "player".into(),
        id: RecordId::new("player"),
        state: CharacterState { worn, held: Some(vec![held_item]) },
    };
    let (ws, shadow) = (RenderLayers::layer(3), RenderLayers::layer(4));
    let a = app.world_mut().spawn((player(held("torch", "hand_l", true), None), RoleLayers::world(), ws.clone(), crate::ShadowLayers(shadow.clone()))).id();
    run_until(&mut app, |w| w.get::<CharacterBody>(a).is_some() && w.get::<HeldProps>(a).is_some_and(|h| !h.props.is_empty()));
    app.update();
    assert!(view_copies(&mut app).is_empty(), "third person: no copies");
    app.world_mut().entity_mut(a).insert(RoleLayers::first_person(false));
    app.update();
    assert!(view_copies(&mut app).is_empty(), "first person without the view model: no copies");

    app.world_mut().entity_mut(a).insert(RoleLayers::first_person(true));
    app.update();
    let check = |app: &mut App| -> Vec<(Entity, Entity, ViewRole)> {
        let copies = view_copies(app);
        assert_eq!(copies.iter().map(|c| c.2).collect::<Vec<_>>(), [ViewRole::Arms, ViewRole::Held], "one copy per arm and held part");
        let w = app.world();
        for &(copy, part, role) in &copies {
            assert_eq!(w.get::<ChildOf>(copy).map(ChildOf::parent), Some(part), "{role:?}: a child of its part");
            assert_eq!(w.get::<RenderLayers>(copy), Some(&ws), "{role:?}: the copy on the root's layers");
            assert_eq!(w.get::<RenderLayers>(part), Some(&shadow), "{role:?}: the part shadow only");
            assert!(w.get::<NotShadowCaster>(copy).is_some() && w.get::<NoRayTracing>(copy).is_some(), "{role:?}: casts nothing, not traced");
            assert_eq!(w.get::<Mesh3d>(copy), w.get::<Mesh3d>(part));
            assert_eq!(w.get::<MeshMaterial3d<StandardMaterial>>(copy), w.get::<MeshMaterial3d<StandardMaterial>>(part));
            assert_eq!(w.get::<SkinnedMesh>(copy).map(|s| &s.joints), w.get::<SkinnedMesh>(part).map(|s| &s.joints), "{role:?}: the part's joints");
        }
        let body = w.get::<CharacterBody>(a).unwrap();
        for &m in &body.meshes {
            let role = w.get::<PartRole>(m).unwrap().0;
            let want = if role == ViewRole::Body { &ws } else { &shadow };
            assert_eq!(w.get::<RenderLayers>(m), Some(want), "{role:?}");
        }
        copies
    };
    let copies = check(&mut app);
    let arm = copies[0];
    assert!(app.world().get::<SkinnedMesh>(arm.0).is_some(), "the arm copy is skinned on the rig");
    let light = app.world().get::<HeldProps>(a).unwrap().props[0].light.unwrap();
    assert_eq!(app.world().get::<RenderLayers>(light), Some(&ws.union(&shadow)), "the torch's light slot keeps its layers");

    // A host swaps the torch's material (a self-lit variant): the copy follows.
    let lit = app.world_mut().resource_mut::<Assets<StandardMaterial>>().add(StandardMaterial::default());
    app.world_mut().entity_mut(copies[1].1).insert(MeshMaterial3d(lit.clone()));
    app.update();
    assert_eq!(app.world().get::<MeshMaterial3d<StandardMaterial>>(copies[1].0).map(|m| m.0.clone()), Some(lit));

    // A held change: the old prop's copy goes with it, the new prop gets one; the arm copy stays.
    *app.world_mut().get_mut::<Character>(a).unwrap() = player(held("sword", "hand_r", false), None);
    run_until(&mut app, |w| w.get::<HeldProps>(a).is_some_and(|h| h.props.iter().any(|p| p.item.item == RecordId::new("sword"))));
    app.update();
    let after_held = check(&mut app);
    assert_eq!(after_held[0].0, arm.0, "the outfit and its arm copy are kept");
    assert!(app.world().get_entity(copies[1].0).is_err(), "the old prop's copy despawned");

    // An outfit swap: the new arm part gets its own copy, the old one goes with its part.
    let meshes = app.world().get::<CharacterBody>(a).unwrap().meshes.clone();
    *app.world_mut().get_mut::<Character>(a).unwrap() = player(held("sword", "hand_r", false), Some(vec![]));
    run_until(&mut app, |w| w.get::<CharacterBody>(a).unwrap().meshes != meshes);
    app.update();
    let after_swap = check(&mut app);
    assert_ne!(after_swap[0].1, arm.1, "a new arm part");
    assert!(app.world().get_entity(arm.0).is_err(), "the old arm copy despawned");

    // Back to third person: copies gone, every part on the root's layers.
    app.world_mut().entity_mut(a).insert(RoleLayers::world());
    app.update();
    assert!(view_copies(&mut app).is_empty());
    assert!(after_swap.iter().all(|c| app.world().get_entity(c.0).is_err()));
    assert!(after_swap.iter().all(|c| app.world().get::<RenderLayers>(c.1) == Some(&ws)));
}

/// FP-2a through travel: the copies on the worldspace's content layer, their parts on its
/// shadow-only layer, both moving with the worldspace.
#[cfg(feature = "world")]
#[test]
fn view_copies_follow_travel() {
    use crate::{RoleLayers, ViewRole};
    use bevy::camera::visibility::RenderLayers;
    use sf_world_bevy::{WorldspaceLayers, WorldspaceRoot};
    let mut app = app();
    let worldspace = |w: &mut World, id: &str, layer: usize| {
        w.spawn((WorldspaceRoot { id: sf_world::WorldspaceId::new(id), layer }, Transform::IDENTITY, Visibility::default())).id()
    };
    let outside = worldspace(app.world_mut(), "outside", 1);
    let inside = worldspace(app.world_mut(), "inside", 3);
    let player = Character::Record {
        kind: "player".into(),
        id: RecordId::new("player"),
        state: CharacterState { worn: None, held: Some(vec![held("torch", "hand_l", true)]) },
    };
    let a = app.world_mut().spawn((player, RoleLayers::first_person(true), Transform::IDENTITY, Visibility::default(), ChildOf(outside))).id();
    run_until(&mut app, |w| w.get::<CharacterBody>(a).is_some() && w.get::<HeldProps>(a).is_some_and(|h| !h.props.is_empty()));
    app.update();
    let layers = |app: &mut App| -> Vec<(ViewRole, Option<RenderLayers>, Option<RenderLayers>)> {
        let copies = view_copies(app);
        let w = app.world();
        copies.iter().map(|&(copy, part, role)| (role, w.get::<RenderLayers>(copy).cloned(), w.get::<RenderLayers>(part).cloned())).collect()
    };
    let expect = |ws: WorldspaceLayers| {
        [ViewRole::Arms, ViewRole::Held].map(|r| (r, Some(ws.content()), Some(ws.shadow_only()))).to_vec()
    };
    assert_eq!(layers(&mut app), expect(WorldspaceLayers::of(1)));
    app.world_mut().entity_mut(a).insert(ChildOf(inside));
    app.update();
    assert_eq!(layers(&mut app), expect(WorldspaceLayers::of(3)));
}
