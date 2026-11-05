//! Delirium (`mw_delirium`, GPL-3.0-only) takes over animated Morrowind characters (ROADMAP 3.2b slice 2a).
//!
//! - **Takeover**: a [`DeliriumReaction`] on a character's root (the entity with `CharacterBody`). Two posed frames
//!   later (the second gives part velocities) its evaluated pose becomes Delirium parts (`bones_to_parts`), the agent
//!   spawns at the actor's size (`Build::scaled` from its scale) on a floor at the ground and starts `bodyBalance`
//!   (`catchFall` when already falling). The root goes `Held` (so `Dormant`: no step, no motion or ground
//!   feed, the processing range leaves it alone) until the hand-back.
//! - **Hits** (owner 2026-10-30, 2026-11-05: "believable reactions depending on the object (mass, direction)"): the
//!   walkers' capsule impacts on human walkers come here ([`hit_reaction`], wired by the host into `mw_walkers_bevy`'s
//!   `RagdollTakeover` hook, kind `Hit`) from [`HitRules::takeover`] up instead of the kinematic hit-react op or the
//!   powered ragdoll: the impact's momentum pushes the struck part with the balancer on (`Takeover::hit`, the
//!   Euphoria / RDR2 hit reaction: stagger, catch steps, a fall when the balance fails); past
//!   [`HitRules::certain_fall`] the push is followed by a knockdown. A body it already holds takes every hit as a
//!   further push. Game-decided knockdowns ([`knockdown_reaction`], kind `Knockdown`) push first too, then go down.
//! - **Deaths** (same hook, kind `Death`; [`death_reaction`]): taken over at once (or a live takeover turns
//!   terminal), `Takeover::kill_by_hit` with a killing blow (its push and a weakened balancer for the dying window,
//!   then the fall) or `Takeover::kill` without (`catchFall` at once), then near-limp `bodyRelax`, never handed back: the root stays `Held`
//!   with [`DeliriumCorpse`]; once at rest the takeover is dropped and the body keeps its last `PhysicsPose` (frozen,
//!   no more steps). A death it can't take after the claim (no rig) falls back to the limp ragdoll.
//! - **Step**: `FixedUpdate`, one Delirium step per fixed tick.
//! - **Copy-back**: every frame the parts (interpolated between the last two steps) go into the takeover pose
//!   (`pose_to_bones`; bones without a part keep the takeover pose) and to the animator as a [`PhysicsPose`] at
//!   weight 1, the death/knockdown ragdoll's path (`sf_physics_bevy` read-back): the graph shows it, ops skip it, root
//!   motion is off. The body's frame follows the pelvis (heading from the hips, on the floor) as in the knockdown
//!   mode, at the uniform height scale (restored at the hand-back). On the floor, not at the mover's render height: the
//!   root (its mover too, `height_offset` zeroed) takes the settled height at the hand-back, not the pelvis.
//! - **Hand-back**: recovered (`mw_delirium::Rules`: no catch step, centre of mass slow) → the blend-out (slice 2b):
//!   the physics-pose weight follows `Takeover::weight` from 1 to 0, the graph's pose (the idle, kept alive under
//!   `Held`) shows through, the frame stays where the blend started; at 0 the graph simply carries on (no
//!   inertialization). A push during it brings the physics back. Timeout (or a blow-up) → the graph inertializes from
//!   the shown pose (`inertialize_from`). Down → the graph's `knocked_out` flag is set while it lies there; once still
//!   (or after a few seconds) Delirium picks the get-up nearest the way the body lies: Morrowind's `Knockout@tail`
//!   (on its side; the baseline, kept within `Rules::baseline_margin` of the nearest) or the graph's [`GET_UP_SLOTS`]
//!   (optional LAFAN1 face up / face down / sides), chosen by the `get_up` param, and rolls the body over to its first pose's tilt only past the tolerance (`Takeover::set_get_ups`,
//!   DE-1); lying, the body is relaxed (`bodyRelax`). Then the frame moves under that pose's pelvis (yaw and
//!   translation only, `get_up_root` without its roll) and the get-up plays out of the lying pose. The root takes the body's move (the walkers'
//!   knockdown glue) and wakes; [`DeliriumHandedBack`] tells the host.
//! - **World**: one flat floor at the ground under the pelvis (one ray down through the walkers' [`Physics`] ground
//!   probe; no physics or no hit: the mover's render height), lowered to the lowest part when the pose starts below it
//!   (on a slope, the downhill foot). Next: copy the Rapier statics around the actor.
//! - **Click-to-push** ([`DeliriumPlugin::click`], dev builds): in the free camera only (F11; `PlayerMode` off: in the
//!   player's views the left click is the attack), left click on a human NPC picks the Delirium part under the screen
//!   centre (capsules; the cursor is grabbed there, a dot marks it) and takes it over with an impulse there along the view ray (Ctrl 20, plain 45, Shift 80 N s); on a
//!   taken-over one it pushes that part.
//!
//! Human skeletons only ([`HUMAN_SKELETONS`]): Delirium's asset is one humanoid per process; beast races are refused
//! and logged. Mover-driven characters only (walkers: the [`Mover`] on the `CharacterBody` entity, which holds still
//! under `Held` and takes the body's move at the hand-back). An action character (the player, the training dummy:
//! the `CharacterBody` on an avatar under an `sf_action_bevy` body) is refused: `Held` doesn't stop its body and the
//! hand-back's move would land in the avatar's local transform, leaving the body beside its capsule for good.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use bevy_app::{App, FixedUpdate, Plugin, Startup, Update};
use bevy_camera::visibility::Visibility;
use bevy_camera::{Camera, Camera3d};
use bevy_color::Color;
use bevy_ecs::prelude::*;
use bevy_input::ButtonInput;
use bevy_input::keyboard::KeyCode;
use bevy_input::mouse::MouseButton;
use bevy_log::{info, warn};
use bevy_math::{Affine3A, Quat, Vec3};
use bevy_time::{Fixed, Time};
use bevy_transform::components::{GlobalTransform, Transform};
use bevy_ui::{BackgroundColor, BorderRadius, Node, PositionType, UiRect, percent, px};
use mw_delirium::delirium::prelude::Transform as Part;
use mw_delirium::delirium::ragdoll::Build;
use mw_delirium::proxies::{PartShapes, part_shapes, shape_frames};
use mw_delirium::takeover::pick;
use mw_delirium::{Body, HandBack, Options, Outcome, Rules, Takeover, authoring_from_model, bones_to_parts, character_from_skeleton, pose_to_bones};
use mw_game_core::player::PlayerMode;
use sf_anim::Trs;
use sf_anim_bevy::{Animator, PhysicsPose};
use sf_character_bevy::CharacterBody;
use sf_controller::ProbeQuery;
use sf_controller_bevy::{ControllerSystems, Dormant, GroundProbe, Held, Mover};
use sf_coords::{WorldPos, WorldTransform};
use sf_physics_bevy::sf_ragdoll::{get_up_root, yaw_of};
use sf_physics_bevy::sf_ragdoll::ShapeSet;
use sf_physics_bevy::{BodyProxies, FrameQuery, Physics, ProxyHolder, Ragdoll, RagdollMode, ShapeSystems, move_frame, world_frame};
use sf_world_bevy::{WorldPosOf, WorldspaceRoot};

/// The skeletons Delirium's humanoid asset fits (male and female Morrowind humans).
pub const HUMAN_SKELETONS: [&str; 2] = ["meshes/base_anim.nif", "meshes/base_anim_female.nif"];

/// The graph flag the fallen hand-back sets while it lies there and clears for the get-up (`mw_graphs` `knocked_out`).
pub const KNOCKED_OUT: &str = "knocked_out";

/// The get-up clip (`mw_graphs` `knockout_end`) when the graph has none of [`GET_UP_SLOTS`].
pub const GET_UP_CLIP: &str = "Knockout@tail";

/// The graph's get-up choice (`mw_graphs::build::params::GET_UP`): 0 `Knockout@tail`, `i + 1` [`GET_UP_SLOTS`]`[i]`.
pub const GET_UP_PARAM: &str = "get_up";

/// The get-up clip slots by how the body lies (`mw_graphs::build::GET_UPS`, sources.ron `getup.*`: LAFAN1 takes).
pub const GET_UP_SLOTS: [&str; 4] = ["getup.supine", "getup.prone", "getup.side_left", "getup.side_right"];

/// Hand-back inertialization, s (timeouts, blow-ups, the get-up).
pub const HAND_BACK_BLEND: f32 = 0.3;

/// A physics weight under this at the hand-back counts as blended out (no inertialization).
pub const BLENDED_OUT: f32 = 0.1;

/// The takeover's ground ray: from the pelvis straight down this far, m.
pub const GROUND_RAY: f32 = 4.0;

/// Click impulses, N s: Ctrl, plain, Shift.
pub const CLICK_IMPULSE: [f32; 3] = [20.0, 45.0, 80.0];

/// Adds the takeover systems; `click`: the dev click-to-push tool.
#[derive(Default)]
pub struct DeliriumPlugin {
    pub click: bool,
}

impl Plugin for DeliriumPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Takeovers>()
            .init_resource::<DeliriumStats>()
            .add_message::<DeliriumHandedBack>()
            .add_message::<DeliriumPush>()
            .add_systems(FixedUpdate, step)
            // After the controller step: a root move (hand-back) is never a frame behind.
            .add_systems(Update, (start, read_back.after(ControllerSystems::Step).before(ShapeSystems::Activate)).chain());
        if self.click {
            app.add_systems(Startup, spawn_click_dot).add_systems(Update, (click.before(start), show_click_dot));
        }
    }
}

/// Take this character over (on its root, the `CharacterBody` entity), optionally with a hit; on one already taken
/// over, the hit pushes its body. `knock_down`: it goes down (game-decided, or past [`HitRules::certain_fall`]): after
/// the hit's push the balancer gives up (`Takeover::knock_down`: `Rules::knock_down_after` later, so it falls along
/// the hit), then the usual lie, get-up and hand-back; on a live one too (a no-op when already falling or down).
/// `kill`: a death (terminal; with a hit `Takeover::kill_by_hit`, its dying window; without, `Takeover::kill`), at
/// the takeover or on a live one: it falls, goes limp, is never handed back, and once at rest its pose freezes; the
/// root stays `Held` and gets [`DeliriumCorpse`]. `live_only`: only for a character Delirium already holds (a hit on
/// a `Held` root: whoever holds it, a further push only when it is Delirium), dropped otherwise.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct DeliriumReaction {
    pub hit: Option<DeliriumHit>,
    pub knock_down: bool,
    pub kill: bool,
    pub live_only: bool,
}

/// Which physics hits Delirium takes and how hard (combat-adapter.md "Ragdoll routing"; measured by
/// `mw_delirium/tests/takeover_real_install.rs` `hit_reactions_vs_impulse`, 80 kg body).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HitRules {
    /// A hit on a free walker is a takeover from this momentum up, N s (after [`Self::gain`]); lighter ones stay the
    /// walkers' kinematic hit-react op (the hook declines). 10 N s (0.125 m/s on 80 kg) is where the push first
    /// reads past the takeover's own settle sway (0.03-0.07 m at 5 N s; 0.09-0.15 m at 10 N s from the front/back).
    pub takeover: f32,
    /// From this momentum up, N s, the push is followed by a knockdown: Delirium's balancer recovers from far more
    /// than a person would (it stays up to 120 N s, 1.5 m/s, with 1.7-2.2 m of catch steps; falls on its own from
    /// about 160). 100 N s (1.25 m/s) is past a standing person's step recovery and above the dev click's Shift
    /// (80 N s, staggers).
    pub certain_fall: f32,
    /// The impact's momentum times this is the push: the hit's share that goes into the body. Our props have no
    /// restitution (they stop on a capsule), so the body takes their whole incoming momentum: 1. A bouncing source
    /// would give `1 + e` (its own data, when sources carry it). A `V` ball (3 kg at 12 m/s, 36 N s head-on)
    /// staggers the body 0.3-0.5 m along the hit within 1 s, between the dev click's Ctrl (20) and plain (45).
    pub gain: f32,
}

impl Default for HitRules {
    fn default() -> Self {
        Self { takeover: 10.0, certain_fall: 100.0, gain: 1.0 }
    }
}

/// The takeover request for a physics hit (combat-adapter.md "Ragdoll routing"; the walkers' `RagdollTakeover` hook,
/// kind `Hit`; the entry point combat's hits take too, with a weapon's mass times its speed): on a human skeleton
/// ([`HUMAN_SKELETONS`]) on a mover-driven character, the impact (`impulse`: N s, world, its momentum along its
/// direction) times [`HitRules::gain`] pushes the part nearest `point` with the balancer on, from
/// [`HitRules::takeover`] up, a knockdown after the push from [`HitRules::certain_fall`] up. `held` (something holds
/// the root already): every hit is a push, only when Delirium is the holder (`live_only`). `None`: the walkers' own
/// reaction (kinematic hit-react, powered ragdoll).
#[must_use]
pub fn hit_reaction(rules: &HitRules, skeleton: &str, mover: bool, held: bool, point: WorldPos, impulse: Vec3, joint: Option<u16>) -> Option<DeliriumReaction> {
    if !mover || !HUMAN_SKELETONS.contains(&skeleton) {
        return None;
    }
    let impulse = impulse * rules.gain;
    let n = impulse.length();
    (held || n >= rules.takeover).then_some(DeliriumReaction {
        hit: Some(DeliriumHit { point, impulse, part: None, joint }),
        knock_down: n >= rules.certain_fall,
        kill: false,
        live_only: held,
    })
}

/// A character Delirium killed (on the root): the takeover runs until the body is at rest, then its last pose stays as
/// the body's `PhysicsPose` (frozen: no more steps) and the root stays `Held`. New requests on a frozen corpse are
/// refused.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct DeliriumCorpse;

/// An impulse on the body.
#[derive(Clone, Copy, Debug)]
pub struct DeliriumHit {
    /// World point.
    pub point: WorldPos,
    /// N s, world.
    pub impulse: Vec3,
    /// Delirium part (`mw_delirium::body::NAMES`); `None`: the part on `joint`, else the part nearest `point`.
    pub part: Option<usize>,
    /// The struck skeleton joint (a hit on one of the body's proxies: the part that owns it).
    pub joint: Option<u16>,
}

/// A further push on a body Delirium holds (a [`DeliriumReaction`] with `live_only`, sent by [`request`]): every one
/// counts, several in one frame included (one per contact start on the body's proxies, PH-15); dropped when Delirium
/// doesn't hold the root.
#[derive(Message, Clone, Copy, Debug)]
pub struct DeliriumPush {
    pub root: Entity,
    pub hit: DeliriumHit,
}

/// Files a reaction on a character's root (the host's takeover hook): a hit on a body Delirium holds is a
/// [`DeliriumPush`] message (so two hits in a frame don't overwrite each other), with the knockdown, if any, as the
/// component; everything else the [`DeliriumReaction`] component.
pub fn request(commands: &mut Commands, root: Entity, reaction: DeliriumReaction) {
    if reaction.live_only
        && !reaction.kill
        && let Some(hit) = reaction.hit
    {
        commands.write_message(DeliriumPush { root, hit });
        if reaction.knock_down {
            commands.entity(root).try_insert(DeliriumReaction { hit: None, ..reaction });
        }
        return;
    }
    commands.entity(root).try_insert(reaction);
}

/// The part a hit lands on: its own, the part on its joint, else the part nearest `at` (sim).
fn hit_part(table: &mw_delirium::PartTable, hit: &DeliriumHit, parts: &[Part], at: Vec3) -> usize {
    hit.part
        .or_else(|| hit.joint.and_then(|j| table.parts.iter().position(|p| p.joint == usize::from(j))))
        .unwrap_or_else(|| nearest_part(parts, at))
}

/// The takeover request for a game-decided knockdown (combat-adapter.md "Ragdoll routing"; the walkers' takeover
/// hook, kind `Knockdown`): a human skeleton ([`HUMAN_SKELETONS`]) on a mover-driven character is taken over with
/// the blow as its hit (`impulse`: N s, world; the part nearest `point`; pushed first, so it falls along it) and goes
/// down (`knock_down`). `None`: beast races, creatures and action characters (no mover) keep the powered-ragdoll
/// knockdown. The takeover then owns the character (`Held`, the get-up, the root's move) until [`DeliriumHandedBack`].
#[must_use]
pub fn knockdown_reaction(skeleton: &str, mover: bool, point: WorldPos, impulse: Vec3) -> Option<DeliriumReaction> {
    let hit = DeliriumHit { point, impulse, part: None, joint: None };
    (mover && HUMAN_SKELETONS.contains(&skeleton)).then_some(DeliriumReaction { hit: Some(hit), knock_down: true, kill: false, live_only: false })
}

/// The takeover request for a death (combat-adapter.md "Ragdoll routing"; the walkers' takeover hook, kind `Death`):
/// as [`knockdown_reaction`], but terminal (`kill`), with the killing blow as its hit when it has one (`impulse` not
/// zero: pushed, then the dying window, `Takeover::kill_by_hit`; none: the collapse). A character Delirium already
/// holds turns terminal. `None`: the death clip and the limp ragdoll.
#[must_use]
pub fn death_reaction(skeleton: &str, mover: bool, point: WorldPos, impulse: Vec3) -> Option<DeliriumReaction> {
    let hit = (impulse.length_squared() > 1e-6).then_some(DeliriumHit { point, impulse, part: None, joint: None });
    (mover && HUMAN_SKELETONS.contains(&skeleton)).then_some(DeliriumReaction { hit, knock_down: false, kill: true, live_only: false })
}

/// A character went back to its animation (on the root).
#[derive(Message, Clone, Copy, Debug)]
pub struct DeliriumHandedBack {
    pub root: Entity,
    pub how: HandBack,
}

#[derive(Resource, Default, Clone, Copy, Debug)]
pub struct DeliriumStats {
    pub live: usize,
    pub started: usize,
    pub refused: usize,
    pub handed_back: usize,
    /// Dead bodies frozen at rest ([`DeliriumCorpse`]).
    pub corpses: usize,
    /// Last fixed tick's Delirium time over every live character, and the peak, µs.
    pub step_us: f32,
    pub peak_step_us: f32,
}

/// A skeleton's Delirium character and its authored build (for picks before a takeover).
struct Rig {
    body: Body,
    build: Build,
}

/// The sim frame of one takeover: sim space is the actor's authoring frame (Z up, facing +Y) at the takeover,
/// anchored at its feet.
#[derive(Clone, Copy)]
struct SimFrame {
    anchor: WorldPos,
    /// World axes (relative to `anchor`) from sim axes: a rotation.
    world_from_sim: Affine3A,
}

impl SimFrame {
    fn at(frame: &WorldTransform) -> Self {
        Self { anchor: frame.pos, world_from_sim: Affine3A::from_quat(yaw_of(frame.rotation)) * authoring_from_model() }
    }

    /// Sim from the model space of an entity at world frame `frame`, uniform scale `scale`.
    fn sim_from_model(&self, frame: &WorldTransform, scale: f32) -> Affine3A {
        let rel = Affine3A::from_scale_rotation_translation(Vec3::splat(scale), frame.rotation, self.anchor.delta_f32(frame.pos));
        self.world_from_sim.inverse() * rel
    }

    fn sim_point(&self, p: WorldPos) -> Vec3 {
        self.world_from_sim.inverse().transform_point3(self.anchor.delta_f32(p))
    }

    fn sim_dir(&self, v: Vec3) -> Vec3 {
        self.world_from_sim.inverse().transform_vector3(v)
    }

    fn world_point(&self, p: Vec3) -> WorldPos {
        self.anchor.offset(self.world_from_sim.transform_point3(p))
    }
}

struct Pending {
    pose: Vec<Trs>,
    frame: WorldTransform,
    scale: f32,
    secs: f32,
    hit: Option<DeliriumHit>,
    knock_down: bool,
    kill: bool,
}

struct Live {
    body: Entity,
    rig: Arc<Rig>,
    takeover: Takeover,
    sim: SimFrame,
    /// The takeover pose: bones without a part keep it.
    frozen: Vec<Trs>,
    scale: f32,
    /// The body's local transform before the takeover (its own scale included).
    body_local: Transform,
    /// The pelvis joint's model position at the takeover (the frame keeps its horizontal offset).
    pelvis: Vec3,
    yaw: Quat,
    /// The last two steps' part frames (interpolated by the fixed overstep).
    frames: [Vec<Part>; 2],
    /// The last two steps' physics weights (`Takeover::weight`, the blend-out).
    weights: [f32; 2],
    /// The weight shown last frame.
    shown_weight: f32,
    pushes: Vec<(usize, Vec3, Vec3)>,
    hand_back: Option<HandBack>,
    flagged: bool,
    last_pose: Option<Vec<Trs>>,
    /// The graph's get-ups, in the takeover's `set_get_ups` order: (`get_up` value, clip slot).
    get_ups: Vec<(f32, usize)>,
    /// The parts as a shape set and each part's turn to its capsule (`mw_delirium::proxies`): the body proxies.
    shapes: (Arc<ShapeSet>, Vec<Quat>),
}

#[derive(Resource, Default)]
pub struct Takeovers {
    rigs: HashMap<String, Option<Arc<Rig>>>,
    pending: HashMap<Entity, Pending>,
    live: HashMap<Entity, Live>,
}

impl Takeovers {
    /// Roots under Delirium now.
    pub fn live(&self) -> impl Iterator<Item = Entity> + '_ {
        self.live.keys().copied()
    }

    fn rig(&mut self, subject: &str, skeleton: &sf_anim::Skeleton) -> Option<Arc<Rig>> {
        self.rigs
            .entry(subject.to_owned())
            .or_insert_with(|| {
                if !HUMAN_SKELETONS.contains(&subject) {
                    info!("delirium: {subject} is not a human skeleton (beast races and creatures are not supported: one humanoid asset)");
                    return None;
                }
                match character_from_skeleton(skeleton, &Options::default()) {
                    Ok(body) => {
                        let build = body.character.build()?.0;
                        Some(Arc::new(Rig { body, build }))
                    }
                    Err(e) => {
                        warn!("delirium: no character for {subject}: {e}");
                        None
                    }
                }
            })
            .clone()
    }
}

fn lerp_parts(a: &[Part], b: &[Part], t: f32) -> Vec<Part> {
    a.iter().zip(b).map(|(a, b)| Part::from_translation(a.translation.lerp(b.translation, t)).with_rotation(a.rotation.slerp(b.rotation, t))).collect()
}

fn nearest_part(parts: &[Part], at: Vec3) -> usize {
    parts.iter().enumerate().min_by(|a, b| a.1.translation.distance_squared(at).total_cmp(&b.1.translation.distance_squared(at))).map_or(0, |p| p.0)
}

/// A death Delirium can't take (the host's hook claimed it, so nobody else will): the limp ragdoll at once, so the
/// character never stands there dead.
fn limp_death(skeleton: &str) -> Ragdoll {
    warn!("delirium: a death it can't take ({skeleton}): the limp ragdoll");
    Ragdoll { skeleton: skeleton.to_owned(), delay: 0.0, mode: RagdollMode::Death { crumple: true } }
}

fn refused_death(request: &DeliriumReaction, cb: &CharacterBody, commands: &mut Commands) {
    if request.kill {
        commands.entity(cb.body).try_insert(limp_death(&cb.skeleton));
    }
}

/// Requests: sample the pose, then (next posed frame) take over; hits on live takeovers become pushes.
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments, clippy::too_many_lines, clippy::type_complexity)]
fn start(
    time: Res<Time>,
    mut takeovers: ResMut<Takeovers>,
    mut stats: ResMut<DeliriumStats>,
    requests: Query<(Entity, &CharacterBody, &DeliriumReaction, Has<Mover>, Has<DeliriumCorpse>)>,
    animators: Query<(&Animator, Has<Ragdoll>)>,
    mut frames: ParamSet<(FrameQuery, Query<&mut Transform>)>,
    roots: Query<&WorldspaceRoot>,
    ground: Option<Res<Physics>>,
    mut pushes: MessageReader<DeliriumPush>,
    mut commands: Commands,
) {
    let dt = time.delta_secs();
    let Takeovers { pending, live, .. } = &mut *takeovers;
    // Further pushes on held bodies (PH-15): each one counts; a root Delirium doesn't hold drops them.
    for p in pushes.read() {
        if let Some(l) = live.get_mut(&p.root) {
            let at = l.sim.sim_point(p.hit.point);
            let part = hit_part(&l.rig.body.table, &p.hit, &l.frames[1], at);
            l.pushes.push((part, at, l.sim.sim_dir(p.hit.impulse)));
        }
    }
    pending.retain(|e, _| requests.contains(*e));
    for p in pending.values_mut() {
        p.secs += dt;
    }
    let mut ready = Vec::new();
    for (root, cb, request, mover, corpse) in &requests {
        if request.live_only && !live.contains_key(&root) {
            // A hit on a character something else holds (a frozen corpse, the limp ragdoll): not ours to push.
            commands.entity(root).try_remove::<DeliriumReaction>();
            continue;
        }
        if !mover {
            // An action character (module docs): its body would end beside its capsule.
            info!("delirium: refused {root} ({}): no mover on it (an action character)", cb.skeleton);
            stats.refused += 1;
            commands.entity(root).try_remove::<DeliriumReaction>();
            refused_death(request, cb, &mut commands);
            continue;
        }
        if let Some(l) = live.get_mut(&root) {
            if let Some(hit) = request.hit {
                let at = l.sim.sim_point(hit.point);
                let part = hit_part(&l.rig.body.table, &hit, &l.frames[1], at);
                l.pushes.push((part, at, l.sim.sim_dir(hit.impulse)));
            }
            if request.knock_down && !request.kill && !l.takeover.is_dead() {
                l.takeover.knock_down();
            }
            if request.kill && !l.takeover.is_dead() {
                // The push lands at the next step, inside the dying window.
                if request.hit.is_some() {
                    l.takeover.kill_by_hit();
                } else {
                    l.takeover.kill();
                }
                // A hand-back stepped this very frame doesn't happen: dead, it stays.
                if l.hand_back != Some(HandBack::Exploded) {
                    l.hand_back = None;
                }
                commands.entity(root).try_insert(DeliriumCorpse);
                let how = if l.takeover.fell() { "down" } else { "standing" };
                info!("delirium: {root} killed while taken over ({how}, {:.1} s after the takeover)", l.takeover.time);
            }
            commands.entity(root).try_remove::<DeliriumReaction>();
            continue;
        }
        if corpse {
            // Frozen: a new takeover would start from the corpse's pose with the balancer and could stand it up.
            info!("delirium: refused {root}: a frozen corpse");
            commands.entity(root).try_remove::<DeliriumReaction>();
            continue;
        }
        let Ok((animator, ragdoll)) = animators.get(cb.body) else { continue };
        if ragdoll || !HUMAN_SKELETONS.contains(&cb.skeleton.as_str()) {
            info!("delirium: refused {root} ({}{})", cb.skeleton, if ragdoll { ", already a ragdoll" } else { ": not a human skeleton" });
            stats.refused += 1;
            commands.entity(root).try_remove::<DeliriumReaction>();
            if !ragdoll {
                refused_death(request, cb, &mut commands);
            }
            continue;
        }
        if !animator.posed() {
            continue;
        }
        let Some((_, frame, scale)) = world_frame(cb.body, &frames.p0(), &roots) else { continue };
        let skeleton = &animator.asset.skeleton;
        let pose: Vec<Trs> = (0..skeleton.len()).map(|j| animator.output.pose.get(j)).collect();
        match pending.remove(&root) {
            None => {
                pending.insert(root, Pending { pose, frame, scale, secs: 0.0, hit: request.hit, knock_down: request.knock_down, kill: request.kill });
            }
            // A death asked between the two frames (it replaced a knockdown's request) still counts.
            Some(prev) => ready.push((root, cb.body, cb.skeleton.clone(), Pending { kill: prev.kill || request.kill, ..prev }, pose, frame, scale)),
        }
    }
    for (root, body, subject, prev, pose, frame, scale) in ready {
        commands.entity(root).try_remove::<DeliriumReaction>();
        let Ok((animator, _)) = animators.get(body) else { continue };
        let skeleton = &animator.asset.skeleton;
        let Some(rig) = takeovers.rig(&subject, skeleton) else {
            stats.refused += 1;
            if prev.kill {
                commands.entity(body).try_insert(limp_death(&subject));
            }
            continue;
        };
        let sim = SimFrame::at(&frame);
        let parts = bones_to_parts(&rig.body.table, skeleton, &pose, &sim.sim_from_model(&frame, scale));
        let before = bones_to_parts(&rig.body.table, skeleton, &prev.pose, &sim.sim_from_model(&prev.frame, prev.scale));
        // The body's own scale: height on Y, build on X/Z; Delirium scales lengths by the height and mass by the volume.
        let Some((body_local, own)) = frames.p1().get_mut(body).ok().map(|mut local| {
            let before = *local;
            local.scale = Vec3::splat(before.scale.y);
            (before, before.scale)
        }) else {
            continue;
        };
        let weight = scale.powi(3) * (own.x * own.z) / (own.y * own.y).max(1e-6);
        #[allow(clippy::cast_precision_loss)]
        let seed = (root.to_bits() % 1000) as f32 / 1000.0;
        // The floor at the ground under the pelvis (`Takeover::start` lowers it to the lowest part); no hit: at the
        // frame (the mover's render height).
        let ground_z = ground
            .as_deref()
            .and_then(|g| g.probe_one(ProbeQuery::down(sim.world_point(parts[0].translation), GROUND_RAY)))
            .map_or(0.0, |hit| sim.sim_point(hit.point).z);
        let mut takeover = Takeover::start(&rig.body, &parts, Some((&before, prev.secs.max(1e-3))), Some((scale, weight)), ground_z, seed, Rules::default());
        // Down, it takes the get-up nearest the way it lies, rolling over to its first pose only past the tolerance
        // (DE-1: no turn left to the blend).
        let get_ups = get_up_clips(animator);
        let tilts: Vec<Vec3> = get_ups.iter().map(|&(_, c)| mw_delirium::clip_tilt(&rig.body.table, skeleton, &first_pose(animator, c).to_vec())).collect();
        takeover.set_get_ups(&tilts);
        // The hit's push, then the outcome (a knockdown or a death after the push: it falls along the hit).
        let outcome = match (prev.kill, prev.knock_down) {
            (true, _) => Outcome::Kill,
            (false, true) => Outcome::KnockDown,
            (false, false) => Outcome::React,
        };
        let hit = prev.hit.map(|h| {
            let at = sim.sim_point(h.point);
            let part = hit_part(&rig.body.table, &h, &parts, at);
            takeover.hit(part, at, sim.sim_dir(h.impulse), outcome);
            (part, h.impulse.length())
        });
        if hit.is_none() {
            match outcome {
                Outcome::Kill => takeover.kill(),
                Outcome::KnockDown => takeover.knock_down(),
                Outcome::React => {}
            }
        }
        if prev.kill {
            commands.entity(root).try_insert(DeliriumCorpse);
        }
        let model = mw_delirium::pose::model_pose(skeleton, &pose);
        let pelvis = Vec3::from(model[rig.body.table.parts[0].joint].translation);
        info!(
            "delirium: took over {root} ({subject}, scale {scale:.2}) with {}{}, floor {:+.2} m (ground {ground_z:+.2}){}",
            takeover.behaviour,
            if prev.kill { " (dead)" } else { "" },
            takeover.floor,
            hit.map_or(String::new(), |(p, i)| format!(", {i:.0} N s on {}", mw_delirium::body::NAMES[p]))
        );
        commands.entity(root).try_insert(Held);
        stats.started += 1;
        // The parts as body proxies (character-shapes S3): balls and props keep hitting the held body.
        let PartShapes { set, turns } = part_shapes(&rig.body.table, &takeover.build, skeleton);
        let shapes = (Arc::new(set), turns);
        takeovers.live.insert(
            root,
            Live {
                body,
                rig,
                frames: [parts.clone(), parts],
                weights: [1.0; 2],
                shown_weight: 1.0,
                takeover,
                sim,
                frozen: pose,
                scale,
                body_local,
                pelvis,
                yaw: yaw_of(frame.rotation),
                pushes: Vec::new(),
                hand_back: None,
                flagged: false,
                last_pose: None,
                get_ups,
                shapes,
            },
        );
    }
    stats.live = takeovers.live.len();
}

/// One Delirium step per fixed tick.
#[allow(clippy::needless_pass_by_value)]
fn step(fixed: Res<Time<Fixed>>, mut takeovers: ResMut<Takeovers>, mut stats: ResMut<DeliriumStats>) {
    let dt = fixed.delta_secs();
    let t0 = Instant::now();
    for (root, l) in &mut takeovers.live {
        if l.hand_back.is_some() || l.takeover.settled() {
            continue;
        }
        for (part, at, impulse) in l.pushes.drain(..) {
            l.takeover.push(part, at, impulse);
        }
        let was = l.takeover.blending();
        l.hand_back = l.takeover.step(dt);
        if !was && l.takeover.blending() {
            info!("delirium: {root} recovered, blending out to the animation ({:.1} s after the takeover)", l.takeover.time);
        }
        let now = l.takeover.frames();
        l.frames = [std::mem::replace(&mut l.frames[1], now.clone()), now];
        l.weights = [l.weights[1], l.takeover.weight];
    }
    stats.step_us = t0.elapsed().as_secs_f32() * 1e6;
    stats.peak_step_us = stats.peak_step_us.max(stats.step_us);
}

/// Where the body's frame goes while Delirium has it: on the floor under the pelvis (keeping the pelvis' horizontal
/// offset in the model), heading from the hips (kept while they are near vertical). On the floor, not at the
/// takeover's render height: the hand-back's root move then carries the settled height.
fn follow_frame(l: &mut Live, parts: &[Part]) -> WorldTransform {
    let left = l.sim.world_from_sim.transform_vector3(parts[1].translation - parts[4].translation).with_y(0.0);
    if left.length() > 0.05 {
        let facing = left.normalize().cross(Vec3::Y);
        l.yaw = Quat::from_rotation_y(facing.x.atan2(facing.z));
    }
    // Sim Z is world Y: the floor's sim height is its height over the anchor.
    let pelvis = l.sim.world_from_sim.transform_point3(parts[0].translation).with_y(l.takeover.floor);
    let offset = l.yaw * (Vec3::new(l.pelvis.x, 0.0, l.pelvis.z) * l.scale);
    WorldTransform { pos: l.sim.anchor.offset(pelvis - offset), rotation: l.yaw, scale: Vec3::ONE }
}

/// The local pose that shows `parts` on a body at `frame`.
fn shown_pose(l: &Live, skeleton: &sf_anim::Skeleton, parts: &[Part], frame: &WorldTransform) -> Vec<Trs> {
    let mut pose = l.frozen.clone();
    pose_to_bones(&l.rig.body.table, skeleton, parts, &l.sim.sim_from_model(frame, l.scale).inverse(), &mut pose);
    pose
}

/// The graph's get-ups as (`get_up` value, clip slot): `Knockout@tail` first (value 0, the pick's baseline:
/// `Rules::baseline_margin`), then the [`GET_UP_SLOTS`] clips the graph resolved (a clip library such as LAFAN1
/// present; optional).
fn get_up_clips(animator: &Animator) -> Vec<(f32, usize)> {
    let graph = &animator.asset.graph;
    let tail = graph.clips.iter().position(|c| c.reference == GET_UP_CLIP).map(|c| (0.0, c));
    #[allow(clippy::cast_precision_loss)]
    let slots = GET_UP_SLOTS.iter().enumerate().filter_map(|(i, s)| graph.slot_clip(s).map(|c| ((i + 1) as f32, usize::from(c))));
    tail.into_iter().chain(slots).collect()
}

/// A get-up's name by its `get_up` value: [`GET_UP_CLIP`] (0) or its [`GET_UP_SLOTS`] slot.
fn get_up_name(value: f32) -> &'static str {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let slot = (value as usize).checked_sub(1).and_then(|i| GET_UP_SLOTS.get(i));
    slot.copied().unwrap_or(GET_UP_CLIP)
}

/// A clip's first frame (local pose).
fn first_pose(animator: &Animator, clip: usize) -> sf_anim::LocalPose {
    let mut local = sf_anim::LocalPose::rest(&animator.asset.skeleton);
    if let Some(clip) = animator.asset.clips.get(clip) {
        clip.sample(0.0, &mut local, None);
    }
    local
}

/// The get-up frame for a body lying in `pose` at `frame`: the picked get-up clip's first pelvis pose on the lying
/// pelvis (yaw and translation only), on the floor. Delirium picked the clip nearest the way the body lies and rolled
/// it to the clip's tilt first (`Takeover::set_get_ups`), so the yaw alone leaves the hand-back's blend little
/// whole-body turn (DE-1); no `ROLL_TO_BACK`.
fn get_up_frame(l: &Live, animator: &Animator, pose: &[Trs], frame: &WorldTransform) -> Option<WorldTransform> {
    let skeleton = &animator.asset.skeleton;
    let joint = l.rig.body.table.parts[0].joint;
    let local = first_pose(animator, l.get_ups.get(l.takeover.get_up()?)?.1);
    let mut model = sf_anim::ModelPose::new(skeleton.len());
    sf_anim::local_to_model(skeleton, &local, &mut model, &Affine3A::IDENTITY);
    let (_, clip_rot, clip_pos) = model.mats[joint].to_scale_rotation_translation();
    let (_, rot, pos) = mw_delirium::pose::model_pose(skeleton, pose)[joint].to_scale_rotation_translation();
    let body = WorldTransform { pos: frame.pos.offset(frame.rotation * (pos * l.scale)), rotation: frame.rotation * rot, scale: Vec3::ONE };
    let (_, rest, _) = skeleton.rest_model()[joint].to_scale_rotation_translation();
    let to = get_up_root(&body, (clip_rot, clip_pos), rest.inverse() * Vec3::Y, l.scale, false);
    let floor = l.sim.anchor.offset(Vec3::Y * l.takeover.floor);
    Some(WorldTransform { pos: to.pos.offset(Vec3::Y * to.pos.delta_f32(floor).y), ..to })
}

/// The held body's proxies (character-shapes S3, PH-15): the shown parts (`parts`, interpolated at `alpha` between the
/// last two steps, so a step behind the virtual clock) as each part's capsule frame in the world, with the parts'
/// velocities over the last step; written in place once the component is there.
fn write_proxies(root: Entity, l: &Live, parts: &[Part], fixed: &Time<Fixed>, alpha: f32, proxies: &mut Query<&mut BodyProxies>, commands: &mut Commands) {
    let rotation = Quat::from_mat3a(&l.sim.world_from_sim.matrix3);
    let step = fixed.timestep().as_secs_f32().max(1e-4);
    let poses = shape_frames(parts, &l.shapes.1)
        .into_iter()
        .map(|(c, r)| WorldTransform { pos: l.sim.world_point(c), rotation: (rotation * r).normalize(), scale: Vec3::ONE })
        .collect();
    let velocities = l.frames[0].iter().zip(&l.frames[1]).map(|(a, b)| l.sim.world_from_sim.transform_vector3((b.translation - a.translation) / step)).collect();
    let at = fixed.elapsed_secs_f64() - f64::from(step) * f64::from(1.0 - alpha);
    match proxies.get_mut(root) {
        Ok(mut p) if Arc::ptr_eq(&p.set, &l.shapes.0) => {
            p.poses = poses;
            p.velocities = velocities;
            p.at = at;
        }
        _ => {
            commands.entity(root).try_insert(BodyProxies { set: l.shapes.0.clone(), poses, velocities, at, holder: ProxyHolder::Layer });
        }
    }
}

/// Copy-back every frame (the parts as a physics pose at weight 1, the frame on the pelvis) and the hand-backs.
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments, clippy::too_many_lines)]
fn read_back(
    time: Res<Time>,
    fixed: Res<Time<Fixed>>,
    mut takeovers: ResMut<Takeovers>,
    mut stats: ResMut<DeliriumStats>,
    mut animators: Query<(&mut Animator, Option<&mut PhysicsPose>)>,
    mut frames: ParamSet<(FrameQuery, Query<&mut Transform>)>,
    mut movers: Query<&mut Mover>,
    roots: Query<&WorldspaceRoot>,
    mut proxies: Query<&mut BodyProxies>,
    mut handed: MessageWriter<DeliriumHandedBack>,
    mut commands: Commands,
) {
    let alpha = fixed.overstep_fraction();
    let dt = time.delta_secs().max(1e-4);
    let mut done = Vec::new();
    let mut corpses = Vec::new();
    let live = &mut takeovers.live;
    // Despawned characters drop their takeover.
    live.retain(|_, l| animators.contains(l.body));
    for (&root, l) in live.iter_mut() {
        let Ok((mut animator, physics)) = animators.get_mut(l.body) else { continue };
        let Some((_, mut frame, _)) = world_frame(l.body, &frames.p0(), &roots) else { continue };
        let parts = lerp_parts(&l.frames[0], &l.frames[1], alpha);
        let weight = l.weights[0] + (l.weights[1] - l.weights[0]) * alpha;
        let asset = animator.asset.clone();
        let skeleton = &asset.skeleton;
        let finite = parts.iter().all(|p| p.translation.is_finite() && p.rotation.is_finite());
        let pose = if finite {
            // The frame follows the pelvis under full physics; during the blend-out it stays put: the graph's pose
            // stands in it (no slide), and the physics pose is read in whichever frame it is.
            if weight >= 1.0 {
                let to = follow_frame(l, &parts);
                if let Ok(mut t) = frames.p1().get_mut(l.body) {
                    move_frame(&mut t, &frame, l.scale, &to);
                    frame = to;
                }
            }
            shown_pose(l, skeleton, &parts, &frame)
        } else {
            l.hand_back = Some(HandBack::Exploded);
            l.last_pose.clone().unwrap_or_else(|| l.frozen.clone())
        };
        if l.takeover.fell() && !l.flagged && !l.takeover.is_dead() {
            l.flagged = true;
            animator.set_bool(KNOCKED_OUT, true);
            animator.set_float(GET_UP_PARAM, 0.0);
            info!("delirium: {root} fell ({:.1} s after the takeover)", l.takeover.time);
        }
        // Dead, never handed back: a blow-up keeps the last good pose; at rest (or blown up) the pose freezes.
        let dead = l.takeover.is_dead();
        let exploded = dead && l.hand_back == Some(HandBack::Exploded);
        if l.hand_back.is_none() || exploded {
            let (pose, weight) = if exploded { (l.last_pose.clone().unwrap_or_else(|| l.frozen.clone()), 1.0) } else { (pose, weight) };
            match physics {
                Some(mut p) => {
                    p.pose.clone_from(&pose);
                    p.weight = weight;
                    p.weights.clear();
                }
                None => {
                    commands.entity(l.body).try_insert(PhysicsPose { pose: pose.clone(), weight, weights: Vec::new() });
                }
            }
            l.shown_weight = weight;
            l.last_pose = Some(pose);
            if finite && !exploded {
                write_proxies(root, l, &parts, &fixed, alpha, &mut proxies, &mut commands);
            }
            if exploded || (dead && l.takeover.settled()) {
                if exploded {
                    warn!("delirium: dead {root} blew up: frozen at its last good pose");
                }
                corpses.push((root, l.takeover.dead.unwrap_or(0.0)));
            }
            continue;
        }
        let Some(how) = l.hand_back else { continue };
        // Hand back: the graph inertializes out of the shown pose, read in the frame it goes on in.
        let previous = l.last_pose.take().unwrap_or_else(|| pose.clone());
        let get_up = if how == HandBack::Fallen { get_up_frame(l, &animator, &pose, &frame) } else { None };
        if let Some(up) = get_up {
            let left = l.takeover.get_up_left().unwrap_or(0.0).to_degrees();
            let picked = l.takeover.get_up();
            let (value, _) = picked.and_then(|i| l.get_ups.get(i).copied()).unwrap_or((0.0, 0));
            let name = get_up_name(value);
            animator.set_float(GET_UP_PARAM, value);
            // The pick and its gaps: each candidate's tilt from the body's when picked, deg.
            let angles = l.takeover.get_up_angles();
            let won = picked.and_then(|i| angles.get(i)).copied().unwrap_or(0.0);
            let others: Vec<String> = l
                .get_ups
                .iter()
                .zip(angles)
                .enumerate()
                .filter(|&(i, _)| Some(i) != picked)
                .map(|(_, (&(v, _), a))| format!("{} {:+.0}", get_up_name(v), (a - won).to_degrees()))
                .collect();
            info!(
                "delirium: {root} gets up with {name} (down {:.1} s, rolled over {:.2} s, {left:.0} deg left to the blend; picked at {:.0} deg, others {})",
                l.takeover.down.unwrap_or(0.0),
                l.takeover.rolling.unwrap_or(0.0),
                won.to_degrees(),
                if others.is_empty() { "none".to_owned() } else { others.join(", ") }
            );
            if let Ok(mut t) = frames.p1().get_mut(l.body) {
                move_frame(&mut t, &frame, l.scale, &up);
            }
            let lying = shown_pose(l, skeleton, &parts, &up);
            animator.instance.inertialize_from(&lying, &lying, 1.0 / 60.0, HAND_BACK_BLEND);
        } else if how == HandBack::Exploded {
            warn!("delirium: {root} blew up: handed back from its last good pose");
            animator.instance.inertialize_from(&previous, &previous, dt, HAND_BACK_BLEND);
        } else if how == HandBack::Balanced && l.shown_weight < BLENDED_OUT {
            // Blended out: the graph's pose is the one shown already; it carries on.
        } else {
            if how == HandBack::Fallen {
                info!("delirium: {root} has no {GET_UP_CLIP} clip: handed back lying, the graph blends it up");
            }
            animator.instance.inertialize_from(&pose, &previous, dt, HAND_BACK_BLEND);
        }
        if l.flagged {
            animator.set_bool(KNOCKED_OUT, false);
        }
        done.push((root, l.body, l.body_local, how, l.takeover.time));
    }
    // Corpses at rest leave the takeovers (no more steps or copy-back): the body keeps its last `PhysicsPose` and frame,
    // the root stays `Held` (and `DeliriumCorpse`).
    for (root, secs) in corpses {
        live.remove(&root);
        // Corpses are S6 (the Rapier ragdoll handover): no proxies on a frozen pose.
        commands.entity(root).try_remove::<BodyProxies>();
        stats.corpses += 1;
        info!("delirium: {root} lies dead, pose frozen ({secs:.1} s after the kill)");
    }
    // The root takes the body's move (heading included, its mover too), and the body its own local transform back.
    // The move carries the floor's height under the takeover's render height: the mover goes to its new render
    // position with `height_offset` zeroed, so the controller doesn't smooth the same height a second time.
    for (root, body, l0, how, secs) in done {
        live.remove(&root);
        let mut transforms = frames.p1();
        if let Ok([mut t, mut local]) = transforms.get_many_mut([root, body]) {
            let yaw = Mover::yaw_of(t.rotation * local.rotation * l0.rotation.inverse());
            let rotation = Quat::from_rotation_y(yaw);
            let shift = t.rotation * (t.scale * local.translation) - rotation * (t.scale * l0.translation);
            t.rotation = rotation;
            t.translation += shift;
            *local = l0;
            if let Ok(mut m) = movers.get_mut(root) {
                m.loco.position = m.loco.render_position().offset(shift);
                m.loco.height_offset = 0.0;
                m.loco.height_offset_velocity = 0.0;
                m.loco.motion.yaw = yaw;
                m.loco.motion.facing_goal = yaw;
                m.loco.motion.angular_velocity = 0.0;
            }
        }
        commands.entity(body).try_remove::<PhysicsPose>();
        // The capsule comes back with the mover (no `Dormant`); the proxies go.
        commands.entity(root).try_remove::<(Held, Dormant, BodyProxies)>();
        stats.handed_back += 1;
        handed.write(DeliriumHandedBack { root, how });
        info!("delirium: {root} handed back ({how:?}) after {secs:.1} s");
    }
    stats.live = live.len();
}

/// Click-to-push runs in the free camera (F11: `PlayerMode` off) or with no player layer at all, never while the
/// player is wanted (third or first person).
fn click_allowed(mode: Option<&PlayerMode>) -> bool {
    mode.is_none_or(|m| !m.on)
}

/// The click aims now: allowed ([`click_allowed`]) and no menu over the game (`PlayerMode::paused`: the click is the
/// menu's).
fn click_aims(mode: Option<&PlayerMode>) -> bool {
    click_allowed(mode) && mode.is_none_or(|m| !m.paused)
}

/// The click's aim: a small dot at the screen centre (the free camera hides and grabs the cursor,
/// `mw_player_bevy::grab_cursor`), shown while the click aims.
#[derive(Component)]
struct ClickDot;

fn spawn_click_dot(mut commands: Commands) {
    const SIZE: f32 = 4.0;
    commands.spawn((
        ClickDot,
        Node {
            position_type: PositionType::Absolute,
            left: percent(50),
            top: percent(50),
            width: px(SIZE),
            height: px(SIZE),
            margin: UiRect { left: px(-SIZE / 2.0), top: px(-SIZE / 2.0), ..UiRect::DEFAULT },
            border_radius: BorderRadius::MAX,
            ..Node::default()
        },
        BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.6)),
        Visibility::Hidden,
    ));
}

#[allow(clippy::needless_pass_by_value)] // system parameters
fn show_click_dot(mode: Option<Res<PlayerMode>>, mut dots: Query<&mut Visibility, With<ClickDot>>) {
    let want = if click_aims(mode.as_deref()) { Visibility::Inherited } else { Visibility::Hidden };
    for mut visibility in &mut dots {
        visibility.set_if_neq(want);
    }
}

/// Dev click-to-push: left click picks the Delirium part under the screen centre ([`ClickDot`]) on a human NPC
/// (Ctrl 20, plain 45, Shift 80 N s along the view ray).
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
fn click(
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    cameras: Query<(Entity, &Camera, &GlobalTransform), With<Camera3d>>,
    pos_of: WorldPosOf,
    mut takeovers: ResMut<Takeovers>,
    mode: Option<Res<PlayerMode>>,
    characters: Query<(Entity, &CharacterBody), With<Mover>>,
    animators: Query<&Animator>,
    frames: FrameQuery,
    roots: Query<&WorldspaceRoot>,
    mut commands: Commands,
) {
    // The free camera only: in the player's views the left click is the attack (and the player's own body is under
    // the cursor). Action characters (no mover) are never picked (module docs).
    if !mouse.just_pressed(MouseButton::Left) || !click_aims(mode.as_deref()) {
        return;
    }
    // The screen centre's ray is the camera's forward from the eye.
    let Some((cam, _, view)) = cameras.iter().filter(|(_, c, _)| c.is_active).max_by_key(|(_, c, _)| c.order) else { return };
    let Some(origin) = pos_of.get(cam) else { return };
    let dir = *view.forward();
    let strength = if keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
        CLICK_IMPULSE[0]
    } else if keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
        CLICK_IMPULSE[2]
    } else {
        CLICK_IMPULSE[1]
    };
    let mut best: Option<(f32, Entity, usize, WorldPos)> = None;
    for (root, cb) in &characters {
        let Ok(animator) = animators.get(cb.body) else { continue };
        let skeleton = &animator.asset.skeleton;
        let (sim, hit) = if let Some(l) = takeovers.live.get(&root) {
            let parts = l.takeover.frames();
            let hit = pick(&l.takeover.build, &parts, l.sim.sim_point(origin), l.sim.sim_dir(dir), 100.0);
            (l.sim, hit)
        } else {
            if !HUMAN_SKELETONS.contains(&cb.skeleton.as_str()) {
                continue;
            }
            let Some((_, frame, scale)) = world_frame(cb.body, &frames, &roots) else { continue };
            // Cheap reject: nowhere near the ray.
            let feet = origin.delta_f32(frame.pos.offset(Vec3::Y));
            if (feet - dir * feet.dot(dir)).length() > 2.0 {
                continue;
            }
            let Some(rig) = takeovers.rig(&cb.skeleton, skeleton) else { continue };
            let sim = SimFrame::at(&frame);
            let pose: Vec<Trs> = (0..skeleton.len()).map(|j| animator.output.pose.get(j)).collect();
            let parts = bones_to_parts(&rig.body.table, skeleton, &pose, &sim.sim_from_model(&frame, scale));
            let hit = pick(&rig.build, &parts, sim.sim_point(origin), sim.sim_dir(dir), 100.0);
            (sim, hit)
        };
        if let Some((part, t, at)) = hit
            && best.is_none_or(|b| t < b.0)
        {
            best = Some((t, root, part, sim.world_point(at)));
        }
    }
    let Some((_, root, part, point)) = best else { return };
    info!("delirium: click on {root}, {} ({strength:.0} N s)", mw_delirium::body::NAMES[part]);
    commands.entity(root).try_insert(DeliriumReaction { hit: Some(DeliriumHit { point, impulse: dir * strength, part: Some(part), joint: None }), ..DeliriumReaction::default() });
}

#[cfg(test)]
mod tests {
    use bevy_app::{App, Startup, Update};
    use bevy_camera::visibility::Visibility;
    use bevy_ecs::prelude::*;
    use mw_game_core::player::PlayerMode;

    use bevy_math::Vec3;
    use sf_coords::WorldPos;

    use super::{ClickDot, HUMAN_SKELETONS, HitRules, click_allowed, death_reaction, hit_reaction, knockdown_reaction, show_click_dot, spawn_click_dot};

    fn mode(on: bool) -> PlayerMode {
        PlayerMode { on, paused: false, body: None, last_ground: None }
    }

    /// Owner 2026-11-05 (PH-15/16): a physics hit on a human walker is a push with the balancer on (no forced fall)
    /// from the takeover threshold, a knockdown after the push past the certain fall; lighter hits stay the walkers'
    /// kinematic hit-react; a held body takes every hit, as a push on Delirium's own takeovers only.
    #[test]
    fn physics_hits_become_pushes() {
        let rules = HitRules { takeover: 10.0, certain_fall: 100.0, gain: 1.5 };
        let point = WorldPos::ORIGIN.offset(Vec3::Y);
        for skeleton in HUMAN_SKELETONS {
            let ball = hit_reaction(&rules, skeleton, true, false, point, Vec3::X * 36.0, None).expect("a ball is a takeover");
            assert!(!ball.knock_down && !ball.kill && !ball.live_only, "a push, the balancer on");
            assert_eq!(ball.hit.map(|h| (h.point, h.impulse, h.part)), Some((point, Vec3::X * 54.0, None)), "momentum times the gain");
            assert!(hit_reaction(&rules, skeleton, true, false, point, Vec3::X * 6.0, None).is_none(), "9 N s: the kinematic hit-react");
            assert!(hit_reaction(&rules, skeleton, true, false, point, Vec3::X * 7.0, None).is_some(), "10.5 N s: a takeover");
            assert!(hit_reaction(&rules, skeleton, true, false, point, Vec3::X * 70.0, None).is_some_and(|r| r.knock_down), "105 N s: down after the push");
            let held = hit_reaction(&rules, skeleton, true, true, point, Vec3::X * 2.0, Some(7)).expect("a held body takes every hit");
            assert!(held.live_only && !held.knock_down, "a further push, only on Delirium's own takeover");
            assert_eq!(held.hit.and_then(|h| h.joint), Some(7), "the struck proxy's joint");
            assert!(hit_reaction(&rules, skeleton, false, false, point, Vec3::X * 36.0, None).is_none(), "an action character (no mover)");
        }
        for other in ["meshes/base_animkna.nif", "meshes/r/guar.nif"] {
            assert!(hit_reaction(&rules, other, true, false, point, Vec3::X * 36.0, None).is_none(), "{other}: the walkers' own reaction");
        }
    }

    /// PH-15 (character-shapes S3): hits on a held body are messages, so several in one frame all push (a component
    /// would keep only the last); a joint picks the part that owns it.
    #[test]
    fn held_hits_are_pushes_that_add_up() {
        use super::{DeliriumPush, DeliriumReaction, Part, hit_part, request};
        use mw_delirium::{PartBone, PartTable};

        #[derive(Resource)]
        struct Root(Entity);
        let mut app = App::new();
        app.add_message::<DeliriumPush>();
        let root = app.world_mut().spawn_empty().id();
        app.insert_resource(Root(root));
        let rules = HitRules::default();
        let point = WorldPos::ORIGIN.offset(Vec3::Y);
        app.add_systems(Update, move |mut commands: Commands, root: Res<Root>| {
            for (n, joint) in [(30.0, Some(3)), (20.0, Some(9)), (150.0, None)] {
                let r = hit_reaction(&rules, HUMAN_SKELETONS[0], true, true, point, Vec3::X * n, joint).unwrap();
                request(&mut commands, root.0, r);
            }
        });
        app.update();
        let pushes: Vec<_> = app.world().resource::<Messages<DeliriumPush>>().iter_current_update_messages().map(|p| (p.root, p.hit.joint)).collect();
        assert_eq!(pushes, vec![(root, Some(3)), (root, Some(9)), (root, None)], "every hit is a push");
        let r = app.world().get::<DeliriumReaction>(root).expect("past the certain fall: the knockdown");
        assert!(r.knock_down && r.live_only && r.hit.is_none(), "the knockdown alone, its push already sent");

        let bone = |joint| PartBone { joint, write: true, offset: Part::from_translation(Vec3::ZERO) };
        let table = PartTable { parts: vec![bone(0), bone(3), bone(9)] };
        let parts = [Part::from_translation(Vec3::ZERO), Part::from_translation(Vec3::Y), Part::from_translation(Vec3::X)];
        let hit = |joint| super::DeliriumHit { point, impulse: Vec3::X, part: None, joint };
        assert_eq!(hit_part(&table, &hit(Some(9)), &parts, Vec3::ZERO), 2, "the part on the struck joint");
        assert_eq!(hit_part(&table, &hit(None), &parts, Vec3::Y), 1, "else the nearest part");
    }

    /// Owner 2026-10-30/11-05: Delirium replaces the powered ragdoll for human knockdowns; beasts, creatures and
    /// action characters keep it.
    #[test]
    fn knockdowns_of_human_walkers_become_takeovers() {
        let point = WorldPos::ORIGIN.offset(Vec3::Y);
        let impulse = Vec3::new(20.0, 0.0, 0.0);
        for skeleton in HUMAN_SKELETONS {
            let reaction = knockdown_reaction(skeleton, true, point, impulse).expect("a human walker is taken over");
            assert!(reaction.knock_down, "a knockdown goes down, whatever the push");
            let hit = reaction.hit.expect("the impact is the takeover's hit");
            assert_eq!((hit.point, hit.impulse, hit.part), (point, impulse, None));
            assert!(knockdown_reaction(skeleton, false, point, impulse).is_none(), "an action character (no mover)");
        }
        for other in ["meshes/base_animkna.nif", "meshes/r/guar.nif"] {
            assert!(knockdown_reaction(other, true, point, impulse).is_none(), "{other}: the powered ragdoll");
        }
    }

    /// Owner 2026-10-30: Delirium replaces the limp ragdoll for human deaths (terminal, never a knockdown's get-up);
    /// the killing blow is the hit when there is one; beasts, creatures and action characters keep the old death.
    #[test]
    fn deaths_of_human_walkers_become_terminal_takeovers() {
        let point = WorldPos::ORIGIN.offset(Vec3::Y);
        for skeleton in HUMAN_SKELETONS {
            let blow = death_reaction(skeleton, true, point, Vec3::X * 20.0).expect("a human walker dies under Delirium");
            assert!(blow.kill && !blow.knock_down);
            assert_eq!(blow.hit.map(|h| (h.point, h.impulse)), Some((point, Vec3::X * 20.0)));
            let plain = death_reaction(skeleton, true, point, Vec3::ZERO).expect("a death with no blow too");
            assert!(plain.kill && plain.hit.is_none(), "no blow, no push");
            assert!(death_reaction(skeleton, false, point, Vec3::ZERO).is_none(), "an action character (no mover)");
        }
        for other in ["meshes/base_animkna.nif", "meshes/r/guar.nif"] {
            assert!(death_reaction(other, true, point, Vec3::ZERO).is_none(), "{other}: the limp ragdoll");
        }
    }

    /// Owner 2026-10-27: the attack's left click also pushed the player's own body (and the dummy) with Delirium,
    /// which then stood beside its capsule. Click-to-push is a free-camera tool only.
    #[test]
    fn click_to_push_only_in_the_free_camera() {
        assert!(!click_allowed(Some(&mode(true))), "player views: the left click is the attack");
        assert!(click_allowed(Some(&mode(false))), "free camera (F11)");
        assert!(click_allowed(None), "no player layer");
    }

    /// Owner 2026-10-27: the free camera hides and grabs the cursor, so the click aims at the screen centre and a
    /// dot marks it, only while the click aims (not in the player's views, not under a menu).
    #[test]
    fn the_centre_dot_shows_while_the_click_aims() {
        let mut app = App::new();
        app.insert_resource(mode(false)).add_systems(Startup, spawn_click_dot).add_systems(Update, show_click_dot);
        let shown = |app: &mut App| {
            app.update();
            let mut dots = app.world_mut().query_filtered::<&Visibility, With<ClickDot>>();
            let v: Vec<_> = dots.iter(app.world()).copied().collect();
            assert_eq!(v.len(), 1, "one dot");
            v[0] == Visibility::Inherited
        };
        assert!(shown(&mut app), "free camera (F11)");
        app.world_mut().resource_mut::<PlayerMode>().paused = true;
        assert!(!shown(&mut app), "a menu over the free camera: the click is the menu's");
        *app.world_mut().resource_mut::<PlayerMode>() = mode(true);
        assert!(!shown(&mut app), "player views: the HUD's crosshair");
    }
}
