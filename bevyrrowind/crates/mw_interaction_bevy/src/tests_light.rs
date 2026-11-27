//! `IN19c` headless tests of [`LightHook`]: a `LIGH` record's facts, the anchor and its saved state
//! at spawn, the index entry (slot disabled with the switch off, re-inserted live), the restored
//! rest, each snuff / relight saved to the overlay, and the hook hiding the point light and
//! darkening the glow surfaces while unlit (restored when relit).

use std::collections::HashMap;
use std::sync::{Arc, PoisonError};

use bevy_app::App;
use bevy_asset::{AssetServer, Assets, UntypedHandle};
use bevy_camera::visibility::{RenderLayers, Visibility};
use bevy_color::{Alpha, LinearRgba};
use bevy_ecs::message::Messages;
use bevy_ecs::prelude::*;
use bevy_light::PointLight;
use bevy_material::AlphaMode;
use bevy_pbr::{MeshMaterial3d, StandardMaterial};
use bevy_transform::components::Transform;
use glam::{DVec3, Quat, Vec3};
use mw_interaction::{LIGHT_SLOT, LightFacts, light_interactable};
use sf_interaction::{LIGHT_STATE_OWNER, LightState, VERB_RELIGHT, VERB_SNUFF};
use sf_interaction_bevy::{GameplayLights, IndexChange, LightRest, Lights, UseApproved};
use sf_records::RecordId;
use sf_script_bevy::sf_script::Handle;
use sf_script_bevy::{Activated, PLAYER};
use sf_tunables_bevy::{SetBy, Tunables};
use sf_world::{IndexedPlacement, Level, PlacementClass, PlacementId};
use sf_world_bevy::{ActivateCx, EndInputs, LoadedModel, PlacementSpawner, SpawnCx, StreamingConfig, WorldOverlay};
use tes3::esp::{Light, LightFlags};

use crate::light::{LightAnchor, LightAnchors, LightFactsTable, LightHook, LitMaterial, ResidentLights, SnuffSwitch, light_facts};
use crate::tests::{Nothing, app, cell, placement, tick};
use crate::tests_door::ws;

const CANDLE: PlacementId = PlacementId(60);
const GLOW: PlacementId = PlacementId(61);
const BASE: &str = "light_com_candle_01";
const GLOW_BASE: &str = "light_glow_01";

fn candle_facts() -> LightFacts {
    LightFacts { radius_m: 2.0, fire: true, carryable: false, negative: false, off_by_default: false }
}

fn candle() -> IndexedPlacement {
    let mut p = placement(CANDLE, BASE, DVec3::new(4.0, 1.0, -2.0), Quat::from_rotation_y(0.3));
    p.placement.class = PlacementClass::Light;
    p
}

/// An app with an empty overlay, the materials, and the light records: a candle and a glow light.
fn light_app() -> App {
    let mut app = app();
    app.init_resource::<WorldOverlay>().init_resource::<Assets<StandardMaterial>>();
    let glow = LightFacts { fire: false, ..candle_facts() };
    let table = HashMap::from([(BASE.to_owned(), candle_facts()), (GLOW_BASE.to_owned(), glow)]);
    app.insert_resource(LightFactsTable(Arc::new(table)));
    app
}

/// Spawns a point light and a mesh wearing `material` under `parent`.
struct Visuals(bevy_asset::Handle<StandardMaterial>);

impl PlacementSpawner for Visuals {
    fn spawn(&self, world: &mut World, cx: &SpawnCx<'_>, _: &IndexedPlacement) -> u32 {
        world.spawn((PointLight::default(), Visibility::Inherited, ChildOf(cx.parent)));
        world.spawn((MeshMaterial3d(self.0.clone()), Visibility::Inherited, ChildOf(cx.parent)));
        2
    }
    fn prepare(&self, _: &AssetServer, _: &IndexedPlacement, _: Option<&LoadedModel>) -> Option<UntypedHandle> {
        None
    }
}

fn spawn(app: &mut App, hook: &LightHook, p: &IndexedPlacement) -> (u32, Entity) {
    let lod = StreamingConfig::default().lod;
    let layers = RenderLayers::default();
    let world = app.world_mut();
    let parent = world.spawn(Transform::IDENTITY).id();
    let worldspace = ws();
    let cx = SpawnCx {
        worldspace: &worldspace,
        cell: cell(),
        level: Level::Near,
        parent,
        layers: &layers,
        local: Transform::IDENTITY,
        model: None,
        max_distance: 200.0,
        end_band: 0.0,
        end: EndInputs::fixed(200.0, 0.0),
        lod: &lod,
    };
    (hook.spawn(world, &cx, p), parent)
}

fn residency(app: &mut App, hook: &LightHook, p: &IndexedPlacement, on: bool) {
    let world = app.world_mut();
    let cell_entity = world.spawn_empty().id();
    let worldspace = ws();
    let cx = ActivateCx { worldspace: &worldspace, cell: cell(), cell_entity, discard: false };
    if on {
        hook.activate(world, &cx, p);
    } else {
        hook.deactivate(world, &cx, p);
    }
}

fn sent<M: Message + Clone>(app: &App) -> Vec<M> {
    let messages = app.world().resource::<Messages<M>>();
    messages.get_cursor().read(messages).cloned().collect()
}

fn changes(app: &App, id: PlacementId) -> Vec<IndexChange> {
    sent::<IndexChange>(app).into_iter().filter(|c| c.placement() == id).collect()
}

fn approve(app: &mut App, verb: sf_interaction::TagId) {
    let activation = Activated { target: Handle(CANDLE.0), actor: PLAYER, verb: Some(verb.0) };
    app.world_mut().write_message(UseApproved { activation, verb: Some(verb), slot: None });
    app.update();
}

fn byte(app: &App) -> Option<Vec<u8>> {
    let overlay = app.world().resource::<WorldOverlay>().0.read().unwrap_or_else(PoisonError::into_inner);
    overlay.state(CANDLE, LIGHT_STATE_OWNER).map(<[u8]>::to_vec)
}

fn save(app: &App, lit: bool) {
    let mut overlay = app.world().resource::<WorldOverlay>().0.write().unwrap_or_else(PoisonError::into_inner);
    overlay.set_state(CANDLE, LIGHT_STATE_OWNER, vec![LightState { lit }.rest_byte()]);
}

fn anchor(app: &App) -> LightAnchor {
    let e = app.world().resource::<LightAnchors>().get(CANDLE).expect("an anchor");
    *app.world().get::<LightAnchor>(e).expect("the anchor component")
}

fn emissive() -> StandardMaterial {
    StandardMaterial { emissive: LinearRgba::rgb(1.0, 0.6, 0.2), ..StandardMaterial::default() }
}

/// The point light's visibility and the mesh's material (handle, and whether it glows).
fn visuals(app: &mut App) -> (Visibility, bevy_asset::Handle<StandardMaterial>, bool) {
    let mut lights = app.world_mut().query_filtered::<&Visibility, With<PointLight>>();
    let v = *lights.single(app.world()).expect("one light");
    let mut meshes = app.world_mut().query::<(&MeshMaterial3d<StandardMaterial>, Option<&LitMaterial>)>();
    let (m, original) = meshes.single(app.world()).expect("one mesh");
    let handle = m.0.clone();
    assert_eq!(original.is_some(), v == Visibility::Hidden, "the original is kept exactly while dark");
    let glows = app.world().resource::<Assets<StandardMaterial>>().get(&handle).is_some_and(crate::light::glows);
    (v, handle, glows)
}

#[test]
fn record_flags_become_facts() {
    let mut l = Light::default();
    l.data.radius = 128;
    l.data.flags = LightFlags::FIRE | LightFlags::CAN_CARRY | LightFlags::FLICKER;
    let f = light_facts(&l);
    assert!((f.radius_m - mw_world::convert::meters(128.0)).abs() < 1e-6);
    assert_eq!((f.fire, f.carryable, f.negative, f.off_by_default), (true, true, false, false));
    l.data.flags = LightFlags::NEGATIVE | LightFlags::OFF_BY_DEFAULT;
    let f = light_facts(&l);
    assert_eq!((f.fire, f.carryable, f.negative, f.off_by_default), (false, false, true, true));
}

#[test]
fn a_fire_light_spawns_under_an_anchor_and_is_indexed_while_resident() {
    let mut app = light_app();
    let hook = LightHook(Arc::new(Nothing));
    let p = candle();
    let (count, parent) = spawn(&mut app, &hook, &p);
    assert_eq!(count, 1);
    let a = anchor(&app);
    assert_eq!(a, LightAnchor { id: CANDLE, lit: true, flame: Vec3::ZERO });
    let e = app.world().resource::<LightAnchors>().get(CANDLE).expect("an anchor");
    assert_eq!(app.world().get::<ChildOf>(e).map(ChildOf::parent), Some(parent));

    residency(&mut app, &hook, &p, true);
    let c = changes(&app, CANDLE);
    assert_eq!(c.len(), 1);
    let IndexChange::Insert { def, object, disabled, .. } = &c[0] else { panic!("an insert: {c:?}") };
    assert_eq!(**def, light_interactable(&RecordId::new(BASE), &candle_facts(), Vec3::ZERO, true).expect("a def"));
    assert_eq!(*object, crate::index::frame_of(&p.placement.transform));
    assert!(disabled.is_empty());
    assert!(sent::<LightRest>(&app).is_empty(), "nothing saved: nothing restored");
    tick(&mut app, 1);
    assert!(app.world().resource::<Lights>().get(CANDLE).is_some_and(|l| l.state.lit));
    assert!(app.world().resource::<GameplayLights>().get(CANDLE).is_some_and(|g| g.lit));

    residency(&mut app, &hook, &p, false);
    let c = changes(&app, CANDLE);
    assert_eq!(c.len(), 2);
    assert!(matches!(c[1], IndexChange::Remove { .. }), "{c:?}");
    assert!(app.world().resource::<ResidentLights>().is_empty());
}

#[test]
fn other_lights_are_left_to_the_inner_spawner() {
    let mut app = light_app();
    let hook = LightHook(Arc::new(Nothing));
    let mut glow = placement(GLOW, GLOW_BASE, DVec3::ZERO, Quat::IDENTITY);
    glow.placement.class = PlacementClass::Light;
    let unknown = placement(PlacementId(62), "light_unknown", DVec3::ZERO, Quat::IDENTITY);
    for p in [glow, unknown] {
        assert_eq!(spawn(&mut app, &hook, &p).0, 0);
        residency(&mut app, &hook, &p, true);
        assert!(changes(&app, p.placement.id).is_empty());
    }
    assert!(app.world().resource::<LightAnchors>().get(GLOW).is_none());
}

#[test]
fn snuff_and_relight_are_saved_and_a_saved_state_is_restored() {
    let mut app = light_app();
    let hook = LightHook(Arc::new(Nothing));
    let p = candle();
    spawn(&mut app, &hook, &p);
    residency(&mut app, &hook, &p, true);
    tick(&mut app, 1);
    assert_eq!(byte(&app), None);
    approve(&mut app, VERB_SNUFF);
    assert_eq!(byte(&app), Some(vec![0]));
    assert!(!anchor(&app).lit);
    approve(&mut app, VERB_RELIGHT);
    assert_eq!(byte(&app), Some(vec![1]));
    assert!(anchor(&app).lit);
    approve(&mut app, VERB_SNUFF);

    // The cell reloads: the anchor spawns dark and the shell restores the saved state.
    residency(&mut app, &hook, &p, false);
    let anchor_entity = app.world().resource::<LightAnchors>().get(CANDLE).expect("an anchor");
    app.world_mut().despawn(anchor_entity);
    assert!(app.world().resource::<LightAnchors>().get(CANDLE).is_none(), "a despawned anchor leaves the map");
    spawn(&mut app, &hook, &p);
    assert!(!anchor(&app).lit);
    residency(&mut app, &hook, &p, true);
    assert_eq!(sent::<LightRest>(&app).last(), Some(&LightRest { placement: CANDLE, lit: false }));
    tick(&mut app, 1);
    assert!(app.world().resource::<Lights>().get(CANDLE).is_some_and(|l| !l.state.lit));
}

#[test]
fn the_snuff_switch_disables_the_slot_live() {
    let mut app = light_app();
    let hook = LightHook(Arc::new(Nothing));
    let p = candle();
    spawn(&mut app, &hook, &p);
    residency(&mut app, &hook, &p, true);
    let id = app.world().resource::<SnuffSwitch>().0;
    app.world_mut().resource_mut::<Tunables>().write(id, false, SetBy::Console).expect("a bool");
    app.update();
    let c = changes(&app, CANDLE);
    assert_eq!(c.len(), 2, "{c:?}");
    let IndexChange::Insert { def, disabled, .. } = &c[1] else { panic!("a re-insert: {c:?}") };
    assert_eq!(disabled.as_slice(), [LIGHT_SLOT]);
    assert!(def.verbs.is_empty(), "no relight in Classic");
    tick(&mut app, 1);
    assert!(app.world().resource::<GameplayLights>().get(CANDLE).is_some(), "still the AI's light");
}

#[test]
fn the_hook_hides_the_light_and_darkens_the_glow_while_unlit() {
    let mut app = light_app();
    let lit_material = app.world_mut().resource_mut::<Assets<StandardMaterial>>().add(emissive());
    let hook = LightHook(Arc::new(Visuals(lit_material.clone())));
    let p = candle();
    save(&app, false);
    assert_eq!(spawn(&mut app, &hook, &p).0, 3);
    app.update();
    let (v, dark, glows) = visuals(&mut app);
    assert_eq!(v, Visibility::Hidden);
    assert_ne!(dark, lit_material);
    assert!(!glows, "the dark copy emits nothing");

    residency(&mut app, &hook, &p, true);
    tick(&mut app, 1);
    approve(&mut app, VERB_RELIGHT);
    let (v, material, _) = visuals(&mut app);
    assert_eq!((v, material), (Visibility::Inherited, lit_material.clone()));
    approve(&mut app, VERB_SNUFF);
    let (v, again, _) = visuals(&mut app);
    assert_eq!((v, again), (Visibility::Hidden, dark), "one shared dark copy per material");
}

#[test]
fn dark_flames_are_transparent_and_plain_surfaces_are_not_glow() {
    let flame = StandardMaterial { alpha_mode: AlphaMode::Add, ..StandardMaterial::default() };
    assert!(crate::light::glows(&flame));
    let dark = crate::light::dark_material(&flame);
    assert!(dark.base_color.alpha().abs() < f32::EPSILON, "transparent");
    assert!(!crate::light::glows(&crate::light::dark_material(&emissive())));
    assert_eq!(crate::light::dark_material(&emissive()).base_color, emissive().base_color);
    assert!(!crate::light::glows(&StandardMaterial::default()));
}
