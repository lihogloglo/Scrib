//! Feature `debug`, slice AI26: NPC perception as immediate-mode gizmos, after UE's Gameplay
//! Debugger perception category (sight and lose-sight radius, peripheral angle lines, last-seen
//! location, stimulus age) and Unity's cone gizmos. Console variable `ai.perception` (off by
//! default: it draws every observer near the camera, not just the selected one).
//!
//! For every [`AiObserver`] within [`DRAW_DISTANCE_M`] of the active camera (dormant ones skipped):
//! - its sight cone ([`AiEffectivePerception`], else the global [`AiPerceptionSettings`] sight):
//!   ground arcs at the sight range (band colour) and the lose range (faded), the two peripheral
//!   edge lines from the eye (band colour) and the two central ones (faded); a full circle and no
//!   edges when the cone is 180° or wider;
//! - its strongest entry's band as the colour ([`band_color`]) and text above the head
//!   ([`entry_lines`]: band and level, LKP distance, confidence and age, the search hold);
//! - the LKP of that entry (sphere, faded line from the eye, a ground ring while a search holds
//!   it); the selected observer ([`AiSelected`]) gets a white ring at its feet and every entry's LKP;
//! - its goal, when the game inserts [`AiDebugGoal`] (a line from the feet and a cross). The game's
//!   searcher lives outside this crate: until it writes the component, the LKP and the hold flag
//!   are what shows the search.
//!
//! Every [`Perceivable`] with an [`AiLightExposure`] (the player) gets its exposure as text and a
//! grey dot (black dark, white lit) above its head. Every [`NoiseEvent`] of the last
//! [`NOISE_WINDOW_S`] (at most [`NOISE_CAPACITY`], kept in [`RecentNoises`] on the fixed tick after
//! [`AiPerceptionSystems::Hearing`]) gets a ground ring at its open-air reach and a small one at
//! its spot, fading with age: blue for a noise that reveals its source, magenta for one that
//! doesn't.
//!
//! Positions: a [`WorldPos`] lands in render space through an observer's mover ([`Anchor`]: its
//! `GlobalTransform` is its render position), so this needs no floating-origin resource. Drawn
//! only when the app has Bevy's gizmo plugin (headless tests have none) and the variable is on.
//! The display ranges and sizes below are presentation constants of a dev tool, not tunables.

use std::collections::VecDeque;
use std::f32::consts::{FRAC_PI_2, PI, TAU};

use bevy_app::{App, FixedUpdate, PostUpdate};
use bevy_camera::Camera;
use bevy_color::{Alpha, Color};
use bevy_ecs::message::Messages;
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use bevy_gizmos::config::GizmoConfigStore;
use bevy_gizmos::prelude::Gizmos;
use bevy_math::{Isometry3d, Quat, Vec2, Vec3};
use bevy_transform::TransformSystems;
use bevy_transform::components::GlobalTransform;
use sf_ai::{Awareness, AwarenessBand, SightConfig, SightParams};
use sf_console_bevy::sf_console::{VarId, VarSpec};
use sf_console_bevy::{ConsoleAppExt, Tunables, console_flag};
use sf_controller::forward;
use sf_controller_bevy::{Dormant, Mover};
use sf_coords::WorldPos;

use crate::{
    AiActor, AiAwareness, AiDebugGoal, AiEffectivePerception, AiLightExposure, AiObserver, AiPerceptClock,
    AiPerceptionSettings, AiPerceptionSystems, AiSelected, NoiseEvent, Perceivable,
};

/// The `ai.perception` console variable's id.
#[derive(Resource, Clone, Copy, Debug)]
pub struct AiPerceptionGizmoVar(pub VarId);

/// Observers and targets farther than this from the camera are not drawn, meters.
// tunables: const (layout: dev-only perception overlay draw size/range, not gameplay)
pub(crate) const DRAW_DISTANCE_M: f32 = 50.0;
/// How long a noise ring stays (fading out), seconds.
// tunables: const (layout: dev-only perception overlay draw size/range, not gameplay)
pub(crate) const NOISE_WINDOW_S: f32 = 2.0;
/// Most noises kept at once (the oldest is dropped first).
pub(crate) const NOISE_CAPACITY: usize = 32;
/// Arc segments for a full circle (fewer for a narrower cone, at least 2).
const ARC_SEGMENTS_FULL: f32 = 48.0;
/// Ground lines sit this far above the feet, meters (no z-fighting with the ground).
// tunables: const (layout: dev-only perception overlay draw size/range, not gameplay)
const GROUND_LIFT_M: f32 = 0.05;
/// Text height and line spacing, meters (as `ai.gizmos`).
// tunables: const (layout: dev-only perception overlay draw size/range, not gameplay)
const TEXT_SIZE: f32 = 0.12;
// tunables: const (layout: dev-only perception overlay draw size/range, not gameplay)
const LINE_STEP: f32 = 0.17;
/// Bottom text line above the feet, meters: above `ai.gizmos`' plan text (which stacks down from
/// 2.1 m), so the two overlays never overlap.
const TEXT_LIFT: f32 = 2.45;
// tunables: const (layout: dev-only perception overlay draw size/range, not gameplay)
const SELECTED_RING_M: f32 = 0.7;
// tunables: const (layout: dev-only perception overlay draw size/range, not gameplay)
const LKP_RADIUS_M: f32 = 0.2;
// tunables: const (layout: dev-only perception overlay draw size/range, not gameplay)
const HELD_RING_M: f32 = 1.0;
// tunables: const (layout: dev-only perception overlay draw size/range, not gameplay)
const GOAL_CROSS_M: f32 = 0.3;
/// The light dot and text above the target's head, meters.
// tunables: const (layout: dev-only perception overlay draw size/range, not gameplay)
const LIGHT_ABOVE_HEAD_M: f32 = 0.35;
// tunables: const (layout: dev-only perception overlay draw size/range, not gameplay)
const LIGHT_DOT_M: f32 = 0.08;
// tunables: const (layout: dev-only perception overlay draw size/range, not gameplay)
const NOISE_SPOT_M: f32 = 0.3;
/// Alpha of the faded parts (lose range, central edges, eye-to-LKP line).
// tunables: const (layout: dev-only perception overlay draw size/range, not gameplay)
const FADED: f32 = 0.45;
const GOAL_COLOR: Color = Color::srgb(0.2, 0.85, 1.0);
const NOISE_REVEALING: Color = Color::srgb(0.35, 0.6, 1.0);
const NOISE_HIDDEN: Color = Color::srgb(0.9, 0.3, 0.9);

pub(crate) fn add(app: &mut App) {
    let var = app.add_console_var(
        VarSpec::bool("ai.perception", false)
            .help("Draw nearby NPCs' perception: sight cones, awareness, last known positions, light, noises"),
    );
    app.insert_resource(AiPerceptionGizmoVar(var))
        .init_resource::<RecentNoises>()
        .add_systems(
            FixedUpdate,
            // Every tick, so the reader keeps up while off: switching on never shows older noises.
            collect_noises.after(AiPerceptionSystems::Hearing).run_if(resource_exists::<Messages<NoiseEvent>>),
        )
        .add_systems(
            PostUpdate,
            draw.after(TransformSystems::Propagate)
                .run_if(resource_exists::<GizmoConfigStore>)
                .run_if(console_flag(var)),
        );
}

// --- Pure builders ----------------------------------------------------------------------------

/// The band's colour: Unaware grey, Curious yellow, Suspicious orange, Searching red-orange,
/// Alerted red.
pub(crate) fn band_color(band: AwarenessBand) -> Color {
    match band {
        AwarenessBand::Unaware => Color::srgb(0.6, 0.6, 0.6),
        AwarenessBand::Curious => Color::srgb(1.0, 0.9, 0.2),
        AwarenessBand::Suspicious => Color::srgb(1.0, 0.6, 0.1),
        AwarenessBand::Searching => Color::srgb(1.0, 0.35, 0.1),
        AwarenessBand::Alerted => Color::srgb(1.0, 0.1, 0.1),
    }
}

/// A sight cone's shape: ranges in meters, half-angles in radians.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Cone {
    pub range_m: f32,
    pub lose_m: f32,
    pub central_half_rad: f32,
    pub peripheral_half_rad: f32,
}

impl Cone {
    /// An actor's resolved sight.
    pub(crate) fn from_params(p: &SightParams) -> Self {
        Self {
            range_m: p.range_m,
            lose_m: p.lose_m,
            central_half_rad: p.central_half_deg.to_radians(),
            peripheral_half_rad: p.peripheral_half_deg.to_radians(),
        }
    }

    /// The globals (an actor with neither profile nor states).
    pub(crate) fn from_config(c: &SightConfig) -> Self {
        Self {
            range_m: c.sight_range_m,
            lose_m: c.lose_sight_range_m,
            central_half_rad: c.central_half_deg.to_radians(),
            peripheral_half_rad: c.peripheral_half_deg.to_radians(),
        }
    }
}

/// The two edge lines' far ends of a cone of `half_rad` around `yaw` (the mover's yaw,
/// [`forward`]), `range` from `eye`: `[yaw + half, yaw - half]`.
pub(crate) fn cone_edges(eye: Vec3, yaw: f32, half_rad: f32, range: f32) -> [Vec3; 2] {
    [eye + forward(yaw + half_rad) * range, eye + forward(yaw - half_rad) * range]
}

/// A horizontal arc of `radius` around `center` from `yaw - half_rad` to `yaw + half_rad`, as a
/// line strip; a closed circle (first point repeated) when `half_rad` reaches π.
pub(crate) fn arc_points(center: Vec3, yaw: f32, half_rad: f32, radius: f32) -> Vec<Vec3> {
    let half = half_rad.clamp(0.0, PI);
    let span = 2.0 * half;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // small, ≥ 2
    let segments = ((ARC_SEGMENTS_FULL * span / TAU).ceil() as usize).max(2);
    #[allow(clippy::cast_precision_loss)]
    let mut points: Vec<Vec3> =
        (0..=segments).map(|i| center + forward(yaw - half + span * i as f32 / segments as f32) * radius).collect();
    if half >= PI {
        // Exactly closed (no float gap between the two ends).
        points[segments] = points[0];
    }
    points
}

/// Seconds from `tick` to `now` (0 if `tick` is later).
#[allow(clippy::cast_precision_loss)]
fn age_s(tick: u64, now: u64, tick_s: f32) -> f32 {
    now.saturating_sub(tick) as f32 * tick_s
}

/// The text for one awareness entry, `from` the observer's feet at tick `now`:
/// `"<band> <level>%"`, `"LKP <d> m, conf <c>, <age> s ago"`, and `"search held"` while a search
/// holds the entry.
pub(crate) fn entry_lines(a: &Awareness, from: WorldPos, now: u64, tick_s: f32) -> Vec<String> {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // 0..=100
    let pct = (a.level.clamp(0.0, 1.0) * 100.0).round() as u32;
    let d = from.delta_f32(a.last_seen.pos).length();
    let age = age_s(a.last_seen.tick, now, tick_s);
    let mut lines = vec![
        format!("{:?} {pct}%", a.band),
        format!("LKP {d:.1} m, conf {:.2}, {age:.1} s ago", a.last_seen.confidence),
    ];
    if a.search_held() {
        lines.push("search held".into());
    }
    lines
}

/// The target's light exposure line.
pub(crate) fn light_line(exposure: f32) -> String {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // 0..=100
    let pct = (exposure.clamp(0.0, 1.0) * 100.0).round() as u32;
    format!("light {pct}%")
}

/// Maps world positions to render space through one mover: its `GlobalTransform` is its render
/// position (`Mover::frame_origin` + the offset to `loco.render_position()`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Anchor {
    pub render: Vec3,
    pub world: WorldPos,
}

impl Anchor {
    pub(crate) fn of(mover: &Mover, at: &GlobalTransform) -> Self {
        Self { render: at.translation(), world: mover.loco.render_position() }
    }

    pub(crate) fn to_render(self, p: WorldPos) -> Vec3 {
        self.render + self.world.delta_f32(p)
    }
}

// --- Recent noises ----------------------------------------------------------------------------

/// One heard-or-not noise, as the overlay keeps it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RecentNoise {
    /// The perception tick it was collected on.
    pub tick: u64,
    pub at: WorldPos,
    pub loudness_m: f32,
    pub reveals_source: bool,
}

/// The noises of the last [`NOISE_WINDOW_S`], oldest first, at most [`NOISE_CAPACITY`] (filled
/// only while `ai.perception` is on).
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub struct RecentNoises(VecDeque<RecentNoise>);

impl RecentNoises {
    /// Adds `noise`, dropping the oldest when full.
    pub fn push(&mut self, noise: RecentNoise) {
        if self.0.len() >= NOISE_CAPACITY {
            self.0.pop_front();
        }
        self.0.push_back(noise);
    }

    /// Drops every noise [`NOISE_WINDOW_S`] old or older at tick `now`.
    pub fn prune(&mut self, now: u64, tick_s: f32) {
        self.0.retain(|n| age_s(n.tick, now, tick_s) < NOISE_WINDOW_S);
    }

    /// Oldest first.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &RecentNoise> {
        self.0.iter()
    }
}

// --- Systems ----------------------------------------------------------------------------------

/// Keeps [`RecentNoises`] while `ai.perception` is on; reads (and drops) the messages while off.
#[allow(clippy::needless_pass_by_value)]
fn collect_noises(
    var: Res<AiPerceptionGizmoVar>,
    tunables: Option<Res<Tunables>>,
    clock: Option<Res<AiPerceptClock>>,
    mut noises: ResMut<RecentNoises>,
    mut events: MessageReader<NoiseEvent>,
) {
    if !tunables.is_some_and(|t| t.flag(var.0)) {
        events.clear();
        return;
    }
    let (now, tick_s) = clock.map_or((None, 0.0), |c| (Some(c.tick), c.tick_s));
    for ev in events.read() {
        let tick = now.unwrap_or(ev.tick);
        noises.push(RecentNoise { tick, at: ev.at, loudness_m: ev.loudness_m, reveals_source: ev.reveals_source });
    }
    if let Some(now) = now {
        noises.prune(now, tick_s);
    }
}

/// The draw system's resources.
#[derive(SystemParam)]
struct View<'w, 's> {
    selected: Res<'w, AiSelected>,
    settings: Option<Res<'w, AiPerceptionSettings>>,
    clock: Option<Res<'w, AiPerceptClock>>,
    noises: Res<'w, RecentNoises>,
    cameras: Query<'w, 's, (&'static Camera, &'static GlobalTransform)>,
}

type ObserverItem<'a> = (
    &'a AiActor,
    &'a AiObserver,
    &'a Mover,
    &'a AiAwareness,
    &'a GlobalTransform,
    Option<&'a AiEffectivePerception>,
    Option<&'a AiDebugGoal>,
);

fn flat(at: Vec3) -> Isometry3d {
    Isometry3d::new(at, Quat::from_rotation_x(FRAC_PI_2))
}

/// `lines` stacked upward from `bottom` (the first line on top), facing the camera.
fn draw_text(gizmos: &mut Gizmos, bottom: Vec3, facing: Quat, lines: &[String], first: Color) {
    let up = facing * Vec3::Y;
    let n = lines.len();
    for (i, line) in lines.iter().enumerate() {
        #[allow(clippy::cast_precision_loss)]
        let pos = bottom + up * ((n - 1 - i) as f32 * LINE_STEP);
        let color = if i == 0 { first } else { Color::WHITE };
        gizmos.text(Isometry3d::new(pos, facing), line, TEXT_SIZE, Vec2::ZERO, color);
    }
}

fn draw_cone(gizmos: &mut Gizmos, cone: Cone, eye: Vec3, ground: Vec3, yaw: f32, color: Color) {
    let faded = color.with_alpha(FADED);
    gizmos.linestrip(arc_points(ground, yaw, cone.peripheral_half_rad, cone.range_m), color);
    gizmos.linestrip(arc_points(ground, yaw, cone.peripheral_half_rad, cone.lose_m), faded);
    if cone.peripheral_half_rad < PI {
        for end in cone_edges(eye, yaw, cone.peripheral_half_rad, cone.range_m) {
            gizmos.line(eye, end, color);
        }
    }
    if cone.central_half_rad < PI && cone.central_half_rad < cone.peripheral_half_rad {
        for end in cone_edges(eye, yaw, cone.central_half_rad, cone.range_m) {
            gizmos.line(eye, end, faded);
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn draw(
    view: View,
    observers: Query<ObserverItem, Without<Dormant>>,
    targets: Query<(&Perceivable, &AiLightExposure, &GlobalTransform)>,
    mut gizmos: Gizmos,
) {
    let Some((_, cam)) = view.cameras.iter().filter(|(c, _)| c.is_active).max_by_key(|(c, _)| c.order) else {
        return;
    };
    let (cam_pos, facing) = (cam.translation(), cam.rotation());
    let near = |p: Vec3| p.distance_squared(cam_pos) <= DRAW_DISTANCE_M * DRAW_DISTANCE_M;
    let (now, tick_s) = view.clock.as_ref().map_or((0, 0.0), |c| (c.tick, c.tick_s));
    let global_cone = view.settings.as_ref().map(|s| Cone::from_config(&s.sight));
    // The observer nearest the camera maps the noises (any one would do; the nearest is exact).
    let mut anchor: Option<(f32, Anchor)> = None;

    for (actor, obs, mover, meter, at, eff, goal) in &observers {
        let render_feet = at.translation();
        if !near(render_feet) {
            continue;
        }
        let a = Anchor::of(mover, at);
        let dist = render_feet.distance_squared(cam_pos);
        if anchor.is_none_or(|(d, _)| dist < d) {
            anchor = Some((dist, a));
        }
        let selected = view.selected.0 == Some(actor.0);
        let strongest = meter.0.strongest();
        let color = band_color(strongest.map_or(AwarenessBand::Unaware, |e| e.band));
        let feet = mover.loco.position;
        let eye = a.to_render(feet.offset(Vec3::Y * obs.eye_m));
        let ground = a.to_render(feet) + Vec3::Y * GROUND_LIFT_M;
        if let Some(cone) = eff.map(|e| Cone::from_params(&e.0.sight)).or(global_cone) {
            draw_cone(&mut gizmos, cone, eye, ground, mover.loco.motion.yaw, color);
        }
        if selected {
            gizmos.circle(flat(ground), SELECTED_RING_M, Color::WHITE);
        }
        let shown = |e: &&Awareness| selected || strongest.is_some_and(|s| s.target == e.target);
        for e in meter.0.entries().iter().filter(shown) {
            let c = band_color(e.band);
            let lkp = a.to_render(e.last_seen.pos);
            gizmos.sphere(Isometry3d::from_translation(lkp), LKP_RADIUS_M, c);
            gizmos.line(eye, lkp, c.with_alpha(FADED));
            if e.search_held() {
                gizmos.circle(flat(lkp + Vec3::Y * GROUND_LIFT_M), HELD_RING_M, c);
            }
        }
        let mut lines = strongest.map(|e| entry_lines(e, feet, now, tick_s)).unwrap_or_default();
        if let Some(goal) = goal {
            let g = a.to_render(goal.0) + Vec3::Y * GROUND_LIFT_M;
            gizmos.line(ground, g, GOAL_COLOR);
            gizmos.cross(flat(g), GOAL_CROSS_M, GOAL_COLOR);
            lines.push(format!("goal {:.1} m", feet.delta_f32(goal.0).length()));
        }
        draw_text(&mut gizmos, render_feet + Vec3::Y * TEXT_LIFT, facing, &lines, color);
    }

    for (p, light, at) in &targets {
        if !near(at.translation()) {
            continue;
        }
        let pos = at.translation() + Vec3::Y * (p.head_m + LIGHT_ABOVE_HEAD_M);
        let level = light.0.clamp(0.0, 1.0);
        gizmos.sphere(Isometry3d::from_translation(pos), LIGHT_DOT_M, Color::srgb(level, level, level));
        draw_text(&mut gizmos, pos + Vec3::Y * LINE_STEP, facing, &[light_line(light.0)], Color::WHITE);
    }

    let Some((_, a)) = anchor else { return };
    for n in view.noises.iter() {
        let age = age_s(n.tick, now, tick_s);
        if age >= NOISE_WINDOW_S {
            continue;
        }
        let alpha = 1.0 - age / NOISE_WINDOW_S;
        let color = if n.reveals_source { NOISE_REVEALING } else { NOISE_HIDDEN }.with_alpha(alpha);
        let at = a.to_render(n.at) + Vec3::Y * GROUND_LIFT_M;
        gizmos.circle(flat(at), n.loudness_m.max(0.0), color);
        gizmos.circle(flat(at), NOISE_SPOT_M, color);
    }
}
