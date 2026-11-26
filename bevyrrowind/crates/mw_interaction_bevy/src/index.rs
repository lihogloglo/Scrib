//! Census furniture into the slot index as cells enter and leave R0 (crate docs, "Index").

use std::collections::BTreeMap;
use std::sync::Arc;

use bevy_asset::{AssetServer, UntypedHandle};
use bevy_ecs::prelude::*;
use bevy_log::info;
use sf_coords::WorldTransform;
use sf_interaction::{Frame, InteractableDef, InvalidReason};
use sf_interaction_bevy::IndexChange;
use sf_tunables_bevy::{Id, Tunables};
use sf_world::{IndexedPlacement, PlacementClass, PlacementId};
use sf_world_bevy::{ActivateCx, LoadedModel, PlacementSpawner, PlacementSpawners, SpawnCx};
use smallvec::SmallVec;

use crate::FurnitureDefs;

/// `mw.rules.interaction.sit` (`mw_interaction::tunables::SIT`), read live.
#[derive(Resource, Clone, Copy, Debug)]
pub struct SitSwitch(pub Id<bool>);

/// One resident placement with a def.
#[derive(Clone, Debug)]
struct Resident {
    def: Arc<InteractableDef>,
    object: Frame,
    owner: Entity,
    /// Inserted into the index (the sit switch was on).
    indexed: bool,
}

/// Every R0 placement whose base has a def, indexed or not (the sit switch re-inserts them).
#[derive(Resource, Clone, Debug, Default)]
pub struct ResidentFurniture(BTreeMap<PlacementId, Resident>);

impl ResidentFurniture {
    /// Resident furniture placements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Whether `placement` is in the index.
    #[must_use]
    pub fn indexed(&self, placement: PlacementId) -> bool {
        self.0.get(&placement).is_some_and(|r| r.indexed)
    }
}

/// Indexed placements per activated cell since the last log line.
#[derive(Resource, Default)]
pub(crate) struct IndexLog(BTreeMap<(String, [i32; 3]), u32>);

/// The index frame of a placement: its world transform narrowed to f32 metres (the action body's
/// space, `sf_interaction_bevy::seat`). The scale is not applied: defs are measured on the model
/// at scale 1.
pub(crate) fn frame_of(t: &WorldTransform) -> Frame {
    Frame { translation: t.pos.to_f64().as_vec3(), rotation: t.rotation }
}

fn insert(placement: PlacementId, r: &Resident) -> IndexChange {
    IndexChange::Insert { placement, def: Arc::clone(&r.def), object: r.object, owner: r.owner, disabled: SmallVec::new() }
}

fn sit_on(world: &World) -> bool {
    let read = world.get_resource::<SitSwitch>().zip(world.get_resource::<Tunables>()).map(|(s, t)| t.read(s.0));
    read.unwrap_or(mw_interaction::tunables::SIT.default_value())
}

/// `placement` entered R0: registered when its base has a def, indexed when sitting is on.
pub(crate) fn activated(world: &mut World, cx: &ActivateCx<'_>, placement: &IndexedPlacement) {
    let p = &placement.placement;
    let Some(def) = world.get_resource::<FurnitureDefs>().and_then(|d| d.get(&p.base.id)).cloned() else { return };
    let indexed = sit_on(world);
    let resident = Resident { def, object: frame_of(&p.transform), owner: cx.cell_entity, indexed };
    if indexed {
        world.write_message(insert(p.id, &resident));
        let key = (cx.worldspace.as_str().to_owned(), cx.cell.to_array());
        *world.get_resource_or_init::<IndexLog>().0.entry(key).or_default() += 1;
    }
    world.get_resource_or_init::<ResidentFurniture>().0.insert(p.id, resident);
}

/// `placement` left R0 (or its cell's state is discarded): out of the index.
pub(crate) fn deactivated(world: &mut World, placement: &IndexedPlacement) {
    let id = placement.placement.id;
    let gone = world.get_resource_mut::<ResidentFurniture>().and_then(|mut r| r.0.remove(&id));
    if gone.is_some_and(|r| r.indexed) {
        world.write_message(IndexChange::unload(id));
    }
}

/// Wraps the census classes' spawners (statics, activators, containers) with [`FurnitureHook`].
pub(crate) fn hook_spawners(spawners: Option<ResMut<PlacementSpawners>>) {
    let Some(mut spawners) = spawners else { return };
    for class in [PlacementClass::Static, mw_world::source::ACTIVATOR, mw_world::source::CONTAINER] {
        if let Some(inner) = spawners.for_class(class).cloned() {
            spawners.set(class, Arc::new(FurnitureHook(inner)));
        }
    }
}

/// Indexes furniture placements on R0 activation, removes them on deactivation.
pub struct FurnitureHook(pub Arc<dyn PlacementSpawner>);

impl PlacementSpawner for FurnitureHook {
    fn spawn(&self, world: &mut World, cx: &SpawnCx<'_>, placement: &IndexedPlacement) -> u32 {
        self.0.spawn(world, cx, placement)
    }
    fn needs_model(&self, placement: &IndexedPlacement) -> bool {
        self.0.needs_model(placement)
    }
    fn prepare(&self, assets: &AssetServer, placement: &IndexedPlacement, model: Option<&LoadedModel>) -> Option<UntypedHandle> {
        self.0.prepare(assets, placement, model)
    }
    fn activate(&self, world: &mut World, cx: &ActivateCx<'_>, placement: &IndexedPlacement) {
        self.0.activate(world, cx, placement);
        activated(world, cx, placement);
    }
    fn deactivate(&self, world: &mut World, cx: &ActivateCx<'_>, placement: &IndexedPlacement) {
        self.0.deactivate(world, cx, placement);
        deactivated(world, placement);
    }
}

/// The sit switch changed: resident furniture leaves the index (off, `SlotDisabled`) or comes back.
#[allow(clippy::needless_pass_by_value)] // system parameters
pub(crate) fn sync_sit(
    switch: Res<SitSwitch>,
    tunables: Res<Tunables>,
    mut resident: ResMut<ResidentFurniture>,
    mut out: MessageWriter<IndexChange>,
) {
    let on = tunables.read(switch.0);
    let mut changed = 0_u32;
    for (placement, r) in resident.0.iter_mut().filter(|(_, r)| r.indexed != on) {
        r.indexed = on;
        changed += 1;
        out.write(if on { insert(*placement, r) } else { IndexChange::Remove { placement: *placement, reason: InvalidReason::SlotDisabled } });
    }
    if changed > 0 {
        info!("interact: mw.rules.interaction.sit {}: {changed} resident furniture placements {}", if on { "on" } else { "off" }, if on { "indexed" } else { "removed" });
    }
}

/// One line per cell that indexed furniture this frame.
pub(crate) fn log_indexed(mut log: ResMut<IndexLog>) {
    if log.0.is_empty() {
        return;
    }
    for ((worldspace, [x, y, z]), n) in std::mem::take(&mut log.0) {
        info!("interact: indexed {n} furniture placements in {worldspace} ({x}, {y}, {z})");
    }
}
