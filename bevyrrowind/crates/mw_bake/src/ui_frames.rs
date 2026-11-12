//! Packs an 8-piece UI frame (4 corners, 4 edges, optional centre) into one RGBA image with
//! slice insets, so Bevy's 9-slice (`TextureSlicer`, one image) can draw Morrowind's menu frames
//! (`bevyrrowind/docs/specs/ui-adapter.md` "Menu art for `mw_classic`").
//!
//! Layout, as `MyGUI` composes the pieces at runtime: corners in the corners, each edge between
//! its corners at its full length, the centre piece (or transparent pixels) in the middle. The
//! insets are the piece thicknesses, so edges tile at their native length.

use crate::textures::Rgba8;

/// Piece suffixes after a frame's prefix (`menu_thick_border` + `_top`), in [`FramePieces`] order.
pub const PIECE_SUFFIXES: [&str; 8] = [
    "_top_left_corner",
    "_top",
    "_top_right_corner",
    "_left",
    "_right",
    "_bottom_left_corner",
    "_bottom",
    "_bottom_right_corner",
];

/// The pieces of one frame.
#[derive(Clone, Debug)]
pub struct FramePieces {
    pub top_left: Rgba8,
    pub top: Rgba8,
    pub top_right: Rgba8,
    pub left: Rgba8,
    pub right: Rgba8,
    pub bottom_left: Rgba8,
    pub bottom: Rgba8,
    pub bottom_right: Rgba8,
    /// Fill between the edges (`menu_head_block_middle`); transparent when `None`.
    pub center: Option<Rgba8>,
}

impl FramePieces {
    /// The pieces from [`PIECE_SUFFIXES`] order.
    pub fn from_array(pieces: [Rgba8; 8], center: Option<Rgba8>) -> Self {
        let [top_left, top, top_right, left, right, bottom_left, bottom, bottom_right] = pieces;
        Self { top_left, top, top_right, left, right, bottom_left, bottom, bottom_right, center }
    }
}

/// Slice insets in image pixels (CSS order: top, right, bottom, left).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Insets {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
}

/// One packed frame.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackedFrame {
    pub image: Rgba8,
    pub insets: Insets,
}

/// Packs `pieces` into one image; an error (never a panic) when the pieces don't fit together.
pub fn pack(pieces: &FramePieces) -> Result<PackedFrame, String> {
    let p = pieces;
    let named = [
        ("top-left corner", &p.top_left),
        ("top edge", &p.top),
        ("top-right corner", &p.top_right),
        ("left edge", &p.left),
        ("right edge", &p.right),
        ("bottom-left corner", &p.bottom_left),
        ("bottom edge", &p.bottom),
        ("bottom-right corner", &p.bottom_right),
    ];
    for (name, img) in named.iter().copied().chain(p.center.as_ref().map(|c| ("centre", c))) {
        if img.width == 0 || img.height == 0 {
            return Err(format!("{name} is empty ({}x{})", img.width, img.height));
        }
        if img.rgba.len() != img.width as usize * img.height as usize * 4 {
            return Err(format!("{name}: {} bytes for {}x{} RGBA", img.rgba.len(), img.width, img.height));
        }
    }
    let (top, bottom) = (p.top.height, p.bottom.height);
    let (left, right) = (p.left.width, p.right.width);
    let (width, height) = (p.top.width, p.left.height);
    let expect = |name: &str, img: &Rgba8, w: u32, h: u32| {
        if (img.width, img.height) == (w, h) {
            Ok(())
        } else {
            Err(format!("{name} is {}x{}, expected {w}x{h} to fit the edges", img.width, img.height))
        }
    };
    expect("top-left corner", &p.top_left, left, top)?;
    expect("top-right corner", &p.top_right, right, top)?;
    expect("bottom-left corner", &p.bottom_left, left, bottom)?;
    expect("bottom-right corner", &p.bottom_right, right, bottom)?;
    expect("bottom edge", &p.bottom, width, bottom)?;
    expect("right edge", &p.right, right, height)?;
    if let Some(center) = &p.center {
        expect("centre", center, width, height)?;
    }
    let mut image =
        Rgba8 { width: left + width + right, height: top + height + bottom, rgba: Vec::new() };
    image.rgba = vec![0; image.width as usize * image.height as usize * 4];
    let (x1, x2, y1, y2) = (left, left + width, top, top + height);
    for (img, x, y) in [
        (&p.top_left, 0, 0),
        (&p.top, x1, 0),
        (&p.top_right, x2, 0),
        (&p.left, 0, y1),
        (&p.right, x2, y1),
        (&p.bottom_left, 0, y2),
        (&p.bottom, x1, y2),
        (&p.bottom_right, x2, y2),
    ] {
        blit(&mut image, img, x, y);
    }
    if let Some(center) = &p.center {
        blit(&mut image, center, x1, y1);
    }
    Ok(PackedFrame { image, insets: Insets { top, right, bottom, left } })
}

/// Copies `src` into `dst` at (`x`, `y`); the caller checked it fits.
fn blit(dst: &mut Rgba8, src: &Rgba8, x: u32, y: u32) {
    let row = src.width as usize * 4;
    for sy in 0..src.height as usize {
        let d = ((y as usize + sy) * dst.width as usize + x as usize) * 4;
        dst.rgba[d..d + row].copy_from_slice(&src.rgba[sy * row..(sy + 1) * row]);
    }
}

/// `img` as PNG bytes.
pub fn encode_png(img: &Rgba8) -> Result<Vec<u8>, String> {
    let mut out = std::io::Cursor::new(Vec::new());
    image::write_buffer_with_format(
        &mut out,
        &img.rgba,
        img.width,
        img.height,
        image::ExtendedColorType::Rgba8,
        image::ImageFormat::Png,
    )
    .map_err(|e| e.to_string())?;
    Ok(out.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solid(width: u32, height: u32, rgba: [u8; 4]) -> Rgba8 {
        Rgba8 { width, height, rgba: rgba.repeat(width as usize * height as usize) }
    }

    fn at(img: &Rgba8, x: u32, y: u32) -> [u8; 4] {
        let i = ((y * img.width + x) * 4) as usize;
        img.rgba[i..i + 4].try_into().unwrap()
    }

    /// Distinct colours per piece; thickness 1 top, 2 right, 3 bottom, 4 left; edges 5 wide, 6 tall.
    fn pieces(center: bool) -> FramePieces {
        FramePieces {
            top_left: solid(4, 1, [1, 0, 0, 255]),
            top: solid(5, 1, [2, 0, 0, 255]),
            top_right: solid(2, 1, [3, 0, 0, 255]),
            left: solid(4, 6, [4, 0, 0, 255]),
            right: solid(2, 6, [5, 0, 0, 255]),
            bottom_left: solid(4, 3, [6, 0, 0, 255]),
            bottom: solid(5, 3, [7, 0, 0, 255]),
            bottom_right: solid(2, 3, [8, 0, 0, 255]),
            center: center.then(|| solid(5, 6, [9, 0, 0, 255])),
        }
    }

    #[test]
    fn ui_frames_pieces_land_in_place() {
        let packed = pack(&pieces(false)).unwrap();
        assert_eq!(packed.insets, Insets { top: 1, right: 2, bottom: 3, left: 4 });
        let img = &packed.image;
        assert_eq!((img.width, img.height), (11, 10));
        // Every pixel's owner, by region.
        for y in 0..img.height {
            for x in 0..img.width {
                let col = if x < 4 { 0 } else if x < 9 { 1 } else { 2 };
                let row = if y < 1 { 0 } else if y < 7 { 1 } else { 2 };
                let expected = match (row, col) {
                    (0, 0) => [1, 0, 0, 255],
                    (0, 1) => [2, 0, 0, 255],
                    (0, 2) => [3, 0, 0, 255],
                    (1, 0) => [4, 0, 0, 255],
                    (1, 2) => [5, 0, 0, 255],
                    (2, 0) => [6, 0, 0, 255],
                    (2, 1) => [7, 0, 0, 255],
                    (2, 2) => [8, 0, 0, 255],
                    _ => [0, 0, 0, 0],
                };
                assert_eq!(at(img, x, y), expected, "pixel ({x}, {y})");
            }
        }
    }

    #[test]
    fn ui_frames_centre_fills_the_middle() {
        let packed = pack(&pieces(true)).unwrap();
        let img = &packed.image;
        for y in 1..7 {
            for x in 4..9 {
                assert_eq!(at(img, x, y), [9, 0, 0, 255], "pixel ({x}, {y})");
            }
        }
        assert_eq!(at(img, 3, 1), [4, 0, 0, 255]);
        assert_eq!(at(img, 9, 6), [5, 0, 0, 255]);
    }

    #[test]
    fn ui_frames_piece_rows_keep_their_order() {
        // A 4x1 corner with four distinct pixels keeps its orientation.
        let mut p = pieces(false);
        p.top_left = Rgba8 { width: 4, height: 1, rgba: (0..16).collect() };
        let img = pack(&p).unwrap().image;
        for x in 0..4u8 {
            assert_eq!(at(&img, u32::from(x), 0), [4 * x, 4 * x + 1, 4 * x + 2, 4 * x + 3]);
        }
    }

    #[test]
    fn ui_frames_mismatched_pieces_are_errors() {
        let mut p = pieces(false);
        p.top_right = solid(2, 2, [3, 0, 0, 255]);
        assert_eq!(pack(&p).unwrap_err(), "top-right corner is 2x2, expected 2x1 to fit the edges");
        let mut p = pieces(false);
        p.bottom = solid(6, 3, [7, 0, 0, 255]);
        assert!(pack(&p).unwrap_err().starts_with("bottom edge is 6x3"));
        let mut p = pieces(false);
        p.right = solid(2, 5, [5, 0, 0, 255]);
        assert!(pack(&p).unwrap_err().starts_with("right edge is 2x5"));
        let mut p = pieces(true);
        p.center = Some(solid(5, 5, [9, 0, 0, 255]));
        assert!(pack(&p).unwrap_err().starts_with("centre is 5x5"));
        let mut p = pieces(false);
        p.top = solid(0, 1, [0; 4]);
        assert!(pack(&p).unwrap_err().starts_with("top edge is empty"));
        let mut p = pieces(false);
        p.left.rgba.pop();
        assert!(pack(&p).unwrap_err().starts_with("left edge: 95 bytes"));
    }

    #[test]
    fn ui_frames_png_round_trips() {
        let packed = pack(&pieces(true)).unwrap();
        let png = encode_png(&packed.image).unwrap();
        let back = crate::textures::decode_rgba("frame.png", &png).unwrap();
        assert_eq!(back, packed.image);
    }
}
