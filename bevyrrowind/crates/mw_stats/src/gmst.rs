//! `MwGmst`: the game settings the stat formulas read, snapshot once from the content (`OpenMW`
//! reads them with `static const` locals, so a snapshot per new game matches). A missing GMST
//! falls back to its Morrowind.esm value and is reported.

use sf_records::{RecordDb, RecordId};
use tes3::esp::{GameSetting, GameSettingValue};

macro_rules! gmsts {
    ($($field:ident: $name:literal = $vanilla:expr),* $(,)?) => {
        /// Typed GMST snapshot (integers as `f32`).
        #[derive(Clone, Debug, PartialEq)]
        pub struct MwGmst {
            $(#[doc = concat!("`", $name, "`")] pub $field: f32,)*
            /// `iLevelUp01Mult` .. `iLevelUp10Mult`: attribute gain by tally (1..=10).
            pub level_up_mult: [f32; 10],
        }

        impl MwGmst {
            /// Morrowind.esm's values (Tribunal and Bloodmoon change none of these).
            #[must_use]
            pub fn vanilla() -> Self {
                Self { $($field: $vanilla,)* level_up_mult: VANILLA_LEVEL_UP_MULT }
            }

            /// The snapshot from `db`, and the names that fell back to vanilla.
            #[must_use]
            pub fn from_db(db: &RecordDb) -> (Self, Vec<String>) {
                let mut g = Self::vanilla();
                let mut missing = Vec::new();
                let mut read = |name: &str, slot: &mut f32| match gmst(db, name) {
                    Some(v) => *slot = v,
                    None => missing.push(name.to_owned()),
                };
                $(read($name, &mut g.$field);)*
                for (i, slot) in g.level_up_mult.iter_mut().enumerate() {
                    read(&format!("iLevelUp{:02}Mult", i + 1), slot);
                }
                (g, missing)
            }

            /// Every setting as `(name, value)`, for reports.
            #[must_use]
            pub fn entries(&self) -> Vec<(String, f32)> {
                let mut e = vec![$(($name.to_owned(), self.$field),)*];
                e.extend(self.level_up_mult.iter().enumerate().map(|(i, v)| (format!("iLevelUp{:02}Mult", i + 1), *v)));
                e
            }
        }
    };
}

const VANILLA_LEVEL_UP_MULT: [f32; 10] = [2.0, 2.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0, 4.0, 5.0];

gmsts! {
    pc_base_magicka_mult: "fPCbaseMagickaMult" = 1.0,
    npc_base_magicka_mult: "fNPCbaseMagickaMult" = 2.0,
    fatigue_base: "fFatigueBase" = 1.25,
    fatigue_mult: "fFatigueMult" = 0.5,
    encumbrance_str_mult: "fEncumbranceStrMult" = 5.0,
    level_up_health_end_mult: "fLevelUpHealthEndMult" = 0.1,
    level_up_total: "iLevelUpTotal" = 10.0,
    major_skill_bonus: "fMajorSkillBonus" = 0.75,
    minor_skill_bonus: "fMinorSkillBonus" = 1.0,
    misc_skill_bonus: "fMiscSkillBonus" = 1.25,
    special_skill_bonus: "fSpecialSkillBonus" = 0.8,
    level_up_major_mult: "iLevelUpMajorMult" = 1.0,
    level_up_minor_mult: "iLevelUpMinorMult" = 1.0,
    level_up_major_mult_attribute: "iLevelUpMajorMultAttribute" = 1.0,
    level_up_minor_mult_attribute: "iLevelUpMinorMultAttribute" = 1.0,
    level_up_misc_mult_attribute: "iLevelupMiscMultAttriubte" = 1.0,
    level_up_specialization: "iLevelupSpecialization" = 1.0,
    fatigue_return_base: "fFatigueReturnBase" = 2.5,
    fatigue_return_mult: "fFatigueReturnMult" = 0.02,
    end_fatigue_mult: "fEndFatigueMult" = 0.04,
    rest_magic_mult: "fRestMagicMult" = 0.15,
    auto_rep_fac_mod: "iAutoRepFacMod" = 2.0,
    auto_rep_lev_mod: "iAutoRepLevMod" = 0.0,
}

/// One GMST as a number (`None` when absent or a string).
#[allow(clippy::cast_precision_loss)]
fn gmst(db: &RecordDb, name: &str) -> Option<f32> {
    match db.get::<GameSetting>(&RecordId::new(name)).map(|g| &g.value) {
        Some(GameSettingValue::Float(v)) => Some(*v),
        Some(GameSettingValue::Integer(v)) => Some(*v as f32),
        _ => None,
    }
}
