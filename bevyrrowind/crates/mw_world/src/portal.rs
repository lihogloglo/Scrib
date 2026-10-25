//! Portal frames from door geometry (M8 preparation, Bevy-free).
//!
//! Spec: `docs/specs/world-adapter.md` ("Portal frames: feasibility").
//! superfargoth's `LinkEnd { frame, extent }` wants a frame on the door
//! plane with +Z pointing into the end's worldspace, and the opening's
//! width and height. Morrowind has no portal data, so this module derives
//! them from each door's model bounds and its reference pose:
//!
//! - the slab is the model's visible bounds (unit scale) times the
//!   reference scale; its thinnest axis is the door normal, the more
//!   upright of the other two is the height, the remaining one the width;
//! - the opening is the slab's face rectangle; the frame sits at its
//!   bottom center (the sill), so floors line up across a size mismatch;
//! - the normal's sign comes from the arrival marker that lands in front
//!   of this door (the partner's destination): travelers appear on the
//!   front side, so the front is the side the worldspace opens into;
//! - the hinge is where the model origin sits along the width (Morrowind
//!   swinging doors rotate about their origin's vertical axis).
//!
//! [`analyze`] runs this over every door end and pairs, and classifies
//! each link as portal-ready, portal-with-adjustment, or fade-only. The
//! baked `DoorLink` format is unchanged: wiring `frame`/`extent` into
//! `worldspaces::link_doors` is the documented next step.

use std::collections::HashMap;

use glam::{Mat3, Quat, Vec2, Vec3};
use sf_coords::{WorldPos, WorldTransform};
use sf_records::{RecordDb, RecordId, SourceId};
use sf_world::{DoorLink, PlacementId};
use tes3::esp::{Cell, Door};

use crate::ids;
use crate::source::ModelBounds;
use crate::worldspaces::{DoorEnd, PAIR_DISTANCE_M};

/// A slab thicker than this fraction of its width is not a door slab
/// (rock cave mouths, hatches with frames, blobs).
pub const SLAB_MAX_THICKNESS_RATIO: f32 = 0.35;
/// The door normal is horizontal (a trapdoor or hatch) above this |y|.
pub const HORIZONTAL_NORMAL_Y: f32 = 0.7;
/// Pairs whose openings differ by at most this ratio (per dimension) are
/// portal-ready; up to [`ADJUST_MAX_RATIO`] they need an extent fix.
pub const READY_MAX_RATIO: f32 = 1.15;
pub const ADJUST_MAX_RATIO: f32 = 2.0;

/// What a door model looks like as a portal opening.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DoorShape {
    /// An upright, thin slab: a real door.
    Slab,
    /// Upright but thick: a cave mouth, an arch, a door with its frame.
    Thick,
    /// The thin axis is vertical: trapdoors, hatches, floor grates.
    Horizontal,
    /// No model, or a model without visible geometry.
    Missing,
}

/// A door reference and its derived portal frame.
#[derive(Clone, Debug, PartialEq)]
pub struct DoorPortal {
    pub id: PlacementId,
    pub cell: RecordId,
    /// Base record id.
    pub base: String,
    pub model: Option<String>,
    pub geometry: DoorFrame,
}

/// Derives a door's portal frame. `bounds` are the model's engine-space
/// visible bounds at unit scale; `front` is a point on the front side
/// (the arrival marker that lands at this door).
#[allow(clippy::many_single_char_names)]
pub fn door_portal(
    pose: &WorldTransform,
    scale: f32,
    bounds: Option<([f32; 3], [f32; 3])>,
    front: Option<WorldPos>,
) -> DoorFrame {
    let Some((lo, hi)) = bounds.map(|(lo, hi)| (Vec3::from_array(lo), Vec3::from_array(hi))) else {
        return DoorFrame {
            shape: DoorShape::Missing,
            frame: level(pose),
            extent: Vec2::ZERO,
            thickness: 0.0,
            hinge: 0.0,
            front_known: false,
            front_flipped: false,
        };
    };
    let size = (hi - lo) * scale;
    let center = (hi + lo) * 0.5 * scale;
    let rot = pose.rotation;
    let axes = [Vec3::X, Vec3::Y, Vec3::Z];
    // Thin axis: the smallest extent.
    let thin = (0..3).min_by(|&a, &b| size[a].total_cmp(&size[b])).unwrap_or(2);
    let others: Vec<usize> = (0..3).filter(|&a| a != thin).collect();
    // Height: the more upright of the other two, in world space.
    let (h, mut w) = if (rot * axes[others[0]]).y.abs() >= (rot * axes[others[1]]).y.abs() {
        (others[0], others[1])
    } else {
        (others[1], others[0])
    };
    let center_world = pose.pos.offset(rot * center);
    // A thick upright model (grate in a frame, cave mouth, dwemer hatch)
    // has no reliable thin axis: the arrival side picks between the two
    // horizontal axes instead.
    let mut thin = thin;
    let upright = (rot * axes[thin]).y.abs() <= HORIZONTAL_NORMAL_Y;
    if let (Some(p), true) = (front, upright && size[thin] > SLAB_MAX_THICKNESS_RATIO * size[w]) {
        let d = center_world.delta_f32(p);
        if d.dot(rot * axes[w]).abs() > d.dot(rot * axes[thin]).abs() {
            std::mem::swap(&mut thin, &mut w);
        }
    }
    let mut normal = rot * axes[thin];
    let mut up = rot * axes[h];
    if up.y < 0.0 {
        up = -up;
    }
    let front_flipped = front.is_some_and(|p| center_world.delta_f32(p).dot(normal) < 0.0);
    if front_flipped {
        normal = -normal;
    }
    let horizontal = normal.y.abs() > HORIZONTAL_NORMAL_Y;
    let (width, height, thickness) = (size[w], size[h], size[thin]);
    let shape = if horizontal {
        DoorShape::Horizontal
    } else if thickness > SLAB_MAX_THICKNESS_RATIO * width.max(thickness) {
        DoorShape::Thick
    } else {
        DoorShape::Slab
    };
    // Orthonormal basis: Z = normal, Y = up made perpendicular, X = Y × Z.
    let z = normal.normalize();
    let y = (up - z * up.dot(z)).try_normalize().unwrap_or(Vec3::Y);
    let x = y.cross(z);
    let rotation = Quat::from_mat3(&Mat3::from_cols(x, y, z)).normalize();
    // Model origin relative to the center, along the width axis.
    let hinge = if width > 0.0 { (rot * -center).dot(x) / width } else { 0.0 };
    DoorFrame {
        shape,
        frame: WorldTransform { pos: center_world.offset(-y * height * 0.5), rotation, scale: Vec3::ONE },
        extent: Vec2::new(width, height),
        thickness,
        hinge,
        front_known: front.is_some(),
        front_flipped,
    }
}

/// A door's portal frame, derived from its geometry by [`door_portal`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DoorFrame {
    pub shape: DoorShape,
    /// The opening's bottom center (sill) on the slab's mid-plane; X along
    /// the width, Y up (the slab's height axis), +Z out of the front. The
    /// opening spans x in ±width/2, y in 0..height. Scale is one.
    pub frame: WorldTransform,
    /// Opening width and height, meters.
    pub extent: Vec2,
    /// Slab thickness, meters.
    pub thickness: f32,
    /// The model origin along frame X as a fraction of the width: -0.5 is
    /// the left edge (seen from the front), 0.5 the right, 0 centered.
    pub hinge: f32,
    /// Whether an arrival marker fixed the front side. Otherwise +Z is the
    /// model's own +thin axis, a guess.
    pub front_known: bool,
    /// The front is the model's -thin axis (meaningful with `front_known`):
    /// for checking per-model conventions.
    pub front_flipped: bool,
}

/// The pose's yaw only, at its position: the frame of a door without
/// geometry.
fn level(pose: &WorldTransform) -> WorldTransform {
    let f = pose.rotation * Vec3::Z;
    let yaw = f.x.atan2(f.z);
    WorldTransform { pos: pose.pos, rotation: Quat::from_rotation_y(yaw), scale: Vec3::ONE }
}

/// The engine-space forward of an arrival marker: Morrowind's +Y (north
/// at zero yaw) through the reference rotation, i.e. engine −Z rotated.
pub fn arrival_forward(arrival: &WorldTransform) -> Vec3 {
    arrival.rotation * Vec3::NEG_Z
}

/// How an arrival marker sits relative to the door it lands at.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArrivalCheck {
    /// Horizontal distance from the opening center, meters.
    pub distance: f32,
    /// Signed distance along the frame's +Z (front), meters.
    pub ahead: f32,
    /// Signed offset along the frame's X, meters.
    pub lateral: f32,
    /// Height above the opening's bottom edge, meters.
    pub above_sill: f32,
    /// Cosine between the marker's horizontal forward and the frame's
    /// horizontal +Z: 1 walks straight out of the door, -1 faces it.
    pub facing: f32,
    /// The marker carries pitch or roll (more than ~3 degrees).
    pub tilted: bool,
}

/// How a marker faces relative to the door it lands at.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Facing {
    /// Within 60 degrees of straight out of the door.
    Away,
    /// 60 to 120 degrees off: along the wall the door sits in.
    Sideways,
    /// More than 120 degrees off: looking back at the door.
    TowardDoor,
}

impl ArrivalCheck {
    pub fn of(door: &DoorFrame, arrival: &WorldTransform) -> Self {
        let d = door.frame.pos.delta_f32(arrival.pos);
        let (x, z) = (door.frame.rotation * Vec3::X, door.frame.rotation * Vec3::Z);
        let flat = |v: Vec3| Vec3::new(v.x, 0.0, v.z).normalize_or_zero();
        let f = arrival_forward(arrival);
        let up = arrival.rotation * Vec3::Y;
        Self {
            distance: Vec2::new(d.x, d.z).length(),
            ahead: d.dot(z),
            lateral: d.dot(x),
            above_sill: d.y,
            facing: flat(f).dot(flat(z)),
            tilted: up.y < 0.9986,
        }
    }

    pub fn facing_class(&self) -> Facing {
        if self.facing >= 0.5 {
            Facing::Away
        } else if self.facing > -0.5 {
            Facing::Sideways
        } else {
            Facing::TowardDoor
        }
    }
}

/// Whether a link can render as a portal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PortalClass {
    /// Both ends are upright slabs of (nearly) the same opening.
    Ready,
    /// Both ends upright, but openings differ or one is thick (cave
    /// mouth): portal with a clamped/shared extent.
    Adjust,
    /// One-way, a horizontal or missing door, or very different openings.
    FadeOnly,
}

/// Why a link got its class (first reason that applied).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Reason {
    Congruent,
    SizeMismatch,
    ThickEnd,
    OneWay,
    Horizontal,
    MissingModel,
    TooDifferent,
}

/// One door link, as portal material.
#[derive(Clone, Debug, PartialEq)]
pub struct LinkPortal {
    /// Index of the near door in [`PortalReport::doors`].
    pub a: usize,
    /// The partner door, for two-way links.
    pub b: Option<usize>,
    /// Larger of the width and height ratios (≥ 1); 0 without a partner.
    pub size_ratio: f32,
    /// The arrival at `a` (from `b`'s destination) and at `b`.
    pub arrival_a: Option<ArrivalCheck>,
    pub arrival_b: Option<ArrivalCheck>,
    pub class: PortalClass,
    pub reason: Reason,
}

/// Every door's frame and every link's class.
#[derive(Clone, Debug, Default)]
pub struct PortalReport {
    pub doors: Vec<DoorPortal>,
    pub links: Vec<LinkPortal>,
    /// Doors with two or more back-pointing candidates within the pairing
    /// threshold (pairing picked the closest).
    pub ambiguous: Vec<usize>,
}

impl PortalReport {
    pub fn count(&self, class: PortalClass) -> usize {
        self.links.iter().filter(|l| l.class == class).count()
    }
}

/// A door reference's model and scale.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DoorModel {
    pub model: Option<String>,
    pub scale: f32,
}

/// The model and scale of every door reference with a destination.
pub fn door_models(db: &RecordDb) -> HashMap<PlacementId, DoorModel> {
    let mut out = HashMap::new();
    for (_, cell) in db.iter::<Cell>() {
        for (&(source, refr_index), reference) in &cell.references {
            if reference.destination.is_none() {
                continue;
            }
            let Some(door) = db.get::<Door>(&RecordId::new(&reference.id)) else { continue };
            let file = &db.source(SourceId(source)).name;
            out.insert(
                ids::placement_id(file, refr_index),
                DoorModel { model: crate::model_path(&door.mesh), scale: reference.scale.unwrap_or(1.0) },
            );
        }
    }
    out
}

/// Visible bounds (engine space, unit scale) of every door model that
/// loads, keyed by VFS path. Door models are a few hundred, so this reads
/// them serially; the bake would use its cooked model bounds instead.
#[allow(clippy::implicit_hasher)]
pub fn load_bounds(
    vfs: &sf_vfs::Vfs,
    models: &HashMap<PlacementId, DoorModel>,
) -> HashMap<String, ([f32; 3], [f32; 3])> {
    let mut paths: Vec<&str> = models.values().filter_map(|m| m.model.as_deref()).collect();
    paths.sort_unstable();
    paths.dedup();
    paths
        .into_iter()
        .filter_map(|p| {
            let bytes = vfs.read(&sf_vfs::VfsPath::new(p).ok()?).ok()?;
            let model = mw_nif::load(&bytes).ok()?;
            Some((p.to_owned(), mw_nif::visible_bounds(&model)?))
        })
        .collect()
}

/// Derives every door's frame and classifies every link. `ends` and
/// `links` come from `worldspaces::door_ends` / `link_doors`.
#[allow(clippy::implicit_hasher, clippy::many_single_char_names)]
pub fn analyze(
    ends: &[DoorEnd],
    links: &[(RecordId, DoorLink)],
    models: &HashMap<PlacementId, DoorModel>,
    bounds: ModelBounds<'_>,
) -> PortalReport {
    let index: HashMap<PlacementId, usize> = ends.iter().enumerate().map(|(i, e)| (e.id, i)).collect();
    let mut partner: Vec<Option<usize>> = vec![None; ends.len()];
    for (_, link) in links.iter().filter(|(_, l)| l.two_way) {
        if let (Some(a), Some(b)) = (link.a.door.and_then(|d| index.get(&d)), link.b.door.and_then(|d| index.get(&d))) {
            partner[*a] = Some(*b);
            partner[*b] = Some(*a);
        }
    }
    let doors: Vec<DoorPortal> = ends
        .iter()
        .enumerate()
        .map(|(i, e)| {
            let m = models.get(&e.id).cloned().unwrap_or(DoorModel { model: None, scale: 1.0 });
            let b = m.model.as_deref().and_then(bounds);
            // The arrival landing here is the partner's destination.
            let front = partner[i].map(|j| ends[j].arrival.pos);
            DoorPortal {
                id: e.id,
                cell: e.cell.clone(),
                base: e.base.clone(),
                model: m.model,
                geometry: door_portal(&e.pose, m.scale, b, front),
            }
        })
        .collect();
    let mut out_links = Vec::new();
    for (_, link) in links {
        let Some(&a) = link.a.door.and_then(|d| index.get(&d)) else { continue };
        let b = partner[a].filter(|_| link.two_way);
        let (da, db_) = (&doors[a], b.map(|j| &doors[j]));
        let size_ratio = db_.map_or(0.0, |d| ratio(da.geometry.extent, d.geometry.extent));
        let shapes = [Some(da.geometry.shape), db_.map(|d| d.geometry.shape)];
        let (class, reason) = match b {
            None => (PortalClass::FadeOnly, Reason::OneWay),
            Some(_) if shapes.contains(&Some(DoorShape::Missing)) => (PortalClass::FadeOnly, Reason::MissingModel),
            Some(_) if shapes.contains(&Some(DoorShape::Horizontal)) => (PortalClass::FadeOnly, Reason::Horizontal),
            Some(_) if size_ratio > ADJUST_MAX_RATIO => (PortalClass::FadeOnly, Reason::TooDifferent),
            Some(_) if shapes.contains(&Some(DoorShape::Thick)) => (PortalClass::Adjust, Reason::ThickEnd),
            Some(_) if size_ratio > READY_MAX_RATIO => (PortalClass::Adjust, Reason::SizeMismatch),
            Some(_) => (PortalClass::Ready, Reason::Congruent),
        };
        let arrival_a = b.map(|j| ArrivalCheck::of(&doors[a].geometry, &ends[j].arrival));
        let arrival_b = b.map(|j| ArrivalCheck::of(&doors[j].geometry, &ends[a].arrival));
        out_links.push(LinkPortal { a, b, size_ratio, arrival_a, arrival_b, class, reason });
    }

    // Ambiguity: more than one back-pointing door within the threshold.
    let limit = f64::from(PAIR_DISTANCE_M);
    let mut ambiguous = Vec::new();
    for (i, a) in ends.iter().enumerate() {
        let n = ends
            .iter()
            .enumerate()
            .filter(|&(j, b)| {
                j != i
                    && b.worldspace == a.destination
                    && b.destination == a.worldspace
                    && b.pose.pos.delta(a.arrival.pos).length() <= limit
                    && a.pose.pos.delta(b.arrival.pos).length() <= limit
            })
            .count();
        if n > 1 {
            ambiguous.push(i);
        }
    }
    PortalReport { doors, links: out_links, ambiguous }
}

/// Larger of the per-dimension ratios between two openings (≥ 1).
fn ratio(a: Vec2, b: Vec2) -> f32 {
    let r = |x: f32, y: f32| if x.min(y) <= 1e-4 { f32::INFINITY } else { x.max(y) / x.min(y) };
    r(a.x, b.x).max(r(a.y, b.y))
}

/// Yaw of a frame's +Z about world up, degrees (0 = engine +Z, south).
pub fn yaw_degrees(frame: &WorldTransform) -> f32 {
    let z = frame.rotation * Vec3::Z;
    z.x.atan2(z.z).to_degrees()
}


/// Floor probes start this far above a frame's sill, meters.
pub const FLOOR_PROBE_HEIGHT_M: f32 = 1.2;
/// ... and reach this far down (to 0.8 m below the sill).
pub const FLOOR_PROBE_DEPTH_M: f32 = 2.0;
/// Probe points in front of the door's mid-plane (frame +Z), meters.
pub const FLOOR_PROBES_M: [f32; 2] = [0.3, 0.6];
/// A floor farther than this from the slab's sill is not the door's floor.
pub const FLOOR_MAX_OFFSET_M: f32 = 0.6;
/// Solids whose origin lies within this distance of a probe are tested.
pub const FLOOR_REACH_M: f32 = 30.0;

/// The solids (statics, activators, containers) of a cell that floor
/// probes test: model path and pose, gathered without parsing models.
pub fn cell_solids(db: &RecordDb, cell: &RecordId) -> Vec<(String, WorldTransform)> {
    let Some(cell) = db.get::<Cell>(cell) else { return Vec::new() };
    let mut out: Vec<(String, WorldTransform)> = cell
        .references
        .values()
        .filter_map(|r| {
            let (kind, model, _) = crate::resolve_base(db, &r.id)?;
            if !matches!(kind, crate::BaseKind::Static | crate::BaseKind::Activator | crate::BaseKind::Container) {
                return None;
            }
            Some((model?, crate::convert::world_transform(r.translation, r.rotation, r.scale.unwrap_or(1.0))))
        })
        .collect();
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

/// A model's visible triangles (model space, meters) and their bounds.
#[derive(Clone, Debug)]
pub struct ModelTris {
    lo: Vec3,
    hi: Vec3,
    tris: Vec<[Vec3; 3]>,
}

impl ModelTris {
    pub fn new(tris: Vec<[Vec3; 3]>) -> Option<Self> {
        let (lo, hi) = tris.iter().flatten().fold((Vec3::MAX, Vec3::MIN), |(lo, hi), p| (lo.min(*p), hi.max(*p)));
        (!tris.is_empty()).then_some(Self { lo, hi, tris })
    }

    /// The visible, unskinned meshes of a NIF.
    pub fn load(vfs: &sf_vfs::Vfs, path: &str) -> Option<Self> {
        let bytes = vfs.read(&sf_vfs::VfsPath::new(path).ok()?).ok()?;
        let model = mw_nif::load(&bytes).ok()?;
        let world = mw_nif::world_transforms(&model);
        let mut tris = Vec::new();
        for (node, t) in model.nodes.iter().zip(&world) {
            let (true, Some(m)) = (node.visible, node.mesh) else { continue };
            let Some(mesh) = model.meshes.get(m) else { continue };
            if mesh.skinned {
                continue;
            }
            let p = |i: u32| mesh.positions.get(i as usize).map(|p| Vec3::from_array(t.transform_point3((*p).into()).to_array()));
            for c in mesh.indices.as_chunks::<3>().0 {
                if let (Some(a), Some(b), Some(c)) = (p(c[0]), p(c[1]), p(c[2])) {
                    tris.push([a, b, c]);
                }
            }
        }
        Self::new(tris)
    }

    /// Distance straight down from `origin` to the first floor-like
    /// (|normal.y| above [`HORIZONTAL_NORMAL_Y`]) triangle of this model
    /// placed at `pose`, within `depth` meters (Moller-Trumbore, both
    /// windings).
    #[allow(clippy::many_single_char_names, reason = "Moller-Trumbore's names")]
    pub fn cast_down(&self, pose: &WorldTransform, origin: WorldPos, depth: f32) -> Option<f32> {
        let s = pose.scale.x.max(1e-6);
        let inv = pose.rotation.inverse();
        let o = inv * pose.pos.delta_f32(origin) / s;
        let d = inv * Vec3::NEG_Y;
        let max_t = depth / s;
        let end = o + d * max_t;
        if o.min(end).cmpgt(self.hi).any() || o.max(end).cmplt(self.lo).any() {
            return None;
        }
        let mut best: Option<f32> = None;
        for [a, b, c] in &self.tris {
            let (e1, e2) = (*b - *a, *c - *a);
            if (pose.rotation * e1.cross(e2).normalize_or_zero()).y.abs() < HORIZONTAL_NORMAL_Y {
                continue;
            }
            let h = d.cross(e2);
            let det = e1.dot(h);
            if det.abs() < 1e-12 {
                continue;
            }
            let f = 1.0 / det;
            let sv = o - *a;
            let u = f * sv.dot(h);
            let q = sv.cross(e1);
            let v = f * d.dot(q);
            let t = f * e2.dot(q);
            if (0.0..=1.0).contains(&u) && v >= 0.0 && u + v <= 1.0 && (0.0..=max_t).contains(&t) && best.is_none_or(|b| t < b) {
                best = Some(t);
            }
        }
        best.map(|t| t * s)
    }
}

/// Height of the walkable floor in front of a door above its frame's sill
/// (meters; positive: the floor is above the slab's bottom). Morrowind door
/// slabs often reach below the floor (the Census Office's interior door),
/// so a sill at the slab's bottom shows the far floor's underside (a void
/// strip) at the bottom of the opening. Two rays go down in front of the
/// door ([`FLOOR_PROBES_M`], from [`FLOOR_PROBE_HEIGHT_M`] above the sill)
/// against `solids` ([`cell_solids`]) within [`FLOOR_REACH_M`]; each takes its first hit and the
/// lower of the two wins (clutter right at a door is rarer than a step).
/// `None`: no floor within [`FLOOR_MAX_OFFSET_M`] of the sill (a door on
/// terrain keeps its slab's sill). `model` returns a parsed model by path.
pub fn floor_offset(
    frame: &WorldTransform,
    solids: &[(String, WorldTransform)],
    mut model: impl FnMut(&str) -> Option<std::sync::Arc<ModelTris>>,
) -> Option<f32> {
    let mut lowest: Option<f32> = None;
    for z in FLOOR_PROBES_M {
        let origin = frame.pos.offset(frame.rotation * Vec3::new(0.0, FLOOR_PROBE_HEIGHT_M, z));
        let first = solids
            .iter()
            .filter(|(_, pose)| pose.pos.delta_f32(origin).length() <= FLOOR_REACH_M)
            .filter_map(|(path, pose)| model(path)?.cast_down(pose, origin, FLOOR_PROBE_DEPTH_M))
            .reduce(f32::min);
        if let Some(t) = first {
            lowest = Some(lowest.map_or(t, |l| l.max(t)));
        }
    }
    lowest.map(|t| FLOOR_PROBE_HEIGHT_M - t).filter(|o| o.abs() <= FLOOR_MAX_OFFSET_M)
}

/// The shared opening of a portal pair (the M8 "common opening"): each end
/// grounded on its floor (`floor`: [`floor_offset`] per end), then the
/// per-dimension minimum of the two openings, centered on each slab and
/// standing on each floor. The crossing transform is rigid, so equal
/// openings make each side's opening coincide with the other's exactly.
pub fn common_opening(a: &mut sf_world::LinkEnd, b: &mut sf_world::LinkEnd, floor: [Option<f32>; 2]) {
    for (end, dy) in [(&mut *a, floor[0]), (&mut *b, floor[1])] {
        if let Some(dy) = dy {
            end.frame.pos = end.frame.pos.offset(end.frame.rotation * Vec3::Y * dy);
            end.extent.y = (end.extent.y - dy).max(0.0);
        }
    }
    let common = a.extent.min(b.extent);
    a.extent = common;
    b.extent = common;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floor_probe_finds_a_raised_floor_and_the_common_opening_aligns() {
        // A floor quad 0.3 m above the sill, 4 x 4 m around the door.
        let floor = ModelTris::new(vec![
            [Vec3::new(-2.0, 0.3, -2.0), Vec3::new(2.0, 0.3, -2.0), Vec3::new(2.0, 0.3, 2.0)],
            [Vec3::new(-2.0, 0.3, -2.0), Vec3::new(2.0, 0.3, 2.0), Vec3::new(-2.0, 0.3, 2.0)],
        ])
        .map(std::sync::Arc::new);
        let frame = WorldTransform { pos: WorldPos::new(glam::IVec3::ZERO, Vec3::ZERO), rotation: Quat::IDENTITY, scale: Vec3::ONE };
        let solids = vec![("floor".to_owned(), frame)];
        let dy = floor_offset(&frame, &solids, |_| floor.clone()).expect("floor");
        assert!((dy - 0.3).abs() < 1e-4, "{dy}");
        // No floor near the sill: keep the slab's.
        assert!(floor_offset(&frame, &[], |_| floor.clone()).is_none());

        // Two ends of different size: equal openings, grounded.
        let end = |x: f32, extent: Vec2| sf_world::LinkEnd {
            frame: WorldTransform { pos: WorldPos::new(glam::IVec3::ZERO, Vec3::new(x, 0.0, 0.0)), ..frame },
            extent,
            worldspace: sf_world::WorldspaceId::new(if x > 0.0 { "a" } else { "b" }),
            arrival: frame,
            door: None,
        };
        let (mut a, mut b) = (end(10.0, Vec2::new(2.08, 3.57)), end(-10.0, Vec2::new(1.64, 2.81)));
        common_opening(&mut a, &mut b, [Some(0.3), None]);
        assert_eq!(a.extent, b.extent);
        assert!((a.extent - Vec2::new(1.64, 2.81)).abs().max_element() < 1e-5);
        assert!(a.frame.pos.delta_f32(WorldPos::new(glam::IVec3::ZERO, Vec3::new(10.0, 0.3, 0.0))).length() < 1e-5);
        // Every corner of b's opening maps onto a's (rigid crossing, same
        // extent): the two sides' openings coincide on screen.
        for (x, y) in [(-0.5, 0.0), (0.5, 0.0), (-0.5, 1.0), (0.5, 1.0)] {
            let local = Vec3::new(x * b.extent.x, y * b.extent.y, 0.1);
            let corner = WorldTransform { pos: b.frame.pos.offset(b.frame.rotation * local), ..frame };
            let mapped = DoorLink::cross(&b, &a, &corner);
            let want = a.frame.pos.offset(a.frame.rotation * Vec3::new(-local.x, local.y, -local.z));
            assert!(mapped.pos.delta_f32(want).length() < 1e-4);
        }
    }

    /// A door model: hinge at the origin, 1.2 m wide along +X, 2.4 m tall,
    /// 0.1 m thick along Z (engine model space).
    const DOOR: ([f32; 3], [f32; 3]) = ([0.0, 0.0, -0.05], [1.2, 2.4, 0.05]);

    fn pose(x: f64, z: f64, yaw: f32) -> WorldTransform {
        WorldTransform {
            pos: WorldPos::from_f64(glam::DVec3::new(x, 0.0, z)),
            rotation: Quat::from_rotation_y(yaw),
            scale: Vec3::ONE,
        }
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn slab_frame_faces_the_arrival_side() {
        let p = pose(10.0, 20.0, std::f32::consts::FRAC_PI_2);
        // Rotated 90 degrees about Y: model +X -> world -Z, model +Z -> world +X.
        let behind = WorldPos::from_f64(glam::DVec3::new(8.0, 0.0, 19.4));
        let f = door_portal(&p, 1.0, Some(DOOR), Some(behind));
        assert_eq!(f.shape, DoorShape::Slab);
        assert!((f.extent - Vec2::new(1.2, 2.4)).length() < 1e-5);
        assert!((f.thickness - 0.1).abs() < 1e-5);
        let z = f.frame.rotation * Vec3::Z;
        assert!((z - Vec3::NEG_X).length() < 1e-5, "{z}");
        assert!(((f.frame.rotation * Vec3::Y) - Vec3::Y).length() < 1e-5);
        let c = f.frame.pos.to_f64();
        assert!((c - glam::DVec3::new(10.0, 0.0, 19.4)).length() < 1e-4, "{c}");
        assert!(f.front_flipped && f.front_known);
        // Hinge at an edge.
        assert!((f.hinge.abs() - 0.5).abs() < 1e-4, "{}", f.hinge);
        // Scale scales the opening.
        let g = door_portal(&p, 2.0, Some(DOOR), Some(behind));
        assert!((g.extent - Vec2::new(2.4, 4.8)).length() < 1e-5);
    }

    #[test]
    fn trapdoors_thick_doors_and_missing_models() {
        let p = pose(0.0, 0.0, 0.0);
        let trap = ([0.0, -0.05, 0.0], [1.0, 0.05, 1.2]);
        assert_eq!(door_portal(&p, 1.0, Some(trap), None).shape, DoorShape::Horizontal);
        let rock = ([-2.0, 0.0, -1.25], [2.0, 3.0, 1.25]);
        assert_eq!(door_portal(&p, 1.0, Some(rock), None).shape, DoorShape::Thick);
        // A thick door takes its normal from the arrival side: here +X.
        let side = door_portal(&p, 1.0, Some(rock), Some(WorldPos::from_f64(glam::DVec3::new(5.0, 0.0, 0.3))));
        assert!(((side.frame.rotation * Vec3::Z) - Vec3::X).length() < 1e-5);
        assert!((side.extent.x - 2.5).abs() < 1e-5 && (side.thickness - 4.0).abs() < 1e-5);
        let none = door_portal(&p, 1.0, None, None);
        assert_eq!(none.shape, DoorShape::Missing);
        assert_eq!(none.extent, Vec2::ZERO);
    }

    #[test]
    fn arrival_checks_and_crossing() {
        let a = door_portal(
            &pose(0.0, 0.0, 0.0),
            1.0,
            Some(DOOR),
            Some(WorldPos::from_f64(glam::DVec3::new(0.6, 0.0, 2.0))),
        );
        // Frame +Z is world +Z here; an arrival 1.5 m out, facing out (+Z
        // is forward when the marker's −Z is turned around).
        let out = WorldTransform { rotation: Quat::from_rotation_y(std::f32::consts::PI), ..pose(0.6, 1.5, 0.0) };
        let check = ArrivalCheck::of(&a, &out);
        assert!((check.ahead - 1.5).abs() < 1e-4 && check.lateral.abs() < 1e-4);
        assert_eq!(check.facing_class(), Facing::Away);
        assert!(check.above_sill.abs() < 1e-4 && !check.tilted);
        let back = pose(0.6, 1.5, 0.0);
        assert_eq!(ArrivalCheck::of(&a, &back).facing_class(), Facing::TowardDoor);

        // Two derived frames make a working portal: stepping 0.5 m through
        // a lands 0.5 m in front of b, facing out of b.
        let b = door_portal(
            &pose(100.0, 50.0, 1.0),
            1.0,
            Some(DOOR),
            Some(WorldPos::from_f64(glam::DVec3::new(100.0, 0.0, 53.0))),
        );
        let end = |f: &DoorFrame| sf_world::LinkEnd {
            worldspace: sf_world::WorldspaceId::new("x"),
            frame: f.frame,
            arrival: f.frame,
            door: None,
            extent: f.extent,
        };
        let through = WorldTransform {
            pos: a.frame.pos.offset(Vec3::new(0.0, 0.0, -0.5)),
            rotation: Quat::from_rotation_y(0.0),
            scale: Vec3::ONE,
        };
        let landed = DoorLink::cross(&end(&a), &end(&b), &through);
        let rel = b.frame.rotation.inverse() * b.frame.pos.delta_f32(landed.pos);
        assert!((rel - Vec3::new(0.0, 0.0, 0.5)).length() < 1e-3, "{rel}");
        let fwd = landed.rotation * Vec3::NEG_Z;
        assert!(fwd.dot(b.frame.rotation * Vec3::Z) > 0.999);
        assert!((ratio(a.extent, b.extent) - 1.0).abs() < 1e-6);
    }
}
