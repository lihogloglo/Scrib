//! Surface weather W6a, the game's stamp source: every animated character's planted feet print the
//! snow (`sf_surface_bevy::TrailStamp`; the framework keeps those within 40 m of the camera).
//!
//! Any `Animator` with legs in its graph (player avatar, NPCs, creatures; the player body is not a
//! `Mover`, so the animator is the common ground): a foot is planted while its leg's contact curve
//! (the cooked clips' `contact.<foot>`, the one foot placement and the stride monitor read) is
//! above one half; legs without a contact curve count as planted while the effector is within a
//! few cm of the character's ground plane. The foot's world position is the animator entity's
//! transform applied to the effector's model-space position (frames the graph posed, so LOD A3
//! characters don't stamp). A foot stamps once, where and how it planted (as footprint decals spawn
//! at foot-down: a sliding foot leaves one print, a standing one no new work). Feet over something other than
//! terrain (a bridge, a rock, a floor) don't stamp: the ring is 2D and would print the ground
//! underneath. Presentation only: stamps never feed gameplay.
//!
//! S1b (surface-weather.md "S1 deformer capture"), the game side of the framework's capture
//! (`sf_surface_bevy::deform`): the same legged animators are [`Deformer`]s ([`tag_deformers`]),
//! so legs, bodies (lying ones too), creatures and held items carve deep cover; first-person
//! view-model copies are [`NoDeform`]; the ground under the capture is the exterior's terrain
//! ([`refresh_ground`]); and sweep gaps (fast movers at low frame rates) get foot prints along
//! them ([`gap_stamps`]). Foot stamps stay as they are for every actor, whether or not the capture
//! runs. Interiors run none of it (no surface camera there); the capture itself only runs with
//! snow or ash under its window.

use std::collections::HashMap;
use std::sync::Arc;

use bevy::math::{DVec2, DVec3};
use bevy::prelude::*;
use mw_world::ids;
use sf_anim_bevy::Animator;
use sf_surface_bevy::sf_surface::trail::FOOT_HALF_M;
use sf_coords::WorldPos;
use sf_surface_bevy::{
    DeformGaps, DeformGround, DeformSettings, Deformer, NoDeform, SurfaceOrigin, SurfaceSettings, SurfaceUniform, SurfaceWeatherCamera, TrailStamp,
};
use sf_terrain_bevy::sf_terrain::TerrainHeights;
use sf_terrain_bevy::{TerrainHeightsMap, TerrainSettings};

/// Contact curve value above which a foot is planted.
const PLANTED: f32 = 0.5;
/// Without a contact curve: the effector's model-space height (m, before the body's scale) below
/// which the foot is down.
const DOWN_M: f32 = 0.12;
/// A foot further than this above or below the terrain stands on something else (m).
const ON_TERRAIN_M: f32 = 0.4;
/// The print's centre lies this far ahead of the effector (the ankle), toward the toes (m).
const TOE_SHIFT_M: f32 = 0.05;

/// Writes one stamp per foot-down of every posed animated character over terrain, at the
/// foot's position and facing when it planted (a foot that slides while planted still leaves one
/// print, as footprint decals spawn at foot-down).
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
pub(crate) fn foot_stamps(
    animators: Query<(Entity, &Animator, &GlobalTransform)>,
    heights: Option<Res<TerrainHeightsMap>>,
    settings: Res<SurfaceSettings>,
    kickup: Res<sf_surface_bevy::KickupSettings>,
    demand: Option<Res<sf_surface_bevy::TrailDemand>>,
    origin: Res<SurfaceOrigin>,
    mut out: MessageWriter<TrailStamp>,
    mut plants: Local<HashMap<(Entity, usize), TrailStamp>>,
) {
    // The same foot-downs feed the prints (W6), the press (GC-T2a: without a cover look too, while
    // a reader asks for it) and the kick-up puffs (W7).
    let press = demand.is_some_and(|d| d.press);
    let prints = settings.trails && (settings.cover || press);
    let puffs = kickup.on && settings.cover;
    if !settings.layer || !(prints || puffs) {
        plants.clear();
        return;
    }
    let Some(terrain) = heights.as_deref().and_then(|h| h.0.get(&ids::exterior())) else { return };
    let mut held = HashMap::with_capacity(plants.len());
    for (e, a, global) in &animators {
        if !a.posed() || a.asset.graph.legs.is_empty() {
            // Frames the graph skipped (LOD) keep the plants they had.
            for k in 0..a.asset.graph.legs.len() {
                if let Some(p) = plants.get(&(e, k)) {
                    held.insert((e, k), *p);
                }
            }
            continue;
        }
        let (scale, rotation, _) = global.to_scale_rotation_translation();
        let forward3 = rotation * Vec3::Z;
        let forward = Vec2::new(forward3.x, forward3.z);
        let half_m = FOOT_HALF_M * Vec2::new(scale.z, scale.x);
        for (k, leg) in a.asset.graph.legs.iter().enumerate() {
            let model = a.output.model.translation(usize::from(leg.effector()));
            let planted = match leg.contact {
                Some(c) => a.output.curves.get(usize::from(c)).copied().unwrap_or(0.0) > PLANTED,
                None => model.y < DOWN_M,
            };
            if !planted {
                continue;
            }
            if let Some(p) = plants.get(&(e, k)) {
                held.insert((e, k), *p);
                continue;
            }
            let at = origin.0.offset(global.transform_point(model + Vec3::Z * TOE_SHIFT_M));
            let Some(ground) = terrain.height(at) else { continue };
            if (at.to_f64().y - f64::from(ground)).abs() > f64::from(ON_TERRAIN_M) {
                continue;
            }
            let stamp = TrailStamp { at, forward, half_m, depth: 1.0 };
            held.insert((e, k), stamp);
            out.write(stamp);
        }
    }
    *plants = held;
}


/// S1b: the actors that carve deep cover through the framework's capture. Whatever prints feet
/// carves: an animator whose graph has legs (player avatar, NPCs, creatures; Morrowind has no
/// mounts). A trait only so the headless tests can stand in for an `Animator`.
pub(crate) trait Legged: Component {
    fn has_legs(&self) -> bool;
}

impl Legged for Animator {
    fn has_legs(&self) -> bool {
        !self.asset.graph.legs.is_empty()
    }
}

/// A body's [`Deformer::width_m`] (the narrowest part that leaves a continuous trench): its foot
/// print's width at the body's scale, the same print the foot and gap stamps leave
/// (`FOOT_HALF_M`). Not the collider: a capsule spans the body, not a leg; a narrower width only
/// lists more sweep gaps (more fill stamps), never fewer.
pub(crate) fn deformer_width(scale: Vec3) -> f32 {
    2.0 * FOOT_HALF_M.y * scale.x.abs()
}

/// Tags each legged animator entity (the body under a character root: its rig, skinned meshes and
/// held props, socket children of its joints, are all descendants, so the capture takes them with
/// it) as a [`Deformer`]; re-tags when its scale changes, untags one that lost its legs.
pub(crate) fn tag_deformers<A: Legged>(mut commands: Commands, actors: Query<(Entity, &A, &GlobalTransform, Option<&Deformer>)>) {
    for (e, actor, global, current) in &actors {
        if !actor.has_legs() {
            if current.is_some() {
                commands.entity(e).try_remove::<Deformer>();
            }
            continue;
        }
        let width_m = deformer_width(global.scale());
        // Maths tolerance: decomposed scales jitter in the last bits.
        if current.is_none_or(|d| (d.width_m - width_m).abs() > 1e-4) {
            commands.entity(e).try_insert(Deformer { width_m });
        }
    }
}

/// First-person view-model copies draw with the camera's projection: kept out of the capture.
pub(crate) fn no_deform_view_copy(add: On<Add<sf_character_bevy::ViewCopy>>, mut commands: Commands) {
    commands.entity(add.entity).try_insert(NoDeform);
}

/// The ground hook over a snapshot of the exterior's resident heights (absolute world height
/// under an absolute X, Z).
pub(crate) fn terrain_ground(heights: TerrainHeights) -> DeformGround {
    DeformGround(Arc::new(move |p: DVec2| heights.height(WorldPos::from_f64(DVec3::new(p.x, 0.0, p.y))).map(f64::from)))
}

/// Keeps [`DeformGround`] on this frame's exterior heights whenever the capture can run (deform
/// on, trails on with cover or a press demand (GC-T2b: the capture presses grass too), a surface
/// camera, a deformer): a snapshot (`TerrainHeights` is a map of
/// shared tiles, a few dozen `Arc`s; no runtime edits in this game) the hook reads, since a hook
/// can't borrow the world. Before `SurfaceSystems`, after the terrain's update.
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn refresh_ground(
    deform: Option<Res<DeformSettings>>,
    settings: Res<SurfaceSettings>,
    demand: Option<Res<sf_surface_bevy::TrailDemand>>,
    heights: Option<Res<TerrainHeightsMap>>,
    cameras: Query<(), With<SurfaceWeatherCamera>>,
    deformers: Query<(), With<Deformer>>,
    mut ground: ResMut<DeformGround>,
) {
    let press = demand.is_some_and(|d| d.press);
    let on = deform.is_some_and(|d| d.on) && settings.layer && settings.trails && (settings.cover || press);
    if !on || cameras.is_empty() || deformers.is_empty() {
        return;
    }
    if let Some(terrain) = heights.as_deref().and_then(|h| h.0.get(&ids::exterior())) {
        *ground = terrain_ground(terrain.clone());
    }
}

/// The fill stamps of a sweep gap of `length_m` at `stride_m` (the deformer's width): fractions
/// of the segment, strictly inside it (both ends were captured), at most `stride_m` apart. None
/// when the gap is no longer than the stride.
pub(crate) fn gap_fractions(length_m: f32, stride_m: f32) -> impl Iterator<Item = f32> {
    let n = if stride_m > 0.0 && length_m > stride_m { (length_m / stride_m).ceil() } else { 1.0 };
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let n = n as u32;
    #[allow(clippy::cast_precision_loss)]
    (1..n).map(move |k| k as f32 / n as f32)
}

/// S1b: a deformer that moved farther than its width since its last capture (a fast mover at a
/// low frame rate) left a gap in its trench: last frame's [`DeformGaps`] get foot prints along the
/// segment, one per stride of the width, facing the travel, over terrain only (as foot stamps),
/// with cover or a press demand (GC-T2b: the stamps press the gap's grass too).
/// Foot stamps ([`foot_stamps`]) don't read the capture at all: they print for every actor, in
/// or out of the capture window, with the capture off or absent from the preset.
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn gap_stamps(
    gaps: Option<Res<DeformGaps>>,
    deformers: Query<&Deformer>,
    heights: Option<Res<TerrainHeightsMap>>,
    settings: Res<SurfaceSettings>,
    demand: Option<Res<sf_surface_bevy::TrailDemand>>,
    mut out: MessageWriter<TrailStamp>,
) {
    let Some(gaps) = gaps.filter(|g| !g.0.is_empty()) else { return };
    let press = demand.is_some_and(|d| d.press);
    if !settings.trails || !(settings.cover || press) || !settings.layer {
        return;
    }
    let Some(terrain) = heights.as_deref().and_then(|h| h.0.get(&ids::exterior())) else { return };
    for gap in &gaps.0 {
        let Ok(deformer) = deformers.get(gap.root) else { continue };
        let d = gap.from.delta_f32(gap.to);
        let travel = Vec2::new(d.x, d.z);
        let Some(forward) = travel.try_normalize() else { continue };
        let half_m = FOOT_HALF_M * (deformer.width_m / (2.0 * FOOT_HALF_M.y));
        for t in gap_fractions(travel.length(), deformer.width_m) {
            let at = gap.from.offset(d * t);
            let Some(ground) = terrain.height(at) else { continue };
            if (at.to_f64().y - f64::from(ground)).abs() > f64::from(ON_TERRAIN_M) {
                continue;
            }
            out.write(TrailStamp { at, forward, half_m, depth: 1.0 });
        }
    }
}

/// S1b wiring (`exterior.rs`, with the surface plugin): deformer tags, the ground hook, gap fills.
pub(crate) fn deform_plugin(app: &mut App) {
    app.insert_resource(DeformGround(Arc::new(|_| None)))
        .add_systems(
            PostUpdate,
            (
                tag_deformers::<Animator>.before(sf_surface_bevy::SurfaceSystems),
                refresh_ground.after(sf_terrain_bevy::TerrainSystems::Update).before(sf_surface_bevy::SurfaceSystems),
                gap_stamps.before(sf_surface_bevy::SurfaceSystems),
            ),
        )
        .add_observer(no_deform_view_copy);
}

/// W6b: the terrain's near CDLOD bounds grow by the deepest cover look's depth (snow drawn raised
/// over the ground, `TerrainSettings::raise_m`), so culling stays conservative. SW-5e: the raise
/// ends where shelter stops being known whatever the camera's place (`ShelterReach`: the far
/// shelter level) or where casters stop (the near-field handoff less its band: past it only far
/// proxies stand, and they never cast), whichever is nearer (`TerrainSettings::raise_reach_m`).
pub fn terrain_raise(
    uniform: Res<SurfaceUniform>,
    reach: Option<Res<sf_surface_bevy::ShelterReach>>,
    proxies: Option<Res<sf_world_bevy::FarProxies>>,
    terrain: Option<ResMut<TerrainSettings>>,
) {
    let Some(mut terrain) = terrain else { return };
    let raise = uniform.cover.iter().filter(|c| c.x > 0.0).map(|c| c.y).fold(0.0, f32::max);
    if terrain.raise_m.to_bits() != raise.to_bits() {
        terrain.raise_m = raise;
    }
    let casters = proxies.and_then(|p| p.handoff).map_or(f32::INFINITY, |(handoff, band)| handoff - band);
    let known = reach.map_or(Some(sf_surface_bevy::sf_surface::view::NEAR_LEVEL.known_m()), |r| r.0);
    let reach_m = sf_surface_bevy::sf_surface::view::raise_reach_m(known, casters);
    if terrain.raise_reach_m.to_bits() != reach_m.to_bits() {
        terrain.raise_reach_m = reach_m;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sf_surface_bevy::DeformGap;
    use sf_terrain_bevy::sf_terrain::{HeightTile, ResidentTile, TILE_SAMPLES, TileCoord};

    /// Stands in for an `Animator` (one needs a cooked graph): whether its graph has legs.
    #[derive(Component)]
    struct Legs(bool);

    impl Legged for Legs {
        fn has_legs(&self) -> bool {
            self.0
        }
    }

    /// Tile (0, 0) at level 0 spans world X, Z -128..128 m; height = 0.25 m per sample along X,
    /// so the ground at world X is `0.5 × (X + 128)`.
    fn sloped() -> TerrainHeights {
        let mut tile = HeightTile::flat(TileCoord::new(0, 0, 0), 0.0);
        for j in 0..TILE_SAMPLES {
            for i in 0..TILE_SAMPLES {
                #[allow(clippy::cast_precision_loss)]
                let h = i as f32 * 0.25;
                tile.heights[j * TILE_SAMPLES + i] = h;
            }
        }
        let mut heights = TerrainHeights::new();
        heights.insert(Arc::new(ResidentTile::new(tile)));
        heights
    }

    fn flat(y: f32) -> TerrainHeights {
        let mut heights = TerrainHeights::new();
        heights.insert(Arc::new(ResidentTile::new(HeightTile::flat(TileCoord::new(0, 0, 0), y))));
        heights
    }

    fn heights_map(heights: TerrainHeights) -> TerrainHeightsMap {
        let mut map = TerrainHeightsMap::default();
        map.0.insert(ids::exterior(), heights);
        map
    }

    fn at(x: f64, y: f64, z: f64) -> WorldPos {
        WorldPos::from_f64(DVec3::new(x, y, z))
    }

    #[test]
    fn legged_actors_are_deformers_statics_are_not_held_items_ride_along() {
        let mut app = App::new();
        app.add_systems(Update, tag_deformers::<Legs>);
        let w = app.world_mut();
        let actor = w.spawn((Legs(true), GlobalTransform::from(Transform::from_scale(Vec3::splat(1.1))))).id();
        let joint = w.spawn((GlobalTransform::default(), ChildOf(actor))).id();
        let prop = w.spawn((GlobalTransform::default(), ChildOf(joint))).id();
        let legless = w.spawn((Legs(false), GlobalTransform::default())).id();
        let fixed = w.spawn(GlobalTransform::default()).id();
        app.update();
        let w = app.world();
        let width = w.get::<Deformer>(actor).expect("actor tagged").width_m;
        assert!((width - 2.0 * FOOT_HALF_M.y * 1.1).abs() < 1e-6, "{width}");
        assert!((deformer_width(Vec3::ONE) - 0.18).abs() < 1e-6);
        assert!(w.get::<Deformer>(legless).is_none() && w.get::<Deformer>(fixed).is_none());
        // The held prop is not tagged itself: the capture takes it as a descendant of the actor.
        assert!(w.get::<Deformer>(prop).is_none());
        let mut up = prop;
        let mut tagged = None;
        while let Some(parent) = w.get::<ChildOf>(up).map(ChildOf::parent) {
            if w.get::<Deformer>(parent).is_some() {
                tagged = Some(parent);
            }
            up = parent;
        }
        assert_eq!(tagged, Some(actor));
        // A rescale re-tags; losing the legs untags.
        app.world_mut().entity_mut(actor).insert(GlobalTransform::from(Transform::from_scale(Vec3::splat(2.0))));
        app.update();
        assert!((app.world().get::<Deformer>(actor).unwrap().width_m - 0.36).abs() < 1e-6);
        app.world_mut().entity_mut(actor).insert(Legs(false));
        app.update();
        assert!(app.world().get::<Deformer>(actor).is_none());
    }

    #[test]
    fn the_ground_hook_reads_the_terrain_heights() {
        let hook = terrain_ground(sloped());
        assert_eq!(hook.0(DVec2::new(10.0, 20.0)), Some(69.0));
        assert_eq!(hook.0(DVec2::new(-128.0, 0.0)), Some(0.0));
        assert_eq!(hook.0(DVec2::new(-100.0, -50.0)), Some(14.0));
        assert_eq!(hook.0(DVec2::new(1000.0, 0.0)), None, "nothing resident there");
    }

    fn ground_app(on: bool) -> App {
        let mut app = App::new();
        app.insert_resource(DeformSettings { on, ..DeformSettings::default() })
            .insert_resource(SurfaceSettings::default())
            .insert_resource(heights_map(sloped()))
            .insert_resource(DeformGround(Arc::new(|_| None)))
            .add_systems(Update, refresh_ground);
        app.world_mut().spawn(SurfaceWeatherCamera);
        app.world_mut().spawn((Deformer { width_m: 0.18 }, GlobalTransform::default()));
        app.update();
        app
    }

    #[test]
    fn the_hook_covers_the_capture_window_so_it_never_falls_back() {
        let app = ground_app(true);
        let hook = app.world().resource::<DeformGround>().clone();
        // Every ground sample of a 16 m window at 0.5 m around the player at the origin (what
        // `sf_surface_bevy::deform::ground_grid` asks): known, so no flat-plane fallback.
        for z in -17..=17 {
            for x in -17..=17 {
                let p = DVec2::new(f64::from(x) * 0.5, f64::from(z) * 0.5);
                let y = hook.0(p).unwrap_or_else(|| panic!("no ground at {p}"));
                assert!((y - f64::midpoint(p.x, 128.0)).abs() < 1e-4, "{p}: {y}");
            }
        }
        // Deform off: the hook is left alone (the capture doesn't run).
        let off = ground_app(false);
        assert_eq!(off.world().resource::<DeformGround>().0(DVec2::ZERO), None);
    }

    /// GC-T2b: with no cover look the hook still follows the terrain on a press demand (the
    /// capture presses grass), and not without one.
    #[test]
    fn the_hook_follows_a_press_demand_without_cover() {
        let run = |press: bool| {
            let mut app = App::new();
            app.insert_resource(DeformSettings::default())
                .insert_resource(SurfaceSettings { cover: false, ..SurfaceSettings::default() })
                .insert_resource(sf_surface_bevy::TrailDemand { press })
                .insert_resource(heights_map(sloped()))
                .insert_resource(DeformGround(Arc::new(|_| None)))
                .add_systems(Update, refresh_ground);
            app.world_mut().spawn(SurfaceWeatherCamera);
            app.world_mut().spawn((Deformer { width_m: 0.18 }, GlobalTransform::default()));
            app.update();
            app.world().resource::<DeformGround>().0(DVec2::ZERO)
        };
        assert!(run(true).is_some(), "press demand: the capture's ground follows the terrain");
        assert_eq!(run(false), None, "no cover, no demand: left alone");
    }

    #[test]
    fn a_gap_gets_one_print_per_stride_strictly_inside() {
        let t: Vec<f32> = gap_fractions(1.0, 0.18).collect();
        assert_eq!(t.len(), 5, "{t:?}");
        assert!(t.iter().all(|&t| t > 0.0 && t < 1.0));
        let mut steps = vec![t[0]];
        steps.extend(t.windows(2).map(|w| w[1] - w[0]));
        steps.push(1.0 - t[4]);
        assert!(steps.iter().all(|&s| s <= 0.18 + 1e-6), "{steps:?}");
        assert_eq!(gap_fractions(3.0, 1.0).count(), 2);
        assert_eq!(gap_fractions(0.1, 0.18).count(), 0);
        assert_eq!(gap_fractions(0.0, 0.18).count(), 0);
        assert_eq!(gap_fractions(1.0, 0.0).count(), 0);
    }

    /// This update's gap prints for `gaps`.
    fn stamps_for(gaps: Vec<DeformGap>, app: &mut App) -> Vec<TrailStamp> {
        app.world_mut().resource_mut::<DeformGaps>().0 = gaps;
        app.update();
        app.world().resource::<Messages<TrailStamp>>().iter_current_update_messages().copied().collect()
    }

    #[test]
    fn gap_stamps_fill_the_segment_over_terrain_only() {
        let mut app = App::new();
        app.add_message::<TrailStamp>()
            .insert_resource(SurfaceSettings::default())
            .insert_resource(heights_map(flat(12.5)))
            .init_resource::<DeformGaps>()
            .add_systems(Update, gap_stamps);
        let root = app.world_mut().spawn(Deformer { width_m: 0.18 }).id();
        // 1 m at a 0.18 m stride: five prints, facing the travel, the foot print's size.
        let gap = DeformGap { root, from: at(0.0, 12.5, 0.0), to: at(1.0, 12.5, 0.0) };
        let prints = stamps_for(vec![gap], &mut app);
        assert_eq!(prints.len(), 5);
        let first = prints[0];
        assert!((first.forward - Vec2::X).length() < 1e-6 && (first.half_m - FOOT_HALF_M).length() < 1e-6, "{first:?}");
        assert!((first.at.to_f64().x - 1.0 / 6.0).abs() < 1e-5);
        // No gap, no prints.
        assert!(stamps_for(Vec::new(), &mut app).is_empty());
        // Over something other than terrain (a bridge 5 m up): none.
        let bridge = DeformGap { root, from: at(0.0, 17.5, 0.0), to: at(1.0, 17.5, 0.0) };
        assert!(stamps_for(vec![bridge], &mut app).is_empty());
        // No cover look: none, unless a reader demands the press (GC-T2b).
        app.world_mut().resource_mut::<SurfaceSettings>().cover = false;
        assert!(stamps_for(vec![gap], &mut app).is_empty());
        app.insert_resource(sf_surface_bevy::TrailDemand { press: true });
        assert_eq!(stamps_for(vec![gap], &mut app).len(), 5);
        // Trails off (the preset or `--off trails`): none.
        app.world_mut().resource_mut::<SurfaceSettings>().trails = false;
        assert!(stamps_for(vec![gap], &mut app).is_empty());
    }
}
