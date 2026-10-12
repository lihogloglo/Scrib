//! Morrowind's held-item rules on `sf_character_bevy`'s props
//! (actors-adapter.md "Held items as built"; character.md "Held items").
//!
//! - **Torches when dark.** Actors re-ask `mw_actors` for their held items
//!   when darkness changes (`held::exterior_dark` on the sky clock: 19:00 to
//!   6:00), and when they spawn in the dark: `CharacterState::held` then
//!   names the torch, and the framework swaps only the props.
//! - **Torch lights.** A prop's `HeldLightSlot` gets the `LIGH` record's
//!   point light: the placed lights' calibration (`LightCalibration`) and
//!   flicker (`AnimatedLight`), no shadows. [`HeldLightBudget`] keeps the
//!   nearest few to the camera lit (each extra clustered light costs every
//!   fragment its range covers).
//! - **Stance.** The graph's `stance` parameter follows the readiness and
//!   the main-hand weapon (`mw_graphs::Stance::of_weapon`).

use std::sync::Arc;

use bevy::prelude::*;
use mw_actors::held::{DayCycle, HeldContext, exterior_dark};
use mw_assets::LightCalibration;
use mw_assets::lights::AnimatedLight;
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

/// Marks the point light on a held item (a child of its `HeldLightSlot`).
#[derive(Component, Clone, Copy, Debug)]
pub struct HeldTorch;

/// The character carrying a held light.
#[derive(Component, Clone, Copy, Debug)]
struct HeldTorchOf(Entity);

/// Near field of carried lights. Morrowind's lights saturate inside a third
/// of their radius (`radius / (3 d)` clamped to 1); ours are physical
/// (inverse-square, calibrated at the radius: `LightCalibration`), so a
/// 3.66 m torch 0.25 m from its carrier lit the body about 24x past "fully lit".
/// Bevy has no per-light near clamp (Frostbite's `max(d², r²)`, Unreal's
/// source radius), so the carried light keeps `clearance_m` horizontally
/// from the carrier's vertical axis (pushed outward from the flame) and
/// shines at `intensity_scale`: the body lands near 2x "fully lit", the
/// pool on the ground near half of it.
#[derive(Resource, Clone, Copy, Debug)]
pub struct HeldLightTuning {
    pub clearance_m: f32,
    pub intensity_scale: f32,
}

impl Default for HeldLightTuning {
    fn default() -> Self {
        Self { clearance_m: 0.7, intensity_scale: 0.7 }
    }
}

#[derive(Resource, Clone)]
struct HeldRules(Arc<MwCharacters>);

pub fn add(app: &mut App, content: Arc<MwCharacters>) {
    app.insert_resource(HeldRules(content))
        .init_resource::<HeldDarkness>()
        .init_resource::<HeldLightBudget>()
        .init_resource::<HeldLightTuning>()
        .add_systems(Update, carry_lights_when_dark.before(CharacterSystems))
        .add_systems(Update, (light_held_slots, place_held_lights, budget_held_lights, feed_stance).chain().after(CharacterSystems));
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
        light.intensity *= tuning.intensity_scale;
        let mut e = commands.spawn((light, HeldTorch, Transform::IDENTITY, Visibility::Inherited, ChildOf(entity)));
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
        }
    }
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
