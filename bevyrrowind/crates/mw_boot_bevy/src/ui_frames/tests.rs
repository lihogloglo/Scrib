use super::*;

use mw_bake::ui_frames::encode_png;

fn solid(width: u32, height: u32, shade: u8) -> Rgba8 {
    Rgba8 { width, height, rgba: [shade, 0, 0, 255].repeat(width as usize * height as usize) }
}

/// Fake pieces for `prefix` (as PNG, the way an HD pack ships them): corners `t`x`t`, edges
/// `len` long, sides 16 tall.
fn add_set(mem: &mut MemSource, prefix: &str, t: u32, len: u32) {
    let sizes = [(t, t), (len, t), (t, t), (t, 16), (t, 16), (t, t), (len, t), (t, t)];
    for (i, (suffix, (w, h))) in PIECE_SUFFIXES.iter().zip(sizes).enumerate() {
        let png = encode_png(&solid(w, h, u8::try_from(i + 1).unwrap())).unwrap();
        mem.insert(&format!("textures/{prefix}{suffix}.png"), png).unwrap();
    }
}

/// Every frame but the thin border (missing), the head block with its middle.
fn vfs() -> Vfs {
    let mut mem = MemSource::new("test");
    add_set(&mut mem, "menu_thick_border", 4, 512);
    add_set(&mut mem, "menu_button_frame", 4, 128);
    add_set(&mut mem, "menu_head_block", 2, 256);
    mem.insert("textures/menu_head_block_middle.png", encode_png(&solid(256, 16, 9)).unwrap()).unwrap();
    Vfs::builder().mount(mem).build()
}

fn read(vfs: &Vfs, path: &str) -> Vec<u8> {
    vfs.read(&VfsPath::new(path).unwrap()).unwrap()
}

#[test]
fn ui_frames_layer_serves_packed_images() {
    let vfs = mount(vfs());
    for (name, w, h) in [("thick_border", 520, 24), ("button_frame", 136, 24), ("head_block", 260, 20)] {
        let path = format!("ui/themes/morrowind/frames/{name}.png");
        let img = decode_rgba(&path, &read(&vfs, &path)).unwrap();
        assert_eq!((img.width, img.height), (w, h), "{name}");
    }
    let head = decode_rgba("x.png", &read(&vfs, "ui/themes/morrowind/frames/head_block.png")).unwrap();
    let px = |x: u32, y: u32| head.rgba[((y * head.width + x) * 4) as usize];
    assert_eq!([px(0, 0), px(100, 0), px(259, 19), px(100, 10)], [1, 2, 8, 9], "corner, edge, corner, middle");
}

#[test]
fn ui_frames_css_has_one_class_per_frame() {
    let (_, report) = source(&vfs());
    let packed: Vec<_> = report.packed.iter().map(|(n, i)| (*n, [i.top, i.right, i.bottom, i.left])).collect();
    assert_eq!(packed, [("thick_border", [4; 4]), ("button_frame", [4; 4]), ("head_block", [2; 4])]);
    let css = String::from_utf8(read(&mount(vfs()), FRAMES_CSS)).unwrap();
    assert!(css.contains(
        ".mw-frame-thick, .sf-window, #sf-message-box-panel {\n    background-image: url(\"sf_ui_content://ui/themes/morrowind/frames/thick_border.png\");\n    \
         -bevy-image-mode: sliced(4px 4px 4px 4px stretch tile(1.0) 1.0);\n    -bevy-image-visual-box: border-box;\n}\n"
    ));
    assert!(css.contains(".mw-frame-button, .sf-framed, #sf-message-box-button {\n    background-image: url(\"sf_ui_content://ui/themes/morrowind/frames/button_frame.png\");"));
    assert!(css.contains("-bevy-image-mode: sliced(4px 4px 4px 4px stretch tile(1.0) 1.0);"));
    assert!(css.contains(".mw-frame-head {\n    background-image: url(\"sf_ui_content://ui/themes/morrowind/frames/head_block.png\");"));
    assert!(css.contains("-bevy-image-mode: sliced(2px 2px 2px 2px tile(1.0) tile(1.0) 1.0);"));
    assert_eq!(css.matches(".mw-frame-").count(), 3);
    // The thin frame (and so `.sf-box`) is skipped in this VFS: no rule names it.
    assert!(!css.contains(".sf-box"));
}

#[test]
fn ui_frames_missing_set_is_skipped() {
    let (layer, report) = source(&vfs());
    assert_eq!(report.skipped.len(), 1);
    assert_eq!(report.skipped[0].0, "thin_border");
    assert_eq!(report.skipped[0].1, "no textures/menu_thin_border_top_left_corner.dds");
    let vfs = Vfs::builder().mount(layer).build();
    assert!(!vfs.contains(&VfsPath::new("ui/themes/morrowind/frames/thin_border.png").unwrap()));
    assert!(!String::from_utf8(read(&vfs, FRAMES_CSS)).unwrap().contains("mw-frame-thin"));
}

#[test]
fn ui_frames_missing_centre_skips_the_frame() {
    let mut mem = MemSource::new("test");
    add_set(&mut mem, "menu_head_block", 2, 256);
    let (_, report) = source(&Vfs::builder().mount(mem).build());
    assert!(report.packed.is_empty());
    assert_eq!(report.skipped.iter().find(|(n, _)| *n == "head_block").unwrap().1, "no textures/menu_head_block_middle.dds");
}

#[test]
fn ui_frames_empty_vfs_still_serves_the_sheet() {
    let vfs = mount(Vfs::builder().build());
    let css = String::from_utf8(read(&vfs, FRAMES_CSS)).unwrap();
    assert!(!css.contains(".mw-frame-"));
}

#[test]
#[ignore = "measurement: ui.frames pack time on the real install"]
fn ui_frames_real_install_timing() {
    let Ok((install, _)) = mw_install::MorrowindInstall::detect(None) else {
        println!("no Morrowind install: skipped");
        return;
    };
    let vfs = crate::mods::mount(&install, false).unwrap();
    let started = Instant::now();
    let (_, report) = source(&vfs);
    println!("ui.frames: {:?} skipped {:?} in {:.1?}", report.packed, report.skipped, started.elapsed());
}
