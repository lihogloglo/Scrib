//! Point-light shadow casters that skip CPU culling (`NoCpuCulling`), listed
//! per light **on change** (spec `docs/specs/shadows.md`, "GPU-culled casters").
//!
//! Bevy 0.20 gives a `NoCpuCulling` mesh no per-view processing in the main
//! world: the render world adds it to every camera and every shadow view and
//! the GPU culls it there. That suits a camera and a sun's cascades (few,
//! long-lived views). A point light is six views that come and go with the
//! shadow budget, and a view that appears has every GPU-culled mesh of the
//! world queued into it at once, so a town of lamps pays the whole world per
//! lamp. The standard answer is to keep, per local light, the list of casters
//! in its range and to update it when a caster or the light changes, not per
//! frame (UE's light-primitive interactions, built from the scene octree when
//! a primitive or a light is added).
//!
//! So: `bevy_light::check_point_light_mesh_visibility` lists the CPU-culled
//! casters of each shadowed point light every frame, as stock, and
//! [`list_gpu_culled_casters`] then adds the GPU-culled ones from a list it
//! keeps per light, with Bevy's own tests (light sphere, then the six face
//! frusta, `RenderLayers`, `InheritedVisibility`, `NotShadowCaster`). A
//! light's list is rebuilt only when the light moves or changes range or
//! layers, or when any GPU-culled caster is added, removed, moved, hidden or
//! shown. Every LOD level in range is listed; the GPU preprocess picks one per
//! view by `VisibilityRange`.
//!
//! The render-world half is the vendored `bevy_pbr::CpuListedPointShadowCasters`
//! switch (inserted here): point faces take their casters from these lists
//! only. Spot lights keep Bevy's stock path (every GPU-culled mesh).

use bevy_app::{App, PostUpdate};
use bevy_camera::primitives::{Aabb, CubemapFrusta, Sphere};
use bevy_camera::visibility::{CubemapVisibleEntities, InheritedVisibility, NoCpuCulling, NoFrustumCulling, RenderLayers};
use bevy_ecs::entity::EntityHashMap;
use bevy_ecs::prelude::*;
use bevy_light::{NotShadowCaster, PointLight, SimulationLightSystems};
use bevy_mesh::Mesh3d;
use bevy_render::RenderApp;
use bevy_transform::components::GlobalTransform;

/// All six cube faces.
const ALL_FACES: u8 = 0b11_1111;

/// The GPU-culled casters of each shadowed point light, per cube face.
#[derive(Resource, Default)]
pub(crate) struct GpuCulledCasterLists {
    lights: EntityHashMap<LightCasters>,
    /// Lists rebuilt since startup (tests: lists are rebuilt on change only).
    pub(crate) scans: u64,
    frame: u32,
}

#[derive(Default)]
struct LightCasters {
    /// What the lists were built for; a difference rebuilds them.
    key: Option<(GlobalTransform, f32, RenderLayers)>,
    /// Sorted, like Bevy's own lists.
    faces: [Vec<Entity>; 6],
    seen: u32,
}

pub(crate) fn add(app: &mut App) {
    app.init_resource::<GpuCulledCasterLists>()
        .add_systems(
            PostUpdate,
            list_gpu_culled_casters
                .after(SimulationLightSystems::CheckLightVisibility)
                // Opt-in (H1 lost, ledger/statics.md 2026-11-15): only GPU-culled statics need it.
                .run_if(any_with_component::<NoCpuCulling>),
        );
    if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
        render_app.init_resource::<bevy_pbr::CpuListedPointShadowCasters>();
    }
}

/// The cube faces (bit `i` = face `i`) of a point light that a caster can
/// shadow: the tests of `bevy_light::check_point_light_mesh_visibility`.
pub(crate) fn caster_faces(light: &Sphere, frusta: &CubemapFrusta, aabb: Option<&Aabb>, transform: &GlobalTransform) -> u8 {
    let Some(aabb) = aabb else {
        return ALL_FACES;
    };
    let world_from_local = transform.affine();
    if !light.intersects_obb(aabb, &world_from_local) {
        return 0;
    }
    let mut faces = 0;
    for (i, frustum) in frusta.iter().enumerate() {
        if frustum.intersects_obb(aabb, &world_from_local, true, true) {
            faces |= 1 << i;
        }
    }
    faces
}

type Caster<'a> =
    (Entity, &'a InheritedVisibility, Option<&'a RenderLayers>, Option<&'a Aabb>, &'a GlobalTransform, Has<NoFrustumCulling>);
type IsCaster = (With<Mesh3d>, With<NoCpuCulling>);
type CasterChanged = Or<(
    Added<NoCpuCulling>,
    Added<Mesh3d>,
    Added<NotShadowCaster>,
    Changed<InheritedVisibility>,
    Changed<RenderLayers>,
    Changed<Aabb>,
    Changed<GlobalTransform>,
)>;

/// Adds each shadowed point light's GPU-culled casters to its
/// `CubemapVisibleEntities` (module docs).
#[allow(clippy::type_complexity)]
pub(crate) fn list_gpu_culled_casters(
    mut lists: ResMut<GpuCulledCasterLists>,
    mut lights: Query<(Entity, &PointLight, &GlobalTransform, &CubemapFrusta, &mut CubemapVisibleEntities, Option<&RenderLayers>)>,
    casters: Query<Caster, (IsCaster, Without<NotShadowCaster>)>,
    changed: Query<(), (IsCaster, CasterChanged)>,
    (mut removed, mut recast): (RemovedComponents<NoCpuCulling>, RemovedComponents<NotShadowCaster>),
) {
    let lists = &mut *lists;
    // Both readers are drained every frame; a despawned caster counts as removed.
    let casters_changed = removed.read().count() + recast.read().count() > 0;
    if !lights.iter().any(|(_, light, ..)| light.shadow_maps_enabled) {
        lists.lights.clear();
        return;
    }
    let casters_changed = casters_changed || !changed.is_empty();
    lists.frame = lists.frame.wrapping_add(1);

    for (entity, light, transform, frusta, mut visible, layers) in &mut lights {
        if !light.shadow_maps_enabled {
            continue;
        }
        let layers = layers.unwrap_or_default();
        let listed = lists.lights.entry(entity).or_default();
        listed.seen = lists.frame;
        let current = listed.key.as_ref().is_some_and(|(t, range, l)| t == transform && range.to_bits() == light.range.to_bits() && l == layers);
        if casters_changed || !current {
            listed.key = Some((*transform, light.range, layers.clone()));
            for face in &mut listed.faces {
                face.clear();
            }
            let sphere = Sphere { center: transform.translation_vec3a(), radius: light.range };
            for (caster, inherited, caster_layers, aabb, caster_transform, no_frustum_culling) in &casters {
                if !inherited.get() || !layers.intersects(caster_layers.unwrap_or_default()) {
                    continue;
                }
                let mask = if no_frustum_culling { ALL_FACES } else { caster_faces(&sphere, frusta, aabb, caster_transform) };
                for (i, face) in listed.faces.iter_mut().enumerate() {
                    if mask & (1 << i) != 0 {
                        face.push(caster);
                    }
                }
            }
            for face in &mut listed.faces {
                face.sort_unstable();
            }
            lists.scans += 1;
        }
        if listed.faces.iter().all(Vec::is_empty) {
            continue;
        }
        // A light no camera sees keeps last frame's list (Bevy clears it only
        // when it checks the light), so merge instead of appending.
        for (face, list) in listed.faces.iter().zip(visible.iter_mut()) {
            list.entities.extend_from_slice(face);
            list.entities.sort_unstable();
            list.entities.dedup();
        }
    }
    let frame = lists.frame;
    lists.lights.retain(|_, listed| listed.seen == frame);
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_camera::visibility::ViewVisibility;
    use bevy_ecs::schedule::Schedule;
    use bevy_light::update_point_light_frusta;
    use bevy_math::Vec3;

    fn world() -> (World, Schedule) {
        let mut world = World::new();
        world.init_resource::<GpuCulledCasterLists>();
        let mut schedule = Schedule::default();
        schedule.add_systems((update_point_light_frusta, list_gpu_culled_casters).chain());
        (world, schedule)
    }

    fn light(world: &mut World, at: Vec3, range: f32) -> Entity {
        world
            .spawn((
                PointLight { shadow_maps_enabled: true, range, ..Default::default() },
                GlobalTransform::from_translation(at),
                ViewVisibility::VISIBLE,
            ))
            .id()
    }

    fn caster(world: &mut World, at: Vec3) -> Entity {
        world
            .spawn((
                Mesh3d::default(),
                NoCpuCulling,
                Aabb::from_min_max(Vec3::splat(-0.5), Vec3::splat(0.5)),
                GlobalTransform::from_translation(at),
                InheritedVisibility::VISIBLE,
            ))
            .id()
    }

    fn faces(world: &World, light: Entity) -> Vec<Vec<Entity>> {
        world.get::<CubemapVisibleEntities>(light).unwrap().iter().map(|face| face.entities.clone()).collect()
    }

    fn scans(world: &World) -> u64 {
        world.resource::<GpuCulledCasterLists>().scans
    }

    /// Bevy clears a checked light's lists each frame before refilling them.
    fn clear(world: &mut World, light: Entity) {
        for face in world.get_mut::<CubemapVisibleEntities>(light).unwrap().iter_mut() {
            face.entities.clear();
        }
    }

    #[test]
    fn casters_in_range_are_listed_in_the_faces_that_see_them() {
        let (mut world, mut schedule) = world();
        let lamp = light(&mut world, Vec3::ZERO, 10.0);
        let east = caster(&mut world, Vec3::new(5.0, 0.0, 0.0));
        let up = caster(&mut world, Vec3::new(0.0, 5.0, 0.0));
        let far = caster(&mut world, Vec3::new(30.0, 0.0, 0.0));
        let cpu_culled = world.spawn((Mesh3d::default(), GlobalTransform::from_translation(Vec3::X), InheritedVisibility::VISIBLE)).id();
        let no_shadow = caster(&mut world, Vec3::new(-5.0, 0.0, 0.0));
        world.entity_mut(no_shadow).insert(NotShadowCaster);
        let hidden = caster(&mut world, Vec3::new(0.0, 0.0, 5.0));
        world.entity_mut(hidden).insert(InheritedVisibility::HIDDEN);
        let other_layer = caster(&mut world, Vec3::new(0.0, 0.0, -5.0));
        world.entity_mut(other_layer).insert(RenderLayers::layer(3));
        schedule.run(&mut world);

        let faces = faces(&world, lamp);
        let holding = |e: Entity| faces.iter().enumerate().filter(|(_, face)| face.contains(&e)).map(|(i, _)| i).collect::<Vec<_>>();
        // Cube faces: +X, -X, +Y, -Y, +Z, -Z.
        assert_eq!(holding(east), vec![0]);
        assert_eq!(holding(up), vec![2]);
        for absent in [far, cpu_culled, no_shadow, hidden, other_layer] {
            assert!(holding(absent).is_empty(), "{absent} listed");
        }
        assert!(faces.iter().all(|face| face.is_sorted()));
    }

    #[test]
    fn lists_are_rebuilt_only_on_change() {
        let (mut world, mut schedule) = world();
        let lamp = light(&mut world, Vec3::ZERO, 10.0);
        let east = caster(&mut world, Vec3::new(5.0, 0.0, 0.0));
        schedule.run(&mut world);
        assert_eq!(scans(&world), 1);

        // Still frames: the kept list is added again, nothing is scanned.
        for _ in 0..3 {
            clear(&mut world, lamp);
            schedule.run(&mut world);
        }
        assert_eq!(scans(&world), 1);
        assert_eq!(faces(&world, lamp)[0], vec![east]);
        // An unchecked light's list is not cleared: no duplicates.
        schedule.run(&mut world);
        assert_eq!(faces(&world, lamp)[0], vec![east]);

        // The light moves: east is now behind it.
        *world.get_mut::<GlobalTransform>(lamp).unwrap() = GlobalTransform::from_translation(Vec3::new(8.0, 0.0, 0.0));
        clear(&mut world, lamp);
        schedule.run(&mut world);
        assert_eq!(scans(&world), 2);
        assert_eq!(faces(&world, lamp)[1], vec![east]);
        assert!(faces(&world, lamp)[0].is_empty());

        // A caster appears, then one is despawned.
        let west = caster(&mut world, Vec3::new(4.0, 0.0, 0.0));
        clear(&mut world, lamp);
        schedule.run(&mut world);
        assert_eq!(scans(&world), 3);
        let mut both = vec![east, west];
        both.sort_unstable();
        assert_eq!(faces(&world, lamp)[1], both);
        world.despawn(east);
        clear(&mut world, lamp);
        schedule.run(&mut world);
        assert_eq!(scans(&world), 4);
        assert_eq!(faces(&world, lamp)[1], vec![west]);

        // The range shrinks below the caster's distance.
        world.get_mut::<PointLight>(lamp).unwrap().range = 2.0;
        clear(&mut world, lamp);
        schedule.run(&mut world);
        assert_eq!(scans(&world), 5);
        assert!(faces(&world, lamp).iter().all(Vec::is_empty));
    }

    #[test]
    fn a_light_without_a_shadow_map_keeps_no_list() {
        let (mut world, mut schedule) = world();
        let lamp = light(&mut world, Vec3::ZERO, 10.0);
        caster(&mut world, Vec3::new(5.0, 0.0, 0.0));
        schedule.run(&mut world);
        world.get_mut::<PointLight>(lamp).unwrap().shadow_maps_enabled = false;
        clear(&mut world, lamp);
        schedule.run(&mut world);
        assert!(world.resource::<GpuCulledCasterLists>().lights.is_empty());
        assert!(faces(&world, lamp).iter().all(Vec::is_empty));
        assert_eq!(scans(&world), 1);
    }
}
