//! The order of the passes between Bevy's opaque and transparent passes
//! (render-order.md RO-8), declared once.
//!
//! `render_sky` is private to `bevy_pbr`, so its system set can't be named. A
//! pass added here is ordered after every system set already ordered before
//! `main_transparent_pass_3d` (Bevy's sky included), found when the pass is
//! added, and after the earlier [`PassStage`]s through the sets themselves, so
//! the order is the enum's, not the plugin add order. Scaffold
//! `SkyOrderScaffold`; removal path: a public `render_sky` set upstream.

use std::collections::HashMap;

use bevy_app::{App, SubApp};
use bevy_core_pipeline::core_3d::{main_opaque_pass_3d, main_transparent_pass_3d};
use bevy_core_pipeline::schedule::Core3d;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::{InternedSystemSet, IntoScheduleConfigs, NodeId, Schedules};
use bevy_ecs::system::ScheduleSystem;
use bevy_render::RenderApp;

/// The passes between the opaque and the transparent pass, in draw order.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PassStage {
    /// Night sky: stars and moons (`sf_sky_bevy`).
    Night,
    /// Cloud and fog composite (`sf_atmos_bevy`).
    Clouds,
    /// The refraction snapshot (`sf_water_bevy`).
    Snapshot,
}

/// The passes added through [`add_pass`] (their own function sets).
#[derive(Resource, Default)]
struct Registered(Vec<InternedSystemSet>);

/// Adds `pass` to the render app's `Core3d` schedule in `stage`: after the
/// opaque pass and everything already ordered before the transparent pass,
/// after earlier stages, before the transparent pass. `own` is the pass's own
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
    render_app.configure_sets(Core3d, (PassStage::Night.before(PassStage::Clouds), PassStage::Clouds.before(PassStage::Snapshot)));
    let mut config = pass.in_set(stage).after(main_opaque_pass_3d).before(main_transparent_pass_3d);
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

    fn run(order: [usize; 3]) -> Vec<&'static str> {
        let mut sub = SubApp::new();
        sub.init_resource::<Log>();
        for i in order {
            match i {
                0 => add_pass_to(&mut sub, PassStage::Night, night.into_system_set().intern(), &[], night),
                1 => add_pass_to(&mut sub, PassStage::Clouds, clouds.into_system_set().intern(), &[], clouds),
                _ => add_pass_to(&mut sub, PassStage::Snapshot, snapshot.into_system_set().intern(), &[], snapshot),
            }
        }
        sub.world_mut().run_schedule(Core3d);
        sub.world().resource::<Log>().0.clone()
    }

    /// The declared order holds for every plugin add order (RO-8).
    #[test]
    fn order_is_the_enums_for_any_add_order() {
        for order in [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]] {
            assert_eq!(run(order), ["night", "clouds", "snapshot"], "added in order {order:?}");
        }
    }
}
