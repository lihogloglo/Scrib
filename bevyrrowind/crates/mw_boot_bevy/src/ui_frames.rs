//! Morrowind's 8-piece menu frames as one image each, mounted at start as a top VFS layer
//! (`bevyrrowind/docs/specs/ui-adapter.md` "Menu art for `mw_classic`").
//!
//! Bevy's 9-slice (`TextureSlicer`) takes one image; Morrowind ships each frame as eight pieces
//! (`menu_thick_border_top`, `..._top_left_corner`, ...), which `MyGUI` composes at runtime. The
//! pieces are tiny (a few KB), so this packs them after the VFS is built ([`mw_bake::ui_frames`])
//! instead of in the bake: an HD menu pack in the VFS is picked up on the next start and nothing
//! goes stale. The layer serves `ui/themes/morrowind/frames/<frame>.png` and
//! [`FRAMES_CSS`], one class per frame (`.mw-frame-thick`, ...: `background-image` plus
//! `-bevy-image-mode: sliced(..)` with the measured insets, edges tiled), which the theme sheet
//! imports ([`crate::ui_theme`]; the builtin mod serves an empty one underneath).

use std::fmt::Write as _;
use std::sync::Arc;
use std::time::Instant;

use mw_bake::textures::{Rgba8, decode_rgba};
use mw_bake::ui_frames::{self, FramePieces, Insets, PIECE_SUFFIXES};
use sf_vfs::{MemSource, Vfs, VfsPath};

/// The generated stylesheet's VFS path (imported by the theme sheet).
pub const FRAMES_CSS: &str = "ui/themes/morrowind/frames.css";
/// Where the packed images go.
const FRAMES_DIR: &str = "ui/themes/morrowind/frames";
/// Extensions an HD menu pack may ship a piece as instead of `.dds` (what `decode_rgba` reads).
const PIECE_ALTERNATES: &[&str] = &["png", "tga", "bmp"];

/// One 8-piece frame.
#[derive(Clone, Copy, Debug)]
pub struct FrameSet {
    /// Packed image name (`<FRAMES_DIR>/<name>.png`).
    pub name: &'static str,
    /// Piece prefix under `textures/` (pieces are `<prefix><suffix>.dds`).
    pub prefix: &'static str,
    /// CSS class drawing it.
    pub class: &'static str,
    /// Further selectors the frame draws on: the framework's role classes (ui.md "Role
    /// classes") by OpenMW's skin per role (`MW_Window`/`MW_Dialog` thick, `MW_Box` thin,
    /// `MW_Button`), and parts that carry no class (the message box, styled by `#name`).
    pub roles: &'static [&'static str],
    /// Centre piece, tiled like `MyGUI`'s `TileRect` fill; transparent without one.
    pub center: Option<&'static str>,
}

/// Morrowind's frames (`OpenMW` `openmw_box.skin.xml`, `openmw_button.skin.xml`, `openmw_windows.skin.xml`).
pub const FRAME_SETS: [FrameSet; 4] = [
    FrameSet {
        name: "thick_border",
        prefix: "menu_thick_border",
        class: "mw-frame-thick",
        roles: &[".sf-window", "#sf-message-box-panel"],
        center: None,
    },
    FrameSet {
        name: "thin_border",
        prefix: "menu_thin_border",
        class: "mw-frame-thin",
        roles: &[".sf-box"],
        center: None,
    },
    FrameSet {
        name: "button_frame",
        prefix: "menu_button_frame",
        class: "mw-frame-button",
        roles: &[".sf-framed", "#sf-message-box-button"],
        center: None,
    },
    FrameSet {
        name: "head_block",
        prefix: "menu_head_block",
        class: "mw-frame-head",
        roles: &[],
        center: Some("menu_head_block_middle"),
    },
];

/// What [`source`] packed.
#[derive(Debug, Default)]
pub struct Report {
    /// Packed frames and their insets.
    pub packed: Vec<(&'static str, Insets)>,
    /// Skipped frames and why.
    pub skipped: Vec<(&'static str, String)>,
}

/// `vfs` with the packed frames on top; prints one `ui.frames:` line (and one per skipped frame).
pub fn mount(vfs: Vfs) -> Vfs {
    let started = Instant::now();
    let (layer, report) = source(&vfs);
    for (name, why) in &report.skipped {
        println!("ui.frames: {name} skipped ({why})");
    }
    let names: Vec<&str> = report.packed.iter().map(|(n, _)| *n).collect();
    println!("ui.frames: {} packed ({}) in {:.1?}", names.len(), names.join(", "), started.elapsed());
    vfs.with_top_layer(Arc::new(layer))
}

/// The layer: one PNG per frame whose pieces are all there, and [`FRAMES_CSS`] with their classes.
pub fn source(vfs: &Vfs) -> (MemSource, Report) {
    let mut layer = MemSource::new("generated:ui_frames");
    let mut report = Report::default();
    let mut css = String::from("/* Generated at start from the menu frame pieces (mw_boot_bevy::ui_frames). */\n");
    for set in FRAME_SETS {
        match pack_set(vfs, &set) {
            Ok((png, insets)) => {
                let path = format!("{FRAMES_DIR}/{}.png", set.name);
                if let Err(e) = layer.insert(&path, png) {
                    report.skipped.push((set.name, e.to_string()));
                    continue;
                }
                css.push_str(&css_rule(&set, &path, insets));
                report.packed.push((set.name, insets));
            }
            Err(why) => report.skipped.push((set.name, why)),
        }
    }
    layer.insert(FRAMES_CSS, css).expect("constant path is valid");
    (layer, report)
}

/// The rule for one packed frame (its class, then its role selectors): the image over the border
/// box, 9-sliced at `insets`, edges tiled at their native length, the centre tiled (or stretched
/// when transparent).
pub fn css_rule(set: &FrameSet, path: &str, insets: Insets) -> String {
    let Insets { top, right, bottom, left } = insets;
    let center = if set.center.is_some() { "tile(1.0)" } else { "stretch" };
    let mut rule = format!(".{}", set.class);
    for role in set.roles {
        let _ = write!(rule, ", {role}");
    }
    let _ = write!(
        rule,
        " {{\n    background-image: url(\"sf_ui_content://{path}\");\n    \
         -bevy-image-mode: sliced({top}px {right}px {bottom}px {left}px {center} tile(1.0) 1.0);\n    \
         -bevy-image-visual-box: border-box;\n}}\n",
    );
    rule
}

fn pack_set(vfs: &Vfs, set: &FrameSet) -> Result<(Vec<u8>, Insets), String> {
    let mut pieces = Vec::with_capacity(8);
    for suffix in PIECE_SUFFIXES {
        pieces.push(read_piece(vfs, &format!("{}{suffix}", set.prefix))?);
    }
    let pieces: [Rgba8; 8] = pieces.try_into().expect("eight suffixes");
    let center = set.center.map(|c| read_piece(vfs, c)).transpose()?;
    let packed = ui_frames::pack(&FramePieces::from_array(pieces, center))?;
    Ok((ui_frames::encode_png(&packed.image)?, packed.insets))
}

fn read_piece(vfs: &Vfs, name: &str) -> Result<Rgba8, String> {
    let dds = VfsPath::new(&format!("textures/{name}.dds")).map_err(|e| e.to_string())?;
    let (path, _) = vfs.resolve_preferring(&dds, PIECE_ALTERNATES).ok_or_else(|| format!("no {dds}"))?;
    let bytes = vfs.read(&path).map_err(|e| format!("{path}: {e}"))?;
    decode_rgba(path.as_str(), &bytes).map_err(|e| format!("{path}: {e}"))
}

#[cfg(test)]
mod tests;
