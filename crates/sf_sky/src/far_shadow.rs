//! The cached far static shadow map (FSM), CPU side (shadows F3b, weather.md
//! "Sun cascades and the far static map"): where the sun-aligned orthographic
//! view sits, when it must be re-rendered, and how a refresh is spread over a
//! few frames. The renderer draws static casters (objects only, no terrain)
//! into the *back* map one horizontal strip per frame with
//! [`FarShadowView::clip_from_world`], then swaps; receivers past the last
//! cascade sample the *front* map with the front view's matrix.

use glam::camera::rh::{proj::directx::orthographic, view::look_to_mat4};
use glam::{Mat4, Vec3};

/// How big the map is and how eagerly it follows the sun and the camera.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FarShadowSettings {
    /// Texels per side (4096: 64 MB per `Depth32` map).
    pub resolution: u32,
    /// Half the side, meters: the map covers the camera ± `reach` (2048 m = 1 m texels at 4096²).
    pub reach: f32,
    /// Depth range along the sun, meters, centred on the camera (casters on
    /// a hill above or a valley below still land in it).
    pub depth_range: f32,
    /// Re-render once the sun has turned this far, degrees. 0.25: a shadow
    /// tip 10 m from its caster moves under one 1 m texel at 30° elevation,
    /// and a 30x clock (0.125°/s) redraws every 2 s.
    pub sun_threshold_deg: f32,
    /// Re-render once the camera has moved this fraction of `reach` (1/16).
    pub recentre_fraction: f32,
    /// Frames a refresh is spread over (one horizontal strip each).
    pub strips: u32,
}

impl Default for FarShadowSettings {
    fn default() -> Self {
        Self { resolution: 4096, reach: 2048.0, depth_range: 4000.0, sun_threshold_deg: 0.25, recentre_fraction: 1.0 / 16.0, strips: 4 }
    }
}

impl FarShadowSettings {
    /// Meters per texel.
    #[expect(clippy::cast_precision_loss, reason = "map sizes are far below 2^24 texels")]
    pub fn texel(&self) -> f32 {
        2.0 * self.reach / self.resolution.max(1) as f32
    }
}

/// One placement of the map: the light-space frame it was (or is being) drawn with.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FarShadowView {
    /// Unit vector toward the sun.
    pub to_sun: Vec3,
    /// Map centre in world space, snapped to whole texels in the light's plane
    /// (re-centring never shifts texel edges, so no shimmer).
    pub centre: Vec3,
    /// World → light view (right-handed, looking down `-to_sun`).
    pub view_from_world: Mat4,
    /// World → clip: orthographic, reverse Z (1 at the sun side, like Bevy's
    /// shadow views), x/y in -1..1 over `centre ± reach`.
    pub clip_from_world: Mat4,
}

impl FarShadowView {
    /// The view for a sun direction and a camera position.
    pub fn fit(to_sun: Vec3, camera: Vec3, settings: &FarShadowSettings) -> Self {
        let to_sun = to_sun.try_normalize().unwrap_or(Vec3::Y);
        // A fixed up hint (world -Z, then X near the poles) keeps the light's
        // x/y axes still while the camera moves: only the sun turns them.
        let up = if to_sun.dot(Vec3::NEG_Z).abs() > 0.99 { Vec3::X } else { Vec3::NEG_Z };
        let rotation = look_to_mat4(Vec3::ZERO, -to_sun, up);
        let texel = settings.texel();
        let local = rotation.transform_point3(camera);
        let snapped = Vec3::new((local.x / texel).round() * texel, (local.y / texel).round() * texel, local.z);
        let centre = rotation.inverse().transform_point3(snapped);
        let half_depth = settings.depth_range * 0.5;
        let view_from_world = look_to_mat4(centre + to_sun * half_depth, -to_sun, up);
        let r = settings.reach;
        // near/far swapped: reverse Z (near plane → 1, far plane → 0).
        let projection = orthographic(-r, r, -r, r, settings.depth_range, 0.0);
        Self { to_sun, centre, view_from_world, clip_from_world: projection * view_from_world }
    }

    /// Whether `next` differs enough from this view to re-render.
    pub fn stale(&self, next: &Self, camera: Vec3, settings: &FarShadowSettings) -> bool {
        let cos = self.to_sun.dot(next.to_sun).clamp(-1.0, 1.0);
        let moved_sun = cos.acos().to_degrees() > settings.sun_threshold_deg;
        let moved_camera = (camera - self.centre).length() > settings.reach * settings.recentre_fraction;
        moved_sun || moved_camera
    }
}

/// What the renderer should do this frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FarShadowWork {
    /// Nothing to draw: the front map is current.
    Idle,
    /// Draw strip `strip` of `strips` (rows `strip·res/strips ..`) into map `target` with `view`.
    Strip { view: FarShadowView, strip: u32, strips: u32, target: usize },
}

/// Two maps (front sampled, back drawn) and the refresh schedule.
#[derive(Clone, Debug, Default)]
pub struct FarShadowCache {
    front: Option<(FarShadowView, usize)>,
    pending: Option<(FarShadowView, u32)>,
    dirty: bool,
}

impl FarShadowCache {
    /// The map receivers sample and its view, once a first refresh finished.
    pub fn front(&self) -> Option<(FarShadowView, usize)> {
        self.front
    }

    /// Forces a refresh (a far cell with casters streamed in or out).
    pub fn invalidate(&mut self) {
        self.dirty = true;
    }

    /// Advances one frame. A refresh in flight finishes before a newer one starts.
    pub fn update(&mut self, to_sun: Vec3, camera: Vec3, settings: &FarShadowSettings) -> FarShadowWork {
        let strips = settings.strips.max(1);
        let back = self.front.map_or(0, |(_, i)| 1 - i);
        if self.pending.is_none() {
            let next = FarShadowView::fit(to_sun, camera, settings);
            let stale = self.front.is_none_or(|(view, _)| view.stale(&next, camera, settings));
            if !(stale || self.dirty) {
                return FarShadowWork::Idle;
            }
            self.dirty = false;
            self.pending = Some((next, 0));
        }
        let Some((view, strip)) = self.pending else { return FarShadowWork::Idle };
        if strip + 1 >= strips {
            self.pending = None;
            self.front = Some((view, back));
        } else {
            self.pending = Some((view, strip + 1));
        }
        FarShadowWork::Strip { view, strip, strips, target: back }
    }
}
