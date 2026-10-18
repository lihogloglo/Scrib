use crate::{Composition, LayerKind, mod_layer_name};

fn sample() -> Composition {
    let mut c = Composition::new();
    c.framework("ui/sf/base.css")
        .preset(None, "ui/presets/pc_dense.css")
        .theme(Some("classic_pack"), "ui/themes/classic/theme.css")
        .mod_sheet("better_inventory", "ui/better_inventory/style.css")
        .mod_sheet("hud_tweaks", "ui/hud_tweaks/style.css")
        .mod_sheet("better_inventory", "ui/better_inventory/extra.css")
        .user("user/ui.css");
    c
}

#[test]
fn layer_order_is_load_order() {
    let composed = sample().compose("content");
    assert_eq!(
        composed.layers,
        [
            "sf",
            "preset",
            "theme",
            "mods-1-better_inventory",
            "mods-2-hud_tweaks",
            "user"
        ]
    );
    let lines: Vec<&str> = composed.css.lines().skip(1).collect();
    assert_eq!(
        lines,
        [
            "@layer sf, preset, theme, mods-1-better_inventory, mods-2-hud_tweaks, user;",
            "@import url(\"content://ui/sf/base.css\") layer(sf);",
            "@import url(\"content://ui/presets/pc_dense.css\") layer(preset);",
            "@import url(\"content://ui/themes/classic/theme.css\") layer(theme);",
            "@import url(\"content://ui/better_inventory/style.css\") layer(mods-1-better_inventory);",
            "@import url(\"content://ui/hud_tweaks/style.css\") layer(mods-2-hud_tweaks);",
            // A mod's second sheet joins its own layer (it stays below the later mod).
            "@import url(\"content://ui/better_inventory/extra.css\") layer(mods-1-better_inventory);",
            "@import url(\"content://user/ui.css\") layer(user);",
        ]
    );

    // Reversed load order reverses the mod layers and nothing else.
    let mut c = Composition::new();
    c.mod_sheet("hud_tweaks", "ui/hud_tweaks/style.css")
        .mod_sheet("better_inventory", "ui/better_inventory/style.css");
    assert_eq!(
        c.compose("content").layers,
        [
            "sf",
            "preset",
            "theme",
            "mods-1-hud_tweaks",
            "mods-2-better_inventory",
            "user"
        ]
    );
}

#[test]
fn sheets_map_back_to_their_owners() {
    let composed = sample().compose("content");
    let extra = composed.sheet("ui/better_inventory/extra.css").unwrap();
    assert_eq!(
        (extra.kind, extra.mod_id.as_deref()),
        (LayerKind::Mod, Some("better_inventory"))
    );
    assert_eq!(extra.layer, "mods-1-better_inventory");
    let theme = composed.sheet("ui/themes/classic/theme.css").unwrap();
    assert_eq!(
        (theme.kind, theme.mod_id.as_deref()),
        (LayerKind::Theme, Some("classic_pack"))
    );
    let user = composed.sheet("user/ui.css").unwrap();
    assert_eq!((user.kind, user.mod_id.as_deref()), (LayerKind::User, None));
    assert!(composed.sheet("ui/other.css").is_none());
    assert_eq!(composed.mod_layer("hud_tweaks"), Some("mods-2-hud_tweaks"));
}

#[test]
fn layer_names_are_css_identifiers() {
    // Mod ids may contain `.`, which a flair layer name can't; positions keep them unique.
    assert_eq!(mod_layer_name(3, "author.pack"), "mods-3-author_pack");
    assert_ne!(mod_layer_name(1, "a.b"), mod_layer_name(2, "a_b"));
    let mut c = Composition::new();
    c.mod_sheet("q\"x", "ui/a\"b.css");
    let css = c.compose("").css;
    assert!(
        css.contains("@import url(\"ui/a\\\"b.css\") layer(mods-1-q_x);"),
        "{css}"
    );
}
