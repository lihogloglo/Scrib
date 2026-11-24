//! The water chain's visibility gate (CW-10a, water.md "Water pass" step 0; `water.gate.visible`,
//! off by default). Two terms, both exact for the frame they gate:
//!
//! - **Frustum (CPU, this frame):** per [`WaterCamera`], the boxes where water can be drawn (the
//!   shore field's wet leaves, [`sf_water::WetTiles`], plus the open sea around it, bounded and
//!   unbounded [`WaterSurface`]s, mesh bodies' boxes, [`crate::WaterMeshSurface`]), grown by [`WaterGateSettings::margin_m`], culled against the
//!   camera's frustum. None left: the camera's snapshot, SSPR, velocity and the probe are not
//!   queued at all, and the FFT runs at [`WaterGateSettings::idle_fft_s`] only while no camera
//!   sees water (foam and the amortised cascades stay current for the re-entry frame).
//! - **Occlusion (GPU, this frame):** the boxes left are drawn depth-only against the view's
//!   depth after the opaque pass inside an occlusion query, resolved on the GPU into the indirect
//!   arguments of the snapshot, SSPR and velocity passes (zero when no sample passed: no water
//!   fragment will pass the depth test in the transmissive pass either, so nothing reads their
//!   outputs). No readback, no latency (`render.rs`, `water_gate.wesl`).
//!
//! The camera inside a box (swimming, standing on the shore) skips the probe and runs every pass.

use std::sync::Arc;

use bevy_camera::primitives::{Aabb, Frustum};
use bevy_ecs::prelude::*;
use bevy_math::{Affine3A, Vec3};
use bevy_transform::components::GlobalTransform;
use sf_tunables::{def, Def, Group, Source, Unit};
use sf_water::WetTiles;

use crate::{render_xz, BodyMesh, ClipmapSettings, OceanSurface, WaterBody, WaterCamera, WaterFrame, WaterMeshSurface, WaterShoreInput, WaterSurface};

/// `water.gate.visible`: gate the water chain by the water each camera can see (CW-10a).
pub const WATER_GATE_VISIBLE: Def<bool> = def!(bool "water.gate.visible" = false)
    .help("Water passes (snapshot, SSPR, velocity, FFT) run only for cameras that can see water")
    .source(Source::Owner("2026-11-15: a change stays opt-in, off, until its interleaved A/B wins"))
    .group(Group::Water);

/// `water.gate.margin_m`: how far beyond a wet texel (and above or below the rest level) a water
/// vertex may be drawn: the horizontal wave displacement, the swash's land margin, the crests.
pub const WATER_GATE_MARGIN_M: Def<f32> = def!(float "water.gate.margin_m" = 16.0)
    .range(4.0, 64.0)
    .unit(Unit::Meters)
    .help("Water visibility boxes reach this far past the wet area and the rest level")
    .source(Source::Guess)
    .group(Group::Water);

/// `water.gate.idle_fft_s`: the FFT's period while no camera sees water (foam decay and the
/// amortised cascades stay current; the FFT's own `dt` clamp is 0.2 s).
pub const WATER_GATE_IDLE_FFT_S: Def<f32> = def!(float "water.gate.idle_fft_s" = 0.125)
    .range(0.0, 0.2)
    .unit(Unit::Seconds)
    .help("Wave FFT period while no water is in view")
    .source(Source::Guess)
    .group(Group::Water);

/// Most boxes one camera's probe draws (a capacity: beyond it the probe is skipped, every pass runs).
pub const MAX_GATE_BOXES: usize = 4096; // tunables: const (capacity: past it the gate runs every pass)

/// The gate's settings (bound from [`crate::TUNABLES`] by the host; live both ways).
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct WaterGateSettings {
    pub enabled: bool,
    pub margin_m: f32,
    pub idle_fft_s: f32,
}

impl Default for WaterGateSettings {
    fn default() -> Self {
        Self { enabled: WATER_GATE_VISIBLE.default_value(), margin_m: WATER_GATE_MARGIN_M.default_value(), idle_fft_s: WATER_GATE_IDLE_FFT_S.default_value() }
    }
}

/// One camera's gate this frame (on the [`WaterCamera`] while the gate is on).
#[derive(Component, Clone, Debug, Default, PartialEq)]
pub struct WaterGate {
    /// Render-space boxes (min, max) where water can be drawn, inside the frustum.
    pub boxes: Arc<Vec<[Vec3; 2]>>,
    /// The camera is inside a box, or the boxes overflowed [`MAX_GATE_BOXES`]: no probe, every pass.
    pub forced: bool,
}

impl WaterGate {
    /// Water can be in this camera's view (the frustum term).
    #[must_use]
    pub fn in_frustum(&self) -> bool {
        self.forced || !self.boxes.is_empty()
    }
}

/// The wet-leaf tree of the current shore field and the threshold it was built at.
#[derive(Resource, Default)]
pub(crate) struct WetTilesCache {
    key: Option<(usize, u32)>,
    pub tiles: Option<WetTiles>,
}

/// Rebuilds the tree when the field or the margin changes (the land margin threshold is `-margin`).
pub(crate) fn update_tiles(settings: Res<WaterGateSettings>, shore: Res<WaterShoreInput>, mut cache: ResMut<WetTilesCache>) {
    if !settings.enabled {
        return;
    }
    let key = shore.field.as_ref().map(|f| (Arc::as_ptr(f) as usize, settings.margin_m.to_bits()));
    if cache.key == key {
        return;
    }
    cache.key = key;
    // Wet down to `margin` inland (the swash's land margin, 2-8 m, and the bilinear tap), within the
    // field's clamped range; the boxes grow by `margin` again for the horizontal displacement.
    cache.tiles = shore.field.as_ref().map(|f| WetTiles::from_field(f, -settings.margin_m.min(0.5 * f.settings.range)));
}

/// The FFT this frame under the gate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct FftPlan {
    /// Dispatch the FFT and the mips.
    pub run: bool,
    /// Evaluate every cascade (no amortised skip): an idle tick, or the first frame back in view.
    pub full: bool,
    /// Copy the displacement to `prev_displacement` before the FFT (last frame's waves: the motion
    /// vectors). After it on the first frame after a frame without FFT: `prev` would be older than a
    /// frame, so that frame's waves have no motion instead of a too-long one.
    pub copy_before: bool,
}

/// The FFT's history under the gate: whether it ran last frame, and when it last ran.
#[derive(Default, Debug)]
pub(crate) struct FftClock {
    ran: bool,
    last: Option<f64>,
}

impl FftClock {
    /// This frame's plan at time `now`. Gate off: every frame, amortised, copy first (as before CW-10a).
    pub fn plan(&mut self, gate: bool, any_open: bool, now: f64, idle_s: f32) -> FftPlan {
        let plan = if !gate {
            FftPlan { run: true, full: false, copy_before: true }
        } else if any_open {
            FftPlan { run: true, full: !self.ran, copy_before: self.ran }
        } else {
            let due = self.last.is_none_or(|l| now - l >= f64::from(idle_s) || now < l);
            FftPlan { run: due, full: true, copy_before: self.ran }
        };
        self.ran = plan.run;
        if plan.run {
            self.last = Some(now);
        }
        plan
    }
}

/// A shown water body (the frustum term's input).
#[derive(Clone, Copy, Debug)]
pub(crate) enum Body {
    Ocean,
    Surface(WaterSurface),
    /// A mesh body ([`crate::WaterMeshSurface`]): its render-space box.
    Mesh([Vec3; 2]),
}

/// The water surfaces (the ocean and the bodies at their own level), with their shown state.
type Surfaces<'w, 's> = Query<'w, 's, (Option<&'static WaterSurface>, Has<OceanSurface>, BodyMesh, &'static bevy_camera::visibility::InheritedVisibility), Or<(With<OceanSurface>, With<WaterSurface>, With<WaterMeshSurface>)>>;

/// Gate counters (diagnostics): frames a camera's frustum term closed, and probes queued.
pub(crate) static FRUSTUM_CLOSED: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub(crate) static PROBES: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Per [`WaterCamera`]: the water boxes in its frustum ([`WaterGate`]); removed while the gate is off.
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub(crate) fn gate_views(
    mut commands: Commands,
    settings: Res<WaterGateSettings>,
    frame: Res<WaterFrame>,
    clip: Res<ClipmapSettings>,
    shore: Res<WaterShoreInput>,
    cache: Res<WetTilesCache>,
    cameras: Query<(Entity, &GlobalTransform, Option<&Frustum>, Has<WaterGate>), With<WaterCamera>>,
    surfaces: Surfaces<'_, '_>,
) {
    // The shown bodies: the ocean while the frame shows it, the others while visible.
    let bodies: Vec<Body> = surfaces
        .iter()
        .filter_map(|(surface, ocean, mesh, inherited)| match WaterBody::of(surface, ocean, mesh) {
            WaterBody::Ocean if frame.enabled => Some(Body::Ocean),
            WaterBody::Surface(w) if inherited.get() => Some(Body::Surface(w)),
            WaterBody::Mesh(_, world) if inherited.get() => Some(Body::Mesh(world)),
            _ => None,
        })
        .collect();
    for (entity, transform, frustum, gated) in &cameras {
        if !settings.enabled {
            if gated {
                commands.entity(entity).remove::<WaterGate>();
            }
            continue;
        }
        let Some(frustum) = frustum else {
            commands.entity(entity).insert(WaterGate { forced: true, ..WaterGate::default() });
            continue;
        };
        let gate = camera_gate(&settings, &frame, &clip, &shore, cache.tiles.as_ref(), transform.translation(), frustum, &bodies);
        if !gate.in_frustum() {
            FRUSTUM_CLOSED.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        commands.entity(entity).insert(gate);
    }
}

/// The frustum term for one camera at render-space `eye`.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(crate) fn camera_gate(
    settings: &WaterGateSettings,
    frame: &WaterFrame,
    clip: &ClipmapSettings,
    shore: &WaterShoreInput,
    tiles: Option<&WetTiles>,
    eye: Vec3,
    frustum: &Frustum,
    bodies: &[Body],
) -> WaterGate {
    let m = settings.margin_m;
    let reach = clip.skirt;
    // Candidates; the frustum and inside tests run once at the end.
    let mut boxes: Vec<[Vec3; 2]> = Vec::new();
    let mut forced = false;
    // The plane test alone passes boxes far wider than the view (every plane has a corner inside):
    // the open sea's strips under a camera looking at the sky or at the ground. The view truncated at
    // twice the skirt (no water is drawn farther) is a pyramid: a full separating-axis test against it.
    let pyramid = view_pyramid(eye, frustum, 2.0 * reach);
    let seen = |lo: Vec3, hi: Vec3| frustum.intersects_obb(&Aabb::from_min_max(lo, hi), &Affine3A::IDENTITY, true, false) && pyramid_meets_box(&pyramid, lo, hi);
    let add = |lo: Vec3, hi: Vec3, boxes: &mut Vec<[Vec3; 2]>| boxes.push([lo, hi]);
    let around = |level: f32| (Vec3::new(eye.x - reach, level - m, eye.z - reach), Vec3::new(eye.x + reach, level + m, eye.z + reach));
    for body in bodies {
        match body {
            // The ocean: the wet leaves and the open sea around them.
            Body::Ocean => {
                let (y0, y1) = (frame.level - m, frame.level + m);
                if let (Some(field), Some(tiles)) = (shore.field.as_ref(), tiles) {
                    let corner = render_xz(field.origin - bevy_math::DVec2::splat(field.texel * 0.5), frame.origin_cell);
                    let to_box = |r: [f32; 4]| (Vec3::new(corner.x + r[0], y0, corner.y + r[1]), Vec3::new(corner.x + r[2], y1, corner.y + r[3]));
                    let mut rects = Vec::new();
                    let complete = tiles.visit(
                        m,
                        MAX_GATE_BOXES,
                        |r| {
                            let (lo, hi) = to_box(r);
                            seen(lo, hi)
                        },
                        &mut rects,
                    );
                    forced |= !complete;
                    rects.extend(tiles.exterior(reach));
                    for r in rects {
                        let (lo, hi) = to_box(r);
                        add(lo, hi, &mut boxes);
                    }
            } else {
                        let (lo, hi) = around(frame.level);
                    add(lo, hi, &mut boxes);
            }
            }
            Body::Surface(w) => {
                let (lo, hi) = match w.bounds {
                    Some([a, b]) => (Vec3::new(a.x - m, w.level - m, a.y - m), Vec3::new(b.x + m, w.level + m, b.y + m)),
                    None => around(w.level),
                };
                add(lo, hi, &mut boxes);
            }
            Body::Mesh([lo, hi]) => add(*lo - Vec3::splat(m), *hi + Vec3::splat(m), &mut boxes),
        }
    }
    boxes.retain(|[lo, hi]| seen(*lo, *hi));
    forced |= boxes.iter().any(|[lo, hi]| eye.cmpge(*lo).all() && eye.cmple(*hi).all());
    if boxes.len() > MAX_GATE_BOXES {
        forced = true;
        boxes.truncate(MAX_GATE_BOXES);
    }
    WaterGate { boxes: Arc::new(boxes), forced }
}

/// The view as a pyramid: the eye, then the four edge rays (left/right × top/bottom planes'
/// intersections, in cyclic order) out to `reach` metres.
fn view_pyramid(eye: Vec3, frustum: &Frustum, reach: f32) -> [Vec3; 5] {
    let n = |i: usize| -> Vec3 { frustum.half_spaces[i].normal().into() };
    let forward = n(4);
    // half_spaces: left, right, top, bottom, near, far.
    let ray = |a: usize, b: usize| {
        let d = n(a).cross(n(b)).normalize_or_zero();
        eye + reach * if d.dot(forward) < 0.0 { -d } else { d }
    };
    [eye, ray(0, 2), ray(1, 2), ray(1, 3), ray(0, 3)]
}

/// Separating-axis test of a pyramid (apex, then its base in cyclic order) against a box: the box's
/// axes, the pyramid's face normals, and the cross products of their edges.
fn pyramid_meets_box(p: &[Vec3; 5], lo: Vec3, hi: Vec3) -> bool {
    let (center, half) = ((lo + hi) * 0.5, (hi - lo) * 0.5);
    let base = [p[1], p[2], p[3], p[4]];
    let mut edges = Vec::with_capacity(8);
    let mut axes = vec![Vec3::X, Vec3::Y, Vec3::Z];
    for i in 0..4 {
        let (a, b) = (base[i], base[(i + 1) % 4]);
        edges.push(a - p[0]);
        edges.push(b - a);
        axes.push((a - p[0]).cross(b - p[0]));
    }
    axes.push((base[1] - base[0]).cross(base[2] - base[0]));
    for e in &edges {
        for k in [Vec3::X, Vec3::Y, Vec3::Z] {
            axes.push(e.cross(k));
        }
    }
    axes.iter().filter(|a| a.length_squared() > 1e-12).all(|a| {
        let (lo_p, hi_p) = p.iter().map(|q| q.dot(*a)).fold((f32::INFINITY, f32::NEG_INFINITY), |(l, h), d| (l.min(d), h.max(d)));
        let (c, r) = (center.dot(*a), half.dot(a.abs()));
        hi_p >= c - r && lo_p <= c + r
    })
}

/// How often the gate has closed a camera's frustum term and queued an occlusion probe.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GateStats {
    pub frustum_closed: u64,
    pub probes: u64,
}

/// The gate's counters since start (diagnostics: the F3 panel, logs).
#[must_use]
pub fn gate_stats() -> GateStats {
    use std::sync::atomic::Ordering::Relaxed;
    GateStats { frustum_closed: FRUSTUM_CLOSED.load(Relaxed), probes: PROBES.load(Relaxed) }
}
