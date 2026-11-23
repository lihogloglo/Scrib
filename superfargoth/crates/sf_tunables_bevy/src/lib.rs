//! Bevy plugin for [`sf_tunables`]. Spec: `docs/specs/tunables.md` "Live in Bevy".
//!
//! - [`Tunables`]: the registry as a resource, the one store (the console's variables live in
//!   it too, `sf_console_bevy`). Reads by [`Id`] / [`TunableId`] are an index, fine every frame.
//! - Plugins register through [`TunablesAppExt`] (in any order relative to [`TunablesPlugin`]):
//!   `app.register_tunables(TUNABLES)`, or `app.add_tunable(&DEF)` and **bind** it into a field
//!   of a settings resource or component the systems already read
//!   (`.bind_resource(|b: &mut Budget, v| b.fade_start = v)`). A binding writes only when the
//!   effective value changed, so Bevy's change detection fires once per change.
//! - Anyone writes [`SetTunable`] / [`PeelTunable`] (a layer's value by name, as text); they apply
//!   in `PreUpdate` ([`TunableSystems::Apply`]), then every effective change of the last frame goes
//!   out as [`TunableChanged`], then bindings run ([`TunableSystems::Bind`]). A refused request is
//!   a warning and a [`TunableRefused`].
//! - A def's deprecated environment alias (`Def::env_alias`) is read when it is registered and
//!   set at the `CommandLine` layer ([`TunableEnv`]); a value that doesn't parse goes to
//!   [`OverrideRefusals`], which the app checks before the first frame.
//! - [`tunable_changed`]: run condition, true once per change (and on its first run).
//! - Feature switches (slice FS-1b): [`FeatureSwitches`] resource,
//!   [`TunablesAppExt::add_feature_switch`], the [`feature_on`] run condition and
//!   [`FeatureToggled`] (`docs/specs/feature-switches.md`).
//! - [`overrides`]: `--set name=value` and `--tunables <file.toml>` parsing (pure; app wiring is T3).
//! - [`brp`]: the `sf.tunables/list|get|set|reset` handlers as pure JSON functions; the `remote`
//!   feature registers them with Bevy Remote.
//! - [`presets`]: presets as data (slice T5b): [`PresetsPlugin`] applies the `sf.graphics_preset` /
//!   `sg.*` driver levels a host loaded from `sf.tunable_preset` records ([`Presets`]).

use std::ops::{Deref, DerefMut};

use bevy_app::{App, Plugin, PostStartup, PreStartup, PreUpdate};
use bevy_ecs::change_detection::DetectChanges;
use bevy_ecs::prelude::*;
use bevy_ecs::query::QueryFilter;
use bevy_log::{info, warn};

pub use sf_tunables;
pub use sf_tunables::{AnyDef, Def, Id, SetBy, SetOutcome, TunableId, Type, Value};

pub mod brp;
mod feature;
pub mod overrides;
pub mod presets;

pub use feature::{FeatureSwitches, FeatureToggled, feature_on};
pub use presets::{PresetApplied, Presets, PresetsPlugin};
pub use sf_tunables::{
    Availability, Category, Cost, Effect, FeatureSwitch, OffValue, SwitchError, SwitchState,
};

#[cfg(test)]
mod tests;

/// Adds the [`Tunables`] resource, the request and change messages and their systems.
pub struct TunablesPlugin;

impl Plugin for TunablesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Tunables>()
            .init_resource::<FeatureSwitches>()
            .add_message::<SetTunable>()
            .add_message::<PeelTunable>()
            .add_message::<TunableChanged>()
            .add_message::<TunableRefused>()
            .add_message::<FeatureToggled>()
            .configure_sets(
                PreUpdate,
                (TunableSystems::Apply, TunableSystems::Bind).chain(),
            )
            .add_systems(
                PreUpdate,
                (apply_requests, report_changes, feature::report_toggles)
                    .chain()
                    .in_set(TunableSystems::Apply),
            )
            .add_systems(PreStartup, feature::seed_feature_states)
            .add_systems(PostStartup, feature::warn_unresolved_requires);
    }

    #[cfg(feature = "remote")]
    fn finish(&self, app: &mut App) {
        brp::register_remote_methods(app);
    }
}

/// Every registered tunable: the core store as a resource.
#[derive(Resource, Default, Debug)]
pub struct Tunables(pub sf_tunables::Tunables);

impl Deref for Tunables {
    type Target = sf_tunables::Tunables;
    fn deref(&self) -> &sf_tunables::Tunables {
        &self.0
    }
}

impl DerefMut for Tunables {
    fn deref_mut(&mut self) -> &mut sf_tunables::Tunables {
        &mut self.0
    }
}

/// Where requests apply and changes are reported ([`TunableSystems::Apply`]), then where bindings
/// copy values into their resources and components ([`TunableSystems::Bind`]); both in
/// `PreUpdate`, so a value set this frame reaches this frame's `Update`. The console runs before
/// `Apply` (`sf_console_bevy`).
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TunableSystems {
    Apply,
    Bind,
}

/// Sets one layer of a tunable, by name, from text (parsed and validated by its kind and range;
/// on/off, `a,b` for pairs). A set under a stronger layer is stored and takes effect when that
/// layer is peeled.
#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct SetTunable {
    pub name: String,
    pub value: String,
    pub layer: SetBy,
    /// Who set it and why (shown by the dump).
    pub note: Option<String>,
}

impl SetTunable {
    pub fn new(name: impl Into<String>, value: impl Into<String>, layer: SetBy) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            layer,
            note: None,
        }
    }

    #[must_use]
    pub fn note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }
}

/// Peels a tunable's layer by name: `Some(layer)` clears that one (the next one down takes
/// effect), `None` clears every layer but the default.
#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct PeelTunable {
    pub name: String,
    pub layer: Option<SetBy>,
}

/// A tunable's effective value changed (one message per tunable per frame, in order of first
/// change). Read the value through [`Tunables`].
#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct TunableChanged {
    pub id: TunableId,
    pub name: String,
}

/// A [`SetTunable`] or [`PeelTunable`] that was refused (unknown name, value out of range or of
/// the wrong kind, the default layer). Also logged as a warning.
#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct TunableRefused {
    pub name: String,
    pub reason: String,
}

fn apply_requests(
    mut sets: MessageReader<SetTunable>,
    mut peels: MessageReader<PeelTunable>,
    mut tunables: ResMut<Tunables>,
    mut refused: MessageWriter<TunableRefused>,
) {
    let mut refuse = |name: &str, reason: String| {
        warn!("tunable {name}: {reason}");
        refused.write(TunableRefused {
            name: name.to_owned(),
            reason,
        });
    };
    for s in sets.read() {
        let Some(id) = tunables.id(&s.name) else {
            refuse(&s.name, "no such tunable".into());
            continue;
        };
        let parsed = tunables.var(id).kind().parse(&s.value);
        let result =
            parsed.and_then(|value| tunables.set_noted(id, value, s.layer, s.note.clone()));
        if let Err(reason) = result {
            refuse(&s.name, reason);
        }
    }
    for p in peels.read() {
        let Some(id) = tunables.id(&p.name) else {
            refuse(&p.name, "no such tunable".into());
            continue;
        };
        match p.layer {
            Some(layer) => tunables.clear(id, layer),
            None => tunables.reset(id),
        };
    }
}

fn report_changes(mut tunables: ResMut<Tunables>, mut changed: MessageWriter<TunableChanged>) {
    // Bypass: taking the change list isn't a change of any value.
    let tunables = tunables.bypass_change_detection();
    if tunables.revision() == 0 {
        return;
    }
    let ids = tunables.take_changes();
    changed.write_batch(ids.into_iter().map(|id| TunableChanged {
        id,
        name: tunables.var(id).name().to_owned(),
    }));
}

/// Run condition: tunable `id` changed since this condition last ran (true on its first run, so
/// a system applying it also applies the starting value; false without [`Tunables`]).
/// `app.add_systems(Update, apply_ssao.run_if(tunable_changed(ssao)))`.
pub fn tunable_changed(
    id: impl Into<TunableId>,
) -> impl FnMut(Option<Res<Tunables>>, Local<Option<u64>>) -> bool + Clone {
    let id = id.into();
    move |tunables, mut seen| {
        let Some(tunables) = tunables else {
            return false;
        };
        let generation = tunables.var(id).generation();
        let changed = *seen != Some(generation);
        *seen = Some(generation);
        changed
    }
}

/// Registration from plugins. Creates the [`Tunables`] resource if it isn't there yet.
///
/// Registering a def that is already registered (another plugin listed it, or it is in the
/// crate's `TUNABLES` and also bound) returns the same tunable; the same name with another kind
/// or default panics, as does a bad def (no range on a number, bad name): bugs in the
/// registering plugin, like a duplicate system set.
pub trait TunablesAppExt {
    /// Registers a crate's whole `TUNABLES` list.
    fn register_tunables(&mut self, defs: &[&dyn AnyDef]) -> &mut Self;

    /// Registers one def; the builder binds it and gives its typed [`Id`].
    fn add_tunable<T: Type + Sync>(&mut self, def: &Def<T>) -> TunableBuilder<'_, T>;

    /// Registers a feature switch on a tunable registered before it. The same switch twice is
    /// one switch; another switch under a taken name, or a missing or wrong-kind tunable, panics.
    fn add_feature_switch(&mut self, switch: &'static FeatureSwitch) -> &mut Self;

    /// The app did not build feature `name` this run (startup `--off`, missing hardware): it
    /// reads as not running until a restart, and so do the switches that require it. One info
    /// line; an unregistered name panics.
    fn mark_feature_not_built(&mut self, name: &str, reason: &str) -> &mut Self;
}

impl TunablesAppExt for App {
    fn register_tunables(&mut self, defs: &[&dyn AnyDef]) -> &mut Self {
        let world = self.world_mut();
        let mut tunables = world.get_resource_or_init::<Tunables>();
        let fresh: Vec<TunableId> = defs
            .iter()
            .filter_map(|def| register_or_check(&mut tunables, *def))
            .collect();
        apply_env_aliases(world, &fresh);
        self
    }

    fn add_tunable<T: Type + Sync>(&mut self, def: &Def<T>) -> TunableBuilder<'_, T> {
        let world = self.world_mut();
        let mut tunables = world.get_resource_or_init::<Tunables>();
        let fresh = register_or_check(&mut tunables, def);
        let id = tunables
            .id_of(def)
            .unwrap_or_else(|e| panic!("tunable {}: {e}", AnyDef::name(def)));
        apply_env_aliases(world, fresh.as_slice());
        TunableBuilder { app: self, id }
    }

    fn add_feature_switch(&mut self, switch: &'static FeatureSwitch) -> &mut Self {
        let world = self.world_mut();
        world.init_resource::<Tunables>();
        world.init_resource::<FeatureSwitches>();
        world.resource_scope(|world, mut switches: Mut<FeatureSwitches>| {
            let tunables = world.resource::<Tunables>();
            match switches.register(switch, tunables) {
                Ok(()) => {}
                Err(SwitchError::Duplicate(_)) if switches.get(switch.name()) == Some(switch) => {}
                Err(e) => panic!("feature switch {}: {e}", switch.name()),
            }
        });
        self
    }

    fn mark_feature_not_built(&mut self, name: &str, reason: &str) -> &mut Self {
        let mut switches = self.world_mut().get_resource_or_init::<FeatureSwitches>();
        switches
            .mark_not_built(name, reason)
            .unwrap_or_else(|e| panic!("feature switch {name}: {e}"));
        info!("feature {name} not built this run: {reason}");
        self
    }
}

/// Applies `--set` / `--tunables` overrides at the `CommandLine` layer once every plugin has
/// registered its tunables (in `Plugin::finish`, after all `build`s), sets in order (a later one
/// wins), one info line per set. Refusals (unknown names, bad values) are warnings and go to
/// [`OverrideRefusals`]: the app decides to fail the run (a plugin added after this one whose
/// `finish` checks the resource).
pub struct CommandLineOverrides(pub Vec<overrides::OverrideSet>);

impl Plugin for CommandLineOverrides {
    fn build(&self, app: &mut App) {
        app.init_resource::<OverrideRefusals>();
    }

    fn finish(&self, app: &mut App) {
        let world = app.world_mut();
        let mut refused = Vec::new();
        {
            let mut tunables = world.get_resource_or_init::<Tunables>();
            for set in &self.0 {
                let before = refused.len();
                refused.extend(
                    overrides::apply_overrides(
                        &mut tunables,
                        &set.overrides,
                        SetBy::CommandLine,
                        Some(&set.origin),
                    )
                    .into_iter()
                    .map(|r| overrides::Refusal {
                        reason: format!("{} ({})", r.reason, set.origin),
                        name: r.name,
                    }),
                );
                info!(
                    "tunables: {}: {} value(s) set, {} refused",
                    set.origin,
                    set.overrides.len() - (refused.len() - before),
                    refused.len() - before
                );
            }
        }
        for r in &refused {
            warn!("tunable {r}");
        }
        world.resource_mut::<OverrideRefusals>().0.extend(refused);
    }
}

/// Where deprecated environment aliases ([`sf_tunables::Def::env_alias`]) are read from when a
/// tunable is registered: the process environment by default; tests insert
/// [`TunableEnv::from_pairs`] before registering.
#[derive(Resource)]
pub struct TunableEnv(Box<EnvLookup>);

/// Reads one environment variable.
type EnvLookup = dyn Fn(&str) -> Option<String> + Send + Sync;

impl Default for TunableEnv {
    fn default() -> Self {
        Self(Box::new(|name| std::env::var(name).ok()))
    }
}

impl TunableEnv {
    /// A fixed environment (tests).
    pub fn from_pairs(pairs: &[(&str, &str)]) -> Self {
        let map: std::collections::HashMap<String, String> = pairs
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect();
        Self(Box::new(move |name| map.get(name).cloned()))
    }

    pub fn get(&self, name: &str) -> Option<String> {
        (self.0)(name)
    }
}

/// Overrides that were refused before the first frame: environment aliases with a value that
/// doesn't parse (at registration), and the app's `--set` / `--tunables` refusals. The app fails
/// the run when this isn't empty (a typo must not silently measure the default).
#[derive(Resource, Default, Debug, Clone, PartialEq, Eq)]
pub struct OverrideRefusals(pub Vec<overrides::Refusal>);

/// Applies the environment aliases of freshly registered tunables at the `CommandLine` layer, one
/// info line per alias in use; refusals go to [`OverrideRefusals`].
fn apply_env_aliases(world: &mut World, fresh: &[TunableId]) {
    {
        let tunables = world.get_resource_or_init::<Tunables>();
        if !fresh
            .iter()
            .any(|id| tunables.var(*id).meta().env_alias.is_some())
        {
            return;
        }
    }
    world.init_resource::<TunableEnv>();
    let mut refused = Vec::new();
    world.resource_scope(|world, env: Mut<TunableEnv>| {
        let mut tunables = world.resource_mut::<Tunables>();
        let lookup = |name: &str| env.get(name);
        for id in fresh {
            match overrides::apply_env_alias(&mut tunables, *id, &lookup, SetBy::CommandLine) {
                Ok(Some(used)) => info!(
                    "tunable {} = {} from env {} (deprecated: use --set {}={})",
                    used.name, used.value, used.var, used.name, used.value
                ),
                Ok(None) => {}
                Err(refusal) => {
                    warn!("tunable {refusal}");
                    refused.push(refusal);
                }
            }
        }
    });
    if !refused.is_empty() {
        world
            .get_resource_or_init::<OverrideRefusals>()
            .0
            .extend(refused);
    }
}

/// Registers `def`, or checks it against the tunable already registered under its name. Returns
/// the id when it was registered just now.
fn register_or_check(tunables: &mut Tunables, def: &dyn AnyDef) -> Option<TunableId> {
    let name = def.name();
    match tunables.id(name) {
        None => Some(
            tunables
                .register_any(def)
                .unwrap_or_else(|e| panic!("tunable {name}: {e}")),
        ),
        Some(id) => {
            let spec = def.spec();
            let have = tunables.var(id);
            assert!(
                have.kind() == spec.kind() && have.default() == spec.default_value(),
                "tunable {name} registered twice with different kinds or defaults: {} = {} vs {} = {}",
                have.kind().describe(),
                have.default(),
                spec.kind().describe(),
                spec.default_value(),
            );
            None
        }
    }
}

/// Binds a registered tunable ([`TunablesAppExt::add_tunable`]).
pub struct TunableBuilder<'a, T: Type> {
    app: &'a mut App,
    id: Id<T>,
}

impl<T: Type + Sync + Send> TunableBuilder<'_, T> {
    /// The typed handle, for direct reads (`tunables.read(id)`) and [`tunable_changed`].
    pub fn id(&self) -> Id<T> {
        self.id
    }

    /// Copies the effective value into resource `R` with `apply`: on the first frame `R` exists,
    /// then only when the value changed (`R` is marked changed exactly then). Systems keep
    /// reading `R` as before.
    #[must_use]
    pub fn bind_resource<R: Resource<Mutability = bevy_ecs::component::Mutable>>(
        self,
        apply: impl for<'v> Fn(&mut R, T::Read<'v>) + Send + Sync + 'static,
    ) -> Self {
        let id = self.id;
        let system = move |tunables: Res<Tunables>,
                           target: Option<ResMut<R>>,
                           mut seen: Local<Option<u64>>| {
            let generation = tunables.var(id.untyped()).generation();
            if *seen == Some(generation) {
                return;
            }
            // Without the resource, keep the change for when it appears.
            let Some(mut target) = target else {
                return;
            };
            apply(&mut *target, tunables.read(id));
            *seen = Some(generation);
        };
        self.app
            .add_systems(PreUpdate, system.in_set(TunableSystems::Bind));
        self
    }

    /// Copies the effective value into every component `C` matching filter `F` with `apply`: all
    /// of them when the value changed, and each newly added one. Idle frames walk the matches
    /// without writing (meant for a handful of entities: cameras, lights, the player).
    #[must_use]
    pub fn bind_component<C: Component<Mutability = bevy_ecs::component::Mutable>, F: QueryFilter + 'static>(
        self,
        apply: impl for<'v> Fn(&mut C, T::Read<'v>) + Send + Sync + 'static,
    ) -> Self {
        let id = self.id;
        let system = move |tunables: Res<Tunables>,
                           mut targets: Query<&mut C, F>,
                           mut seen: Local<Option<u64>>| {
            let generation = tunables.var(id.untyped()).generation();
            let changed = *seen != Some(generation);
            *seen = Some(generation);
            for mut target in &mut targets {
                if changed || target.is_added() {
                    apply(&mut *target, tunables.read(id));
                }
            }
        };
        self.app
            .add_systems(PreUpdate, system.in_set(TunableSystems::Bind));
        self
    }
}
