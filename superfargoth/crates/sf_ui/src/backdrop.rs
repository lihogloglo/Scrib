//! The menu backdrop's resolver and fit (menus.md "### 2. Backdrop and title"): plain logic, no
//! engine. The content pack lists ordered candidates (adapter hook `MenuBackdropCandidates`);
//! the first whose file exists wins, and a video that fails to open falls through to the rest
//! (the caller resolves again from the next index). [`cover_size`] is the `cover` fit (the
//! default): the media fills the container keeping its aspect, cropped, centred; [`contain_size`]
//! the `contain` fit (letterbox: the whole media, black bars).

/// What a backdrop candidate holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BackdropKind {
    /// A video, played looped and silent (the decoder probes the header).
    Video,
    /// A still image (any texture format the asset server loads).
    Image,
}

/// One entry of the backdrop candidate list: a VFS path and what it holds.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BackdropCandidate {
    pub path: String,
    pub kind: BackdropKind,
}

impl BackdropCandidate {
    pub fn video(path: impl Into<String>) -> Self {
        Self { path: path.into(), kind: BackdropKind::Video }
    }

    pub fn image(path: impl Into<String>) -> Self {
        Self { path: path.into(), kind: BackdropKind::Image }
    }
}

/// The index of the first candidate at or after `from` whose file `exists`; `None`: no backdrop
/// (the plain theme background). After a video at `i` fails to open, call again with `i + 1`.
pub fn resolve_backdrop(candidates: &[BackdropCandidate], from: usize, exists: impl Fn(&str) -> bool) -> Option<usize> {
    candidates.iter().enumerate().skip(from).find(|(_, c)| exists(&c.path)).map(|(i, _)| i)
}

/// The `cover` fit: the size (same units as `container`) that fills `container` with media of
/// size `media`, aspect kept, the overflow cropped by the caller (centred). A degenerate size
/// (zero or negative on either axis) gives `container` (stretch), never a NaN.
pub fn cover_size(container: (f32, f32), media: (f32, f32)) -> (f32, f32) {
    let (cw, ch) = container;
    let (mw, mh) = media;
    if !(cw > 0.0 && ch > 0.0 && mw > 0.0 && mh > 0.0) {
        return container;
    }
    let scale = (cw / mw).max(ch / mh);
    (mw * scale, mh * scale)
}

/// The `contain` fit (letterbox): the size (same units as `container`) of media of size `media`
/// shown whole inside `container`, aspect kept, centred by the caller over black. A degenerate
/// size gives `container` (stretch), never a NaN.
pub fn contain_size(container: (f32, f32), media: (f32, f32)) -> (f32, f32) {
    let (cw, ch) = container;
    let (mw, mh) = media;
    if !(cw > 0.0 && ch > 0.0 && mw > 0.0 && mh > 0.0) {
        return container;
    }
    let scale = (cw / mw).min(ch / mh);
    (mw * scale, mh * scale)
}
