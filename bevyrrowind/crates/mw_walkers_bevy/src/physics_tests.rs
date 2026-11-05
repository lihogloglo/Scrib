//! Knockdown and death routing (combat-adapter.md "Ragdoll routing"): a hard impact or a death goes to the
//! [`RagdollTakeover`] hook when it claims the character, else to the powered-ragdoll knockdown or the death clip and
//! limp ragdoll; without the hook (the app built without `delirium`) every one is the ragdoll's. The hook here mirrors
//! the Delirium claim (human skeleton on a mover: `mw_delirium_bevy::knockdown_reaction` / `death_reaction`, tested
//! there) without the GPL-3.0-only crate.

use std::sync::Arc;

use bevy_app::{App, Update};
use bevy_ecs::prelude::*;
use bevy_math::Vec3;
use bevy_transform::components::{GlobalTransform, Transform};
use sf_anim::fixtures::{biped, gait};
use sf_anim::{ClipRules, CookSettings, cook_clip};
use sf_anim_bevy::{AnimAsset, Animator};
use sf_anim_graph::{CompileSource, compile_str};
use sf_bake::ProductKey;
use sf_character::OutfitKey;
use sf_character_bevy::{CharacterBody, OutfitCacheKey, SmoothBodies};
use sf_controller_bevy::{Held, Mover, MoverParams};
use sf_coords::WorldPos;
use sf_physics_bevy::{ImpactOnCharacter, Ragdoll, RagdollMode, RagdollSource};

use super::{Dead, KNOCKDOWN_IMPULSE, KnockedDown, PhysicsTakeover, RagdollTakeover, TakeoverKind, Victim, Victims, kill, knockdown};

const HUMAN: &str = "meshes/base_anim.nif";
const CREATURE: &str = "meshes/r/guar.nif";

struct OneClip(sf_anim::Clip);

impl CompileSource for OneClip {
    fn clip(&mut self, reference: &str) -> Result<(ProductKey, sf_anim::Clip), String> {
        Ok((ProductKey::new("anim.cook", reference), self.0.clone()))
    }
    fn subgraph(&mut self, _: &str) -> Result<sf_anim_graph::asset::GraphAsset, String> {
        Err("none".into())
    }
}

fn asset() -> Arc<AnimAsset> {
    let skeleton = biped();
    let src = gait(&skeleton, "walk", 1.0, 1.4, 0.5, 0.6, &[("thigh_l", "foot_l", 0.0), ("thigh_r", "foot_r", 0.5)]);
    let clip = cook_clip(&skeleton, &src, &ClipRules::default(), &CookSettings::default()).unwrap();
    let graph = compile_str("walk", r#"(nodes: {"walk": Clip(clip: "walk")}, layers: [(node: "walk")])"#, &skeleton, &mut OneClip(clip.clone())).unwrap();
    let clips = graph.clips.iter().map(|_| Arc::new(clip.clone())).collect();
    Arc::new(AnimAsset::new(Arc::new(skeleton), Arc::new(graph), clips).unwrap())
}

/// The claim the Delirium hook makes, recorded on the root.
#[derive(Component)]
struct Claimed {
    impulse: Vec3,
    kind: TakeoverKind,
}

fn delirium_like() -> RagdollTakeover {
    RagdollTakeover(Arc::new(|t: &PhysicsTakeover, commands: &mut Commands| {
        let claim = t.mover && t.skeleton == HUMAN;
        if claim {
            commands.entity(t.root).insert(Claimed { impulse: t.impulse, kind: t.kind });
        }
        claim
    }))
}

/// A walker (root: `CharacterBody` + mover; body: animator) hit once with `impulse` along +X; returns (root, body).
fn hit(hook: Option<RagdollTakeover>, skeleton: &str, impulse: f32) -> (App, Entity, Entity) {
    let mut app = App::new();
    app.add_message::<ImpactOnCharacter>().insert_resource(RagdollSource(Arc::new(|_| None))).add_systems(Update, knockdown);
    if let Some(hook) = hook {
        app.insert_resource(hook);
    }
    let body = app.world_mut().spawn((Transform::IDENTITY, GlobalTransform::IDENTITY, Animator::new(asset()))).id();
    let key = OutfitCacheKey { skeleton: skeleton.to_owned(), outfit: OutfitKey(0), lod: 0, shape: 0, bodies: SmoothBodies::default() };
    let character = CharacterBody { body, skeleton: skeleton.to_owned(), key, outfit: Arc::default(), meshes: Vec::new(), cloth_joints: Vec::new() };
    let mover = Mover::new(WorldPos::ORIGIN, 0.0, MoverParams::default(), WorldPos::ORIGIN);
    let root = app.world_mut().spawn((character, mover, Transform::IDENTITY)).id();
    app.world_mut().write_message(ImpactOnCharacter { entity: root, impulse, point: WorldPos::ORIGIN.offset(Vec3::Y), direction: Vec3::X });
    app.update();
    (app, root, body)
}

fn powered_knockdown(app: &App, root: Entity, body: Entity) -> bool {
    let ragdoll = app.world().get::<Ragdoll>(body).is_some_and(|r| matches!(r.mode, RagdollMode::Knockdown { .. }));
    ragdoll && app.world().get::<KnockedDown>(root).is_some() && app.world().get::<Held>(root).is_some()
}

#[test]
fn human_knockdown_goes_to_the_takeover() {
    let (app, root, body) = hit(Some(delirium_like()), HUMAN, KNOCKDOWN_IMPULSE + 5.0);
    let claimed = app.world().get::<Claimed>(root).expect("the hook took the knockdown");
    assert_eq!((claimed.impulse, claimed.kind), (Vec3::X * (KNOCKDOWN_IMPULSE + 5.0), TakeoverKind::Knockdown));
    assert!(app.world().get::<Ragdoll>(body).is_none(), "a taken-over knockdown spawns no powered ragdoll");
    assert!(app.world().get::<KnockedDown>(root).is_none(), "the takeover owns the bookkeeping (its hand-back)");
}

#[test]
fn creature_knockdown_keeps_the_powered_ragdoll() {
    let (app, root, body) = hit(Some(delirium_like()), CREATURE, KNOCKDOWN_IMPULSE + 5.0);
    assert!(app.world().get::<Claimed>(root).is_none());
    assert!(powered_knockdown(&app, root, body), "declined: Ragdoll {{ Knockdown }}, KnockedDown, Held");
}

#[test]
fn without_the_hook_humans_get_the_powered_ragdoll() {
    let (app, root, body) = hit(None, HUMAN, KNOCKDOWN_IMPULSE + 5.0);
    assert!(powered_knockdown(&app, root, body));
}

#[test]
fn light_hits_never_reach_the_hook() {
    let (app, root, body) = hit(Some(delirium_like()), HUMAN, KNOCKDOWN_IMPULSE * 0.5);
    assert!(app.world().get::<Claimed>(root).is_none(), "below the knockdown threshold: a HitReact kick only");
    assert!(app.world().get::<Ragdoll>(body).is_none());
    assert!(app.world().get::<Held>(root).is_none());
}

/// Kills every living character (the console's `kill` without the camera pick).
fn kill_all(victims: Victims, takeover: Option<Res<RagdollTakeover>>, mut animators: Query<&mut Animator>, mut commands: Commands) {
    for (root, body, mover, held, down) in &victims {
        let victim = Victim { root, body, mover, held, down };
        kill(victim, WorldPos::ORIGIN.offset(Vec3::Y), Vec3::ZERO, takeover.as_deref(), &mut animators, &mut commands);
    }
}

/// A walker (as [`hit`]) with `extra` on its root, killed once; returns (root, body).
fn die(hook: Option<RagdollTakeover>, skeleton: &str, extra: impl Bundle) -> (App, Entity, Entity) {
    let mut app = App::new();
    app.add_systems(Update, kill_all);
    if let Some(hook) = hook {
        app.insert_resource(hook);
    }
    let body = app.world_mut().spawn((Transform::IDENTITY, GlobalTransform::IDENTITY, Animator::new(asset()))).id();
    let key = OutfitCacheKey { skeleton: skeleton.to_owned(), outfit: OutfitKey(0), lod: 0, shape: 0, bodies: SmoothBodies::default() };
    let character = CharacterBody { body, skeleton: skeleton.to_owned(), key, outfit: Arc::default(), meshes: Vec::new(), cloth_joints: Vec::new() };
    let mover = Mover::new(WorldPos::ORIGIN, 0.0, MoverParams::default(), WorldPos::ORIGIN);
    let root = app.world_mut().spawn((character, mover, Transform::IDENTITY, extra)).id();
    app.update();
    (app, root, body)
}

fn limp_death(app: &App, body: Entity) -> bool {
    app.world().get::<Ragdoll>(body).is_some_and(|r| matches!(r.mode, RagdollMode::Death { .. }))
}

/// Owner 2026-10-30: a killed human walker dies under the takeover (Delirium), not the death clip + limp ragdoll.
#[test]
fn human_death_goes_to_the_takeover() {
    let (app, root, body) = die(Some(delirium_like()), HUMAN, ());
    let claimed = app.world().get::<Claimed>(root).expect("the hook took the death");
    assert_eq!(claimed.kind, TakeoverKind::Death);
    assert!(!limp_death(&app, body), "no Ragdoll {{ Death }}");
    assert!(app.world().get::<Dead>(root).is_some() && app.world().get::<Held>(root).is_some(), "Dead (and Held)");
}

#[test]
fn creature_death_keeps_the_limp_ragdoll() {
    let (app, root, body) = die(Some(delirium_like()), CREATURE, ());
    assert!(app.world().get::<Claimed>(root).is_none());
    assert!(limp_death(&app, body) && app.world().get::<Dead>(root).is_some());
}

#[test]
fn without_the_hook_humans_die_as_the_limp_ragdoll() {
    let (app, root, body) = die(None, HUMAN, ());
    assert!(limp_death(&app, body) && app.world().get::<Dead>(root).is_some());
}

/// A character the takeover already holds (`Held`, not `KnockedDown`: a Delirium knockdown or push) can be killed:
/// the hook gets the death and turns it terminal.
#[test]
fn death_of_a_held_character_goes_to_its_takeover() {
    let (app, root, body) = die(Some(delirium_like()), HUMAN, Held);
    assert_eq!(app.world().get::<Claimed>(root).map(|c| c.kind), Some(TakeoverKind::Death));
    assert!(!limp_death(&app, body) && app.world().get::<Dead>(root).is_some());
}

/// Held by a layer that declines the death: not killed (the old ragdoll would fight the layer for the body).
#[test]
fn a_declined_death_of_a_held_character_is_no_death() {
    let (app, root, body) = die(None, HUMAN, Held);
    assert!(!limp_death(&app, body) && app.world().get::<Dead>(root).is_none());
}

/// A powered-ragdoll knockdown dies in its ragdoll: the hook isn't asked (the body is the ragdoll's already).
#[test]
fn death_of_a_powered_knockdown_stays_in_the_ragdoll() {
    let (app, root, body) = die(Some(delirium_like()), HUMAN, KnockedDown { body_local: Transform::IDENTITY });
    assert!(app.world().get::<Claimed>(root).is_none());
    assert!(limp_death(&app, body) && app.world().get::<Dead>(root).is_some());
}
