//! CPU twin of the per-view cull and LOD pass (static-scene.md §4, "Per-view culling and LOD").
//!
//! The GPU pass (`sf_scene_bevy`, later slices) is tested against this: same inputs, same work
//! items. In the spec's order: (1) layer mask (per table, format v2) and active flag, (2) sphere
//! vs frustum, (3) terrain horizon, (4) sub-pixel, (5) LOD; Hi-Z occlusion (6) is GPU only.
//! [`cull_cell`] first tests the table's layers and the cell's AABB against the frustum and the
//! horizon, as the CPU pre-cull does, and applies the horizon per cell (SPUD `Space::land_hides`,
//! `docs/research/spud-code-2026-11-08.md` item 3).
//!
//! **LOD** follows how the view projects ([`SceneProjection`], §4 step 5):
//!
//! - **Perspective** (the camera, lamp cube faces): the coarsest level whose error `e` (metres)
//!   satisfies `e² · (pixels_per_radian / lod_px)² ≤ d²`, `d` the distance from the eye to the
//!   bounding sphere (0 inside it). `pixels_per_radian` is
//!   `sf_model::lod::LodScreen::focal_px`'s `H / (2 tan(fov_y / 2))` (copied, not imported:
//!   `sf_model` builds meshoptimizer), from the view's **internal** resolution under an upscaler.
//! - **Orthographic** (sun cascades, §8): no distance term, because an orthographic projection
//!   has none: the coarsest level with `e · texels_per_metre ≤ lod_px`, one level for the whole
//!   cascade per chain (per chain and instance scale, below).
//!
//! Level errors are model-space in both forms: the instance's largest axis scale
//! ([`StaticInstance::max_scale`]) multiplies them first, so a placement scaled 2× switches each
//! level at twice the distance (the GPU pass does the same). [`ScenePreset`] sets `lod_px` and
//! `size_cull_px`, the only knobs the quality presets turn — and `size_cull_px` is **per view**,
//! not one number per preset: a preset's value is for main views, and shadow views get 0 (off)
//! until a caster size rule is measured ([`ViewKind`]).

use glam::{IVec2, Vec2, Vec3, Vec4, Vec4Swizzles};

use crate::eligibility::Promotions;
use crate::table::{Aabb, InstanceFlags, StaticCellTableView, StaticInstance};
use crate::PlacementId;

/// Pixels per radian at the screen centre: `height_px / (2 tan(fov_y / 2))` (`sf_model`
/// `LodScreen::focal_px`). Pass the internal (pre-upscale) height.
pub fn pixels_per_radian(height_px: f32, fov_y: f32) -> f32 {
    height_px / (2.0 * (fov_y * 0.5).tan())
}

/// Texels per metre of an orthographic view: its map side in texels over the world extent it
/// covers (a sun cascade: `cascade_px / cascade_extent_m`). Constant over the whole view, which
/// is why the orthographic LOD form has no distance term.
pub fn texels_per_metre(map_px: f32, extent_m: f32) -> f32 {
    map_px / extent_m
}

/// How a view projects, with the one term its LOD pick and sub-pixel test need (§4 step 5).
/// Not one overloaded number: the perspective term is an angular density (pixels per radian,
/// so what it covers grows with distance), the orthographic one a surface density (texels per
/// metre, the same everywhere in the view).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SceneProjection {
    /// Camera views, and the lamp cube faces (§8: "lamp cube faces are perspective and use the
    /// normal form"): [`pixels_per_radian`] of the view's internal resolution.
    Perspective { pixels_per_radian: f32 },
    /// Sun cascades and any other orthographic view: [`texels_per_metre`] of its map.
    Orthographic { texels_per_metre: f32 },
}

impl SceneProjection {
    /// The coarsest acceptable level of `errors` (finest first, **already multiplied** by the
    /// instance's scale, so in world metres), §4 step 5. `surface_distance` is the distance from
    /// the eye to the bounding sphere's surface, ignored by [`Self::Orthographic`]. `None` only
    /// for a chain without levels (tables reject those), so a pick is always a level that exists.
    pub fn pick_level(self, errors: impl IntoIterator<Item = f32>, surface_distance: f32, lod_px: f32) -> Option<usize> {
        match self {
            Self::Perspective { pixels_per_radian } => pick_level(errors, surface_distance, pixels_per_radian, lod_px),
            Self::Orthographic { texels_per_metre } => pick_level_ortho(errors, texels_per_metre, lod_px),
        }
    }
}

/// What a view is for. The one thing it decides here is the sub-pixel cull's default
/// ([`ScenePreset::size_cull_px`]): a size cutoff that is right in the view the player looks
/// through is not automatically right in a shadow view, where it drops a caster's **shadow**
/// while the caster itself is still on screen.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ViewKind {
    /// The camera, and anything else drawn for the player to look at.
    Main,
    /// A view drawn only to gather casters: the sun cascades (orthographic) and the lamp cube
    /// faces (perspective, §8).
    Shadow,
}

/// Quality presets (§4): they change only `lod_px` and `size_cull_px`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ScenePreset {
    Ultra,
    High,
    Medium,
    Low,
}

impl ScenePreset {
    pub const ALL: [Self; 4] = [Self::Ultra, Self::High, Self::Medium, Self::Low];

    /// Allowed LOD error on screen, pixels (spec proposal: 0.5 / 1 / 2 / 4).
    pub fn lod_px(self) -> f32 {
        match self {
            Self::Ultra => 0.5,
            Self::High => 1.0,
            Self::Medium => 2.0,
            Self::Low => 4.0,
        }
    }

    /// Projected radius under which an instance is dropped, in the view's own units
    /// ([`cull_instance`]). The spec gives one number per preset for both knobs; they are
    /// separate here so S4 can tune them apart.
    ///
    /// **Off ([`Self::lod_px`] worth of pixels in main views, 0 in shadow views.)** A
    /// main-view-style size cutoff applied to casters was measured on this project and reverted
    /// the same day — "its cascades refit with the view, so small casters lost and regained
    /// shadows as the camera turned" (`bevyrrowind/docs/perf.md:55-57`) — and what ships
    /// elsewhere is a *per-cascade* threshold over a stable cascade fit (Unity 6.6 small-mesh
    /// culling; UE's `r.Nanite.ShadowRaster.PixelsPerEdgeScaling` is a per-cascade LOD bias).
    /// The orthographic test in [`cull_instance`] is that rule's cascade-texel form and stays
    /// implemented, so the owner can turn it on for a view by setting
    /// [`SceneView::size_cull_px`] directly; it is only off by default.
    pub fn size_cull_px(self, kind: ViewKind) -> f32 {
        match kind {
            ViewKind::Main => self.lod_px(),
            ViewKind::Shadow => 0.0,
        }
    }
}

/// Six planes, `xyz` the unit inward normal, `w` the offset: a point `p` is inside a plane when
/// `xyz · p + w ≥ 0`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Frustum {
    pub planes: [Vec4; 6],
}

impl Frustum {
    /// A perspective frustum at `eye` looking along `forward` (`up` need not be orthogonal).
    /// `far` may be infinite (Bevy's reverse-Z projection): that plane then never culls.
    pub fn perspective(eye: Vec3, forward: Vec3, up: Vec3, fov_y: f32, aspect: f32, near: f32, far: f32) -> Self {
        let ahead = forward.normalize();
        let right = ahead.cross(up).normalize();
        let above = right.cross(ahead);
        // Tangents of the half-angles: a side plane holds `ahead · slope ± side`.
        let slope_vertical = (fov_y * 0.5).tan();
        let slope_horizontal = slope_vertical * aspect;
        let through_eye = |inward: Vec3| {
            let normal = inward.normalize();
            normal.extend(-normal.dot(eye))
        };
        let far_plane = if far.is_finite() { (-ahead).extend(ahead.dot(eye) + far) } else { Vec4::W };
        Self {
            planes: [
                through_eye(right + ahead * slope_horizontal),
                through_eye(-right + ahead * slope_horizontal),
                through_eye(above + ahead * slope_vertical),
                through_eye(-above + ahead * slope_vertical),
                ahead.extend(-ahead.dot(eye) - near),
                far_plane,
            ],
        }
    }

    /// An orthographic frustum: the box a sun cascade covers, looking along `forward` from
    /// `center`, `half_width` × `half_height` metres wide and from `near` to `far` along
    /// `forward` (`far` may be infinite, as in [`Self::perspective`]).
    pub fn orthographic(
        center: Vec3,
        forward: Vec3,
        up: Vec3,
        half_width: f32,
        half_height: f32,
        near: f32,
        far: f32,
    ) -> Self {
        let ahead = forward.normalize();
        let right = ahead.cross(up).normalize();
        let above = right.cross(ahead);
        // Inward normal `n` through the box face at `± half` of its axis.
        let side = |n: Vec3, half: f32| n.extend(-n.dot(center) + half);
        let far_plane = if far.is_finite() { (-ahead).extend(ahead.dot(center) + far) } else { Vec4::W };
        Self {
            planes: [
                side(-right, half_width),
                side(right, half_width),
                side(-above, half_height),
                side(above, half_height),
                ahead.extend(-ahead.dot(center) - near),
                far_plane,
            ],
        }
    }

    /// The sphere is wholly outside one plane.
    pub fn sphere_outside(&self, center: Vec3, radius: f32) -> bool {
        self.planes.iter().any(|p| p.xyz().dot(center) + p.w < -radius)
    }

    /// The box is wholly outside one plane (its most inward corner is outside).
    pub fn aabb_outside(&self, aabb: &Aabb) -> bool {
        self.planes.iter().any(|p| {
            let n = p.xyz();
            let corner = Vec3::select(n.cmpge(Vec3::ZERO), aabb.max, aabb.min);
            n.dot(corner) + p.w < 0.0
        })
    }
}

/// One view's cull inputs, all in the cull frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SceneView {
    /// Where the view is seen from. An orthographic view has no eye point: its LOD and
    /// sub-pixel tests ignore this, and only the horizon pre-cull (main views) reads it — which
    /// is why `horizon` and [`SceneProjection::Orthographic`] together are a caller bug
    /// ([`cull_cell`] asserts it in debug builds).
    pub eye: Vec3,
    pub frustum: Frustum,
    /// Drawn layers; a table needs one in common ([`crate::StaticCellTable::layers`]).
    pub layers: u32,
    /// How the view projects: the LOD pick and the sub-pixel test read its density term.
    pub projection: SceneProjection,
    pub lod_px: f32,
    /// Smallest projected radius an instance may have, in the view's own units ([`cull_instance`]:
    /// screen pixels in a perspective view, shadow-map texels in an orthographic one). **0 turns
    /// the sub-pixel cull off**, and that is the default for shadow views
    /// ([`ScenePreset::size_cull_px`]).
    pub size_cull_px: f32,
    /// Apply the terrain horizon (exterior main views). Never on an orthographic view: it has no
    /// eye point and the test marches from `eye` ([`cull_cell`], [`HorizonGrid::land_hides`]).
    pub horizon: bool,
}

impl SceneView {
    /// The view with a preset's pixel thresholds for a view of this kind. `kind` is explicit so
    /// that a slice building shadow views cannot inherit the main view's sub-pixel cutoff by
    /// forgetting about it ([`ScenePreset::size_cull_px`]).
    #[must_use]
    pub fn with_preset(self, preset: ScenePreset, kind: ViewKind) -> Self {
        Self { lod_px: preset.lod_px(), size_cull_px: preset.size_cull_px(kind), ..self }
    }
}

/// The perspective pick: the coarsest level of `errors` (finest first, metres) acceptable at
/// `distance` metres, the largest index with `e² · (ppr / lod_px)² ≤ d²`; level 0 when none
/// passes. `None` only for a chain without levels (tables reject those), so a pick is always a
/// level that exists.
pub fn pick_level(errors: impl IntoIterator<Item = f32>, distance: f32, pixels_per_radian: f32, lod_px: f32) -> Option<usize> {
    let scale = pixels_per_radian / lod_px;
    let d2 = distance * distance;
    let mut pick = None;
    for (k, e) in errors.into_iter().enumerate() {
        let projected = e * scale;
        if k == 0 || projected * projected <= d2 {
            pick = Some(k);
        }
    }
    pick
}

/// The orthographic pick (§4 step 5, "Orthographic views (sun cascades) have no distance term"):
/// the coarsest level of `errors` (finest first, metres) with `e · texels_per_metre ≤ lod_px`;
/// level 0 when none passes, `None` for no levels (as [`pick_level`]).
///
/// The cascade's texel density is the same everywhere in the view, so this is one level for the
/// whole cascade per chain (per chain and instance scale: `errors` arrive scaled). A distance
/// term would be meaningless here — an orthographic projection's footprint per texel does not
/// grow with distance — and it is what made the cascades pick main-view levels before.
pub fn pick_level_ortho(errors: impl IntoIterator<Item = f32>, texels_per_metre: f32, lod_px: f32) -> Option<usize> {
    let mut pick = None;
    for (k, e) in errors.into_iter().enumerate() {
        if k == 0 || e * texels_per_metre <= lod_px {
            pick = Some(k);
        }
    }
    pick
}

/// Why an instance produced no work item.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CullReason {
    Layer,
    Inactive,
    Frustum,
    Horizon,
    /// Smaller than `size_cull_px` in the view's own units: screen pixels in a perspective
    /// view, shadow-map texels in an orthographic one ([`cull_instance`]).
    SubPixel,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Draw { level: u8 },
    Culled(CullReason),
}

/// One draw: what the GPU pass emits per surviving instance (§4 step 7).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WorkItem {
    /// The caller's cell index.
    pub cell: u32,
    /// Index in the cell's table (slot = the cell's range start + this).
    pub instance: u32,
    pub placement: PlacementId,
    /// The batch set key (§5): the chain's material class.
    pub material_class: u16,
    /// Cell-local chain index (the GPU pass reads the global slot from the remapped copy).
    pub chain: u32,
    pub level: u8,
}

/// Instance counts per outcome.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CullStats {
    pub drawn: u32,
    pub layer: u32,
    pub inactive: u32,
    pub frustum: u32,
    pub horizon: u32,
    pub sub_pixel: u32,
}

impl CullStats {
    fn count(&mut self, reason: CullReason, n: u32) {
        let slot = match reason {
            CullReason::Layer => &mut self.layer,
            CullReason::Inactive => &mut self.inactive,
            CullReason::Frustum => &mut self.frustum,
            CullReason::Horizon => &mut self.horizon,
            CullReason::SubPixel => &mut self.sub_pixel,
        };
        *slot += n;
    }
}

/// Steps 1 (active flag), 2, 4 and 5 for one instance (layers and the horizon are per table,
/// [`cull_cell`]). `offset` places the cell in the cull frame ([`crate::cell_offset`]); `errors`
/// are its chain's level errors, model-space (scaled here by [`StaticInstance::max_scale`]).
/// Both the sub-pixel test and the LOD pick take the form of the view's [`SceneProjection`].
pub fn cull_instance(view: &SceneView, instance: &StaticInstance, offset: Vec3, errors: impl IntoIterator<Item = f32>) -> Verdict {
    if instance.flags.contains(InstanceFlags::INACTIVE) {
        return Verdict::Culled(CullReason::Inactive);
    }
    let center = instance.sphere.center + offset;
    let r = instance.sphere.radius;
    if view.frustum.sphere_outside(center, r) {
        return Verdict::Culled(CullReason::Frustum);
    }
    let size = view.size_cull_px;
    // Step 4 is a *threshold*, so 0 is "no threshold": it turns the test off in either
    // projection, and is the default for shadow views ([`ScenePreset::size_cull_px`]).
    let sub_pixel_cull = size > 0.0;
    let surface = match view.projection {
        SceneProjection::Perspective { pixels_per_radian: ppr } => {
            // Projected radius at the screen centre, ppr · r / √(d² − r²): drop under
            // size_cull_px (squared, as the GPU pass does it: no sqrt, no boundary drift).
            let d2 = center.distance_squared(view.eye);
            if sub_pixel_cull && d2 > r * r && ppr * ppr * r * r < size * size * (d2 - r * r) {
                return Verdict::Culled(CullReason::SubPixel);
            }
            (d2.sqrt() - r).max(0.0)
        }
        SceneProjection::Orthographic { texels_per_metre: tpm } => {
            // Step 4 in the view's own units: an orthographic projection has no distance term,
            // so the radius covers r · texels_per_metre texels of the shadow map wherever the
            // instance sits. "Sub-pixel on screen" would be the wrong question here; dropping a
            // caster smaller than a map texel is the standard minimum-caster-radius cull of
            // shipped cascade implementations (UE's shadow radius threshold), and it falls out
            // of the cascade's own density: a small object keeps its shadow in the dense near
            // cascade and loses it in a far one, where its shadow is sub-pixel on screen anyway.
            // `size_cull_px = 0` turns it off for a view that wants every caster, which is what
            // a shadow view gets by default until the per-cascade form is measured
            // ([`ScenePreset::size_cull_px`]).
            if sub_pixel_cull && r * tpm < size {
                return Verdict::Culled(CullReason::SubPixel);
            }
            // No distance: the pick ignores it.
            0.0
        }
    };
    let scale = instance.max_scale();
    let level = view.projection.pick_level(errors.into_iter().map(|e| e * scale), surface, view.lod_px).unwrap_or(0);
    Verdict::Draw { level: u8::try_from(level).unwrap_or(u8::MAX) }
}

/// The whole cell: pre-cull (the table's layers, the AABB against the frustum, then the horizon
/// when `view.horizon` and a grid are given), then every instance; promoted placements count as
/// inactive. Work items are appended to `out` in table order.
///
/// # Panics
///
/// Debug builds only: `view.horizon` on an orthographic view. The horizon test marches from
/// `view.eye` ([`HorizonGrid::land_hides`]) and an orthographic view has no eye point, so the
/// combination has no meaning and the result would depend on whatever was left in `eye`. Both
/// fields document it; the assert is so the slices that build shadow views (S5) cannot get it
/// wrong silently. Not an error in release: a release build culls one cell too few or too many,
/// which must never take the game down.
pub fn cull_cell(
    view: &SceneView,
    cell: u32,
    table: &StaticCellTableView<'_>,
    offset: Vec3,
    horizon: Option<&HorizonGrid>,
    promotions: &Promotions,
    out: &mut Vec<WorkItem>,
) -> CullStats {
    debug_assert!(
        !(view.horizon && matches!(view.projection, SceneProjection::Orthographic { .. })),
        "SceneView::horizon on an orthographic view: it has no eye point, and the horizon test \
         marches from SceneView::eye (HorizonGrid::land_hides). Shadow views set horizon: false."
    );
    let mut stats = CullStats::default();
    let n = u32::try_from(table.len()).unwrap_or(u32::MAX);
    if table.layers() & view.layers == 0 {
        stats.count(CullReason::Layer, n);
        return stats;
    }
    let aabb = table.aabb().translated(offset);
    if view.frustum.aabb_outside(&aabb) {
        stats.count(CullReason::Frustum, n);
        return stats;
    }
    if view.horizon && horizon.is_some_and(|h| h.land_hides(view.eye, &aabb)) {
        stats.count(CullReason::Horizon, n);
        return stats;
    }
    for (k, mut instance) in (0u32..).zip(table.instances()) {
        if promotions.is_promoted(instance.placement) {
            instance.flags |= InstanceFlags::INACTIVE;
        }
        let errors = table.levels(instance.chain).into_iter().flatten().map(|l| l.error);
        match cull_instance(view, &instance, offset, errors) {
            Verdict::Draw { level } => {
                stats.drawn += 1;
                out.push(WorkItem {
                    cell,
                    instance: k,
                    placement: instance.placement,
                    material_class: table.material_class(instance.chain).map_or(0, |c| c as u16),
                    chain: instance.chain,
                    level,
                });
            }
            Verdict::Culled(reason) => stats.count(reason, 1),
        }
    }
    stats
}

/// Horizon test settings.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HorizonParams {
    /// Metres added above the box top before marching (conservative for terrain between samples
    /// and thin tops). SPUD: 3 m.
    pub margin: f32,
    /// Boxes closer than this many grid cells (horizontal distance) are never horizon-culled.
    pub min_cells: f32,
}

impl Default for HorizonParams {
    fn default() -> Self {
        Self { margin: 3.0, min_cells: 4.0 }
    }
}

/// A grid of land minimum heights over the XZ plane of the cull frame: under no point of a grid
/// cell is the land lower than its value, so a sight line below it there is blocked. Cells the
/// grid doesn't cover, and cells with terrain holes, are `f32::NEG_INFINITY` (never block).
#[derive(Clone, Debug, PartialEq)]
pub struct HorizonGrid {
    /// XZ of grid cell (0, 0)'s minimum corner.
    pub origin: Vec2,
    pub cell_size: f32,
    pub width: u32,
    pub depth: u32,
    /// Row-major, `z * width + x`.
    pub min_height: Vec<f32>,
    pub params: HorizonParams,
}

impl HorizonGrid {
    /// A grid with no land (nothing blocks).
    pub fn new(origin: Vec2, cell_size: f32, width: u32, depth: u32) -> Self {
        Self {
            origin,
            cell_size,
            width,
            depth,
            min_height: vec![f32::NEG_INFINITY; width as usize * depth as usize],
            params: HorizonParams::default(),
        }
    }

    pub fn set(&mut self, x: u32, z: u32, min_height: f32) {
        if x < self.width && z < self.depth {
            self.min_height[z as usize * self.width as usize + x as usize] = min_height;
        }
    }

    /// The land floor of grid cell `cell` (`NEG_INFINITY` outside the grid).
    pub fn floor(&self, cell: IVec2) -> f32 {
        match (u32::try_from(cell.x), u32::try_from(cell.y)) {
            (Ok(x), Ok(z)) if x < self.width && z < self.depth => self.min_height[z as usize * self.width as usize + x as usize],
            _ => f32::NEG_INFINITY,
        }
    }

    fn grid_cell(&self, p: Vec3) -> IVec2 {
        ((Vec2::new(p.x, p.z) - self.origin) / self.cell_size).floor().as_ivec2()
    }

    /// SPUD's `land_hides`: march from `eye` to 9 points on the box top (corners, edge middles,
    /// centre; raised by the margin); hidden only when the land floor blocks all 9. Never hides
    /// a box within `min_cells`, nor anything when the eye is under its own cell's floor.
    pub fn land_hides(&self, eye: Vec3, aabb: &Aabb) -> bool {
        let nearest = Vec2::new(eye.x.clamp(aabb.min.x, aabb.max.x), eye.z.clamp(aabb.min.z, aabb.max.z));
        if nearest.distance(Vec2::new(eye.x, eye.z)) < self.params.min_cells * self.cell_size {
            return false;
        }
        if eye.y < self.floor(self.grid_cell(eye)) {
            return false;
        }
        let top = aabb.max.y + self.params.margin;
        let size = aabb.max - aabb.min;
        [0.0, 0.5, 1.0].iter().all(|&fx| {
            [0.0, 0.5, 1.0].iter().all(|&fz| {
                let target = Vec3::new(aabb.min.x + size.x * fx, top, aabb.min.z + size.z * fz);
                self.segment_blocked(eye, target)
            })
        })
    }

    /// Walks the grid cells the segment crosses (Amanatides-Woo in XZ); blocked when, inside
    /// some cell, the segment dips under that cell's floor (the segment is linear, so its lowest
    /// point in a cell is at the cell's entry or exit).
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn segment_blocked(&self, from: Vec3, to: Vec3) -> bool {
        // Grid coordinates: x = world X, y = world Z, in cells.
        let start = (Vec2::new(from.x, from.z) - self.origin) / self.cell_size;
        let end = (Vec2::new(to.x, to.z) - self.origin) / self.cell_size;
        let delta = end - start;
        let mut cell = start.floor().as_ivec2();
        let last = end.floor().as_ivec2();
        let step = IVec2::new(delta.x.signum() as i32, delta.y.signum() as i32);
        // Per axis: the segment parameter of the next cell boundary, and its increment per cell.
        let boundary = |g: f32, c: i32, d: f32| {
            if d > 0.0 {
                ((c as f32 + 1.0 - g) / d, 1.0 / d)
            } else if d < 0.0 {
                ((c as f32 - g) / d, -1.0 / d)
            } else {
                (f32::INFINITY, f32::INFINITY)
            }
        };
        let columns = boundary(start.x, cell.x, delta.x);
        let rows = boundary(start.y, cell.y, delta.y);
        let mut t_next = Vec2::new(columns.0, rows.0);
        let t_inc = Vec2::new(columns.1, rows.1);
        let steps = (i64::from(last.x) - i64::from(cell.x)).abs() + (i64::from(last.y) - i64::from(cell.y)).abs();
        let height = |t: f32| from.y + (to.y - from.y) * t.min(1.0);
        let mut t_enter = 0.0f32;
        for _ in 0..=steps {
            let t_exit = t_next.min_element().min(1.0);
            if height(t_enter).min(height(t_exit)) < self.floor(cell) {
                return true;
            }
            if t_next.x < t_next.y {
                cell.x += step.x;
                t_enter = t_next.x;
                t_next.x += t_inc.x;
            } else {
                cell.y += step.y;
                t_enter = t_next.y;
                t_next.y += t_inc.y;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests;
