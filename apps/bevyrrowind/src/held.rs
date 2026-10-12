//! Morrowind's held-item rules on `sf_character_bevy`'s props
//! (actors-adapter.md "Held items as built"; character.md "Held items").
//!
//! - **Torches when dark.** Actors re-ask `mw_actors` for their held items
//!   when darkness changes (`held::exterior_dark` on the sky clock: 19:00 to
//!   6:00), and when they spawn in the dark: `CharacterState::held` then
//!   names the torch, and the framework swaps only the props.
//! - **Torch lights.** A prop's `HeldLightSlot` gets the `LIGH` record's
//!   point light: the placed lights' calibration (`LightCalibration`) and
//!   flicker (`AnimatedLight`); shadows within the point-shadow budget
//!   ([`TORCH_SHADOW_PRIORITY`]). [`HeldLightBudget`] keeps the
//!   nearest few to the camera lit (each extra clustered light costs every
//!   fragment its range covers).
//! - **Stance.** The graph's `stance` parameter follows the readiness and
//!   the main-hand weapon (`mw_graphs::Stance::of_weapon`); its `torch`
//!   parameter raises the torch arm while a light shows in the off hand.

use std::sync::Arc;

use bevy::prelude::*;
use mw_actors::held::{DayCycle, HeldContext, exterior_dark};
use mw_assets::LightCalibration;
use mw_assets::lights::AnimatedLight;
use bevy::light::NotShadowCaster;
use sf_sky_bevy::BudgetedShadow;
use mw_graphs::Stance;
use mw_world::LightAnimation;
use sf_character::{Hand, Readiness};
use sf_character_bevy::{Character, CharacterBody, CharacterReadiness, CharacterSystems, HeldLightSlot, HeldProps};
use sf_records::RecordDb;
use tes3::esp::{Light, LightFlags, Weapon};

use crate::actors::MwCharacters;

/// Whether actors carry their lights. `forced` overrides the clock (the
/// `character` viewer's N key); without a sky clock it is day.
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct HeldDarkness {
    pub forced: Option<bool>,
    /// The state actors were last given.
    applied: Option<bool>,
}

impl HeldDarkness {
    /// Darkness set by hand (`None`: the clock's).
    pub fn forced(dark: Option<bool>) -> Self {
        Self { forced: dark, applied: None }
    }
}

/// How many held lights shine at once (the nearest to the camera).
#[derive(Resource, Clone, Copy, Debug)]
pub struct HeldLightBudget {
    pub max: usize,
}

impl Default for HeldLightBudget {
    fn default() -> Self {
        Self { max: 12 }
    }
}

/// Carried lights rank for a shadow map as if this many times nearer than
/// placed ones (`sf_sky_bevy::PointShadowBudget`): a moving torch-bearer's
/// shadow is the one players notice.
pub const TORCH_SHADOW_PRIORITY: f32 = 2.0;

/// Marks the point light on a held item (a child of its `HeldLightSlot`).
#[derive(Component, Clone, Copy, Debug)]
pub struct HeldTorch;

/// The character carrying a held light.
#[derive(Component, Clone, Copy, Debug)]
struct HeldTorchOf(Entity);

/// Near field of carried lights. Morrowind's lights saturate inside a third
/// of their radius (`radius / (3 d)` clamped to 1); ours are physical
/// (inverse-square), and the placed lights' calibration reads "fully lit"
/// at 1.15x the radius (`LightCalibration::fully_lit_distance`), so a torch
/// at full power lit its carrier ~50x past "fully lit" at 0.5 m and the
/// ground under it ~8x (torchfix 2026-10-12). Bevy's point light has no near
/// clamp (Frostbite's `max(d², r²)`): its diffuse is `1 / max(d², 1 cm²)`
/// and `PointLight::radius` only softens specular. So the carried light:
/// - is dimmed until "fully lit" falls at `fully_lit_fraction` of its radius
///   (1/3: where the original's clamp ends), which matches the original at
///   the ground under the torch and is brighter only inside it (the carrier
///   lands near 6x "fully lit" for white, textured after tone mapping);
/// - keeps `clearance_m` horizontally from the carrier's vertical axis
///   (pushed outward from the flame);
/// - has the flame's size as its source radius (wider, softer highlights);
/// - lights its own model through `mw_assets::self_lit` (the original's
///   saturated texture), not the inverse-square spike centimeters away.
#[derive(Resource, Clone, Copy, Debug)]
pub struct HeldLightTuning {
    pub clearance_m: f32,
    pub fully_lit_fraction: f32,
    pub source_radius_m: f32,
}

impl Default for HeldLightTuning {
    fn default() -> Self {
        Self { clearance_m: 0.7, fully_lit_fraction: 1.0 / 3.0, source_radius_m: 0.12 }
    }
}

impl HeldLightTuning {
    /// The carried light's intensity factor over the placed lights'.
    pub fn intensity_scale(&self, calibration: &LightCalibration, radius: f32) -> f32 {
        let full = calibration.fully_lit_distance(radius);
        if full <= 0.0 { 1.0 } else { ((self.fully_lit_fraction * radius) / full).powi(2).min(1.0) }
    }
}

/// On a held light's prop root: its surfaces are `mw_assets::self_lit`.
#[derive(Component, Clone, Copy, Debug)]
struct SelfLitProp;

/// Self-lit variants of prop materials, shared by every carried light.
#[derive(Resource, Default)]
struct SelfLitMaterials(std::collections::HashMap<AssetId<StandardMaterial>, Handle<StandardMaterial>>);

#[derive(Resource, Clone)]
struct HeldRules(Arc<MwCharacters>);

pub fn add(app: &mut App, content: Arc<MwCharacters>) {
    app.insert_resource(HeldRules(content))
        .init_resource::<HeldDarkness>()
        .init_resource::<HeldLightBudget>()
        .init_resource::<HeldLightTuning>()
        .init_resource::<SelfLitMaterials>()
        .add_systems(Update, carry_lights_when_dark.before(CharacterSystems))
        .add_systems(
            Update,
            (light_held_slots, place_held_lights, budget_held_lights, feed_stance, mark_light_props, self_light_props).chain().after(CharacterSystems),
        );
}

/// Marks the prop root of each held light.
fn mark_light_props(mut commands: Commands, carriers: Query<&HeldProps, Changed<HeldProps>>) {
    for props in &carriers {
        for prop in props.props.iter().filter(|p| p.light.is_some()) {
            commands.entity(prop.entity).try_insert(SelfLitProp);
        }
    }
}

type AddedMaterial = Added<MeshMaterial3d<StandardMaterial>>;
type MaterialSlot = &'static mut MeshMaterial3d<StandardMaterial>;

/// Swaps the materials under held-light props for their self-lit variants:
/// meshes added under a marked prop, and everything under a newly marked one.
/// They also cast no shadows (the carried light shines from inside them).
fn self_light_props(
    mut commands: Commands,
    mut cache: ResMut<SelfLitMaterials>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    props: Query<(Entity, Ref<SelfLitProp>)>,
    parents: Query<&ChildOf>,
    children: Query<&Children>,
    mut meshes: ParamSet<(Query<Entity, AddedMaterial>, Query<MaterialSlot>)>,
) {
    let mut targets: Vec<Entity> = props.iter().filter(|(_, m)| m.is_added()).flat_map(|(p, _)| children.iter_descendants(p)).collect();
    targets.extend(meshes.p0().iter().filter(|e| parents.iter_ancestors(*e).any(|a| props.contains(a))));
    let mut meshes = meshes.p1();
    for entity in targets {
        let Ok(mut mesh) = meshes.get_mut(entity) else { continue };
        // The light sits in the prop: its surfaces cast no shadow of it.
        commands.entity(entity).try_insert(NotShadowCaster);
        let id = mesh.0.id();
        let handle = if let Some(h) = cache.0.get(&id) {
            h.clone()
        } else {
            let Some(source) = materials.get(id) else { continue };
            // Flames and smoke keep their own (animated) materials.
            let Some(lit) = mw_assets::self_lit(source) else { continue };
            let h = materials.add(lit);
            cache.0.insert(id, h.clone());
            // A variant is its own variant (re-marked props keep theirs).
            cache.0.insert(h.id(), h.clone());
            h
        };
        if mesh.0 != handle {
            mesh.0 = handle;
        }
    }
}

/// Dark by the sky clock (exterior rule; interiors' ambient rule needs the
/// actor's cell: follow-up).
fn clock_dark(time: Option<&sf_sky_bevy::SkyTime>, config: Option<&sf_sky_bevy::SkyConfig>) -> bool {
    match (time, config) {
        (Some(t), Some(c)) => exterior_dark(t.0.hour(&c.0.calendar), DayCycle::default()),
        _ => false,
    }
}

/// The held override for an actor: the dark loadout while dark, else the
/// recipe's own (daylight) one.
fn held_override(rules: &MwCharacters, kind: &str, id: &sf_records::RecordId, dark: bool) -> Option<Vec<sf_character::HeldItem>> {
    if !dark {
        return None;
    }
    rules.actors().held(kind, id, HeldContext { dark, in_combat: false }).ok()
}

fn carry_lights_when_dark(
    rules: Res<HeldRules>,
    mut darkness: ResMut<HeldDarkness>,
    time: Option<Res<sf_sky_bevy::SkyTime>>,
    config: Option<Res<sf_sky_bevy::SkyConfig>>,
    mut characters: Query<&mut Character>,
) {
    let dark = darkness.forced.unwrap_or_else(|| clock_dark(time.as_deref(), config.as_deref()));
    let changed = darkness.applied != Some(dark);
    if changed {
        darkness.applied = Some(dark);
    }
    for mut character in &mut characters {
        if !changed && !character.is_added() {
            continue;
        }
        let Character::Record { kind, id, state } = &*character else { continue };
        let held = held_override(&rules.0, kind, id, dark);
        if state.held != held
            && let Character::Record { state, .. } = &mut *character
        {
            state.held = held;
        }
    }
}

/// The `LIGH` record's light on each new held light slot.
fn light_held_slots(
    mut commands: Commands,
    rules: Res<HeldRules>,
    calibration: Option<Res<LightCalibration>>,
    tuning: Res<HeldLightTuning>,
    slots: Query<(Entity, &HeldLightSlot), Added<HeldLightSlot>>,
    parents: Query<&ChildOf>,
    carriers: Query<(), With<HeldProps>>,
) {
    let calibration = calibration.map(|c| *c).unwrap_or_default();
    for (entity, slot) in &slots {
        let Some(info) = torch_info(rules.0.actors().db(), slot) else { continue };
        let mut light = calibration.point_light(&info);
        light.intensity *= tuning.intensity_scale(&calibration, info.radius);
        light.radius = tuning.source_radius_m;
        let mut e = commands.spawn((light, HeldTorch, BudgetedShadow { priority: TORCH_SHADOW_PRIORITY }, Transform::IDENTITY, Visibility::Inherited, ChildOf(entity)));
        if let Some(carrier) = parents.iter_ancestors(entity).find(|a| carriers.contains(*a)) {
            e.insert(HeldTorchOf(carrier));
        }
        let entity = e.id();
        if info.animation != LightAnimation::Steady {
            e.insert(AnimatedLight { intensity: light.intensity, animation: info.animation, seed: u32::try_from(entity.to_bits() & u64::from(u32::MAX)).unwrap_or(0) });
        }
    }
}

/// Keeps each carried light [`HeldLightTuning::clearance_m`] off its
/// carrier's axis (last frame's transforms: a frame of lag is invisible).
fn place_held_lights(
    tuning: Res<HeldLightTuning>,
    mut lights: Query<(&HeldTorchOf, &ChildOf, &mut Transform), With<HeldTorch>>,
    globals: Query<&GlobalTransform>,
) {
    for (carrier, slot, mut transform) in &mut lights {
        let (Ok(carrier), Ok(slot)) = (globals.get(carrier.0), globals.get(slot.parent())) else { continue };
        let affine = slot.affine();
        if affine.matrix3.determinant().abs() < 1e-9 {
            continue;
        }
        let at = held_light_position(slot.translation(), carrier.translation(), carrier.forward().into(), tuning.clearance_m);
        let local = affine.inverse().transform_point3(at);
        if transform.translation.distance_squared(local) > 1e-8 {
            transform.translation = local;
        }
    }
}

/// Where a carried light shines from: the flame, pushed horizontally away
/// from the carrier's vertical axis (through `axis`) until `clearance` off
/// it (straight ahead when the flame is on the axis).
pub fn held_light_position(flame: Vec3, axis: Vec3, forward: Vec3, clearance: f32) -> Vec3 {
    let mut off = flame - axis;
    off.y = 0.0;
    let d = off.length();
    if d >= clearance {
        return flame;
    }
    let out = if d > 1e-3 { off / d } else { Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero() };
    flame + out * (clearance - d)
}

/// A carried light's info: the record's, `Off by default` ignored (vanilla
/// carried torches have it and still shine), else the slot's color and radius.
fn torch_info(db: &RecordDb, slot: &HeldLightSlot) -> Option<mw_world::LightInfo> {
    let from_record = db.get::<Light>(&slot.item).and_then(|l| {
        let mut lit = l.clone();
        lit.data.flags.remove(LightFlags::OFF_BY_DEFAULT);
        mw_world::light_info(&lit)
    });
    from_record.or_else(|| {
        let l = &slot.light;
        (l.radius > 0.0).then(|| mw_world::LightInfo { color: l.color.to_array(), radius: l.radius, animation: LightAnimation::Steady, fire: false })
    })
}

/// Keeps the [`HeldLightBudget`] nearest held lights to the camera lit.
fn budget_held_lights(
    budget: Res<HeldLightBudget>,
    cameras: Query<&GlobalTransform, With<Camera3d>>,
    mut torches: Query<(Entity, &GlobalTransform, &ChildOf, &mut Visibility), With<HeldTorch>>,
    props: Query<&InheritedVisibility>,
) {
    let Some(eye) = cameras.iter().next().map(GlobalTransform::translation) else { return };
    // Ranked by distance among those whose prop shows (a parked torch takes no slot).
    let mut ranked: Vec<(f32, Entity)> = torches
        .iter()
        .filter(|(_, _, prop, _)| props.get(prop.parent()).is_ok_and(|v| v.get()))
        .map(|(e, t, _, _)| (t.translation().distance_squared(eye), e))
        .collect();
    ranked.sort_by(|a, b| a.0.total_cmp(&b.0));
    for (rank, (_, entity)) in ranked.iter().enumerate() {
        if let Ok((_, _, _, mut visibility)) = torches.get_mut(*entity) {
            let want = if rank < budget.max { Visibility::Inherited } else { Visibility::Hidden };
            visibility.set_if_neq(want);
        }
    }
}

/// The graph's `stance` from readiness and the main-hand weapon.
#[allow(clippy::type_complexity)]
fn feed_stance(
    rules: Res<HeldRules>,
    characters: Query<
        (&CharacterBody, Option<&CharacterReadiness>, Option<&HeldProps>),
        Or<(Changed<CharacterBody>, Changed<CharacterReadiness>, Changed<HeldProps>)>,
    >,
    mut animators: Query<&mut sf_anim_bevy::Animator>,
) {
    let db = rules.0.actors().db();
    for (body, readiness, props) in &characters {
        let readiness = readiness.map_or(Readiness::Sheathed, |r| r.0);
        let weapon = props
            .and_then(|p| p.props.iter().find(|p| p.item.hand == Hand::Main))
            .and_then(|p| db.get::<Weapon>(&p.item.item))
            .map(|w| w.data.weapon_type);
        if let Ok(mut animator) = animators.get_mut(body.body) {
            animator.set_float(mw_graphs::build::params::STANCE, Stance::of_weapon(readiness, weapon).value());
            let torch = props.is_some_and(|p| p.props.iter().any(|p| carries_light(&p.item, readiness)));
            animator.set_float(mw_graphs::build::params::TORCH, if torch { 1.0 } else { 0.0 });
        }
    }
}

/// Whether `item` is a light shown in the off hand at `readiness` (the
/// torch arm pose plays; hidden while a two-hander is drawn).
fn carries_light(item: &sf_character::HeldItem, readiness: Readiness) -> bool {
    item.hand == Hand::Off && item.light.is_some() && item.mount(readiness).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn torch_slots_light_with_the_record_or_the_slot() {
        let db = RecordDb::new();
        let slot = HeldLightSlot {
            item: sf_records::RecordId::new("torch"),
            light: sf_character::HeldLight { attach_node: None, color: Vec3::new(1.0, 0.6, 0.3), radius: 3.0 },
        };
        let info = torch_info(&db, &slot).expect("no record: the slot's own light");
        assert!((info.radius - 3.0).abs() < 1e-6 && info.animation == LightAnimation::Steady);
        let dark = HeldLightSlot { light: sf_character::HeldLight { radius: 0.0, ..slot.light.clone() }, ..slot };
        assert!(torch_info(&db, &dark).is_none());
    }

    #[test]
    fn carried_lights_keep_clear_of_the_carrier() {
        let axis = Vec3::new(10.0, 0.0, 5.0);
        let flame = axis + Vec3::new(0.3, 1.2, 0.0);
        let at = held_light_position(flame, axis, Vec3::Z, 0.7);
        assert!((at - (axis + Vec3::new(0.7, 1.2, 0.0))).length() < 1e-5, "pushed outward, height kept: {at}");
        let far = axis + Vec3::new(0.0, 1.0, 0.9);
        assert_eq!(held_light_position(far, axis, Vec3::Z, 0.7), far, "already clear");
        let on_axis = held_light_position(axis + Vec3::Y, axis, Vec3::NEG_Z, 0.7);
        assert!((on_axis - (axis + Vec3::new(0.0, 1.0, -0.7))).length() < 1e-5, "ahead when on the axis");
    }

    #[test]
    fn carried_lights_read_fully_lit_where_the_original_saturates() {
        let calibration = LightCalibration::default();
        let tuning = HeldLightTuning::default();
        let radius = 256.0 * 0.014_285_714; // the vanilla torch, in meters
        let scale = tuning.intensity_scale(&calibration, radius);
        assert!(scale > 0.05 && scale < 0.15, "{scale}");
        // Inverse-square brightness relative to "fully lit" for a white surface facing the light.
        let full = calibration.fully_lit_distance(radius);
        let at = |d: f32| scale * (full / d).powi(2);
        assert!((at(radius / 3.0) - 1.0).abs() < 1e-3, "fully lit at a third of the radius");
        assert!(at(0.5) < 8.0, "carrier at 0.5 m: {}", at(0.5));
        assert!(at(1.5) > 0.5, "ground under the torch: {}", at(1.5));
    }

    #[test]
    fn a_light_models_own_surfaces_glow_at_fully_lit_and_take_no_light() {
        let opaque = StandardMaterial { base_color: Color::linear_rgb(0.5, 0.5, 0.5), ..StandardMaterial::default() };
        let lit = mw_assets::self_lit(&opaque).expect("opaque surfaces are self-lit");
        assert!(lit.base_color.to_linear().red.abs() < 1e-6, "no lit base");
        assert!(lit.reflectance == 0.0 && (lit.emissive_exposure_weight - 1.0).abs() < 1e-6);
        assert!((lit.emissive.red - 0.5 * mw_assets::emissive_nits()).abs() < 1e-3, "fully lit at the scene exposure");
        let flame = StandardMaterial { alpha_mode: AlphaMode::Add, ..StandardMaterial::default() };
        assert!(mw_assets::self_lit(&flame).is_none(), "flames keep their material");
    }

    #[test]
    fn torch_arm_follows_a_shown_off_hand_light() {
        let mount = sf_character::Mount::at("Shield Bone");
        let light = sf_character::HeldLight { attach_node: None, color: Vec3::ONE, radius: 3.0 };
        let mut torch = sf_character::HeldItem::always(sf_records::RecordId::new("torch"), sf_vfs::VfsPath::new("l/t.nif").unwrap(), Hand::Off, mount);
        torch.light = Some(light);
        assert!(carries_light(&torch, Readiness::Sheathed) && carries_light(&torch, Readiness::Drawn));
        torch.drawn = None; // hidden while a two-hander is drawn
        assert!(!carries_light(&torch, Readiness::Drawn));
        torch.light = None;
        assert!(!carries_light(&torch, Readiness::Sheathed), "a shield");
    }

    #[test]
    fn darkness_follows_the_clock() {
        let config = sf_sky_bevy::SkyConfig::default();
        let at = |hour: f64| {
            let mut t = sf_sky_bevy::SkyTime::default();
            t.0.set_hour(hour, &config.0.calendar);
            clock_dark(Some(&t), Some(&config))
        };
        assert!(at(21.0) && at(3.0), "torches at night");
        assert!(!at(12.0) && !at(18.5), "none by day or at dusk before 19:00");
        assert!(!clock_dark(None, None), "no clock: day");
    }
}
