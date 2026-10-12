//! Which ARMO/CLOT items an NPC wears from its inventory, before gameplay
//! (`sf_rpg`) owns equipment: the original engine's auto-equip rules, as
//! `OpenMW` documents them (`InventoryStore::autoEquipArmor`,
//! `Armor::getEffectiveArmorRating`, `autoCalculateSkills`).
//!
//! Armor first, then clothing, each in inventory order. Armor is skipped when
//! its effective rating is no better than the (integer) unarmored rating. In a taken slot, armor
//! keeps the lower armor type (a gauntlet beats a bracer) or, for the same
//! type, the higher rating; armor replaces clothing, clothing never replaces
//! armor, clothing against clothing keeps the more valuable. Beast races
//! refuse items with a head or foot part. Shields and weapons are held items
//! (not here); leveled-list inventory entries are skipped.

use sf_records::{RecordDb, RecordId};
use tes3::esp::{
    Armor, ArmorType, BipedObjectType, Class, Clothing, ClothingType, GameSetting, GameSettingValue, Npc, NpcFlags, Race, Skill, SkillId,
};

/// Equipment slots (the original inventory's), in the order the body parts
/// are processed: on a part-priority tie the earlier slot wins.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EquipSlot {
    Robe,
    Skirt,
    Helmet,
    Cuirass,
    Greaves,
    LeftPauldron,
    RightPauldron,
    Boots,
    LeftGauntlet,
    RightGauntlet,
    Shirt,
    Pants,
    CarriedLeft,
    Belt,
    Amulet,
    LeftRing,
    RightRing,
}

const SLOT_COUNT: usize = 17;

/// An equippable item record.
#[derive(Clone, Copy, Debug)]
pub enum Item<'a> {
    Armor(&'a Armor),
    Clothing(&'a Clothing),
}

impl<'a> Item<'a> {
    pub fn find(db: &'a RecordDb, id: &RecordId) -> Option<Self> {
        db.get::<Armor>(id).map(Item::Armor).or_else(|| db.get::<Clothing>(id).map(Item::Clothing))
    }

    pub fn id(&self) -> &'a str {
        match self {
            Item::Armor(a) => &a.id,
            Item::Clothing(c) => &c.id,
        }
    }

    pub fn parts(&self) -> &'a [tes3::esp::BipedObject] {
        match self {
            Item::Armor(a) => &a.biped_objects,
            Item::Clothing(c) => &c.biped_objects,
        }
    }

    /// The slots the item can go in (rings: either hand).
    pub fn slots(&self) -> &'static [EquipSlot] {
        use EquipSlot as S;
        match self {
            Item::Armor(a) => match a.data.armor_type {
                ArmorType::Helmet => &[S::Helmet],
                ArmorType::Cuirass => &[S::Cuirass],
                ArmorType::LeftPauldron => &[S::LeftPauldron],
                ArmorType::RightPauldron => &[S::RightPauldron],
                ArmorType::Greaves => &[S::Greaves],
                ArmorType::Boots => &[S::Boots],
                ArmorType::LeftGauntlet | ArmorType::LeftBracer => &[S::LeftGauntlet],
                ArmorType::RightGauntlet | ArmorType::RightBracer => &[S::RightGauntlet],
                ArmorType::Shield => &[S::CarriedLeft],
            },
            Item::Clothing(c) => match c.data.clothing_type {
                ClothingType::Pants => &[S::Pants],
                ClothingType::Shoes => &[S::Boots],
                ClothingType::Shirt => &[S::Shirt],
                ClothingType::Belt => &[S::Belt],
                ClothingType::Robe => &[S::Robe],
                ClothingType::RightGlove => &[S::RightGauntlet],
                ClothingType::LeftGlove => &[S::LeftGauntlet],
                ClothingType::Skirt => &[S::Skirt],
                ClothingType::Ring => &[S::LeftRing, S::RightRing],
                ClothingType::Amulet => &[S::Amulet],
            },
        }
    }

    /// Whether a beast race can wear it (no head or foot part).
    pub fn fits_beast(&self) -> bool {
        !self.parts().iter().any(|p| {
            matches!(p.biped_object_type, BipedObjectType::Head | BipedObjectType::LeftFoot | BipedObjectType::RightFoot)
        })
    }
}

/// A float game setting (integers converted), or `fallback`.
#[allow(clippy::cast_precision_loss)]
pub fn gmst(db: &RecordDb, id: &str, fallback: f32) -> f32 {
    match db.get::<GameSetting>(&RecordId::new(id)).map(|g| &g.value) {
        Some(GameSettingValue::Float(v)) => *v,
        Some(GameSettingValue::Integer(v)) => *v as f32,
        _ => fallback,
    }
}

/// The 27 skills of an NPC: its stored skills, or the auto-calculated ones
/// when the record asks for them (or stores none).
pub fn npc_skills(db: &RecordDb, npc: &Npc) -> [f32; 27] {
    match &npc.data.stats {
        Some(stats) if !npc.npc_flags.contains(NpcFlags::AUTO_CALCULATE) => stats.skills.map(f32::from),
        _ => auto_skills(db, npc),
    }
}

/// Auto-calculated skills: 5, +25 major / +10 minor, + race bonus, +5 in the
/// class specialization, + (level − 1) × (1 for class skills, else 0.1; +0.5 in
/// the specialization), rounded, at most 100.
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
pub fn auto_skills(db: &RecordDb, npc: &Npc) -> [f32; 27] {
    let class = db.get::<Class>(&RecordId::new(&npc.class));
    let race = db.get::<Race>(&RecordId::new(&npc.race));
    let mut spec_of = [-2_i32; 27];
    for (_, skill) in db.iter::<Skill>() {
        if let Some(i) = skill_index(skill.skill_id) {
            spec_of[i] = skill.data.specialization;
        }
    }
    let (majors, minors, spec) = class.map_or(([SkillId::None; 5], [SkillId::None; 5], -1), |c| {
        let d = &c.data;
        ([d.major1, d.major2, d.major3, d.major4, d.major5], [d.minor1, d.minor2, d.minor3, d.minor4, d.minor5], d.specialization as i32)
    });
    let bonuses = race.map(|r| {
        let b = &r.data.skill_bonuses;
        [
            (b.skill_0, b.bonus_0),
            (b.skill_1, b.bonus_1),
            (b.skill_2, b.bonus_2),
            (b.skill_3, b.bonus_3),
            (b.skill_4, b.bonus_4),
            (b.skill_5, b.bonus_5),
            (b.skill_6, b.bonus_6),
        ]
    });
    let level = f32::from(npc.data.level.max(1) - 1);
    let mut out = [0.0; 27];
    for (i, value) in out.iter_mut().enumerate() {
        let is = |s: &SkillId| skill_index(*s) == Some(i);
        let mut base = 5.0;
        if majors.iter().any(is) {
            base += 25.0;
        } else if minors.iter().any(is) {
            base += 10.0;
        }
        #[allow(clippy::cast_precision_loss)]
        let race_bonus = bonuses.and_then(|b| b.iter().find(|(s, _)| is(s)).map(|(_, v)| *v as f32)).unwrap_or(0.0);
        let class_skill = majors.iter().chain(&minors).any(is);
        let in_spec = spec >= 0 && spec_of[i] == spec;
        let per_level = if class_skill { 1.0 } else { 0.1 } + if in_spec { 0.5 } else { 0.0 };
        let spec_bonus = if in_spec { 5.0 } else { 0.0 };
        *value = (base + race_bonus + spec_bonus + level * per_level).round().min(100.0);
    }
    out
}

fn skill_index(s: SkillId) -> Option<usize> {
    usize::try_from(s as i32).ok().filter(|&i| i < 27)
}

/// Armor weight classes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeightClass {
    Light,
    Medium,
    Heavy,
}

/// The rules' game settings, read once.
#[derive(Clone, Copy, Debug)]
pub struct EquipRules {
    pub unarmored_base1: f32,
    pub unarmored_base2: f32,
    pub base_armor_skill: f32,
    pub light_max_mod: f32,
    pub med_max_mod: f32,
    /// `iHelmWeight`, `iCuirassWeight`, `iPauldronWeight`, `iGreavesWeight`,
    /// `iBootsWeight`, `iGauntletWeight`, `iShieldWeight`.
    pub weights: [f32; 7],
}

impl EquipRules {
    pub fn from_db(db: &RecordDb) -> Self {
        let w = |id, v| gmst(db, id, v).floor();
        Self {
            unarmored_base1: gmst(db, "fUnarmoredBase1", 0.1),
            unarmored_base2: gmst(db, "fUnarmoredBase2", 0.065),
            base_armor_skill: gmst(db, "iBaseArmorSkill", 30.0),
            light_max_mod: gmst(db, "fLightMaxMod", 0.6),
            med_max_mod: gmst(db, "fMedMaxMod", 0.9),
            weights: [
                w("iHelmWeight", 5.0),
                w("iCuirassWeight", 30.0),
                w("iPauldronWeight", 10.0),
                w("iGreavesWeight", 15.0),
                w("iBootsWeight", 20.0),
                w("iGauntletWeight", 5.0),
                w("iShieldWeight", 15.0),
            ],
        }
    }

    pub fn weight_class(&self, armor: &Armor) -> WeightClass {
        let base = self.weights[match armor.data.armor_type {
            ArmorType::Helmet => 0,
            ArmorType::Cuirass => 1,
            ArmorType::LeftPauldron | ArmorType::RightPauldron => 2,
            ArmorType::Greaves => 3,
            ArmorType::Boots => 4,
            ArmorType::LeftGauntlet | ArmorType::RightGauntlet | ArmorType::LeftBracer | ArmorType::RightBracer => 5,
            ArmorType::Shield => 6,
        }];
        let eps = 0.0005;
        if armor.data.weight <= base * self.light_max_mod + eps {
            WeightClass::Light
        } else if armor.data.weight <= base * self.med_max_mod + eps {
            WeightClass::Medium
        } else {
            WeightClass::Heavy
        }
    }

    /// Armor rating scaled by the wearer's skill in its weight class.
    #[allow(clippy::cast_precision_loss)]
    pub fn effective_rating(&self, armor: &Armor, skills: &[f32; 27]) -> f32 {
        let rating = armor.data.armor_rating as f32;
        if armor.data.weight == 0.0 {
            return rating;
        }
        let skill = skills[match self.weight_class(armor) {
            WeightClass::Light => SkillId::LightArmor,
            WeightClass::Medium => SkillId::MediumArmor,
            WeightClass::Heavy => SkillId::HeavyArmor,
        } as usize];
        rating * skill / self.base_armor_skill
    }

    /// The rating of wearing nothing.
    pub fn unarmored_rating(&self, skills: &[f32; 27]) -> f32 {
        let s = skills[SkillId::Unarmored as usize];
        (self.unarmored_base1 * s) * (self.unarmored_base2 * s)
    }
}

/// The items an NPC equips from `items` (ids, inventory order): one per
/// slot, in [`EquipSlot`] order. Shields are left to the held-item rules.
pub fn auto_equip<'a>(
    db: &'a RecordDb,
    rules: &EquipRules,
    items: impl IntoIterator<Item = &'a str>,
    skills: &[f32; 27],
    beast: bool,
) -> Vec<(EquipSlot, Item<'a>)> {
    let items: Vec<Item<'a>> = items.into_iter().filter_map(|id| Item::find(db, &RecordId::new(id))).collect();
    // The original truncates the unarmored rating to an integer here.
    let unarmored = rules.unarmored_rating(skills).trunc().max(0.0);
    let mut slots: [Option<Item<'a>>; SLOT_COUNT] = [None; SLOT_COUNT];
    let armor = items.iter().filter(|i| matches!(i, Item::Armor(_)));
    let clothing = items.iter().filter(|i| matches!(i, Item::Clothing(_)));
    for item in armor.chain(clothing) {
        if beast && !item.fits_beast() {
            continue;
        }
        if let Item::Armor(a) = item
            && (a.data.armor_type == ArmorType::Shield || rules.effective_rating(a, skills) <= unarmored) {
                continue;
            }
        for &slot in item.slots() {
            let current = &mut slots[slot as usize];
            let keep_old = match (*current, item) {
                (None, _) | (Some(Item::Clothing(_)), Item::Armor(_)) => false,
                (Some(Item::Armor(old)), Item::Armor(new)) => {
                    let (o, n) = (old.data.armor_type as u32, new.data.armor_type as u32);
                    o < n || (o == n && rules.effective_rating(old, skills) >= rules.effective_rating(new, skills))
                }
                (Some(Item::Clothing(old)), Item::Clothing(new)) => old.data.value >= new.data.value,
                (Some(Item::Armor(_)), Item::Clothing(_)) => true,
            };
            if !keep_old {
                *current = Some(*item);
                break;
            }
        }
    }
    slots
        .into_iter()
        .enumerate()
        .filter_map(|(i, item)| item.map(|item| (ALL_SLOTS[i], item)))
        .collect()
}

const ALL_SLOTS: [EquipSlot; SLOT_COUNT] = {
    use EquipSlot as S;
    [
        S::Robe,
        S::Skirt,
        S::Helmet,
        S::Cuirass,
        S::Greaves,
        S::LeftPauldron,
        S::RightPauldron,
        S::Boots,
        S::LeftGauntlet,
        S::RightGauntlet,
        S::Shirt,
        S::Pants,
        S::CarriedLeft,
        S::Belt,
        S::Amulet,
        S::LeftRing,
        S::RightRing,
    ]
};
