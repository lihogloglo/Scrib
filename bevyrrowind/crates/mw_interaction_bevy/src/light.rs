//! Fire lights into the slot index and the render light following their state (crate docs,
//! "Lights"; interaction-adapter.md "Lights", slice `IN19c`).
//!
//! - **Records** ([`LightFactsTable`]): each `LIGH` record's [`LightFacts`] by lowercase id, built
//!   once from the host's record db ([`insert_records`]); empty: no light is indexed.
//! - **Anchor** ([`LightHook`], wrapping the Light class's spawner): a light with a def
//!   (`mw_interaction::light_def`: a lit fire light) spawns its visuals (model and point light)
//!   under a [`LightAnchor`] at identity, carrying whether it is lit: the overlay's saved byte
//!   (owner `light`), else lit. Outside R0 the anchor keeps that state, so a snuffed candle in the
//!   Near ring stays dark.
//! - **Index** (R0): activation indexes the light with `mw_interaction::light_interactable` (its
//!   slot at the model bounds' centre, scale applied; the origin without a cooked model), the slot
//!   disabled when `mw.rules.interaction.snuff` is off (Classic: still a gameplay light, no verb),
//!   and restores a saved state (`LightRest`); deactivation removes it. Flipping the switch live
//!   re-inserts the resident lights (the shell keeps their state).
//! - **Follow and save** ([`follow_changes`], after the shell's light default): each
//!   `LightChanged` of a resident light sets its anchor's `lit` and writes its rest byte to the
//!   overlay, so the state survives a cell reload and a save.
//! - **Light hook** ([`apply_anchors`]): an anchor's state reaches its visuals: an unlit anchor's
//!   point lights are `Visibility::Hidden` (Bevy's extraction skips them) and its glow surfaces
//!   (an emissive material, or an additive / premultiplied flame) wear a dark copy of their
//!   material ([`dark_material`]: no emission; a flame fully transparent), shared per source
//!   material; a lit anchor gets the originals back. Visuals that spawn later (the model's scene)
//!   follow their anchor's state as they appear.
//!
//! Log: `interact: N lights indexed in <worldspace> cell (x, y, z)`.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::{Arc, PoisonError};

use bevy_asset::{AssetId, AssetServer, Assets, Handle, UntypedHandle};
use bevy_camera::visibility::Visibility;
use bevy_color::{Color, LinearRgba};
use bevy_ecs::prelude::*;
use bevy_light::PointLight;
use bevy_log::info;
use bevy_material::AlphaMode;
use bevy_math::Vec3;
use bevy_pbr::{MeshMaterial3d, StandardMaterial};
use bevy_transform::components::Transform;
use mw_interaction::{LIGHT_SLOT, LightFacts, light_def, light_interactable};
use sf_interaction::{Frame, LIGHT_STATE_OWNER, LightState};
use sf_interaction_bevy::{IndexChange, LightChanged, LightRest};
use sf_records::{RecordDb, RecordId};
use sf_tunables_bevy::{Id, Tunables};
use sf_world::{IndexedPlacement, PlacementClass, PlacementFlags, PlacementId};
use sf_world_bevy::{ActivateCx, LoadedModel, PlacementSpawner, PlacementSpawners, SpawnCx, WorldOverlay};
use smallvec::{SmallVec, smallvec};
use tes3::esp::{Light, LightFlags};

use crate::index::frame_of;

/// Every `LIGH` record's [`LightFacts`], by lowercase id.
#[derive(Resource, Clone, Debug, Default)]
pub struct LightFactsTable(pub Arc<HashMap<String, LightFacts>>);

impl LightFactsTable {
    /// The facts of every `LIGH` record of `db`.
    #[must_use]
    pub fn build(db: &RecordDb) -> Self {
        Self(Arc::new(db.iter::<Light>().map(|(id, l)| (id.as_str().to_ascii_lowercase(), light_facts(l))).collect()))
    }

    /// The facts of record `id` (any case).
    #[must_use]
    pub fn get(&self, id: &RecordId) -> Option<&LightFacts> {
        self.0.get(&id.as_str().to_ascii_lowercase())
    }
}

/// What the adapter reads from a `LIGH` record (radius in metres as `mw_world::convert::meters`).
#[must_use]
pub fn light_facts(light: &Light) -> LightFacts {
    let flags = light.data.flags;
    #[allow(clippy::cast_precision_loss)] // radii are small game units
    let radius_m = mw_world::convert::meters(light.data.radius as f32);
    LightFacts {
        radius_m,
        fire: flags.contains(LightFlags::FIRE),
        carryable: flags.contains(LightFlags::CAN_CARRY),
        negative: flags.contains(LightFlags::NEGATIVE),
        off_by_default: flags.contains(LightFlags::OFF_BY_DEFAULT),
    }
}

/// Inserts the content's [`LightFactsTable`] (the host's record db) and logs its size.
pub fn insert_records(app: &mut bevy_app::App, db: &RecordDb) {
    let table = LightFactsTable::build(db);
    let snuffable = table.0.values().filter(|f| light_def(f).is_some()).count();
    info!("interact: {} LIGH records, {snuffable} snuffable fire lights", table.0.len());
    app.insert_resource(table);
}

/// `mw.rules.interaction.snuff` (`mw_interaction::tunables::SNUFF`), read live.
#[derive(Resource, Clone, Copy, Debug)]
pub struct SnuffSwitch(pub Id<bool>);

/// The anchor of a def'd light's visuals and the state they show.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct LightAnchor {
    pub id: PlacementId,
    pub lit: bool,
    /// The flame point in the object's frame, metres, scale applied (the model bounds' centre;
    /// zero without a cooked model).
    pub flame: Vec3,
}

/// The spawned anchor of each def'd light, by placement.
#[derive(Resource, Clone, Debug, Default)]
pub struct LightAnchors(BTreeMap<PlacementId, Entity>);

impl LightAnchors {
    #[must_use]
    pub fn get(&self, id: PlacementId) -> Option<Entity> {
        self.0.get(&id).copied()
    }
}

/// One resident (R0) def'd light.
#[derive(Clone, Debug)]
struct Resident {
    base: RecordId,
    facts: LightFacts,
    flame: Vec3,
    object: Frame,
    owner: Entity,
    /// Indexed with the light verbs (the snuff switch was on).
    snuff: bool,
}

impl Resident {
    fn insert(&self, placement: PlacementId) -> Option<IndexChange> {
        let def = light_interactable(&self.base, &self.facts, self.flame, self.snuff)?;
        let disabled = if self.snuff { SmallVec::new() } else { smallvec![LIGHT_SLOT] };
        Some(IndexChange::Insert { placement, def: Arc::new(def), object: self.object, owner: self.owner, disabled })
    }
}

/// The def'd lights in R0, by placement.
#[derive(Resource, Clone, Debug, Default)]
pub struct ResidentLights(BTreeMap<PlacementId, Resident>);

impl ResidentLights {
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub fn contains(&self, id: PlacementId) -> bool {
        self.0.contains_key(&id)
    }
}

/// Indexed lights per activated cell since the last log line.
#[derive(Resource, Default)]
pub(crate) struct LightLog(BTreeMap<(String, [i32; 3]), u32>);

/// The lit state the overlay holds for `id`, if it decodes.
fn saved_lit(world: &World, id: PlacementId) -> Option<bool> {
    let overlay = world.get_resource::<WorldOverlay>()?;
    let overlay = overlay.0.read().unwrap_or_else(PoisonError::into_inner);
    LightState::from_rest_byte(overlay.state(id, LIGHT_STATE_OWNER)?).map(|s| s.lit)
}

fn snuff_on(world: &World) -> bool {
    let read = world.get_resource::<SnuffSwitch>().zip(world.get_resource::<Tunables>()).map(|(s, t)| t.read(s.0));
    read.unwrap_or(mw_interaction::tunables::SNUFF.default_value())
}

/// The facts of `placement` when it is a def'd light, enabled.
fn def_facts(world: &World, placement: &IndexedPlacement) -> Option<LightFacts> {
    let p = &placement.placement;
    if p.flags.intersects(PlacementFlags::DISABLED | PlacementFlags::EDITOR_ONLY) {
        return None;
    }
    let facts = *world.get_resource::<LightFactsTable>()?.get(&p.base.id)?;
    light_def(&facts).map(|_| facts)
}

/// Wraps the Light class's spawner with [`LightHook`].
pub(crate) fn hook_spawners(spawners: Option<ResMut<PlacementSpawners>>) {
    let Some(mut spawners) = spawners else { return };
    if let Some(inner) = spawners.for_class(PlacementClass::Light).cloned() {
        spawners.set(PlacementClass::Light, Arc::new(LightHook(inner)));
    }
}

/// Drops an anchor's entry when it loses [`LightAnchor`] (its cell level despawned), unless a
/// respawn already replaced it.
pub(crate) fn forget_anchor(remove: On<Remove<LightAnchor>>, anchors: Query<&LightAnchor>, mut map: ResMut<LightAnchors>) {
    let Ok(anchor) = anchors.get(remove.entity) else { return };
    if map.0.get(&anchor.id) == Some(&remove.entity) {
        map.0.remove(&anchor.id);
    }
}

/// Spawns def'd lights under an anchor and indexes them in R0 (module docs).
pub struct LightHook(pub Arc<dyn PlacementSpawner>);

impl PlacementSpawner for LightHook {
    fn spawn(&self, world: &mut World, cx: &SpawnCx<'_>, placement: &IndexedPlacement) -> u32 {
        let p = &placement.placement;
        if def_facts(world, placement).is_none() {
            return self.0.spawn(world, cx, placement);
        }
        let flame = match cx.model {
            Some(LoadedModel::Cooked(template)) => Vec3::from(template.aabb.center) * cx.local.scale,
            _ => Vec3::ZERO,
        };
        let lit = saved_lit(world, p.id).unwrap_or(true);
        let anchor = world
            .spawn((Transform::IDENTITY, Visibility::Inherited, ChildOf(cx.parent), LightAnchor { id: p.id, lit, flame }))
            .id();
        world.get_resource_or_init::<LightAnchors>().0.insert(p.id, anchor);
        let inner = SpawnCx { parent: anchor, ..*cx };
        1 + self.0.spawn(world, &inner, placement)
    }
    fn needs_model(&self, placement: &IndexedPlacement) -> bool {
        self.0.needs_model(placement)
    }
    fn prepare(&self, assets: &AssetServer, placement: &IndexedPlacement, model: Option<&LoadedModel>) -> Option<UntypedHandle> {
        self.0.prepare(assets, placement, model)
    }
    fn activate(&self, world: &mut World, cx: &ActivateCx<'_>, placement: &IndexedPlacement) {
        self.0.activate(world, cx, placement);
        let Some(facts) = def_facts(world, placement) else { return };
        let p = &placement.placement;
        let anchor = world.get_resource::<LightAnchors>().and_then(|m| m.get(p.id));
        let flame = anchor.and_then(|e| world.get::<LightAnchor>(e)).map_or(Vec3::ZERO, |a| a.flame);
        let resident =
            Resident { base: p.base.id.clone(), facts, flame, object: frame_of(&p.transform), owner: cx.cell_entity, snuff: snuff_on(world) };
        let Some(insert) = resident.insert(p.id) else { return };
        world.write_message(insert);
        if let Some(lit) = saved_lit(world, p.id) {
            world.write_message(LightRest { placement: p.id, lit });
        }
        world.get_resource_or_init::<ResidentLights>().0.insert(p.id, resident);
        let key = (cx.worldspace.as_str().to_owned(), cx.cell.to_array());
        *world.get_resource_or_init::<LightLog>().0.entry(key).or_default() += 1;
    }
    fn deactivate(&self, world: &mut World, cx: &ActivateCx<'_>, placement: &IndexedPlacement) {
        self.0.deactivate(world, cx, placement);
        let id = placement.placement.id;
        if world.get_resource_mut::<ResidentLights>().and_then(|mut r| r.0.remove(&id)).is_some() {
            world.write_message(IndexChange::unload(id));
        }
    }
}

/// The snuff switch changed: the resident lights are re-inserted with or without their verbs (the
/// shell keeps their state).
#[allow(clippy::needless_pass_by_value)] // system parameters
pub(crate) fn sync_snuff(
    switch: Res<SnuffSwitch>,
    tunables: Res<Tunables>,
    mut resident: ResMut<ResidentLights>,
    mut out: MessageWriter<IndexChange>,
) {
    let on = tunables.read(switch.0);
    let mut changed = 0_u32;
    for (placement, r) in resident.0.iter_mut().filter(|(_, r)| r.snuff != on) {
        r.snuff = on;
        if let Some(insert) = r.insert(*placement) {
            out.write(insert);
            changed += 1;
        }
    }
    if changed > 0 {
        info!("interact: mw.rules.interaction.snuff {}: {changed} resident lights re-indexed", if on { "on" } else { "off" });
    }
}

/// Each `LightChanged` of a resident light into its anchor and its overlay byte (module docs).
#[allow(clippy::needless_pass_by_value)] // system parameters
pub(crate) fn follow_changes(
    mut changed: MessageReader<LightChanged>,
    resident: Res<ResidentLights>,
    map: Res<LightAnchors>,
    mut anchors: Query<&mut LightAnchor>,
    overlay: Option<Res<WorldOverlay>>,
) {
    let flips: Vec<(PlacementId, bool)> =
        changed.read().filter(|c| resident.contains(c.0.placement)).map(|c| (c.0.placement, c.0.lit)).collect();
    if flips.is_empty() {
        return;
    }
    for &(id, lit) in &flips {
        if let Some(mut anchor) = map.get(id).and_then(|e| anchors.get_mut(e).ok())
            && anchor.lit != lit
        {
            anchor.lit = lit;
        }
    }
    let Some(overlay) = overlay else { return };
    let mut overlay = overlay.0.write().unwrap_or_else(PoisonError::into_inner);
    for (id, lit) in flips {
        overlay.set_state(id, LIGHT_STATE_OWNER, vec![LightState { lit }.rest_byte()]);
    }
}

/// A glow surface's original material while its light is out.
#[derive(Component, Clone, Debug)]
pub struct LitMaterial(pub Handle<StandardMaterial>);

/// The shared dark copies of glow materials, by source material.
#[derive(Resource, Clone, Debug, Default)]
pub struct DarkMaterials {
    by_source: HashMap<AssetId<StandardMaterial>, Handle<StandardMaterial>>,
    /// The copies themselves (a part already dark is left alone).
    dark: HashSet<AssetId<StandardMaterial>>,
}

impl DarkMaterials {
    /// The shared dark copy of `source` (`id`), made on first use.
    fn of(&mut self, id: AssetId<StandardMaterial>, source: &StandardMaterial, materials: &mut Assets<StandardMaterial>) -> Handle<StandardMaterial> {
        if let Some(h) = self.by_source.get(&id) {
            return h.clone();
        }
        let h = materials.add(dark_material(source));
        self.dark.insert(h.id());
        self.by_source.insert(id, h.clone());
        h
    }
}

/// A flame surface: additive or premultiplied (pure light).
fn flame(m: &StandardMaterial) -> bool {
    matches!(m.alpha_mode, AlphaMode::Add | AlphaMode::Premultiplied)
}

/// Whether `m` is a light's glow: a flame, or an emissive surface.
#[must_use]
pub fn glows(m: &StandardMaterial) -> bool {
    flame(m) || m.emissive.red.max(m.emissive.green).max(m.emissive.blue) > 0.0
}

/// The material a glow surface wears while its light is out: no emission; a flame fully
/// transparent (it adds nothing).
#[must_use]
pub fn dark_material(m: &StandardMaterial) -> StandardMaterial {
    let mut dark = m.clone();
    dark.emissive = LinearRgba::BLACK;
    if flame(m) {
        dark.base_color = Color::linear_rgba(0.0, 0.0, 0.0, 0.0);
    }
    dark
}

/// The anchor `entity` is under, if any.
fn anchor_of(entity: Entity, parents: &Query<&ChildOf>, anchors: &Query<&LightAnchor>) -> Option<LightAnchor> {
    std::iter::once(entity).chain(parents.iter_ancestors(entity)).find_map(|a| anchors.get(a).ok().copied())
}

/// The visual parts of the light hook (module docs, "Light hook").
type Parts<'w, 's> = Query<
    'w,
    's,
    (Option<&'static mut Visibility>, Option<&'static mut MeshMaterial3d<StandardMaterial>>, Option<&'static LitMaterial>, Has<PointLight>),
>;

/// Shows `entity` as its light's `lit` state says; `true` when it is a light part.
fn apply(
    entity: Entity,
    lit: bool,
    parts: &mut Parts,
    commands: &mut Commands,
    materials: Option<&mut Assets<StandardMaterial>>,
    dark: &mut DarkMaterials,
) {
    let Ok((visibility, material, original, light)) = parts.get_mut(entity) else { return };
    if light && let Some(mut v) = visibility {
        let want = if lit { Visibility::Inherited } else { Visibility::Hidden };
        if *v != want {
            *v = want;
        }
    }
    let Some(mut material) = material else { return };
    match (lit, original) {
        (true, Some(original)) => {
            material.0 = original.0.clone();
            commands.entity(entity).remove::<LitMaterial>();
        }
        (false, None) => {
            let id = material.0.id();
            let Some(materials) = materials.filter(|_| !dark.dark.contains(&id)) else { return };
            let Some(source) = materials.get(id).filter(|m| glows(m)).cloned() else { return };
            let handle = dark.of(id, &source, materials);
            commands.entity(entity).insert(LitMaterial(std::mem::replace(&mut material.0, handle)));
        }
        _ => {}
    }
}

/// An anchor's state reaches its visuals: every part under a changed anchor, and each part that
/// appears under an anchor (module docs, "Light hook").
#[allow(clippy::needless_pass_by_value, clippy::type_complexity, clippy::too_many_arguments)] // system parameters
pub(crate) fn apply_anchors(
    mut commands: Commands,
    changed: Query<(Entity, &LightAnchor), Changed<LightAnchor>>,
    children: Query<&Children>,
    parents: Query<&ChildOf>,
    anchors: Query<&LightAnchor>,
    // The added parts' filter reads what the parts query writes: one at a time.
    mut set: ParamSet<(Query<Entity, Or<(Added<PointLight>, Added<MeshMaterial3d<StandardMaterial>>)>>, Parts)>,
    mut materials: Option<ResMut<Assets<StandardMaterial>>>,
    mut dark: ResMut<DarkMaterials>,
) {
    let added: Vec<(Entity, bool)> =
        set.p0().iter().filter_map(|part| anchor_of(part, &parents, &anchors).filter(|a| !a.lit).map(|a| (part, a.lit))).collect();
    let mut parts = set.p1();
    for (root, anchor) in &changed {
        for part in children.iter_descendants(root) {
            apply(part, anchor.lit, &mut parts, &mut commands, materials.as_deref_mut(), &mut dark);
        }
    }
    for (part, lit) in added {
        apply(part, lit, &mut parts, &mut commands, materials.as_deref_mut(), &mut dark);
    }
}

/// One line per cell that indexed lights this frame.
pub(crate) fn log_indexed(mut log: ResMut<LightLog>) {
    if log.0.is_empty() {
        return;
    }
    for ((worldspace, [x, y, z]), n) in std::mem::take(&mut log.0) {
        info!("interact: {n} lights indexed in {worldspace} cell ({x}, {y}, {z})");
    }
}
