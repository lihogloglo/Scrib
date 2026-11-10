//! Magic effects on the player, NPCs and creatures (effects.md E4, effects-adapter.md "Crates"):
//! `sf_effects_bevy` with Morrowind's MGEF table (`mw_effects`).
//!
//! - **Setup** ([`add`], called by [`crate::stats::add`]: effects need the sheets): the MGEF
//!   records as `EffectTableRes` (`MwEffects`; A1 defs only, the other indices land as
//!   `Rejected { UnknownEffect }`), the modern rules (`EffectRules::standard`), every SPEL
//!   record as a source (`mw_effects::source_table`, `SourceTableRes`, adapter A3), and the
//!   `MwEffects` themselves as [`MwEffectsRes`] (potions map their ALCH on use, `item_use`).
//! - **Attach**: `Effects` sits beside every `Stats` the stats glue attaches (the player
//!   character, NPCs, creatures), same owner; see [`crate::stats`]. A new character learns its
//!   start spells ([`EffectLoad::learn`], lists from `mw_effects::start_spells`): abilities
//!   start their instance on the Permanent layer and their modifiers go on the sheet before it
//!   settles, so a new character's pools fill to the fortified maximum (`OpenMW` `buildPlayer`
//!   sets each current to the modified value after the spells).
//! - **Clock** ([`feed_clock`]): `EffectClock` follows the dialogue's game clock (`DaysPassed`,
//!   `GameHour`, [`crate::dialogue::Rpg::game_time`]) every fixed tick, until ROADMAP 1.17's
//!   single host clock replaces both. Durations count fixed ticks, not this clock.
//! - **Saves** (effects.md "Saves"): the player's in its `CharacterSave` ([`player_effects_save`],
//!   restored with the sheet by `stats::restore_player_sheet`), an NPC's or creature's in the
//!   placement's [`EFFECTS_BLOB`] beside its stats blob, written on unload and into the save's
//!   overlay copy, read when the placement spawns again ([`EffectLoad`]); timers resume where
//!   they stopped (no time passes while unloaded). Either way the restored set re-registers its
//!   modifiers on the loaded sheet before it settles (stats.md S8 load order): the sheet and its
//!   effects come back in one step, at `CHARACTERS` order for the player, at spawn for NPCs.

use std::sync::Arc;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use bevy_log::prelude::*;
use mw_effects::{MwEffectGmst, MwEffects};
use mw_game_core::player::PlayerCharacter;
use sf_character_bevy::CharacterPlacement;
use sf_effects_bevy::sf_effects::{
    EffectCtx, EffectEnv, EffectEvent, EffectOp, EffectOut, EffectRules, EffectsSave, GameMinutes,
};
use sf_effects_bevy::{
    EffectClock, EffectIds, EffectRulesRes, EffectSystems, EffectTableRes, Effects, LoadTables,
    SfEffectsPlugin, SourceTableRes, stat_commands,
};
use sf_records::{RecordDb, RecordId};
use sf_stats_bevy::sf_stats::{Cause, StatOp, StatOwner};
use sf_stats_bevy::{StatCommand, StatTableRes, StatTick};
use sf_world::PlacementOverlay;
use tes3::esp::Spell;

use crate::dialogue::Rpg;

/// Owner name of the per-placement effects blob (`PlacementOverlay::set_state`): a postcard
/// [`EffectsSave`].
pub const EFFECTS_BLOB: &str = "effects";
/// Its schema version (`sf_save_bevy::sections::BlobVersions`).
pub const EFFECTS_BLOB_VERSION: u32 = 1;

/// Morrowind's MGEF table (with each effect's `MwEffectInfo`), for mapping item and spell
/// records to effect sources (`crate::item_use`: potions).
#[derive(Resource, Clone)]
pub struct MwEffectsRes(pub Arc<MwEffects>);

/// Re-registered modifiers: `(owner, op, cause)`.
pub type Modifiers = Vec<(StatOwner, StatOp, Cause)>;

/// The effects layer over `db`'s MGEF records (after the stats setup).
pub fn add(app: &mut App, db: &RecordDb) {
    let fx = MwEffects::from_db(db);
    if !fx.bad_records().is_empty() {
        warn!(
            "effects: MGEF records with indices outside the table ignored: {:?}",
            fx.bad_records()
        );
    }
    let missing = fx.missing().count();
    if missing > 0 {
        info!("effects: {missing} effect indices have no MGEF record (their entries are dropped)");
    }
    let (gmst, missing_gmst) = MwEffectGmst::from_db(db);
    if !missing_gmst.is_empty() {
        warn!(
            "effects: GMSTs missing, vanilla values used: {}",
            missing_gmst.join(", ")
        );
    }
    let (sources, notes) = mw_effects::source_table(&fx, &gmst, db.iter::<Spell>().map(|(_, s)| s));
    if !notes.is_empty() {
        info!(
            "effects: {} SPEL entries changed by the mapping (unknown effects dropped, bad arguments)",
            notes.len()
        );
    }
    if !app.is_plugin_added::<SfEffectsPlugin>() {
        app.add_plugins(SfEffectsPlugin);
    }
    app.insert_resource(EffectTableRes(Arc::new(fx.table().clone())))
        .insert_resource(SourceTableRes(Arc::new(sources)))
        .insert_resource(MwEffectsRes(Arc::new(fx)))
        .insert_resource(EffectRulesRes(Arc::new(EffectRules::standard())))
        .add_systems(FixedUpdate, feed_clock.before(EffectSystems::Commands));
}

/// `EffectClock` = the game clock in minutes (see the module docs).
#[allow(clippy::needless_pass_by_value)] // system parameters
pub fn feed_clock(rpg: Option<Res<Rpg>>, mut clock: ResMut<EffectClock>) {
    if let Some(rpg) = rpg {
        let now = GameMinutes(rpg.game_time().total_minutes());
        if clock.0 != now {
            clock.0 = now;
        }
    }
}

/// What restoring a character's effects reads and writes, as a system parameter (the NPC attach).
#[derive(SystemParam)]
pub struct EffectLoad<'w> {
    defs: Option<Res<'w, EffectTableRes>>,
    sources: Res<'w, SourceTableRes>,
    rules: Res<'w, EffectRulesRes>,
    clock: Res<'w, EffectClock>,
    stats: Option<Res<'w, StatTableRes>>,
    tick: Res<'w, StatTick>,
    ids: ResMut<'w, EffectIds>,
    out: MessageWriter<'w, StatCommand>,
}

impl EffectLoad<'_> {
    /// `owner`'s effects from `save` and the modifiers for its own sheet; other owners' modifiers
    /// (an absorb's caster side) go out as `StatCommand`s. No save, or no tables: none active.
    pub fn restore(&mut self, owner: StatOwner, save: Option<EffectsSave>) -> (Effects, Modifiers) {
        let (Some(save), Some(defs), Some(stats)) = (save, &self.defs, &self.stats) else {
            return (Effects::new(owner), Vec::new());
        };
        let tables = LoadTables {
            defs: &defs.0,
            sources: &self.sources.0,
            stats: &stats.0,
        };
        let (effects, ops) = Effects::restore(owner, save, tables, self.tick.0, &mut self.ids.0);
        let (own, others): (Modifiers, Modifiers) =
            ops.into_iter().partition(|(o, _, _)| *o == owner);
        self.out.write_batch(stat_commands(others));
        (effects, own)
    }

    /// A new character's effects: `spells` learned in order (`Learn`; abilities, diseases and
    /// curses start their instance with the owner as caster, resistances ignored), and the
    /// modifiers they put on its own sheet, to apply before the sheet settles. Refusals (an
    /// unknown source) are warned. No tables: none learned.
    pub fn learn(&mut self, owner: StatOwner, spells: &[RecordId]) -> (Effects, Modifiers) {
        let mut effects = Effects::new(owner);
        let (Some(defs), Some(stats)) = (&self.defs, &self.stats) else {
            return (effects, Vec::new());
        };
        if spells.is_empty() {
            return (effects, Vec::new());
        }
        let env = EffectEnv::new(
            &defs.0,
            &stats.0,
            &self.sources.0,
            &self.rules.0,
            None,
            self.clock.0,
        );
        // `dt` turns durations into ticks; book sources live with the book and have none.
        let mut ctx = EffectCtx {
            target: owner,
            tick: self.tick.0,
            dt: 0.0,
            ids: &mut self.ids.0,
        };
        let ops: Vec<EffectOp> = spells
            .iter()
            .map(|s| EffectOp::Learn { source: s.clone() })
            .collect();
        let mut out = EffectOut::default();
        sf_effects_bevy::sf_effects::apply_book(
            &env,
            &mut effects.active,
            &mut effects.book,
            &ops,
            &mut ctx,
            &mut out,
        );
        for event in &out.events {
            if matches!(
                event,
                EffectEvent::BookRejected { .. } | EffectEvent::Rejected { .. }
            ) {
                warn!("effects: {owner:?}: start spell refused: {event:?}");
            }
        }
        let (own, others): (Modifiers, Modifiers) =
            out.stat_ops.into_iter().partition(|(o, _, _)| *o == owner);
        self.out.write_batch(stat_commands(others));
        (effects, own)
    }
}

/// [`EffectLoad::restore`] from exclusive world access (the character section's `set`).
pub fn restore_in_world(
    world: &mut World,
    owner: StatOwner,
    save: Option<EffectsSave>,
) -> (Effects, Modifiers) {
    with_load(world, owner, |load| load.restore(owner, save))
}

/// [`EffectLoad::learn`] from exclusive world access (the player character's spawn).
pub fn learn_in_world(
    world: &mut World,
    owner: StatOwner,
    spells: &[RecordId],
) -> (Effects, Modifiers) {
    with_load(world, owner, |load| load.learn(owner, spells))
}

fn with_load(
    world: &mut World,
    owner: StatOwner,
    f: impl FnOnce(&mut EffectLoad) -> (Effects, Modifiers),
) -> (Effects, Modifiers) {
    if !world.contains_resource::<EffectIds>() {
        return (Effects::new(owner), Vec::new());
    }
    let mut state = bevy_ecs::system::SystemState::<EffectLoad>::new(world);
    let restored = match state.get_mut(world) {
        Ok(mut load) => f(&mut load),
        Err(e) => {
            warn!("effects: {owner:?}: not restored: {e}");
            (Effects::new(owner), Vec::new())
        }
    };
    state.apply(world);
    restored
}

/// The placement's saved effects, if its blob decodes.
pub fn saved_effects(
    overlay: &PlacementOverlay,
    placement: &CharacterPlacement,
) -> Option<EffectsSave> {
    let bytes = overlay.state(placement.0, EFFECTS_BLOB)?;
    postcard::from_bytes(bytes)
        .map_err(|e| {
            warn!(
                "effects: placement {:032x}: bad {EFFECTS_BLOB} blob: {e}",
                placement.0.0
            );
        })
        .ok()
}

/// Writes a character's effects into `overlay` as its placement's [`EFFECTS_BLOB`] at fixed tick
/// `tick`: when something is active or the book is not empty, or over an older blob (so effects
/// that ended since do not come back; the overlay has no blob removal).
pub fn write_effects(
    overlay: &mut PlacementOverlay,
    placement: &CharacterPlacement,
    effects: &Effects,
    tick: u32,
) {
    let save = match effects.to_save(tick) {
        Some(save) => save,
        None if overlay.state(placement.0, EFFECTS_BLOB).is_some() => EffectsSave::default(),
        None => return,
    };
    match postcard::to_stdvec(&save) {
        Ok(bytes) => overlay.set_state(placement.0, EFFECTS_BLOB, bytes),
        Err(e) => warn!("effects: {:?} not saved: {e}", effects.owner),
    }
}

/// The player's effects for its `CharacterSave` (`None`: no effects). Saved even when empty:
/// `None` on load means "from before abilities" and learns the start spells
/// (`stats::restore_player_sheet`), so a book emptied in play must not read as `None`.
pub fn player_effects_save(world: &World, key: sf_rpg::PlayerKey) -> Option<EffectsSave> {
    let tick = world.get_resource::<StatTick>().map_or(0, |t| t.0);
    let mut q = world.try_query::<(&PlayerCharacter, &Effects)>()?;
    q.iter(world)
        .find(|(p, _)| p.key == key)
        .map(|(_, fx)| EffectsSave::capture(&fx.active, &fx.book, tick))
}

#[cfg(test)]
mod tests;

/// Synthetic records for headless tests of the stats and effects glue without an install (this
/// crate's and `mw_saves_bevy`'s): the 27 SKIL records, the MGEF records of Fortify and Drain
/// Attribute and Restore Health, two potions ([`RESTORE_HEALTH_POTION`],
/// [`FORTIFY_STRENGTH_POTION`]), a misc item ([`MISC_ITEM`]), and creatures with Strength
/// `strength`; [`spell_db`] adds start spells (a race, a birthsign, a `player` and an NPC).
#[doc(hidden)]
pub mod test_records {
    use std::sync::Arc;

    use sf_records::{RecordDb, RecordId};
    use tes3::esp::{
        Alchemy, AttributeId2, Birthsign, Creature, Effect, EffectId, EffectId2, EffectRange,
        MagicEffect, MiscItem, Npc, NpcData, NpcStats, Race, Skill, SkillData, SkillId, SkillId2,
        Spell, SpellData, SpellFlags, SpellType,
    };

    /// ALCH: Restore Health 10 points a second for 5 seconds.
    pub const RESTORE_HEALTH_POTION: &str = "p_test_restore_health";
    /// ALCH: Fortify Strength 10 points for 1 second.
    pub const FORTIFY_STRENGTH_POTION: &str = "p_test_fortify_strength";
    /// MISC: a bottle.
    pub const MISC_ITEM: &str = "misc_test_bottle";
    /// SPEL ability: Resist Fire 50 (the race's).
    pub const RACE_ABILITY: &str = "test_race_resist_fire";
    /// SPEL power: Fortify Strength 20 for 60 s (the race's).
    pub const RACE_POWER: &str = "test_race_power";
    /// SPEL ability: Fortify Strength 10 and Fortify Maximum Magicka 0.5 x Int (magnitude 5;
    /// the birthsign's, as The Mage's).
    pub const SIGN_ABILITY: &str = "test_sign_might";
    /// SPEL ability: Fortify Magicka 20 (in no list).
    pub const POOL_ABILITY: &str = "test_fortify_magicka";
    /// SPEL ability: Fortify Strength 5 (the NPC's own).
    pub const NPC_ABILITY: &str = "test_npc_might";
    /// SPEL spell: Resist Fire 10 for 10 s on self (the NPC's and the `player` record's).
    pub const NPC_SPELL: &str = "test_npc_spell";
    /// RACE: [`RACE_ABILITY`], [`RACE_POWER`].
    pub const RACE: &str = "test_race";
    /// BSGN: [`SIGN_ABILITY`].
    pub const SIGN: &str = "test_sign";
    /// NPC of [`RACE`] with [`NPC_ABILITY`], [`NPC_SPELL`], [`RACE_ABILITY`] again and an
    /// unknown id; attributes 40.
    pub const NPC: &str = "test_npc";
    /// Every attribute of the `player` record and [`NPC`].
    pub const ATTRIBUTE: u8 = 40;

    fn potion(id: &str, effect: Effect) -> Alchemy {
        Alchemy {
            id: id.into(),
            name: id.into(),
            effects: vec![effect],
            ..Alchemy::default()
        }
    }

    fn on_self(
        magic_effect: EffectId2,
        attribute: AttributeId2,
        magnitude: u32,
        duration: u32,
    ) -> Effect {
        Effect {
            magic_effect,
            skill: SkillId2::None,
            attribute,
            range: EffectRange::OnSelf,
            area: 0,
            duration,
            min_magnitude: magnitude,
            max_magnitude: magnitude,
        }
    }

    const SKILLS: [SkillId; 27] = [
        SkillId::Block,
        SkillId::Armorer,
        SkillId::MediumArmor,
        SkillId::HeavyArmor,
        SkillId::BluntWeapon,
        SkillId::LongBlade,
        SkillId::Axe,
        SkillId::Spear,
        SkillId::Athletics,
        SkillId::Enchant,
        SkillId::Destruction,
        SkillId::Alteration,
        SkillId::Illusion,
        SkillId::Conjuration,
        SkillId::Mysticism,
        SkillId::Restoration,
        SkillId::Alchemy,
        SkillId::Unarmored,
        SkillId::Security,
        SkillId::Sneak,
        SkillId::Acrobatics,
        SkillId::LightArmor,
        SkillId::ShortBlade,
        SkillId::Marksman,
        SkillId::Mercantile,
        SkillId::Speechcraft,
        SkillId::HandToHand,
    ];

    fn ability(id: &str, spell_type: SpellType, effects: Vec<Effect>) -> Spell {
        Spell {
            id: id.into(),
            name: id.into(),
            effects,
            data: SpellData {
                spell_type,
                cost: 5,
                flags: SpellFlags::empty(),
            },
            ..Spell::default()
        }
    }

    fn npc(id: &str, spells: &[&str]) -> Npc {
        Npc {
            id: id.into(),
            name: id.into(),
            race: RACE.into(),
            spells: spells.iter().map(|&s| s.to_owned()).collect(),
            data: NpcData {
                level: 1,
                stats: Some(NpcStats {
                    attributes: [ATTRIBUTE; 8],
                    skills: [10; 27],
                    health: 50,
                    magicka: 80,
                    fatigue: 100,
                }),
                ..NpcData::default()
            },
            ..Npc::default()
        }
    }

    /// The records, with one creature per `(id, strength)`.
    pub fn db(creatures: &[(&str, u32)]) -> Arc<RecordDb> {
        build(creatures, false)
    }

    /// [`db`] plus the start-spell records: MGEF Resist Fire, Fortify Health / Magicka / Fatigue
    /// and Fortify Maximum Magicka, the SPEL
    /// constants above, [`RACE`], [`SIGN`], a `player` of [`RACE`] whose own list
    /// ([`NPC_SPELL`]) a new player does not learn, and [`NPC`].
    pub fn spell_db(creatures: &[(&str, u32)]) -> Arc<RecordDb> {
        build(creatures, true)
    }

    #[allow(clippy::too_many_lines, reason = "one flat list of records")]
    fn build(creatures: &[(&str, u32)], spells: bool) -> Arc<RecordDb> {
        let mut db = RecordDb::new();
        mw_records::register_kinds(&mut db).expect("kinds register");
        let src = db.add_source("test", &[]);
        for (i, skill_id) in SKILLS.into_iter().enumerate() {
            let skill = Skill {
                skill_id,
                data: SkillData {
                    actions: [1.0; 4],
                    ..SkillData::default()
                },
                ..Skill::default()
            };
            db.define(src, RecordId::new(&format!("skill{i}")), skill)
                .expect("skill");
        }
        for (id, effect_id) in [
            ("fortifyattribute", EffectId::FortifyAttribute),
            ("drainattribute", EffectId::DrainAttribute),
            ("restorehealth", EffectId::RestoreHealth),
        ] {
            db.define(
                src,
                RecordId::new(id),
                MagicEffect {
                    effect_id,
                    ..MagicEffect::default()
                },
            )
            .expect("mgef");
        }
        let restore = on_self(EffectId2::RestoreHealth, AttributeId2::None, 10, 5);
        let fortify = on_self(EffectId2::FortifyAttribute, AttributeId2::Strength, 10, 1);
        for p in [
            potion(RESTORE_HEALTH_POTION, restore),
            potion(FORTIFY_STRENGTH_POTION, fortify),
        ] {
            db.define(src, RecordId::new(&p.id), p).expect("alch");
        }
        let misc = MiscItem {
            id: MISC_ITEM.into(),
            name: "Bottle".into(),
            ..MiscItem::default()
        };
        db.define(src, RecordId::new(MISC_ITEM), misc)
            .expect("misc");
        for &(id, strength) in creatures {
            let mut c = Creature {
                id: id.into(),
                name: id.into(),
                ..Creature::default()
            };
            c.data.strength = strength;
            c.data.health = 50;
            db.define(src, RecordId::new(id), c).expect("creature");
        }
        if !spells {
            return Arc::new(db);
        }
        for (id, effect_id) in [
            ("resistfire", EffectId::ResistFire),
            ("fortifyhealth", EffectId::FortifyHealth),
            ("fortifymagicka", EffectId::FortifyMagicka),
            ("fortifyfatigue", EffectId::FortifyFatigue),
            ("fortifymaximummagicka", EffectId::FortifyMagickaMultiplier),
        ] {
            let m = MagicEffect {
                effect_id,
                ..MagicEffect::default()
            };
            db.define(src, RecordId::new(id), m).expect("mgef");
        }
        let resist = |magnitude, duration| {
            on_self(
                EffectId2::ResistFire,
                AttributeId2::None,
                magnitude,
                duration,
            )
        };
        let strength = |magnitude, duration| {
            on_self(
                EffectId2::FortifyAttribute,
                AttributeId2::Strength,
                magnitude,
                duration,
            )
        };
        let magicka = on_self(EffectId2::FortifyMagicka, AttributeId2::None, 20, 0);
        let max_magicka = on_self(
            EffectId2::FortifyMagickaMultiplier,
            AttributeId2::None,
            5,
            0,
        );
        for s in [
            ability(RACE_ABILITY, SpellType::Ability, vec![resist(50, 0)]),
            ability(RACE_POWER, SpellType::Power, vec![strength(20, 60)]),
            ability(
                SIGN_ABILITY,
                SpellType::Ability,
                vec![strength(10, 0), max_magicka],
            ),
            ability(POOL_ABILITY, SpellType::Ability, vec![magicka]),
            ability(NPC_ABILITY, SpellType::Ability, vec![strength(5, 0)]),
            ability(NPC_SPELL, SpellType::Spell, vec![resist(10, 10)]),
        ] {
            db.define(src, RecordId::new(&s.id), s).expect("spel");
        }
        let race = Race {
            id: RACE.into(),
            name: RACE.into(),
            spells: vec![RACE_ABILITY.into(), RACE_POWER.into()],
            ..Race::default()
        };
        db.define(src, RecordId::new(RACE), race).expect("race");
        let sign = Birthsign {
            id: SIGN.into(),
            name: SIGN.into(),
            spells: vec![SIGN_ABILITY.into()],
            ..Birthsign::default()
        };
        db.define(src, RecordId::new(SIGN), sign).expect("bsgn");
        db.define(src, RecordId::new("player"), npc("player", &[NPC_SPELL]))
            .expect("player");
        let list = [NPC_ABILITY, NPC_SPELL, RACE_ABILITY, "test_no_such_spell"];
        db.define(src, RecordId::new(NPC), npc(NPC, &list))
            .expect("npc");
        Arc::new(db)
    }
}
