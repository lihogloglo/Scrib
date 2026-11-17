//! The camera's medium (water.md "Underwater", first bullet; research
//! `underwater-2026-11-17.md` "Camera medium decision"): per frame, the water query at the near
//! plane's four corners and centre decides whether the lens is above the water, under it, or split
//! by the surface. All five below is [`MediumState::Under`], mixed is [`MediumState::Split`], all
//! above (or no water) is [`MediumState::Above`]. A hysteresis band on the height difference keeps a
//! point riding the surface from flickering (Crest's 1 cm, the host's tunable).
//!
//! The near plane, not the eye: the image is formed there, so a camera whose eye is just above the
//! surface while the bottom of its near plane dips under it is already split. The medium also
//! carries the body (its optics colour the medium), the camera's depth below the local surface and
//! the cues for audio and gameplay (`underwater` entered / left, `surface_break` on each crossing).

use glam::{DVec2, Quat, Vec3};
use sf_coords::WorldPos;
use sf_records::RecordId;

use crate::{OpticsMap, WaterKind, WaterSample};

/// Where the camera's lens is relative to the water.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum MediumState {
    /// Every near-plane point is above the surface (or there is no water).
    #[default]
    Above,
    /// The surface crosses the near plane: the image has a waterline.
    Split,
    /// Every near-plane point is below the surface.
    Under,
}

impl MediumState {
    /// The GPU's code: 0 above, 1 split, 2 under.
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        match self {
            Self::Above => 0,
            Self::Split => 1,
            Self::Under => 2,
        }
    }
}

/// The near plane in world space: its four corners, then its centre ([`NearPlane::CENTRE`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NearPlane {
    pub points: [WorldPos; 5],
}

impl NearPlane {
    /// Index of the centre point in [`Self::points`].
    pub const CENTRE: usize = 4;

    /// From the eye, the view's rotation and the near plane's corners in view space (Bevy's
    /// the inverse projection at the near plane; −Z forward). The centre is their mean.
    #[must_use]
    pub fn from_view(eye: WorldPos, rotation: Quat, corners: [Vec3; 4]) -> Self {
        let centre = corners.iter().copied().sum::<Vec3>() * 0.25;
        let at = |v: Vec3| eye.offset(rotation * v);
        Self { points: [at(corners[0]), at(corners[1]), at(corners[2]), at(corners[3]), at(centre)] }
    }

    /// A symmetric perspective: the near plane `near` metres ahead, vertical field of view `fov_y`
    /// (radians), `aspect` width over height.
    #[must_use]
    pub fn perspective(eye: WorldPos, rotation: Quat, near: f32, fov_y: f32, aspect: f32) -> Self {
        let h = near * (fov_y * 0.5).tan();
        let w = h * aspect;
        Self::from_view(
            eye,
            rotation,
            [Vec3::new(w, -h, -near), Vec3::new(w, h, -near), Vec3::new(-w, h, -near), Vec3::new(-w, -h, -near)],
        )
    }

    /// All five points at the eye (a camera without a projection).
    #[must_use]
    pub fn point(eye: WorldPos) -> Self {
        Self { points: [eye; 5] }
    }
}

/// This frame's cues (water.md "Exposure and cues"); `None` = no cue.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MediumCues {
    /// `underwater`: `Some(true)` when the eye (the near plane's centre) went under, `Some(false)`
    /// when it came out. The audio system's low-pass follows it.
    pub underwater: Option<bool>,
    /// `surface_break`: the eye crossed the surface this frame; `Some(true)` going down.
    pub surface_break: Option<bool>,
}

/// One camera's medium, kept across frames (the hysteresis needs the last state).
#[derive(Clone, Debug, PartialEq)]
pub struct CameraMedium {
    pub state: MediumState,
    /// The water body at the near plane's centre, else at the first point that has water.
    pub body: Option<RecordId>,
    pub kind: Option<WaterKind>,
    /// Metres below the displaced surface at the near plane's centre: negative above it,
    /// `f32::NEG_INFINITY` with no water there.
    pub depth: f32,
    /// Water column (surface − bed) at the centre, metres; infinite where the bed is unknown (a
    /// deep-water colour), 0 with no water.
    pub column: f32,
    /// The eye is under the surface (the `underwater` cue's state, with the same hysteresis).
    pub submerged: bool,
    /// [`Self::state`] changed this frame.
    pub changed: bool,
    /// World XZ of the near plane's centre (the optics region lookup).
    pub centre_xz: DVec2,
    started: bool,
}

impl Default for CameraMedium {
    fn default() -> Self {
        Self {
            state: MediumState::Above,
            body: None,
            kind: None,
            depth: f32::NEG_INFINITY,
            column: 0.0,
            submerged: false,
            changed: false,
            centre_xz: DVec2::ZERO,
            started: false,
        }
    }
}

impl CameraMedium {
    /// This frame's medium from the near plane and the water query (`sample`: the active
    /// worldspace's [`crate::WaterQuery::sample`]). `hysteresis_m` ≥ 0: a point is under once it is
    /// more than this below the surface and above once more than this above it; in between it keeps
    /// the last state. The first update has no last state and classifies at the surface itself.
    pub fn update(&mut self, plane: &NearPlane, sample: impl Fn(WorldPos) -> Option<WaterSample>, hysteresis_m: f32) -> MediumCues {
        let samples = plane.points.map(&sample);
        // Height below the surface per point (positive under), −∞ with no water.
        let below: [f64; 5] = std::array::from_fn(|i| {
            samples[i].as_ref().map_or(f64::NEG_INFINITY, |s| s.height - plane.points[i].to_f64().y)
        });
        let h = if self.started { f64::from(hysteresis_m.max(0.0)) } else { 0.0 };
        let under = |d: f64| if self.started { d > h } else { d > 0.0 };
        let over = |d: f64| if self.started { d < -h } else { d <= 0.0 };
        let all_under = below.iter().all(|d| under(*d));
        let all_over = below.iter().all(|d| over(*d));
        let any_under = below.iter().any(|d| under(*d));
        let any_over = below.iter().any(|d| over(*d));
        let prev = self.state;
        // Every point past the band decides alone; otherwise one point past it leaves the last
        // state for Split, and in-band points keep it.
        let next = if all_under {
            MediumState::Under
        } else if all_over {
            MediumState::Above
        } else if self.started {
            match prev {
                MediumState::Above if any_under => MediumState::Split,
                MediumState::Under if any_over => MediumState::Split,
                s => s,
            }
        } else {
            MediumState::Split
        };
        let c = NearPlane::CENTRE;
        let picked = samples[c].as_ref().or_else(|| samples.iter().flatten().next());
        #[allow(clippy::cast_possible_truncation)]
        let depth = below[c] as f32;
        let submerged = match next {
            MediumState::Under => true,
            MediumState::Above => false,
            MediumState::Split if !self.started => below[c] > 0.0,
            MediumState::Split if self.submerged => !over(below[c]),
            MediumState::Split => under(below[c]),
        };
        let mut cues = MediumCues::default();
        if submerged != self.submerged {
            cues.underwater = Some(submerged);
            if self.started {
                cues.surface_break = Some(submerged);
            }
        }
        let p = plane.points[c].to_f64();
        *self = Self {
            state: next,
            body: picked.map(|s| s.body.clone()),
            kind: picked.map(|s| s.kind),
            depth,
            column: samples[c].as_ref().map_or(0.0, |s| s.depth),
            submerged,
            changed: if self.started { next != prev } else { next != MediumState::Above },
            centre_xz: DVec2::new(p.x, p.z),
            started: true,
        };
        cues
    }

    /// The switch is off (`water.underwater`): always above. Says `underwater` left if the eye was
    /// under; the next [`Self::update`] starts afresh.
    pub fn force_above(&mut self) -> MediumCues {
        let cues = MediumCues { underwater: self.submerged.then_some(false), surface_break: None };
        let changed = self.state != MediumState::Above;
        *self = Self { changed, ..Self::default() };
        cues
    }
}

/// The optics table index at world XZ `p` (the nearest cell of the map; outside it, 0), as the
/// GPU's bilinear blend reads it at a cell centre.
#[must_use]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
pub fn optics_index_at(map: &OpticsMap, p: DVec2) -> u8 {
    let q = ((p - map.origin) / map.cell).round();
    if q.x < 0.0 || q.y < 0.0 || q.x >= map.width as f64 || q.y >= map.height as f64 {
        return 0;
    }
    map.index.get(q.y as usize * map.width + q.x as usize).copied().unwrap_or(0)
}
