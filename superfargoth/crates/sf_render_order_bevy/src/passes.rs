//! The order of the passes between Bevy's opaque and transparent passes
//! (render-order.md RO-8, "Frame graph"), declared once.
//!
//! The water draws in Bevy's `main_transmissive_pass_3d`, between the opaque and
//! the transparent pass (W-ORDER; UE5 Single Layer Water's placement): the
//! [`PassStage::Snapshot`] and [`PassStage::Reflection`] (the water's SSPR) run
//! before it (after the opaque pass only), every later stage after it, so the sky, clouds and fog read the water's depth and
//! cover it as they cover land.
//!
//! `render_sky` is private to `bevy_pbr`, so its system set can't be named. A
//! stage after the water is ordered after every system set already ordered
//! before `main_transparent_pass_3d` (Bevy's sky and the transmissive pass
//! included; the vendored `bevy_pbr` orders `render_sky` after the transmissive
//! pass), found when the pass is added, and after the earlier [`PassStage`]s
//! through the sets themselves, so the order is the enum's, not the plugin add
//! order. Scaffold `SkyOrderScaffold`; removal path: a public `render_sky` set
//! upstream.

use std::collections::HashMap;

use bevy_app::{App, SubApp};
use bevy_core_pipeline::core_3d::{main_opaque_pass_3d, main_transparent_pass_3d};
use bevy_core_pipeline::schedule::Core3d;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::{InternedSystemSet, IntoScheduleConfigs, NodeId, Schedules};
use bevy_ecs::system::ScheduleSystem;
use bevy_pbr::main_transmissive_pass_3d;
use bevy_render::RenderApp;

/// The passes between the opaque and the transparent pass, in draw order.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PassStage {
    /// The refraction snapshot (`sf_water_bevy`): the opaque scene, taken before
    /// the water draws in the transmissive pass.
    Snapshot,
    /// Local reflections of the snapshot (`sf_water_bevy` SSPR): after the snapshot, before the
    /// water draws.
    Reflection,
    /// Night sky: stars and moons (`sf_sky_bevy`), after the water and `render_sky`.
    Night,
    /// Cloud and fog composite (`sf_atmos_bevy`), after the water and `render_sky`.
    Clouds,
}

/// The passes added through [`add_pass`] (their own function sets).
#[derive(Resource, Default)]
struct Registered(Vec<InternedSystemSet>);

/// Adds `pass` to the render app's `Core3d` schedule in `stage`. The snapshot
/// and the reflection go after the opaque pass, before the transmissive (water) pass; every
/// other stage after the transmissive pass, everything already ordered before
/// the transparent pass and the earlier stages, before the transparent pass.
/// `own` is the pass's own
/// system set (`pass.into_system_set().intern()`), `ignore` any other sets of
/// the crate that must not be waited for (they would form a cycle).
pub fn add_pass<M>(app: &mut App, stage: PassStage, own: InternedSystemSet, ignore: &[InternedSystemSet], pass: impl IntoScheduleConfigs<ScheduleSystem, M>) {
    let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
    add_pass_to(render_app, stage, own, ignore, pass);
}

fn add_pass_to<M>(render_app: &mut SubApp, stage: PassStage, own: InternedSystemSet, ignore: &[InternedSystemSet], pass: impl IntoScheduleConfigs<ScheduleSystem, M>) {
    let mut skip: Vec<InternedSystemSet> = ignore.to_vec();
    skip.push(own);
    skip.extend(render_app.world().get_resource::<Registered>().map(|r| r.0.clone()).unwrap_or_default());
    let found = sets_before_transparent(render_app, &skip);
    render_app.init_resource::<Registered>().world_mut().resource_mut::<Registered>().0.push(own);
    render_app.configure_sets(Core3d, (PassStage::Snapshot.before(PassStage::Reflection), PassStage::Reflection.before(PassStage::Night), PassStage::Night.before(PassStage::Clouds)));
    let config = pass.in_set(stage).after(main_opaque_pass_3d);
    if matches!(stage, PassStage::Snapshot | PassStage::Reflection) {
        render_app.add_systems(Core3d, config.before(main_transmissive_pass_3d));
        return;
    }
    let mut config = config.after(main_transmissive_pass_3d).before(main_transparent_pass_3d);
    for set in found {
        config = config.after(set);
    }
    render_app.add_systems(Core3d, config);
}

/// The system sets of the systems ordered before `main_transparent_pass_3d`,
/// without `skip`.
fn sets_before_transparent(render_app: &mut SubApp, skip: &[InternedSystemSet]) -> Vec<InternedSystemSet> {
    let transparent = main_transparent_pass_3d.into_system_set().intern();
    let world = render_app.world_mut();
    let schedules = world.resource::<Schedules>();
    let Some(schedule) = schedules.get(Core3d) else { return Vec::new() };
    let graph = schedule.graph();
    let Some(key) = graph.system_sets.get_key(transparent) else { return Vec::new() };
    let systems: HashMap<_, _> = graph.systems.iter().map(|(k, system, _)| (k, system)).collect();
    let mut out = Vec::new();
    for node in graph.dependency().graph().neighbors_directed(NodeId::Set(key), bevy_ecs::schedule::graph::Direction::Incoming) {
        if let NodeId::System(k) = node
            && let Some(system) = systems.get(&k)
        {
            out.extend(system.default_system_sets().into_iter().filter(|set| *set != transparent && !skip.contains(set)));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Resource, Default)]
    struct Log(Vec<&'static str>);

    fn night(mut l: ResMut<Log>) {
        l.0.push("night");
    }
    fn clouds(mut l: ResMut<Log>) {
        l.0.push("clouds");
    }
    fn snapshot(mut l: ResMut<Log>) {
        l.0.push("snapshot");
    }
    fn reflection(mut l: ResMut<Log>) {
        l.0.push("reflection");
    }

    /// Stands in for Bevy's private `render_sky` (ordered like it: after the
    /// opaque pass, before the transparent pass).
    fn sky(mut l: ResMut<Log>) {
        l.0.push("sky");
    }

    fn setup(order: &[usize]) -> SubApp {
        let mut sub = SubApp::new();
        sub.init_resource::<Log>();
        sub.add_systems(Core3d, sky.after(main_opaque_pass_3d).before(main_transparent_pass_3d));
        for &i in order {
            match i {
                0 => add_pass_to(&mut sub, PassStage::Night, night.into_system_set().intern(), &[], night),
                1 => add_pass_to(&mut sub, PassStage::Clouds, clouds.into_system_set().intern(), &[], clouds),
                2 => add_pass_to(&mut sub, PassStage::Snapshot, snapshot.into_system_set().intern(), &[], snapshot),
                _ => add_pass_to(&mut sub, PassStage::Reflection, reflection.into_system_set().intern(), &[], reflection),
            }
        }
        sub
    }

    fn run(order: &[usize]) -> Vec<&'static str> {
        let mut sub = setup(order);
        sub.world_mut().run_schedule(Core3d);
        sub.world().resource::<Log>().0.clone()
    }

    /// The declared order holds for every plugin add order (RO-8), and the
    /// stages after the water also wait for the sky.
    #[test]
    fn order_is_the_enums_for_any_add_order() {
        for order in [[0, 1, 2, 3], [0, 2, 1, 3], [1, 0, 3, 2], [1, 3, 2, 0], [3, 2, 0, 1], [2, 1, 0, 3], [3, 0, 1, 2]] {
            let log = run(&order);
            let at = |name| log.iter().position(|s| *s == name).unwrap();
            assert!(at("snapshot") < at("reflection") && at("reflection") < at("night") && at("night") < at("clouds"), "added in order {order:?}: {log:?}");
            assert!(at("sky") < at("night"), "added in order {order:?}: {log:?}");
        }
    }

    /// The snapshot and the SSPR (W-SSPR) run before the transmissive (water) pass, the night sky
    /// and the cloud/fog composite after it (W-ORDER: weather covers water).
    #[test]
    fn water_draws_between_snapshot_and_weather() {
        let mut sub = setup(&[3, 2, 0, 1]);
        let transmissive = main_transmissive_pass_3d.into_system_set().intern();
        let world = sub.world_mut();
        let schedules = world.resource::<Schedules>();
        let graph = schedules.get(Core3d).unwrap().graph();
        let key = graph.system_sets.get_key(transmissive).expect("the transmissive pass is named");
        // Each system by its own function set (names need bevy_ecs's `debug` feature).
        let sets: HashMap<_, _> = graph.systems.iter().map(|(k, system, _)| (k, system.default_system_sets())).collect();
        let linked = |direction| -> Vec<InternedSystemSet> {
            graph
                .dependency()
                .graph()
                .neighbors_directed(NodeId::Set(key), direction)
                .filter_map(|node| if let NodeId::System(k) = node { sets.get(&k).cloned() } else { None })
                .flatten()
                .collect()
        };
        let before = linked(bevy_ecs::schedule::graph::Direction::Incoming);
        let after = linked(bevy_ecs::schedule::graph::Direction::Outgoing);
        let (snap, refl) = (snapshot.into_system_set().intern(), reflection.into_system_set().intern());
        let (nig, clo) = (night.into_system_set().intern(), clouds.into_system_set().intern());
        assert!(before.contains(&snap) && before.contains(&refl), "snapshot and SSPR before the water: {before:?}");
        assert!(after.contains(&nig) && after.contains(&clo), "weather after the water: {after:?}");
        assert!(!after.contains(&snap) && !after.contains(&refl) && !before.contains(&nig) && !before.contains(&clo));
    }
}
