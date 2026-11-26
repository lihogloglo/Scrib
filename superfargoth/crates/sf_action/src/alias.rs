//! Per-character slot aliases (`docs/specs/action.md` "Per-character slot aliases", `AN26c`): a rule
//! names fixed slots (`interact.sit.enter`), the clip to play depends on what the character uses
//! (`seat.chair.b48.enter.left`). [`SlotAliases`] maps a rule slot to a clip slot; [`Aliased`] is the
//! [`SlotSource`] the step reads timing and root paths through.

use crate::{RootPath, SlotInfo, SlotKey, SlotSource};

/// A small ordered map "rule slot -> clip slot" (a handful of entries per character: a linear scan).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SlotAliases {
    entries: Vec<(SlotKey, SlotKey)>,
}

impl SlotAliases {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Binds `rule` to `clip`; `true` if that changed the map.
    pub fn bind(&mut self, rule: SlotKey, clip: SlotKey) -> bool {
        match self.entries.iter_mut().find(|(r, _)| *r == rule) {
            Some((_, c)) if *c == clip => false,
            Some((_, c)) => {
                *c = clip;
                true
            }
            None => {
                self.entries.push((rule, clip));
                true
            }
        }
    }

    /// Removes `rule`'s alias; `true` if it had one.
    pub fn unbind(&mut self, rule: SlotKey) -> bool {
        let before = self.entries.len();
        self.entries.retain(|(r, _)| *r != rule);
        self.entries.len() != before
    }

    /// The clip slot `rule` is bound to, if any.
    #[must_use]
    pub fn get(&self, rule: SlotKey) -> Option<SlotKey> {
        self.entries.iter().find(|(r, _)| *r == rule).map(|&(_, c)| c)
    }

    /// The clip slot to read for `key`: its alias, else `key` itself.
    #[must_use]
    pub fn resolve(&self, key: SlotKey) -> SlotKey {
        self.get(key).unwrap_or(key)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// The bindings `(rule, clip)`, in bind order.
    pub fn iter(&self) -> impl Iterator<Item = (SlotKey, SlotKey)> + '_ {
        self.entries.iter().copied()
    }
}

/// A [`SlotSource`] seen through a character's [`SlotAliases`]: an aliased slot answers with its
/// clip slot's [`SlotInfo`] and root path, any other slot as `base` does.
#[derive(Clone, Copy)]
pub struct Aliased<'a> {
    pub base: &'a dyn SlotSource,
    pub aliases: &'a SlotAliases,
}

impl SlotSource for Aliased<'_> {
    fn slot(&self, key: SlotKey) -> Option<&SlotInfo> {
        self.base.slot(self.aliases.resolve(key))
    }

    fn root(&self, key: SlotKey) -> Option<&dyn RootPath> {
        self.base.root(self.aliases.resolve(key))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::{ClockTime, SlotTable};

    struct Half;

    impl RootPath for Half {
        fn duration(&self) -> f32 {
            0.5
        }

        fn root_at(&self, _t: f32) -> sf_anim::RootPose {
            sf_anim::RootPose::default()
        }
    }

    fn table() -> SlotTable {
        SlotTable::new()
            .with("loco.move", SlotInfo::looping())
            .with("seat.chair.b48.enter.front", SlotInfo::once(0.75))
            .with_root("seat.chair.b48.enter.front", Arc::new(Half))
    }

    #[test]
    fn an_aliased_slot_answers_with_the_clip_slot() {
        let base = table();
        let mut aliases = SlotAliases::new();
        aliases.bind(SlotKey::of("interact.sit.enter"), SlotKey::of("seat.chair.b48.enter.front"));
        let src = Aliased { base: &base, aliases: &aliases };
        let info = src.slot(SlotKey::of("interact.sit.enter")).expect("aliased slot");
        assert_eq!(info.length, Some(ClockTime::from_secs(0.75)));
        let root = src.root(SlotKey::of("interact.sit.enter")).expect("aliased root");
        assert!((root.duration() - 0.5).abs() < f32::EPSILON);
        // The base itself has no such slot.
        assert!(base.slot(SlotKey::of("interact.sit.enter")).is_none());
    }

    #[test]
    fn an_unaliased_slot_passes_through() {
        let base = table();
        let mut aliases = SlotAliases::new();
        aliases.bind(SlotKey::of("interact.sit.enter"), SlotKey::of("seat.chair.b48.enter.front"));
        let src = Aliased { base: &base, aliases: &aliases };
        assert_eq!(src.slot(SlotKey::of("loco.move")), base.slot(SlotKey::of("loco.move")));
        assert!(src.slot(SlotKey::of("loco.move")).is_some());
        assert!(src.root(SlotKey::of("loco.move")).is_none());
        assert!(src.slot(SlotKey::of("air.fall")).is_none());
    }

    #[test]
    fn an_alias_to_a_missing_slot_answers_none() {
        let base = table();
        let mut aliases = SlotAliases::new();
        aliases.bind(SlotKey::of("loco.move"), SlotKey::of("seat.bench.b40.loop"));
        let src = Aliased { base: &base, aliases: &aliases };
        assert!(src.slot(SlotKey::of("loco.move")).is_none());
        assert!(src.root(SlotKey::of("loco.move")).is_none());
    }

    #[test]
    fn bind_and_unbind_report_changes() {
        let (rule, a, b) = (SlotKey::of("interact.sit.loop"), SlotKey::of("seat.chair.b48.loop"), SlotKey::of("seat.stool.b60.loop"));
        let mut aliases = SlotAliases::new();
        assert!(aliases.is_empty());
        assert!(aliases.bind(rule, a));
        assert!(!aliases.bind(rule, a), "same binding is no change");
        assert!(aliases.bind(rule, b), "rebinding changes");
        assert_eq!(aliases.get(rule), Some(b));
        assert_eq!(aliases.resolve(rule), b);
        assert_eq!(aliases.resolve(a), a, "unbound key resolves to itself");
        assert_eq!(aliases.iter().collect::<Vec<_>>(), vec![(rule, b)]);
        assert!(aliases.unbind(rule));
        assert!(!aliases.unbind(rule), "nothing left to unbind");
        assert!(aliases.is_empty());
        assert_eq!(aliases.get(rule), None);
    }
}
