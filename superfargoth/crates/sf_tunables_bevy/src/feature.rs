//! Feature switches in Bevy (slice FS-1b; spec `docs/specs/feature-switches.md`).
//!
//! - [`FeatureSwitches`]: the core registry ([`sf_tunables::FeatureSwitches`]) as a resource, with
//!   each switch's running state cached as of the last refresh.
//! - Plugins register with [`TunablesAppExt::add_feature_switch`](crate::TunablesAppExt) after
//!   the switch's tunable; the app marks a feature it didn't build with
//!   [`TunablesAppExt::mark_feature_not_built`](crate::TunablesAppExt) (or
//!   `FeatureSwitches::mark_not_built` on the resource at run time).
//! - [`feature_on`]: run condition, the feature runs (on and available); a cached read, no
//!   allocation.
//! - [`FeatureToggled`]: a switch's running state changed (its own tunable, a required switch, or
//!   a not-built mark), sent in [`TunableSystems::Apply`](crate::TunableSystems) after
//!   [`TunableChanged`].
//!
//! The cache is seeded in `PreStartup` (after every plugin's `finish`, so command-line values
//! are in) without messages; requirements that name no registered switch are one warning in
//! `PostStartup`.

use std::ops::{Deref, DerefMut};

use bevy_ecs::prelude::*;
use bevy_log::warn;

use crate::{TunableChanged, Tunables};

/// Every registered feature switch: the core registry as a resource. Reads go through `Deref`
/// (`state(&tunables, name)`); `mark_not_built` through `DerefMut` is picked up on the next
/// `PreUpdate`.
#[derive(Resource, Default, Debug)]
pub struct FeatureSwitches {
    inner: sf_tunables::FeatureSwitches,
    /// Running state per switch, in registration order, as of the last refresh.
    running: Vec<bool>,
}

impl Deref for FeatureSwitches {
    type Target = sf_tunables::FeatureSwitches;
    fn deref(&self) -> &sf_tunables::FeatureSwitches {
        &self.inner
    }
}

impl DerefMut for FeatureSwitches {
    fn deref_mut(&mut self) -> &mut sf_tunables::FeatureSwitches {
        &mut self.inner
    }
}

impl FeatureSwitches {
    /// The running state of the switch at `index` (registration order) as of the last refresh;
    /// `false` before the first one.
    pub fn running_cached(&self, index: usize) -> bool {
        self.running.get(index).copied().unwrap_or(false)
    }

    /// The registration index of switch `name` (case-insensitive), without allocating.
    pub fn index_of(&self, name: &str) -> Option<usize> {
        self.inner
            .iter()
            .position(|s| s.name().eq_ignore_ascii_case(name))
    }

    /// Recomputes every running state; `toggled` gets each one that changed. Switches new since
    /// the last refresh are cached without a call.
    fn refresh(&mut self, store: &Tunables, mut toggled: impl FnMut(&'static str, bool)) {
        for (i, switch) in self.inner.iter().enumerate() {
            let now = self.inner.running(store, switch.name());
            match self.running.get_mut(i) {
                Some(was) if *was != now => {
                    *was = now;
                    toggled(switch.name(), now);
                }
                Some(_) => {}
                None => self.running.push(now),
            }
        }
    }
}

/// A feature switch's running state (on and available) changed: its tunable's effective value,
/// a required switch, or a not-built mark. One message per switch per change.
#[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
pub struct FeatureToggled {
    pub name: &'static str,
    pub running: bool,
}

/// Run condition: feature `name` runs (on and available), as of this frame's
/// [`TunableSystems::Apply`](crate::TunableSystems); false for an unregistered name or without
/// [`FeatureSwitches`]. `app.add_systems(Update, inject_froxels.run_if(feature_on("sky.froxels.enabled")))`.
pub fn feature_on(
    name: &'static str,
) -> impl FnMut(Option<Res<FeatureSwitches>>, Local<Option<usize>>) -> bool + Clone {
    move |switches, mut index| {
        let Some(switches) = switches else {
            return false;
        };
        if index.is_none() {
            *index = switches.index_of(name);
        }
        index.is_some_and(|i| switches.running_cached(i))
    }
}

/// `PreStartup`: caches every running state, no messages.
pub(crate) fn seed_feature_states(tunables: Res<Tunables>, mut switches: ResMut<FeatureSwitches>) {
    switches
        .bypass_change_detection()
        .refresh(&tunables, |_, _| {});
}

/// `PostStartup`: one warning for requirements that name no registered switch (they read as
/// blocked).
pub(crate) fn warn_unresolved_requires(switches: Res<FeatureSwitches>) {
    let missing = switches.unresolved_requires();
    if !missing.is_empty() {
        let list: Vec<String> = missing
            .iter()
            .map(|(switch, required)| format!("{switch} requires {required}"))
            .collect();
        warn!(
            "feature switches: {} unresolved requirement(s), read as blocked: {}",
            missing.len(),
            list.join(", ")
        );
    }
}

/// [`TunableSystems::Apply`](crate::TunableSystems), after the changes went out: on a frame with
/// tunable changes or a registry change (not-built mark), sends [`FeatureToggled`] for every
/// switch whose running state changed.
pub(crate) fn report_toggles(
    mut changed: MessageReader<TunableChanged>,
    tunables: Res<Tunables>,
    mut switches: ResMut<FeatureSwitches>,
    mut toggled: MessageWriter<FeatureToggled>,
) {
    let any_change = changed.read().count() > 0;
    if !any_change && !switches.is_changed() {
        return;
    }
    // Bypass: refreshing the cache isn't a change of the registry.
    switches
        .bypass_change_detection()
        .refresh(&tunables, |name, running| {
            toggled.write(FeatureToggled { name, running });
        });
}
