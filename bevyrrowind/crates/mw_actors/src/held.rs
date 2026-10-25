//! Held items: which weapon, shield and light an actor carries, where they
//! attach, and when they show. The original engine's rules as `OpenMW`
//! documents them (GPL reference; this crate is GPL):
//! `InventoryStore::autoEquipWeapon`/`autoEquipShield`,
//! `Actors::updateEquippedLight`, `World::isDark`, `NpcAnimation` (part list,
//! `showCarriedLeft`, `getShieldMesh`), `Animation::updateCarriedLeftVisible`
//! and `SceneUtil::attach` (`BoneOffset`).
//!
//! - **Weapon** (right hand, `Weapon Bone`): weapon skills are tried from the
//!   highest (skills at 0 never), ties in the order long blade, axe, spear,
//!   short blade, marksman, blunt. Within a skill the weapon with the highest
//!   maximum chop, slash or thrust damage wins (ties: the later item). A bow
//!   or crossbow needs its ammunition in the inventory, else the next skill
//!   is tried. Ammunition: the arrow (bolt) with the highest maximum chop
//!   damage (ties: the later). Thrown weapons need none.
//! - **Shield** (`Shield Bone`, the off hand): not with a two-handed weapon
//!   (bows and crossbows are two-handed); the highest condition wins (ties:
//!   the earlier). Its model is the shield `BODY` of its part list (female,
//!   else male), else the item's ground model.
//! - **Light** (`Shield Bone` too: it takes the carried-left slot): only when
//!   it is dark ([`is_dark`]) and the actor carries a `LIGH` that can be
//!   carried (the first in inventory order). Out of combat it replaces the
//!   shield; in combat a shield keeps the slot. With a two-handed weapon the
//!   original equips it anyway (the left slot is free) and hides it
//!   while the two-handed weapon is drawn, as it hides a shield.
//! - **Visibility**: the original shows no sheathed weapons: a weapon shows
//!   only while drawn ([`sf_character::Readiness::Drawn`]); there are no
//!   quivers or scabbards (the arrow appears at the bow's `ArrowBone` during
//!   the attack, an animation event, not a held item). The carried-left item
//!   always shows, except while a two-handed weapon is drawn. `OpenMW`'s
//!   optional weapon and shield sheathing (off by default) needs mod content
//!   (`Bip01 <weapon type>` bones, `_sh` models): not implemented.
//! - **Offsets**: the model's root transform, as for rigid body parts, under
//!   a `BoneOffset` node's translation turned −90° about X when the model has
//!   one (`OpenMW`'s `SceneUtil::attach`). Neither bone is a `Left` bone, so
//!   nothing is mirrored.
//!
//! Creatures hold items only with the `Weapon and Shield` flag; their weapon
//! skills all count as their combat skill, so the skill order decides.

use glam::{Affine3A, Quat, Vec3};
use sf_character::{Hand, HeldItem, HeldLight, Mount};
use sf_records::{RecordDb, RecordId};
use tes3::esp::{Armor, ArmorType, BipedObjectType, Bodypart, Cell, CellFlags, LeveledItem, Light, LightFlags, SkillId, Weapon, WeaponType};

/// Attach bones of the humanoid skeletons (and of weapon-carrying creatures).
pub mod bone {
    /// The weapon in the right hand.
    pub const WEAPON: &str = "Weapon Bone";
    /// The shield on the left forearm; also the carried light.
    pub const SHIELD: &str = "Shield Bone";
}

/// Node of a light's model the light sits at (the flame).
pub const ATTACH_LIGHT: &str = "AttachLight";

/// Weapon skills in the original's tie-break order.
const WEAPON_SKILLS: [SkillId; 6] = [SkillId::LongBlade, SkillId::Axe, SkillId::Spear, SkillId::ShortBlade, SkillId::Marksman, SkillId::BluntWeapon];

/// The skill a weapon type trains (ammunition: marksman).
pub fn weapon_skill(t: WeaponType) -> SkillId {
    use WeaponType as W;
    match t {
        W::ShortBladeOneHand => SkillId::ShortBlade,
        W::LongBladeOneHand | W::LongBladeTwoClose => SkillId::LongBlade,
        W::BluntOneHand | W::BluntTwoClose | W::BluntTwoWide => SkillId::BluntWeapon,
        W::SpearTwoWide => SkillId::Spear,
        W::AxeOneHand | W::AxeTwoHand => SkillId::Axe,
        W::MarksmanBow | W::MarksmanCrossbow | W::MarksmanThrown | W::Arrow | W::Bolt => SkillId::Marksman,
    }
}

/// Whether a weapon type takes both hands (bows and crossbows do).
pub fn two_handed(t: WeaponType) -> bool {
    use WeaponType as W;
    matches!(t, W::LongBladeTwoClose | W::BluntTwoClose | W::BluntTwoWide | W::SpearTwoWide | W::AxeTwoHand | W::MarksmanBow | W::MarksmanCrossbow)
}

/// Arrows and bolts: fired, never wielded.
pub fn is_ammo(t: WeaponType) -> bool {
    matches!(t, WeaponType::Arrow | WeaponType::Bolt)
}

/// The ammunition a ranged weapon needs.
pub fn ammo_for(t: WeaponType) -> Option<WeaponType> {
    match t {
        WeaponType::MarksmanBow => Some(WeaponType::Arrow),
        WeaponType::MarksmanCrossbow => Some(WeaponType::Bolt),
        _ => None,
    }
}

/// What the light rule needs to know about the moment.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HeldContext {
    /// Dark where the actor is ([`is_dark`]): actors carrying a light hold it.
    pub dark: bool,
    pub in_combat: bool,
}

/// The item in the carried-left slot.
#[derive(Clone, Copy, Debug)]
pub enum CarriedLeft<'a> {
    Shield(&'a Armor),
    Light(&'a Light),
}

/// The held items an actor carries.
#[derive(Clone, Copy, Debug, Default)]
pub struct Loadout<'a> {
    pub weapon: Option<&'a Weapon>,
    /// Ammunition for a bow or crossbow (not shown: no quivers).
    pub ammo: Option<&'a Weapon>,
    pub carried_left: Option<CarriedLeft<'a>>,
}

impl Loadout<'_> {
    pub fn weapon_type(&self) -> Option<WeaponType> {
        self.weapon.map(|w| w.data.weapon_type)
    }

    /// Whether the weapon takes both hands.
    pub fn two_handed(&self) -> bool {
        self.weapon_type().is_some_and(two_handed)
    }

    pub fn shield(&self) -> Option<&Armor> {
        match self.carried_left {
            Some(CarriedLeft::Shield(a)) => Some(a),
            _ => None,
        }
    }

    pub fn light(&self) -> Option<&Light> {
        match self.carried_left {
            Some(CarriedLeft::Light(l)) => Some(l),
            _ => None,
        }
    }
}

/// The weapon an actor wields and its ammunition, from `items` (ids,
/// inventory order) and its 27 skills.
pub fn best_weapon<'a>(db: &'a RecordDb, items: &[&str], skills: &[f32; 27]) -> (Option<&'a Weapon>, Option<&'a Weapon>) {
    let weapons: Vec<&Weapon> = items.iter().filter_map(|id| db.get::<Weapon>(&RecordId::new(id))).collect();
    let best_ammo = |kind: WeaponType| {
        let mut best: Option<&Weapon> = None;
        for w in weapons.iter().filter(|w| w.data.weapon_type == kind) {
            if best.is_none_or(|b| w.data.chop_max >= b.data.chop_max) {
                best = Some(w);
            }
        }
        best
    };
    let (arrow, bolt) = (best_ammo(WeaponType::Arrow), best_ammo(WeaponType::Bolt));
    let mut visited = [false; WEAPON_SKILLS.len()];
    loop {
        // The highest unvisited skill above 0 (ties: the earlier in the order).
        let mut pick: Option<(usize, f32)> = None;
        for (j, s) in WEAPON_SKILLS.iter().enumerate() {
            let value = skills[*s as usize];
            if !visited[j] && value > pick.map_or(0.0, |p| p.1) {
                pick = Some((j, value));
            }
        }
        let Some((j, _)) = pick else { return (None, None) };
        visited[j] = true;
        let mut max = 0_u8;
        let mut weapon: Option<&Weapon> = None;
        for w in weapons.iter().filter(|w| !is_ammo(w.data.weapon_type) && weapon_skill(w.data.weapon_type) == WEAPON_SKILLS[j]) {
            for damage in [w.data.chop_max, w.data.slash_max, w.data.thrust_max] {
                if damage >= max {
                    max = damage;
                    weapon = Some(w);
                }
            }
        }
        let Some(weapon) = weapon else { continue };
        match ammo_for(weapon.data.weapon_type) {
            None => return (Some(weapon), None),
            Some(WeaponType::Arrow) if arrow.is_some() => return (Some(weapon), arrow),
            Some(WeaponType::Bolt) if bolt.is_some() => return (Some(weapon), bolt),
            Some(_) => {}
        }
    }
}

/// The shield an actor carries: the one in best condition (ties: the earlier).
pub fn best_shield<'a>(db: &'a RecordDb, items: &[&str]) -> Option<&'a Armor> {
    let mut best: Option<&Armor> = None;
    for a in items.iter().filter_map(|id| db.get::<Armor>(&RecordId::new(id))) {
        if a.data.armor_type == ArmorType::Shield && a.data.health > 0 && best.is_none_or(|b| a.data.health > b.data.health) {
            best = Some(a);
        }
    }
    best
}

/// The first light in `items` that can be carried.
pub fn carried_light<'a>(db: &'a RecordDb, items: &[&str]) -> Option<&'a Light> {
    items.iter().filter_map(|id| db.get::<Light>(&RecordId::new(id))).find(|l| l.data.flags.contains(LightFlags::CAN_CARRY))
}

/// The held items of an actor with `items` (ids, inventory order) and skills.
pub fn auto_held<'a>(db: &'a RecordDb, items: &[&str], skills: &[f32; 27], ctx: HeldContext) -> Loadout<'a> {
    let (weapon, ammo) = best_weapon(db, items, skills);
    let two_hands = weapon.is_some_and(|w| two_handed(w.data.weapon_type));
    let shield = if two_hands { None } else { best_shield(db, items) };
    let light = ctx.dark.then(|| carried_light(db, items)).flatten();
    let carried_left = match (shield, light) {
        (Some(s), Some(_)) if ctx.in_combat => Some(CarriedLeft::Shield(s)),
        (_, Some(l)) => Some(CarriedLeft::Light(l)),
        (Some(s), None) => Some(CarriedLeft::Shield(s)),
        (None, None) => None,
    };
    Loadout { weapon, ammo, carried_left }
}

/// Day cycle hours from `Morrowind.ini` `[Weather]`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DayCycle {
    /// `Sunrise Time`.
    pub sunrise: f32,
    /// `Sunset Time` + `Sunset Duration`.
    pub night_start: f32,
}

impl Default for DayCycle {
    /// Vanilla: sunrise 6, sunset 18 lasting 2 hours.
    fn default() -> Self {
        Self { sunrise: 6.0, night_start: 20.0 }
    }
}

/// Dark outside: before sunrise, or from an hour before night starts
/// (19:00 to 6:00 in vanilla).
pub fn exterior_dark(hour: f32, cycle: DayCycle) -> bool {
    hour < cycle.sunrise || hour > cycle.night_start - 1.0
}

/// Dark inside: the cell's ambient color sums (R + G + B) to at most 201,
/// unless resting is illegal there (shops, homes, temples stay lit).
pub fn interior_dark(ambient: [u8; 3], resting_illegal: bool) -> bool {
    !resting_illegal && ambient.iter().map(|&c| u32::from(c)).sum::<u32>() <= 201
}

/// Whether actors in `cell` light their torches at `hour` (game hours).
/// Interiors that behave like exteriors follow the clock (our choice: the
/// original tests the interior rule there too).
pub fn is_dark(cell: &Cell, hour: f32, cycle: DayCycle) -> bool {
    let flags = cell.data.flags;
    if !flags.contains(CellFlags::IS_INTERIOR) || flags.contains(CellFlags::BEHAVES_LIKE_EXTERIOR) {
        return exterior_dark(hour, cycle);
    }
    let [r, g, b, _] = cell.atmosphere_data.as_ref().map_or([0; 4], |a| a.ambient_color);
    interior_dark([r, g, b], flags.contains(CellFlags::RESTING_IS_ILLEGAL))
}

/// The model of a shield as held: its shield `BODY` (female first for a
/// female body, else male), else its ground model (`mesh` field).
pub fn shield_mesh<'a>(db: &'a RecordDb, shield: &'a Armor, female: bool) -> &'a str {
    let part = shield.biped_objects.iter().find(|o| o.biped_object_type == BipedObjectType::Shield);
    let body = part.and_then(|o| {
        let female_part = female.then_some(&o.female_bodypart);
        female_part.into_iter().chain([&o.male_bodypart]).filter(|n| !n.is_empty()).find_map(|n| db.get::<Bodypart>(&RecordId::new(n)))
    });
    body.map_or(shield.mesh.as_str(), |b| b.mesh.as_str())
}

/// A held model's offset under its bone: its root transform, under a
/// `BoneOffset` node's position turned −90° about X when it has one.
pub fn mount_offset(root: Affine3A, bone_offset: Option<Vec3>) -> Affine3A {
    match bone_offset {
        Some(p) => Affine3A::from_rotation_translation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2), p) * root,
        None => root,
    }
}

/// A loadout as framework held items, given each model's VFS path and
/// offset (`None`: no model, the item is skipped).
pub fn held_items(
    loadout: &Loadout<'_>,
    mut model: impl FnMut(&str) -> Option<(sf_vfs::VfsPath, Affine3A)>,
    shield_model: Option<&str>,
) -> Vec<HeldItem> {
    let mut out = Vec::new();
    if let Some(w) = loadout.weapon
        && let Some((path, offset)) = model(&w.mesh)
    {
        out.push(HeldItem {
            item: RecordId::new(&w.id),
            model: path,
            hand: Hand::Main,
            drawn: Some(Mount { socket: bone::WEAPON.into(), offset }),
            sheathed: None,
            light: None,
        });
    }
    let left = match loadout.carried_left {
        Some(CarriedLeft::Shield(s)) => model(shield_model.unwrap_or(&s.mesh)).map(|m| (RecordId::new(&s.id), m, None)),
        Some(CarriedLeft::Light(l)) => {
            // `Off by default` is for placed lights: carried torches have it
            // (vanilla `torch_infinite_time`) and still shine.
            let mut lit = l.clone();
            lit.data.flags.remove(LightFlags::OFF_BY_DEFAULT);
            let light = mw_world::light_info(&lit).map(|i| HeldLight {
                attach_node: Some(ATTACH_LIGHT.into()),
                color: Vec3::from_array(i.color),
                radius: i.radius,
            });
            model(&l.mesh).map(|m| (RecordId::new(&l.id), m, light))
        }
        None => None,
    };
    if let Some((item, (path, offset), light)) = left {
        let mount = Mount { socket: bone::SHIELD.into(), offset };
        out.push(HeldItem {
            item,
            model: path,
            hand: Hand::Off,
            // Hidden while a two-handed weapon is drawn.
            drawn: (!loadout.two_handed()).then(|| mount.clone()),
            sheathed: Some(mount),
            light,
        });
    }
    out
}

/// **Scaffold** (`leveled item pick`, like [`crate::leveled_pick`] for
/// creatures): an inventory entry naming a leveled item list (`LEVI`) holds
/// the list's lowest-level item (ties: the first), following nested lists,
/// ignoring `chance_none`; other ids pass through. Used for held items only
/// (guards' `Random Weapon`/`Random Shield` lists); worn items still skip
/// lists. Removal: `sf_rpg` resolves inventories when an actor is
/// instantiated (player level, a seeded roll) and hands the adapter items.
pub fn leveled_item_pick<'a>(db: &'a RecordDb, id: &'a str) -> Option<&'a str> {
    let mut id = id;
    for _ in 0..8 {
        let Some(list) = db.get::<LeveledItem>(&RecordId::new(id)) else { return Some(id) };
        id = list.items.iter().min_by_key(|(_, level)| *level)?.0.as_str();
    }
    None
}
