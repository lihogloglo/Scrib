//! Census furniture kinds (motion-generation.md "As built (MG4)", interaction-adapter.md "Defs from
//! the census"): what a `furn_*` / `active_*` record is used for, from its id or model name.
//!
//! `mw_bake::census` switches to `pub use mw_interaction::furniture::{Kind, classify}` once its
//! current writer frees `mw_bake` (`IN32a` follow-up / `IN32b`); until then the census has its own copy
//! (`bevyrrowind/crates/mw_bake/src/census.rs`), which this one must match word for word.

/// What a piece of furniture is used for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
    Seat,
    Table,
    Counter,
    Bed,
    Shelf,
}

impl Kind {
    pub const ALL: [Self; 5] = [
        Self::Seat,
        Self::Table,
        Self::Counter,
        Self::Bed,
        Self::Shelf,
    ];

    /// The key in `bands.json` (scrib-motiongen `motiongen/default_bands.json`).
    pub fn key(self) -> &'static str {
        match self {
            Self::Seat => "seat",
            Self::Table => "table",
            Self::Counter => "counter",
            Self::Bed => "bed",
            Self::Shelf => "shelf",
        }
    }

    /// Highest surface this kind measures (above it: backrests, canopies,
    /// hutches), meters above the floor.
    pub fn cap(self) -> f32 {
        match self {
            Self::Seat => 0.8,
            Self::Table | Self::Bed => 1.2,
            Self::Counter => 1.5,
            Self::Shelf => 2.5,
        }
    }
}

/// The kind of a furniture record from its id or, failing that, its model's
/// file name: `furn_*` / `active_*` ids (and `furn_*` models) whose
/// `_`-separated words (trailing digits dropped) name a bed, counter,
/// table, seat or shelf, in that order (`furn_de_bar_01` is a counter,
/// `furn_de_r_bench_01` a seat, `active_de_bedroll` a bed). Words, not
/// substrings: `barrel` is not a `bar`.
pub fn classify(id: &str, model: Option<&str>) -> Option<Kind> {
    fn words(name: &str) -> Option<Vec<String>> {
        let lower = name.to_ascii_lowercase();
        let words: Vec<String> = lower
            .split(|c: char| !c.is_ascii_alphanumeric())
            .map(|w| w.trim_end_matches(|c: char| c.is_ascii_digit()).to_owned())
            .filter(|w| !w.is_empty())
            .collect();
        matches!(words.first().map(String::as_str), Some("furn" | "active")).then_some(words)
    }
    fn kind(words: &[String]) -> Option<Kind> {
        let any = |names: &[&str]| words.iter().any(|w| names.contains(&w.as_str()));
        if any(&["door"]) {
            return None;
        }
        if any(&["bed", "beds", "bedroll", "hammock", "cot", "bunk"]) {
            Some(Kind::Bed)
        } else if any(&["counter", "bar", "bartop"]) {
            Some(Kind::Counter)
        } else if any(&["table", "desk"]) {
            Some(Kind::Table)
        } else if any(&["chair", "stool", "bench", "seat", "throne", "pew"]) {
            Some(Kind::Seat)
        } else if any(&["shelf", "shelves", "bookshelf", "bookcase"]) {
            Some(Kind::Shelf)
        } else {
            None
        }
    }
    let stem = |m: &str| {
        m.rsplit(['/', '\\'])
            .next()
            .unwrap_or(m)
            .split('.')
            .next()
            .unwrap_or(m)
            .to_owned()
    };
    words(id).and_then(|w| kind(&w)).or_else(|| {
        model
            .map(stem)
            .and_then(|s| words(&s))
            .and_then(|w| kind(&w))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The census's own cases (`mw_bake/src/census.rs` `classify_by_words`).
    #[test]
    fn classify_by_words() {
        assert_eq!(classify("furn_de_p_chair_01", None), Some(Kind::Seat));
        assert_eq!(classify("furn_com_rm_table_03", None), Some(Kind::Table));
        assert_eq!(classify("active_de_bed_29", None), Some(Kind::Bed));
        assert_eq!(classify("furn_de_bar_01", None), Some(Kind::Counter));
        assert_eq!(classify("furn_de_r_bench_01", None), Some(Kind::Seat));
        assert_eq!(classify("furn_barrel_01", None), None);
        assert_eq!(classify("in_de_bar_01", None), None);
        assert_eq!(classify("active_de_bar_door", None), None);
        assert_eq!(
            classify("xyz", Some("meshes/f/furn_com_chair01.nif")),
            Some(Kind::Seat)
        );
    }

    /// Word order, the kind order (bed before counter before table before seat before shelf),
    /// case, separators and Windows model paths.
    #[test]
    fn classify_order_and_names() {
        assert_eq!(classify("active_de_bedroll", None), Some(Kind::Bed));
        assert_eq!(classify("furn_bunk_bed_table", None), Some(Kind::Bed));
        assert_eq!(classify("furn_bar_table", None), Some(Kind::Counter));
        assert_eq!(classify("furn_desk_chair", None), Some(Kind::Table));
        assert_eq!(classify("furn_stool_shelf", None), Some(Kind::Seat));
        assert_eq!(classify("FURN_Com_Bookshelf_02", None), Some(Kind::Shelf));
        assert_eq!(classify("furn-de-pew-01", None), Some(Kind::Seat));
        assert_eq!(classify("furn_hammock", None), Some(Kind::Bed));
        assert_eq!(classify("furn_de_throne", None), Some(Kind::Seat));
        // Not furniture: no prefix, or no kind word.
        assert_eq!(classify("de_chair", None), None);
        assert_eq!(classify("furn_rug_01", None), None);
        // The id wins over the model; the model only when the id names nothing.
        assert_eq!(classify("furn_de_table", Some("f\\furn_de_chair_01.nif")), Some(Kind::Table));
        assert_eq!(classify("misc_thing", Some("f\\Furn_De_Bed_03.NIF")), Some(Kind::Bed));
        assert_eq!(classify("misc_thing", Some("x/chair.nif")), None);
        assert_eq!(classify("misc_thing", None), None);
    }

    #[test]
    fn keys_and_caps() {
        let keys: Vec<_> = Kind::ALL.iter().map(|k| k.key()).collect();
        assert_eq!(keys, ["seat", "table", "counter", "bed", "shelf"]);
        assert!((Kind::Seat.cap() - 0.8).abs() < f32::EPSILON);
        assert!((Kind::Table.cap() - 1.2).abs() < f32::EPSILON);
        assert!((Kind::Bed.cap() - 1.2).abs() < f32::EPSILON);
        assert!((Kind::Counter.cap() - 1.5).abs() < f32::EPSILON);
        assert!((Kind::Shelf.cap() - 2.5).abs() < f32::EPSILON);
    }
}
