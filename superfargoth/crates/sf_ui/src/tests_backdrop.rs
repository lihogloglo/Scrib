//! The backdrop resolver and the cover fit (`backdrop.rs`).

use crate::{BackdropCandidate, BackdropKind, cover_size, resolve_backdrop};

fn list() -> Vec<BackdropCandidate> {
    vec![
        BackdropCandidate::video("video/menu_background.bik"),
        BackdropCandidate::video("video/mw_menu.bik"),
        BackdropCandidate::image("textures/menu_morrowind.dds"),
    ]
}

#[test]
fn the_first_existing_candidate_wins() {
    let c = list();
    assert_eq!(resolve_backdrop(&c, 0, |_| true), Some(0), "a mod's menu_background.bik over vanilla");
    assert_eq!(resolve_backdrop(&c, 0, |p| p != "video/menu_background.bik"), Some(1), "vanilla's menu video");
    assert_eq!(resolve_backdrop(&c, 0, |p| p.starts_with("textures/")), Some(2), "no video: the title image");
    assert_eq!(resolve_backdrop(&c, 0, |_| false), None, "nothing: the plain theme background");
    assert_eq!(c[2].kind, BackdropKind::Image);
}

#[test]
fn a_failed_video_falls_through_to_the_rest() {
    let c = list();
    // mw_menu.bik exists but failed to open: resolve again after it.
    assert_eq!(resolve_backdrop(&c, 2, |_| true), Some(2));
    assert_eq!(resolve_backdrop(&c, 2, |p| p.starts_with("video/")), None);
    assert_eq!(resolve_backdrop(&c, 3, |_| true), None, "past the end");
    assert_eq!(resolve_backdrop(&[], 0, |_| true), None);
}

#[test]
fn cover_fills_and_keeps_the_aspect() {
    // 640x480 in 16:9: full width, cropped top and bottom.
    assert_eq!(cover_size((1920.0, 1080.0), (640.0, 480.0)), (1920.0, 1440.0));
    // In a tall window: full height, cropped at the sides.
    assert_eq!(cover_size((960.0, 1280.0), (640.0, 480.0)), (1_706.666_7, 1280.0));
    // Same aspect: exact fit.
    assert_eq!(cover_size((1280.0, 960.0), (640.0, 480.0)), (1280.0, 960.0));
    // Degenerate: stretch, no NaN.
    assert_eq!(cover_size((1920.0, 1080.0), (0.0, 480.0)), (1920.0, 1080.0));
    assert_eq!(cover_size((0.0, 0.0), (640.0, 480.0)), (0.0, 0.0));
}
