use super::*;

const SHIPPED: &str = include_str!("climate_classes.toml");

fn name(t: &ClassTable, t_c: f64, p_mm: f64) -> &str {
    &t.names()[usize::from(t.classify(t_c, p_mm))]
}

fn grid(w: usize, h: usize, data: Vec<f64>) -> Grid {
    Grid { w, h, spacing: 1.0, data }
}

#[test]
fn the_shipped_table_parses() {
    let t = ClassTable::whittaker();
    assert_eq!(t.names().len(), 12);
    assert_eq!(t.names()[0], "tundra");
    assert_eq!(t.t_edges.len() + 1, 5);
    assert_eq!(t.p_edges.len() + 1, 6);
    assert_eq!(t.cells.len(), 5 * 6);
}

#[test]
fn corners_of_the_table() {
    let t = ClassTable::whittaker();
    assert_eq!(name(&t, -20.0, 100.0), "tundra");
    assert_eq!(name(&t, 25.0, 100.0), "desert");
    assert_eq!(name(&t, 25.0, 3000.0), "rainforest");
    assert_eq!(name(&t, 8.0, 100.0), "cold desert");
    assert_eq!(name(&t, 15.0, 1500.0), "temperate forest");
    assert_eq!(name(&t, 0.0, 800.0), "taiga");
}

#[test]
fn edges_and_out_of_range() {
    let t = ClassTable::whittaker();
    assert_eq!(name(&t, 3.0, 600.0), "woodland");
    assert_eq!(name(&t, 2.999, 600.0), "taiga");
    assert_eq!(name(&t, -100.0, 0.0), "tundra");
    assert_eq!(name(&t, 60.0, 1e6), "rainforest");
    assert_eq!(name(&t, 20.0, 2500.0), "rainforest");
    assert_eq!(name(&t, f64::NAN, 600.0), "tundra");
}

#[test]
fn rejects_bad_tables() {
    const T_EDGES: &str = "t_edges_c = [-5.0, 3.0, 12.0, 20.0]";
    let cases: Vec<(&str, String)> = vec![
        ("unknown key", SHIPPED.replace("source = ", "extra = 1\nsource = ")),
        ("invalid source", SHIPPED.replace("source = \"Industry(", "source = \"Wild(")),
        ("empty edges", SHIPPED.replace(T_EDGES, "t_edges_c = []")),
        ("non-finite edge", SHIPPED.replace(T_EDGES, "t_edges_c = [-5.0, 3.0, 12.0, inf]")),
        ("nan edge", SHIPPED.replace("2500.0]", "nan]")),
        ("not ascending", SHIPPED.replace(T_EDGES, "t_edges_c = [-5.0, 3.0, 3.0, 20.0]")),
        (
            "missing row",
            SHIPPED.replace("  [\"tundra\", \"tundra\", \"tundra\", \"tundra\", \"tundra\", \"tundra\"],\n", ""),
        ),
        (
            "short row",
            SHIPPED.replace(
                "\"seasonal forest\", \"seasonal forest\", \"rainforest\"]",
                "\"seasonal forest\", \"rainforest\"]",
            ),
        ),
        ("empty name", SHIPPED.replace("[\"desert\", \"savanna\"", "[\"\", \"savanna\"")),
    ];
    for (what, text) in &cases {
        assert_ne!(text, SHIPPED, "{what}: the replace did not apply");
        assert!(ClassTable::parse_toml(text).is_err(), "{what}: accepted");
    }

    // More than 255 distinct names: 2 temperature bands by 128 precipitation bands, every cell its own name.
    let p_edges: Vec<String> = (0..127).map(|i| format!("{i}.0")).collect();
    let row = |r: usize| -> String {
        let names: Vec<String> = (0..128).map(|c| format!("\"c{r}_{c}\"")).collect();
        format!("  [{}],\n", names.join(", "))
    };
    let many = |rows: [String; 2]| -> String {
        format!(
            "source = \"Guess\"\nt_edges_c = [0.0]\np_edges_mm = [{}]\nclasses = [\n{}{}]\n",
            p_edges.join(", "),
            rows[0],
            rows[1]
        )
    };
    assert!(ClassTable::parse_toml(&many([row(0), row(1)])).is_err(), "256 names: accepted");
    // 255 distinct names still fit (the last cell repeats the first name).
    let last = row(1).replace("\"c1_127\"", "\"c0_0\"");
    let t = ClassTable::parse_toml(&many([row(0), last])).expect("255 names");
    assert_eq!(t.names().len(), 255);
}

#[test]
fn classes_maps_every_sample() {
    let table = ClassTable::whittaker();
    let temperature = grid(
        4,
        3,
        vec![-20.0, 0.0, 8.0, 15.0, 25.0, 25.0, 3.0, 2.999, -100.0, 60.0, 20.0, f64::NAN],
    );
    let rain = grid(
        4,
        3,
        vec![100.0, 800.0, 100.0, 1500.0, 100.0, 3000.0, 600.0, 600.0, 0.0, 1e6, 2500.0, 100.0],
    );
    // tundra 0, cold desert 1, taiga 2, woodland 4, temperate forest 5, desert 7, rainforest 11.
    assert_eq!(
        classes(&temperature, &rain, &table).unwrap(),
        vec![0, 2, 1, 5, 7, 11, 4, 2, 0, 11, 11, 0]
    );
    let other = grid(3, 4, rain.data.clone());
    assert!(matches!(classes(&temperature, &other, &table), Err(Error::Invalid(_))));
}
