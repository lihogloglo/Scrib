//! Dialogue text macros (rpg-adapter.md "Text"; `OpenMW` `Interpreter::fixDefinesDialog`).

/// A dialogue macro (`%PCName`, `^Name`, ...).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Macro {
    PcName,
    PcRace,
    PcClass,
    PcRank,
    PcNextRank,
    PcCrimeLevel,
    Name,
    Race,
    Class,
    Faction,
    Rank,
    NextRank,
    Cell,
}

/// Names, longest first so a prefix match picks the longest (`PCName` before `Name`).
/// `NextPCRank` is an alias of `PCNextRank`, as in the original.
const MACROS: [(&str, Macro); 14] = [
    ("pccrimelevel", Macro::PcCrimeLevel),
    ("pcnextrank", Macro::PcNextRank),
    ("nextpcrank", Macro::PcNextRank),
    ("nextrank", Macro::NextRank),
    ("pcclass", Macro::PcClass),
    ("faction", Macro::Faction),
    ("pcname", Macro::PcName),
    ("pcrace", Macro::PcRace),
    ("pcrank", Macro::PcRank),
    ("class", Macro::Class),
    ("name", Macro::Name),
    ("race", Macro::Race),
    ("rank", Macro::Rank),
    ("cell", Macro::Cell),
];

/// What macros read.
pub trait MacroSource {
    fn macro_value(&self, which: Macro) -> String;
    /// A global's value, for `%<global id>`.
    fn global(&self, id: &str) -> Option<GlobalNumber>;
}

/// A global as stored: shorts and longs print as integers, floats with up to 6
/// significant digits (C++ stream default).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GlobalNumber {
    Int(i32),
    Float(f32),
}

impl GlobalNumber {
    pub fn as_f32(self) -> f32 {
        match self {
            #[allow(clippy::cast_precision_loss)]
            Self::Int(i) => i as f32,
            Self::Float(f) => f,
        }
    }
}

/// Expands `%Macro` / `^Macro` (case-insensitive prefix, longest name first), then globals
/// (longest matching global id prefix). Unknown macros stay as written.
pub fn expand(text: &str, source: &dyn MacroSource) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find(['%', '^']) {
        out.push_str(&rest[..at]);
        let after = &rest[at + 1..];
        if let Some((len, value)) = lookup(after, source) {
            out.push_str(&value);
            rest = &after[len..];
        } else {
            out.push_str(&rest[at..=at]);
            rest = after;
        }
    }
    out.push_str(rest);
    out
}

fn lookup(after: &str, source: &dyn MacroSource) -> Option<(usize, String)> {
    let starts = |name: &str| {
        after
            .get(..name.len())
            .is_some_and(|p| p.eq_ignore_ascii_case(name))
    };
    if let Some((name, which)) = MACROS.iter().find(|(name, _)| starts(name)) {
        return Some((name.len(), source.macro_value(*which)));
    }
    // Globals: the longest identifier prefix that names one.
    let ident = after
        .bytes()
        .take_while(|b| b.is_ascii_alphanumeric() || *b == b'_')
        .count();
    (1..=ident).rev().find_map(|len| {
        source
            .global(&after[..len])
            .map(|g| (len, format_global(g)))
    })
}

/// C++ `ostream << value` formatting: integers as is, floats `%g` with 6 significant digits.
pub fn format_global(value: GlobalNumber) -> String {
    match value {
        GlobalNumber::Int(i) => i.to_string(),
        GlobalNumber::Float(f) => format_g6(f),
    }
}

fn format_g6(f: f32) -> String {
    if f == 0.0 {
        return "0".into();
    }
    if !f.is_finite() {
        return if f.is_nan() {
            "nan".into()
        } else if f > 0.0 {
            "inf".into()
        } else {
            "-inf".into()
        };
    }
    let exp = format!("{f:.5e}");
    let (mantissa, e) = exp.split_once('e').expect("exponent form");
    let e: i32 = e.parse().expect("exponent");
    let trim = |s: &str| -> String {
        if s.contains('.') {
            s.trim_end_matches('0').trim_end_matches('.').to_owned()
        } else {
            s.to_owned()
        }
    };
    if (-4..6).contains(&e) {
        let decimals = usize::try_from(5 - e).unwrap_or(0);
        trim(&format!("{f:.decimals$}"))
    } else {
        let sign = if e < 0 { '-' } else { '+' };
        format!("{}e{sign}{:02}", trim(mantissa), e.abs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Src;

    impl MacroSource for Src {
        fn macro_value(&self, which: Macro) -> String {
            format!("<{which:?}>")
        }

        fn global(&self, id: &str) -> Option<GlobalNumber> {
            match id.to_ascii_lowercase().as_str() {
                "day" => Some(GlobalNumber::Int(7)),
                "dayspassed" => Some(GlobalNumber::Int(12)),
                "rate" => Some(GlobalNumber::Float(1.5)),
                _ => None,
            }
        }
    }

    #[test]
    fn macros_longest_first_and_case_insensitive() {
        assert_eq!(
            expand("Hi %pcname, I'm %Name.", &Src),
            "Hi <PcName>, I'm <Name>."
        );
        assert_eq!(
            expand("^Rank/%NextPCRank/%PCNextRank", &Src),
            "<Rank>/<PcNextRank>/<PcNextRank>"
        );
        assert_eq!(expand("%Names", &Src), "<Name>s");
    }

    #[test]
    fn globals_and_unknown() {
        assert_eq!(
            expand("%DaysPassed days, %Day, %rate", &Src),
            "12 days, 7, 1.5"
        );
        assert_eq!(expand("100% sure, %Bogus", &Src), "100% sure, %Bogus");
    }

    #[test]
    fn g6_formatting() {
        assert_eq!(format_g6(2.0), "2");
        assert_eq!(format_g6(0.25), "0.25");
        assert_eq!(format_g6(1_234_567.0), "1.23457e+06");
        assert_eq!(format_g6(123_456.0), "123456");
        assert_eq!(format_g6(0.000_012_5), "1.25e-05");
    }
}
