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
        check_class(name, "stems_per_ha", &class_declared(v), &class_declared(0.0)).unwrap();
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

/// WG-E3a: two rock rows, two units (one on defaults), both entry kinds.
const ROCKS: &str = r#"
[columns]
size_min_m = { unit = "m", min = 0.2, max = 40, source = "Guess" }
size_max_m = { unit = "m", min = 0.2, max = 40, source = "Guess" }
sink_small = { unit = "ratio", min = 0, max = 0.6, source = "Guess" }
rockfall = { unit = "ratio", min = 0, max = 4, source = "Guess" }
align = { unit = "ratio", min = 0, max = 1, source = "Measured(learned rock up axes)" }

[rocks.talus]
size_min_m = 0.4
size_max_m = 12
sink_small = 0.4
align = 0.7

[rocks.field]
size_min_m = 0.3
size_max_m = 2

[rock_units.lava]
rockfall = 2
talus = [
  { kind = "class", class = "block", weight = 1 },
  { kind = "model", model = "rocks/scree_a", axis_m = 3.2, scale_min = 0.8, scale_max = 1.5, weight = 2.5 },
]
field = [{ kind = "class", class = "cobble", weight = 1 }]

[rock_units.ash]
"#;

#[test]
#[allow(clippy::float_cmp)]
fn rock_rows_units_and_entries_parse_and_round_trip() {
    let f = parse(ROCKS).unwrap();
    let talus = f.rocks[&RockContext::Talus];
    assert_eq!((talus.size_min_m, talus.size_max_m, talus.sink_small, talus.align), (0.4, 12.0, 0.4, 0.7));
    let d = rock_declared();
    let default = |n: &str| d.iter().find(|d| d.name == n).unwrap().v;
    assert_eq!(talus.sink_large, default("sink_large"), "a value left out takes its declared default");
    assert!(!f.rocks.contains_key(&RockContext::Outcrop), "a context without a row has none");
    let lava = &f.rock_units["lava"];
    assert_eq!((lava.rockfall, lava.field_stone_per_ha), (2.0, default("field_stone_per_ha")));
    assert_eq!(lava.entries(RockContext::Talus).len(), 2);
    assert_eq!(
        lava.entries(RockContext::Talus)[1],
        RockEntry::Model { model: "rocks/scree_a".into(), axis_m: 3.2, scale_min: 0.8, scale_max: 1.5, weight: 2.5 }
    );
    assert_eq!(lava.entries(RockContext::Field), [RockEntry::Class { class: "cobble".into(), weight: 1.0 }]);
    assert!(lava.entries(RockContext::Outcrop).is_empty());
    let ash = &f.rock_units["ash"];
    assert_eq!((ash.rockfall, ash.field_stone_per_ha), (default("rockfall"), default("field_stone_per_ha")));
    assert!(ash.entries.is_empty());
    let text = f.to_toml().unwrap();
    let g = parse(&text).unwrap();
    assert_eq!(f, g);
    assert_eq!(text, g.to_toml().unwrap(), "writing is stable");
    for d in rock_declared() {
        crate::recipe::check_range(d.name, d.v, d.min, d.max).unwrap();
        crate::recipe::check_source(d.name, &d.source).unwrap();
        assert!(!d.help.is_empty(), "{} has no help", d.name);
        assert!(SPECIES_COLUMNS.iter().any(|(n, u)| *n == d.name && *u == d.unit), "{} not a column", d.name);
    }
}

/// Each case edits the rock fixture and must be refused with a message containing `want`.
fn rock_refused(from: &str, to: &str, want: &str) {
    assert!(ROCKS.contains(from), "rock fixture lacks {from:?}");
    let text = ROCKS.replacen(from, to, 1);
    let err = parse(&text).expect_err(&format!("accepted: {from:?} -> {to:?}")).to_string();
    assert!(err.contains(want), "{from:?} -> {to:?}: {err:?} lacks {want:?}");
}

#[test]
fn bad_rock_weights_kinds_and_values_are_refused() {
    let w = "weight = 2.5";
    rock_refused(w, "weight = 0", "\"rocks/scree_a\" weight 0 is not finite and positive");
    rock_refused(w, "weight = -1", "weight -1 is not finite and positive");
    rock_refused(w, "weight = nan", "weight NaN is not finite and positive");
    rock_refused(w, "weight = inf", "weight inf is not finite and positive");
    rock_refused("class = \"block\", weight = 1", "class = \"block\"", "missing field `weight`");
    rock_refused("kind = \"class\", class = \"block\"", "kind = \"stamp\", class = \"block\"", "unknown variant `stamp`");
    rock_refused("kind = \"class\", class = \"block\"", "class = \"block\"", "missing field `kind`");
    rock_refused("kind = \"class\", class = \"block\"", "kind = \"model\", class = \"block\"", "unknown field `class`");
    rock_refused("weight = 1 },\n", "weight = 1, axis_m = 2 },\n", "unknown field `axis_m`");
    rock_refused("class = \"block\"", "class = \"\"", "has a class entry with an empty name");
    rock_refused("{ kind = \"class\", class = \"block\", weight = 1 },",
        "{ kind = \"model\", model = \"rocks/scree_a\", axis_m = 1, scale_min = 1, scale_max = 1, weight = 1 },",
        "Talus model \"rocks/scree_a\" repeated");
    rock_refused("axis_m = 3.2", "axis_m = 0", "axis_m 0 is not finite and positive");
    rock_refused("scale_min = 0.8", "scale_min = 1.8", "scale [1.8, 1.5] is not positive and ordered");
    rock_refused("scale_min = 0.8", "scale_min = 0", "scale [0, 1.5] is not positive and ordered");
    rock_refused("[rocks.field]", "[rocks.scree]", "unknown field `scree`");
    rock_refused("field = [{", "scree = [{", "unknown field `scree`");
    rock_refused("rockfall = 2", "rockfall = 5", "rock unit \"lava\" rockfall = 5 outside");
    rock_refused("rockfall = 2", "field_stone_per_ha = 5", "gives field_stone_per_ha, not declared");
    rock_refused("sink_small = 0.4", "sink_small = 0.7", "rock row Talus sink_small = 0.7 outside");
    rock_refused("size_max_m = 12", "size_max_m = 0.3", "sizes [0.4, 0.3] are not positive and ordered");
    rock_refused("size_min_m = 0.3\n", "", "missing field `size_min_m`");
    rock_refused("size_min_m = 0.3\n", "size_min_m = 0.1\n", "size_min_m = 0.1 outside");
    rock_refused("[rock_units.ash]", "[rock_units.\"\"]", "empty rock unit name");
}
