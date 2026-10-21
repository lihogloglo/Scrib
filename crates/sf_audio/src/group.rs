//! Sound groups (audio.md "Cues"): one cue id names a set of variants with a bus, attenuation,
//! random volume/pitch and an instance cap, the "random container" of Wwise / FMOD multi-sound
//! / UE Sound Cue Random. Gameplay asks for a cue; the group picks the file.

use std::collections::HashMap;

use sf_records::RecordId;

use crate::{Attenuation, Bus, Rng};

/// One choice in a group: a sound id (resolved through the `SoundBank`) and its relative weight.
#[derive(Clone, Debug, PartialEq)]
pub struct Variant {
    pub sound: RecordId,
    pub weight: f32,
}

/// A cue's playback rules.
#[derive(Clone, Debug, PartialEq)]
pub struct SoundGroup {
    pub variants: Vec<Variant>,
    pub bus: Bus,
    /// `None` plays non-positional (UI, stingers); `Some` is a 3D sound with this curve.
    pub attenuation: Option<Attenuation>,
    /// Random volume multiplier range (uniform), e.g. `[0.9, 1.0]`.
    pub volume: [f32; 2],
    /// Random playback-rate range (uniform; 1 = unchanged), e.g. `[0.95, 1.05]`.
    pub pitch: [f32; 2],
    /// At most this many instances of the cue at once (0 = no cap). New plays over the cap are
    /// dropped (the industry default "kill newest"; footsteps and impacts would otherwise stack).
    pub max_instances: u16,
    /// Never pick the variant played last time (when there are two or more).
    pub no_repeat: bool,
}

impl Default for SoundGroup {
    fn default() -> Self {
        Self {
            variants: Vec::new(),
            bus: Bus::Sfx,
            attenuation: Some(Attenuation::default()),
            volume: [1.0, 1.0],
            pitch: [1.0, 1.0],
            max_instances: 0,
            no_repeat: true,
        }
    }
}

impl SoundGroup {
    /// A group of equally likely sounds with the defaults.
    pub fn of(sounds: impl IntoIterator<Item = RecordId>) -> Self {
        Self { variants: sounds.into_iter().map(|sound| Variant { sound, weight: 1.0 }).collect(), ..Self::default() }
    }

    /// Picks a variant index by weight, skipping `last` when `no_repeat` and another variant
    /// has weight. `None` for an empty (or all-zero) group.
    pub fn pick(&self, rng: &mut Rng, last: Option<usize>) -> Option<usize> {
        let skip = |i: usize| self.no_repeat && Some(i) == last && self.variants.len() > 1;
        let weight = |(i, v): (usize, &Variant)| if skip(i) { 0.0 } else { v.weight.max(0.0) };
        let total: f32 = self.variants.iter().enumerate().map(weight).sum();
        if total <= 0.0 {
            // Everything but `last` has zero weight: replay `last` rather than go silent.
            return last.filter(|&i| i < self.variants.len() && self.variants[i].weight > 0.0);
        }
        let mut r = rng.unit() * total;
        let mut chosen = None;
        for (i, v) in self.variants.iter().enumerate() {
            let w = weight((i, v));
            if w <= 0.0 {
                continue;
            }
            chosen = Some(i);
            if r < w {
                break;
            }
            r -= w;
        }
        chosen
    }
}

/// A resolved play: which sound, on which bus, how loud and how fast.
#[derive(Clone, Debug, PartialEq)]
pub struct Cue {
    pub cue: RecordId,
    pub sound: RecordId,
    pub bus: Bus,
    pub attenuation: Option<Attenuation>,
    /// Random volume multiplier for this instance (times the file's own volume later).
    pub volume: f32,
    pub pitch: f32,
    pub max_instances: u16,
}

/// Cue id -> group, filled by the content adapter. Ids that are not groups play as a single
/// sound with `default_group`'s rules, so every bank sound is also a cue.
#[derive(Clone, Debug)]
pub struct CueBank {
    pub groups: HashMap<RecordId, SoundGroup>,
    pub default_group: SoundGroup,
    last: HashMap<RecordId, usize>,
    rng: Rng,
}

impl Default for CueBank {
    fn default() -> Self {
        Self { groups: HashMap::new(), default_group: SoundGroup::default(), last: HashMap::new(), rng: Rng::new(0xA0D1) }
    }
}

impl CueBank {
    pub fn insert(&mut self, cue: RecordId, group: SoundGroup) {
        self.groups.insert(cue, group);
    }

    /// Picks a variant and the instance's random volume and pitch. `None` for an empty group.
    pub fn resolve(&mut self, cue: &RecordId) -> Option<Cue> {
        let (group, sound) = if let Some(group) = self.groups.get(cue) {
            let i = group.pick(&mut self.rng, self.last.get(cue).copied())?;
            self.last.insert(cue.clone(), i);
            (group, group.variants[i].sound.clone())
        } else {
            (&self.default_group, cue.clone())
        };
        Some(Cue {
            cue: cue.clone(),
            sound,
            bus: group.bus,
            attenuation: group.attenuation,
            volume: self.rng.range(group.volume[0], group.volume[1]).max(0.0),
            pitch: self.rng.range(group.pitch[0], group.pitch[1]).max(0.01),
            max_instances: group.max_instances,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(s: &str) -> RecordId {
        RecordId::new(s)
    }

    #[test]
    fn no_repeat_never_plays_the_same_variant_twice() {
        let g = SoundGroup::of([id("a"), id("b"), id("c")]);
        let mut rng = Rng::new(9);
        let mut last = None;
        let mut seen = [0; 3];
        for _ in 0..600 {
            let i = g.pick(&mut rng, last).unwrap();
            assert_ne!(Some(i), last);
            seen[i] += 1;
            last = Some(i);
        }
        assert!(seen.iter().all(|&n| n > 150), "{seen:?}");
    }

    #[test]
    fn weights_bias_the_pick_and_zero_weight_is_never_picked() {
        let mut g = SoundGroup::of([id("a"), id("b"), id("c")]);
        g.no_repeat = false;
        g.variants[0].weight = 3.0;
        g.variants[2].weight = 0.0;
        let mut rng = Rng::new(4);
        let mut seen = [0; 3];
        for _ in 0..4000 {
            seen[g.pick(&mut rng, None).unwrap()] += 1;
        }
        assert_eq!(seen[2], 0);
        assert!((f64::from(seen[0]) / 4000.0 - 0.75).abs() < 0.04, "{seen:?}");
    }

    #[test]
    fn single_and_empty_groups() {
        let one = SoundGroup::of([id("a")]);
        let mut rng = Rng::new(1);
        assert_eq!(one.pick(&mut rng, Some(0)), Some(0));
        assert_eq!(SoundGroup::default().pick(&mut rng, None), None);
    }

    #[test]
    fn bank_resolves_groups_and_falls_back_to_plain_sounds() {
        let mut bank = CueBank::default();
        let mut g = SoundGroup::of([id("step1"), id("step2")]);
        g.bus = Bus::Sfx;
        g.volume = [0.5, 0.7];
        g.pitch = [0.9, 1.1];
        g.max_instances = 4;
        bank.insert(id("footstep"), g);
        let mut prev = None;
        for _ in 0..20 {
            let c = bank.resolve(&id("footstep")).unwrap();
            assert!(c.sound == id("step1") || c.sound == id("step2"));
            assert_ne!(Some(c.sound.clone()), prev);
            assert!((0.5..=0.7).contains(&c.volume) && (0.9..=1.1).contains(&c.pitch));
            assert_eq!(c.max_instances, 4);
            prev = Some(c.sound);
        }
        let plain = bank.resolve(&id("door_open")).unwrap();
        assert_eq!(plain.sound, id("door_open"));
        assert_eq!(plain.bus, Bus::Sfx);
        assert!(plain.attenuation.is_some());
        bank.insert(id("empty"), SoundGroup::default());
        assert!(bank.resolve(&id("empty")).is_none());
    }
}
