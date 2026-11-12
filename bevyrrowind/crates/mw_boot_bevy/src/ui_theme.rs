//! The Morrowind UI theme (ROADMAP 2.2 U3): a `ui.theme` record plus its stylesheet, served by
//! the builtin mod's VFS layer like any mod's content (hard rule 3), restyling the stock
//! `sf_ui_screens` screens through `sf_ui_css` with no code in them.
//!
//! - **Palette**: Morrowind.ini `[FontColor]` (`color_normal`, `color_link`, `color_answer`, ...)
//!   as CSS custom properties `--mw-normal`, `--mw-link`, `--mw-answer`, ... generated at mount
//!   from the install's ini, vanilla values where a key is missing ([`VANILLA_COLORS`]).
//! - **Textures**: the journal book is the BSA's `textures/tx_menubook.dds`. Morrowind's frames
//!   (`menu_thick_border_*`, `menu_button_frame_*`) are eight separate pieces, packed into one
//!   image each at start by [`crate::ui_frames`] (`.mw-frame-*` classes, imported first); the
//!   screens draw bronze CSS borders until they take those classes (look pass L1b).
//! - **Font**: `OpenMW`'s Mystic Cards (SIL OFL 1.1, derived from Isak Larborn's Pelagiad), a
//!   TrueType lookalike of Morrowind's "Magic Cards" bitmap font, shipped unmodified with its
//!   licence next to it (`ui/morrowind/fonts/`).

use std::fmt::Write as _;
use std::sync::Arc;

use bevy_app::{App, Startup};
use bevy_ecs::prelude::*;
use bevy_log::warn;
use mw_install::MorrowindIni;
use sf_ui::ThemeId;
use sf_ui_bevy::UiSettings;
use sf_ui_css::{UiCssPlugin, UiCssSourcePlugin};
use sf_vfs::{MemSource, Vfs};

/// The theme's id (`theme-morrowind` root class).
pub const THEME_ID: &str = "morrowind";
/// Its stylesheet's VFS path.
pub const STYLESHEET: &str = "ui/themes/morrowind/theme.css";
/// The font, beside the sheet.
const FONT: &str = "ui/themes/morrowind/MysticCards.ttf";
const FONT_LICENSE: &str = "ui/themes/morrowind/MysticCardsFontLicense.txt";

/// The `ui.theme` record (applied with the game's screen records).
pub const THEME_RECORD: &str = r#"
[[define]]
kind = "ui.theme"
id = "morrowind"
value = { name = "theme-morrowind", stylesheet = "ui/themes/morrowind/theme.css", fonts = { body = ["Mystic Cards"] } }
"#;

/// Vanilla Morrowind.ini `[FontColor]` (GOTY install), the fallback for each missing key.
pub const VANILLA_COLORS: &[(&str, [u8; 3])] = &[
    ("color_normal", [202, 165, 96]),
    ("color_normal_over", [223, 201, 159]),
    ("color_normal_pressed", [243, 237, 221]),
    ("color_active", [96, 112, 202]),
    ("color_active_over", [159, 169, 223]),
    ("color_active_pressed", [223, 226, 244]),
    ("color_disabled", [179, 168, 135]),
    ("color_disabled_over", [223, 201, 159]),
    ("color_disabled_pressed", [243, 237, 221]),
    ("color_link", [112, 126, 207]),
    ("color_link_over", [143, 155, 218]),
    ("color_link_pressed", [175, 184, 228]),
    ("color_journal_link", [37, 49, 112]),
    ("color_journal_link_over", [58, 77, 175]),
    ("color_journal_link_pressed", [112, 126, 207]),
    ("color_journal_topic", [0, 0, 0]),
    ("color_journal_topic_over", [58, 77, 175]),
    ("color_journal_topic_pressed", [112, 126, 207]),
    ("color_answer", [150, 50, 30]),
    ("color_answer_over", [223, 201, 159]),
    ("color_answer_pressed", [243, 237, 221]),
    ("color_header", [223, 201, 159]),
    ("color_notify", [223, 201, 159]),
    ("color_background", [0, 0, 0]),
    ("color_focus", [80, 80, 80]),
    ("color_health", [200, 60, 30]),
    ("color_magic", [53, 69, 159]),
    ("color_fatigue", [0, 150, 60]),
    ("color_misc", [0, 205, 205]),
    ("color_positive", [223, 201, 159]),
    ("color_negative", [200, 60, 30]),
    ("color_count", [223, 201, 159]),
];

/// `r,g,b` (each 0-255) as written in Morrowind.ini.
fn parse_rgb(value: &str) -> Option<[u8; 3]> {
    let mut parts = value.split(',').map(|p| p.trim().parse::<u8>().ok());
    let rgb = [parts.next()??, parts.next()??, parts.next()??];
    parts.next().is_none().then_some(rgb)
}

/// The `:root` token block: `--mw-normal-over: rgb(223, 201, 159);` per `[FontColor]` key.
pub fn tokens(ini: Option<&MorrowindIni>) -> String {
    let mut css = String::from(":root {\n");
    for (key, vanilla) in VANILLA_COLORS {
        let [r, g, b] = ini.and_then(|i| i.get("FontColor", key)).and_then(parse_rgb).unwrap_or(*vanilla);
        let name = key.trim_start_matches("color_").replace('_', "-");
        let _ = writeln!(css, "    --mw-{name}: rgb({r}, {g}, {b});");
    }
    css.push_str("}\n\n");
    css
}

/// The sheet's first line: the packed menu frames' classes ([`crate::ui_frames`], a top layer
/// mounted at start; the builtin mod serves an empty sheet under it so the import always resolves).
const FRAMES_IMPORT: &str = "@import url(\"sf_ui_content://ui/themes/morrowind/frames.css\");\n";

/// The builtin mod's theme layer: the stylesheet (the frames import, tokens from `ini`, then the
/// shipped rules), an empty frames sheet, the font and its licence.
pub fn source(ini: Option<&MorrowindIni>) -> MemSource {
    let sheet = String::from(FRAMES_IMPORT) + &tokens(ini) + include_str!("../ui/morrowind/theme.css");
    MemSource::new("builtin:mw_ui_theme")
        .with(STYLESHEET, sheet)
        .with(crate::ui_frames::FRAMES_CSS, "/* Menu frames: generated at start (mw_boot_bevy::ui_frames). */\n")
        .with(FONT, include_bytes!("../ui/morrowind/fonts/MysticCards.ttf").as_slice())
        .with(FONT_LICENSE, include_str!("../ui/morrowind/fonts/MysticCardsFontLicense.txt"))
}

/// Before `DefaultPlugins`: the content source the composed sheet imports from (the game VFS;
/// not watched: sheet hot reload needs the watched source shared, TODO).
pub fn add_source(app: &mut App, vfs: Arc<Vfs>) {
    app.add_plugins(UiCssSourcePlugin::new(sf_vfs::bevy::asset_source(vfs)));
}

/// After `SfUiPlugin`: styling on, Morrowind theme selected.
pub fn add(app: &mut App) {
    if !app.world().contains_resource::<sf_ui_css::UiCssRootDir>() {
        warn!("ui theme off: no CSS content source (ui_theme::add_source before DefaultPlugins)");
        return;
    }
    app.add_plugins(UiCssPlugin);
    let mut settings = app.world().get_resource::<UiSettings>().cloned().unwrap_or_default();
    settings.theme = ThemeId::from(THEME_ID);
    app.insert_resource(settings);
    app.add_systems(Startup, no_focus_ring);
}

/// The sheet shows focus Morrowind's way (`.sf-button:focus-visible` takes the "over" colour),
/// so the framework's fallback outline goes.
fn no_focus_ring(colors: Option<ResMut<sf_ui_screens::ScreenColors>>) {
    if let Some(mut colors) = colors {
        colors.focus_ring = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokens_take_the_ini_and_fall_back_to_vanilla() {
        let ini = MorrowindIni::parse(b"[FontColor]\r\ncolor_normal=1,2,3\r\ncolor_link=bad\r\n");
        let css = tokens(Some(&ini));
        assert!(css.contains("--mw-normal: rgb(1, 2, 3);"), "{css}");
        assert!(css.contains("--mw-link: rgb(112, 126, 207);"), "malformed falls back: {css}");
        assert!(css.contains("--mw-journal-link-over: rgb(58, 77, 175);"));
        assert_eq!(tokens(None).matches("--mw-").count(), VANILLA_COLORS.len());
    }

    /// Windows, boxes and framed buttons are styled by role (the frame art comes from
    /// frames.css); the bronze CSS borders they replaced are gone.
    #[test]
    fn sheet_styles_roles_not_bronze_borders() {
        let sheet = include_str!("../ui/morrowind/theme.css");
        for role in [".sf-window {", ".sf-box {", ".sf-framed,"] {
            assert!(sheet.contains(role), "no {role} rule");
        }
        assert!(!sheet.contains("var(--mw-frame)"), "thick bronze border left");
        assert!(!sheet.contains("--mw-frame:"), "bronze token left");
        // Only the HUD bars keep a CSS border (not role-tagged yet).
        assert_eq!(sheet.matches("border-color:").count(), 1, "border-color outside the HUD");
        let frames: Vec<_> = crate::ui_frames::FRAME_SETS.iter().flat_map(|s| s.roles).collect();
        for role in [".sf-window", ".sf-box", ".sf-framed"] {
            assert!(frames.contains(&&role), "no frame draws {role}");
        }
    }
}
