//! WG-E2a: the species file round trips; a bad range, column, value or key is refused with its own message.

use super::*;

/// One canopy tree with hats and classes, one ground row on defaults; two params, one class overridden.
const FIXTURE: &str = r#"
[params]
"scatter.stand_wavelength_m" = { v = 300, unit = "m", min = 50, max = 1000, source = "Guess" }
"scatter.competition_rounds" = { v = 4, unit = "count", min = 1, max = 6, source = "Guess" }

[columns]
parent_per_ha = { unit = "1/ha", min = 0, max = 100, source = "Guess" }
size_min_m = { unit = "m", min = 0.5, max = 40, source = "Guess" }
size_max_m = { unit = "m", min = 0.5, max = 40, source = "Guess" }
offset_m = { unit = "m", min = -1, max = 1, source = "Measured(learned offsets)" }
hat_temperature_c = { unit = "C", min = -40, max = 40, source = "Guess" }
hat_slope_deg = { unit = "deg", min = 0, max = 90, source = "Guess" }

[classes."taiga"]
stems_per_ha = { v = 300, unit = "1/ha", min = 0, max = 1000, source = "Guess" }

[classes."ash waste"]
stems_per_ha = { v = 2, unit = "1/ha", min = 0, max = 10, source = "Guess" }

[[species]]
name = "pine"
layer = "canopy"
model = "trees/pine_a"
size_law = "weibull"
classes = ["taiga", "woodland"]
parent_per_ha = 6
size_min_m = 3
size_max_m = 18
offset_m = -0.2
hat_temperature_c = [-10, -2, 12, 20]
hat_slope_deg = [0, 0, 25, 35]

[[species]]
name = "fern"
layer = "ground"
size_law = "reverse_j"
size_min_m = 0.5
size_max_m = 1.5
"#;

fn parse(text: &str) -> Result<SpeciesFile> {
    SpeciesFile::parse_toml(text)
}

#[test]
#[allow(clippy::float_cmp)]
fn fixture_parses_to_the_given_values_and_defaults() {
    let f = parse(FIXTURE).unwrap();
    assert_eq!(f.values.stand_wavelength_m, 300.0);
    assert_eq!(f.values.competition_rounds, 4);
    assert_eq!(f.values.edge_band_m, ScatterValues::default().edge_band_m);
    assert_eq!(f.classes["taiga"].v, 300.0);
    assert_eq!(f.classes["ash waste"].v, 2.0);
    assert_eq!(f.classes["woodland"].v, 60.0, "a class left out keeps its default");
    let pine = &f.species[0];
    assert_eq!(pine.layer, Layer::Canopy);
    assert_eq!(pine.size_law, SizeLaw::Weibull);
    assert_eq!(pine.model.as_deref(), Some("trees/pine_a"));
    assert_eq!(pine.classes, ["taiga", "woodland"]);
    assert_eq!((pine.parent_per_ha, pine.size_min_m, pine.size_max_m, pine.offset_m), (6.0, 3.0, 18.0, -0.2));
    assert_eq!(pine.hats.temperature_c, Some(Hat { e_min: -10.0, i_min: -2.0, i_max: 12.0, e_max: 20.0 }));
    assert_eq!(pine.hats.moisture, None, "a hat left out is open");
    let fern = &f.species[1];
    let d = species_declared();
    let default = |n: &str| d.iter().find(|d| d.name == n).unwrap().v;
    assert_eq!(fern.parent_per_ha, default("parent_per_ha"));
    assert_eq!(fern.crown_k, default("crown_k"));
    assert_eq!(fern.model, None);
    assert!(fern.classes.is_empty());
}

#[test]
fn round_trip_is_exact() {
    let f = parse(FIXTURE).unwrap();
    let text = f.to_toml().unwrap();
    let g = parse(&text).unwrap();
    assert_eq!(f, g);
    assert_eq!(text, g.to_toml().unwrap(), "writing is stable");
    let d = SpeciesFile::defaults();
    assert_eq!(parse(&d.to_toml().unwrap()).unwrap(), d);
    assert_eq!(parse("").unwrap(), d, "an empty file is the defaults");
}

#[test]
fn declared_defaults_are_valid() {
    for d in declared().iter().chain(species_declared().iter()) {
        crate::recipe::check_range(d.name, d.v, d.min, d.max).unwrap();
        crate::recipe::check_source(d.name, &d.source).unwrap();
        assert!(!d.help.is_empty(), "{} has no help", d.name);
    }
    assert!(declared().iter().all(|d| d.name.starts_with("scatter.")));
    for d in species_declared() {
        assert!(SPECIES_COLUMNS.iter().any(|(n, u)| *n == d.name && *u == d.unit), "{} not a column", d.name);
    }
    for (name, v) in class_defaults() {
        check_class(name, &class_declared(v)).unwrap();
    }
}

/// Each case edits the fixture and must be refused with a message containing `want`.
fn refused(from: &str, to: &str, want: &str) {
    assert!(FIXTURE.contains(from), "fixture lacks {from:?}");
    let text = FIXTURE.replacen(from, to, 1);
    let err = parse(&text).expect_err(&format!("accepted: {from:?} -> {to:?}")).to_string();
    assert!(err.contains(want), "{from:?} -> {to:?}: {err:?} lacks {want:?}");
}

#[test]
fn bad_ranges_are_refused() {
    let col = "parent_per_ha = { unit = \"1/ha\", min = 0, max = 100,";
    refused(col, "parent_per_ha = { unit = \"1/ha\", min = 100, max = 0,", "not finite and ordered");
    refused(col, "parent_per_ha = { unit = \"1/ha\", min = 0, max = 900,", "outside its declared");
    refused(col, "parent_per_ha = { unit = \"1/ha\", min = 0, max = nan,", "not finite and ordered");
    refused("parent_per_ha = 6", "parent_per_ha = 150", "\"pine\" parent_per_ha = 150 outside");
    refused("offset_m = -0.2", "offset_m = -2", "offset_m = -2 outside");
    refused("hat_slope_deg = [0, 0, 25, 35]", "hat_slope_deg = [0, 30, 25, 35]", "is not ordered");
    refused("hat_slope_deg = [0, 0, 25, 35]", "hat_slope_deg = [0, 0, 25, 95]", "hat_slope_deg = 95 outside");
    refused("size_max_m = 18", "size_max_m = 2", "size_min_m 3 above size_max_m 2");
    // A default outside a declared column's range: the fern takes parent_per_ha 4, the column now starts at 5.
    refused(col, "parent_per_ha = { unit = \"1/ha\", min = 5, max = 100,", "\"fern\" parent_per_ha = 4 outside");
    refused("v = 300, unit = \"m\"", "v = 30, unit = \"m\"", "scatter.stand_wavelength_m = 30 outside");
    refused("v = 4, unit = \"count\"", "v = 2.5, unit = \"count\"", "not a whole count");
    refused("min = 50, max = 1000", "min = 500, max = 100", "range [500, 100] is not finite and ordered");
    refused("v = 300, unit = \"1/ha\", min = 0, max = 1000", "v = 300, unit = \"1/ha\", min = 0, max = 3000",
        "not ordered inside");
    refused("v = 2, unit = \"1/ha\"", "v = 20, unit = \"1/ha\"", "\"ash waste\" stems_per_ha = 20 outside");
}

#[test]
fn bad_columns_and_keys_are_refused() {
    refused("[columns]\n", "[columns]\ncrown_q = { unit = \"ratio\", min = 0, max = 1, source = \"Guess\" }\n",
        "unknown column crown_q");
    refused("offset_m = { unit = \"m\"", "offset_m = { unit = \"cm\"", "column offset_m unit \"cm\"");
    refused("source = \"Measured(learned offsets)\"", "source = \"learned\"", "is not one of Guess");
    refused("hat_temperature_c = { unit = \"C\", min = -40, max = 40, source = \"Guess\" }\n", "",
        "\"pine\" gives hat_temperature_c, not declared");
    refused("offset_m = { unit = \"m\", min = -1, max = 1, source = \"Measured(learned offsets)\" }\n", "",
        "\"pine\" gives offset_m, not declared");
    refused("name = \"fern\"", "name = \"pine\"", "species \"pine\" repeated");
    refused("name = \"fern\"", "name = \"\"", "empty species name");
    refused("layer = \"ground\"", "layer = \"moss\"", "unknown variant");
    refused("size_law = \"reverse_j\"", "size_law = \"lognormal\"", "unknown variant");
    refused("offset_m = -0.2", "offset_m = -0.2\nheight = 3", "unknown field");
    refused("\"scatter.competition_rounds\"", "\"scatter.rounds\"", "unknown param scatter.rounds");
    refused("v = 4, unit = \"count\"", "v = 4, unit = \"ratio\"", "unit \"ratio\", expected \"count\"");
    refused("classes = [\"taiga\", \"woodland\"]", "classes = [\"taiga\", \"taiga\"]", "is empty or repeated");
    refused("model = \"trees/pine_a\"", "model = \"\"", "empty model");
    refused("stems_per_ha = { v = 2, unit = \"1/ha\"", "stems_per_ha = { v = 2, unit = \"1/km2\"",
        "stems_per_ha unit \"1/km2\"");
}

#[test]
fn stand_thresholds_must_be_ordered() {
    let text = format!(
        "{FIXTURE}\n",
    )
    .replacen(
        "[columns]",
        "\"scatter.stand_sparse_threshold\" = { v = 0.8, unit = \"ratio\", min = 0, max = 1, source = \"Guess\" }\n\n[columns]",
        1,
    );
    let err = parse(&text).unwrap_err().to_string();
    assert!(err.contains("clearing 0.35 < sparse 0.8 < inner 0.7 do not hold"), "{err}");
}
