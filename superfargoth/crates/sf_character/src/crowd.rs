//! Crowd tiers by projected size (superfargoth `docs/specs/crowd-lod.md`): the band a character
//! would take, and the CROWD-1 far-hide rule (`C0` threshold with the spec's hysteresis). Bevy-free;
//! the tier machinery (dwell, swap budget, proxies) is CROWD-2.

/// Reference pixels (projected height at a 1440 px view) a character needs for `C0` (full rig).
/// A 1.8 m figure at 70° vertical FOV: ≈ 93 m.
pub const C0_PIXELS: f32 = 20.0;
/// Reference pixels below which a character is `C2` (hidden). 1.8 m figure: ≈ 460 m.
pub const C1_PIXELS: f32 = 4.0;
/// Promotion margin over a threshold (the spec's 15%, the A-tiers' value): back to `C0` above
/// 23 px (≈ 81 m).
pub const HYSTERESIS: f32 = 0.15;

/// The crowd tier a character's size puts it in.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CrowdBand {
    /// Full rig: at least [`C0_PIXELS`], or pinned.
    C0,
    /// Rigid proxy: [`C1_PIXELS`] up to [`C0_PIXELS`].
    C1,
    /// Hidden: below [`C1_PIXELS`].
    C2,
}

impl CrowdBand {
    pub const ALL: [Self; 3] = [Self::C0, Self::C1, Self::C2];

    /// The plain band for `pixels` (no hysteresis: what the gauges count).
    pub fn of(pixels: f32, pinned: bool) -> Self {
        if pinned || pixels >= C0_PIXELS {
            Self::C0
        } else if pixels >= C1_PIXELS {
            Self::C1
        } else {
            Self::C2
        }
    }

    pub fn index(self) -> usize {
        self as usize
    }
}

/// Whether the far-hide switch (`C0` or nothing) keeps a character hidden: below [`C0_PIXELS`]
/// it hides, and once hidden it shows again only above `C0_PIXELS × (1 + HYSTERESIS)`. Pinned
/// characters always show.
pub fn far_hidden(pixels: f32, pinned: bool, hidden: bool) -> bool {
    if pinned {
        return false;
    }
    let threshold = if hidden { C0_PIXELS * (1.0 + HYSTERESIS) } else { C0_PIXELS };
    pixels < threshold
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bands_split_at_the_spec_thresholds_and_pins_force_c0() {
        assert_eq!(CrowdBand::of(20.0, false), CrowdBand::C0);
        assert_eq!(CrowdBand::of(19.9, false), CrowdBand::C1);
        assert_eq!(CrowdBand::of(4.0, false), CrowdBand::C1);
        assert_eq!(CrowdBand::of(3.9, false), CrowdBand::C2);
        assert_eq!(CrowdBand::of(0.0, true), CrowdBand::C0);
    }

    #[test]
    fn far_hide_holds_in_the_hysteresis_band() {
        assert!(far_hidden(19.0, false, false));
        assert!(!far_hidden(21.0, false, false), "shown stays shown above 20 px");
        assert!(far_hidden(22.0, false, true), "hidden stays hidden below 23 px");
        assert!(!far_hidden(23.5, false, true));
        assert!(!far_hidden(1.0, true, true), "pinned always shows");
    }
}
