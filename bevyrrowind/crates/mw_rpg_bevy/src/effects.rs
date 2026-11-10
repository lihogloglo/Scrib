//! Magic effects on the player, NPCs and creatures (effects.md E4, effects-adapter.md "Crates"):
//! `sf_effects_bevy` with Morrowind's MGEF table (`mw_effects`).
//!
//! - **Setup** ([`add`], called by [`crate::stats::add`]: effects need the sheets): the MGEF
//!   records as `EffectTableRes` (`MwEffects`; A1 defs only, the other indices land as
//!   `Rejected { UnknownEffect }`), the modern rules (`EffectRules::standard`), no sources yet
//!   (the SPEL source table comes with abilities and casting, adapter A3 / A5), and the
//!   `MwEffects` themselves as [`MwEffectsRes`] (potions map their ALCH on use, `item_use`).
//! - **Attach**: `Effects` sits beside every `Stats` the stats glue attaches (the player
//!   character, NPCs, creatures), same owner; see [`crate::stats`].
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
use mw_effects::MwEffects;
use mw_game_core::player::PlayerCharacter;
use sf_character_bevy::CharacterPlacement;
use sf_effects_bevy::sf_effects::{EffectRules, EffectsSave, GameMinutes};
use sf_effects_bevy::{
    EffectClock, EffectIds, EffectRulesRes, EffectSystems, EffectTableRes, Effects, LoadTables,
    SfEffectsPlugin, SourceTableRes, stat_commands,
};
use sf_records::RecordDb;
use sf_stats_bevy::sf_stats::{Cause, StatOp, StatOwner};
use sf_stats_bevy::{StatCommand, StatTableRes, StatTick};
use sf_world::PlacementOverlay;

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
    if !app.is_plugin_added::<SfEffectsPlugin>() {
        app.add_plugins(SfEffectsPlugin);
    }
    app.insert_resource(EffectTableRes(Arc::new(fx.table().clone())))
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
}

/// [`EffectLoad::restore`] from exclusive world access (the character section's `set`).
pub fn restore_in_world(
    world: &mut World,
    owner: StatOwner,
    save: Option<EffectsSave>,
) -> (Effects, Modifiers) {
    if !world.contains_resource::<EffectIds>() {
        return (Effects::new(owner), Vec::new());
    }
    let mut state = bevy_ecs::system::SystemState::<EffectLoad>::new(world);
    let restored = match state.get_mut(world) {
        Ok(mut load) => load.restore(owner, save),
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

/// The player's effects for its `CharacterSave` (`None`: nothing active, an empty book, or no
/// effects).
pub fn player_effects_save(world: &World, key: sf_rpg::PlayerKey) -> Option<EffectsSave> {
    let tick = world.get_resource::<StatTick>().map_or(0, |t| t.0);
    let mut q = world.try_query::<(&PlayerCharacter, &Effects)>()?;
    q.iter(world)
        .find(|(p, _)| p.key == key)
        .and_then(|(_, fx)| fx.to_save(tick))
}

#[cfg(test)]
mod tests;

/// Synthetic records for headless tests of the stats and effects glue without an install (this
/// crate's and `mw_saves_bevy`'s): the 27 SKIL records, the MGEF records of Fortify and Drain
/// Attribute and Restore Health, two potions ([`RESTORE_HEALTH_POTION`],
/// [`FORTIFY_STRENGTH_POTION`]), a misc item ([`MISC_ITEM`]), and creatures with Strength
/// `strength`.
#[doc(hidden)]
pub mod test_records {
    use std::sync::Arc;

    use sf_records::{RecordDb, RecordId};
    use tes3::esp::{
        Alchemy, AttributeId2, Creature, Effect, EffectId, EffectId2, EffectRange, MagicEffect,
        MiscItem, Skill, SkillData, SkillId, SkillId2,
    };

    /// ALCH: Restore Health 10 points a second for 5 seconds.
    pub const RESTORE_HEALTH_POTION: &str = "p_test_restore_health";
    /// ALCH: Fortify Strength 10 points for 1 second.
    pub const FORTIFY_STRENGTH_POTION: &str = "p_test_fortify_strength";
    /// MISC: a bottle.
    pub const MISC_ITEM: &str = "misc_test_bottle";

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

    /// The records, with one creature per `(id, strength)`.
    pub fn db(creatures: &[(&str, u32)]) -> Arc<RecordDb> {
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
        Arc::new(db)
    }
}
