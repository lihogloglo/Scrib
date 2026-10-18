//! UI state in the player profile (`sf_mods::Profile`): built once from a
//! loaded profile, saved back when the player changes settings or bindings.
//!
//! - [`insert_from_profile`] resolves the registry from the profile's records,
//!   loads setting values, builds the Fluent strings from every mod's
//!   `l10n/<locale>/<mod id>.ftl` and applies rebound inputs, then inserts the
//!   resources and a [`UiProfile`] that remembers where to save.
//! - Rebound UI actions go to `[bindings.ui]` as input names (`key:Enter`,
//!   `key:Ctrl+KeyS`, `pad:South`, `axis:LeftStickX`, `mouse:Left`); only
//!   actions that differ from [`UiBindings::default`] are written.
//! - Saving follows console and PC convention: changes are written when the
//!   player leaves the menus (no Menu or Modal screen takes input), or on exit,
//!   never once per slider step.

use std::path::PathBuf;

use bevy_app::AppExit;
use bevy_ecs::prelude::*;
use bevy_enhanced_input::prelude::{Binding, ModKeys};
use bevy_input::gamepad::{GamepadAxis, GamepadButton};
use bevy_input::keyboard::KeyCode;
use bevy_input::mouse::MouseButton;
use bevy_reflect::enums::{DynamicEnum, DynamicVariant, Enum, VariantInfo, VariantType};
use bevy_reflect::{FromReflect, TypeInfo, Typed};
use sf_mods::{LoadedProfile, Profile};
use sf_ui::{
    L10nIssue, LanguageIdentifier, Layer, Localizer, SettingError, SettingsStore, UiConflict,
    UiRegistry,
};

use crate::actions::{UiAction, UiBindings};
use crate::{ScreenStackRes, UiLocalizer, UiRegistryRes, UiSettingsStore};

/// Context name of the UI actions in `Profile::bindings`.
pub const BINDINGS_CONTEXT: &str = "ui";

/// The profile the UI saves into, and where.
#[derive(Resource, Debug, Clone)]
pub struct UiProfile {
    pub profile: Profile,
    /// Profiles folder; `None` keeps changes in memory only (tests, demos).
    pub dir: Option<PathBuf>,
    /// Changes not yet written.
    pub dirty: bool,
    /// Why the last save failed, for the options screen or a log.
    pub last_error: Option<String>,
}

/// What [`insert_from_profile`] found, each item attributed where it can be.
#[derive(Debug, Default)]
pub struct UiLoadReport {
    pub conflicts: Vec<UiConflict>,
    pub strings: Vec<L10nIssue>,
    pub settings: Vec<SettingError>,
    pub bindings: Vec<String>,
}

/// Locale choice for [`insert_from_profile`].
#[derive(Debug, Clone)]
pub struct UiLocales {
    /// Player preference first (`["de-AT"]`); negotiated over the locales mods ship.
    pub requested: Vec<String>,
    /// Last fallback, normally the content pack's source language.
    pub default: LanguageIdentifier,
    /// Fluent's bidi isolation marks (U+2068/U+2069) around placeables; needs a
    /// text stack that handles them.
    pub use_isolating: bool,
}

/// Builds the UI resources from `loaded` and inserts them into `world`:
/// [`UiRegistryRes`], [`UiSettingsStore`] (profile values), [`UiLocalizer`],
/// [`UiBindings`] (profile overrides on the defaults) and [`UiProfile`].
pub fn insert_from_profile(
    world: &mut World,
    loaded: &LoadedProfile,
    locales: &UiLocales,
    dir: Option<PathBuf>,
) -> UiLoadReport {
    let (registry, conflicts) = UiRegistry::build(&loaded.records);
    let mut store = SettingsStore::new(&registry);
    let settings = store.load_from(&loaded.profile);
    let requested: Vec<&str> = locales.requested.iter().map(String::as_str).collect();
    let (localizer, strings) =
        Localizer::from_profile(loaded, &requested, &locales.default, locales.use_isolating);
    let empty = toml::Table::new();
    let table = loaded
        .profile
        .bindings
        .get(BINDINGS_CONTEXT)
        .unwrap_or(&empty);
    let (bindings, binding_issues) = UiBindings::from_table(table);

    world.insert_resource(UiRegistryRes(registry));
    world.insert_resource(UiSettingsStore(store));
    world.insert_resource(UiLocalizer(localizer));
    world.insert_resource(bindings);
    world.insert_resource(UiProfile {
        profile: loaded.profile.clone(),
        dir,
        dirty: false,
        last_error: None,
    });
    UiLoadReport {
        conflicts,
        strings,
        settings,
        bindings: binding_issues,
    }
}

impl UiBindings {
    /// Profile table of the actions whose bindings differ from the defaults.
    /// A binding with no input name (an unidentified key) is left out.
    pub fn to_table(&self) -> toml::Table {
        let defaults = UiBindings::default();
        UiAction::ALL
            .iter()
            .filter(|a| self.get(**a) != defaults.get(**a))
            .map(|a| {
                let names = self
                    .get(*a)
                    .iter()
                    .filter_map(binding_name)
                    .map(toml::Value::String)
                    .collect();
                (a.name().to_owned(), toml::Value::Array(names))
            })
            .collect()
    }

    /// Defaults with the table's actions replaced. Unknown actions, values
    /// that aren't lists of names, and unknown input names are reported; an
    /// action with a bad entry keeps the rest of its list.
    pub fn from_table(table: &toml::Table) -> (Self, Vec<String>) {
        let mut out = UiBindings::default();
        let mut issues = Vec::new();
        for (name, value) in table {
            let Some(action) = UiAction::from_name(name) else {
                issues.push(format!(
                    "[bindings.{BINDINGS_CONTEXT}] unknown action `{name}`"
                ));
                continue;
            };
            let Some(list) = value.as_array() else {
                issues.push(format!(
                    "[bindings.{BINDINGS_CONTEXT}] `{name}` must be a list of input names"
                ));
                continue;
            };
            let mut bindings = Vec::with_capacity(list.len());
            for item in list {
                match item.as_str().map(parse_binding) {
                    Some(Ok(b)) => bindings.push(b),
                    Some(Err(e)) => {
                        issues.push(format!("[bindings.{BINDINGS_CONTEXT}] `{name}`: {e}"));
                    }
                    None => issues.push(format!(
                        "[bindings.{BINDINGS_CONTEXT}] `{name}`: {item} is not a name"
                    )),
                }
            }
            out.set(action, bindings);
        }
        (out, issues)
    }
}

const MOD_NAMES: [(ModKeys, &str); 4] = [
    (ModKeys::CONTROL, "Ctrl"),
    (ModKeys::SHIFT, "Shift"),
    (ModKeys::ALT, "Alt"),
    (ModKeys::SUPER, "Super"),
];

/// Input name of a binding: `key:Enter`, `key:Ctrl+Shift+KeyS`, `mouse:Left`,
/// `pad:South`, `axis:LeftStickX`, `mouse_motion`, `mouse_wheel`, `any_key`.
/// Variant names are Bevy's (`KeyCode`, `GamepadButton`, ...), read through
/// reflection. `None` for inputs without a name (`KeyCode::Unidentified`,
/// `GamepadButton::Other`).
pub fn binding_name(binding: &Binding) -> Option<String> {
    let with_mods = |prefix: &str, mods: ModKeys, name: &str| {
        let mut s = format!("{prefix}:");
        for (_, label) in MOD_NAMES.iter().filter(|(m, _)| mods.contains(*m)) {
            s.push_str(label);
            s.push('+');
        }
        s.push_str(name);
        s
    };
    Some(match binding {
        Binding::Keyboard { key, mod_keys } => with_mods("key", *mod_keys, unit_name(key)?),
        Binding::MouseButton { button, mod_keys } => {
            with_mods("mouse", *mod_keys, unit_name(button)?)
        }
        Binding::GamepadButton(b) => format!("pad:{}", unit_name(b)?),
        Binding::GamepadAxis(a) => format!("axis:{}", unit_name(a)?),
        Binding::MouseMotion { mod_keys } if mod_keys.is_empty() => "mouse_motion".to_owned(),
        Binding::MouseWheel { mod_keys } if mod_keys.is_empty() => "mouse_wheel".to_owned(),
        Binding::AnyKey => "any_key".to_owned(),
        _ => return None,
    })
}

/// Parses a name written by [`binding_name`].
pub fn parse_binding(name: &str) -> Result<Binding, String> {
    match name {
        "mouse_motion" => return Ok(Binding::mouse_motion()),
        "mouse_wheel" => return Ok(Binding::mouse_wheel()),
        "any_key" => return Ok(Binding::AnyKey),
        _ => {}
    }
    let (kind, rest) = name
        .split_once(':')
        .ok_or_else(|| format!("`{name}` has no `kind:` prefix"))?;
    let mut parts: Vec<&str> = rest.split('+').collect();
    let input = parts.pop().unwrap_or_default();
    let mut mod_keys = ModKeys::empty();
    for part in parts {
        let (m, _) = MOD_NAMES
            .iter()
            .find(|(_, label)| label.eq_ignore_ascii_case(part))
            .ok_or_else(|| format!("`{part}` in `{name}` is not Ctrl, Shift, Alt or Super"))?;
        mod_keys |= *m;
    }
    let unknown = || format!("unknown input `{name}`");
    let no_mods = |b: Binding| {
        if mod_keys.is_empty() {
            Ok(b)
        } else {
            Err(format!("`{name}` can't have modifiers"))
        }
    };
    match kind {
        "key" => Ok(Binding::Keyboard {
            key: from_unit_name(input).ok_or_else(unknown)?,
            mod_keys,
        }),
        "mouse" => Ok(Binding::MouseButton {
            button: from_unit_name(input).ok_or_else(unknown)?,
            mod_keys,
        }),
        "pad" => no_mods(Binding::GamepadButton(
            from_unit_name::<GamepadButton>(input).ok_or_else(unknown)?,
        )),
        "axis" => no_mods(Binding::GamepadAxis(
            from_unit_name::<GamepadAxis>(input).ok_or_else(unknown)?,
        )),
        _ => Err(format!(
            "unknown input kind `{kind}` in `{name}` (key, mouse, pad, axis)"
        )),
    }
}

/// The variant name of a unit enum variant.
fn unit_name<T: Enum>(value: &T) -> Option<&str> {
    (value.variant_type() == VariantType::Unit).then(|| value.variant_name())
}

/// The unit variant called `name`.
fn from_unit_name<T: FromReflect + Typed>(name: &str) -> Option<T> {
    let TypeInfo::Enum(info) = T::type_info() else {
        return None;
    };
    matches!(info.variant(name)?, VariantInfo::Unit(_))
        .then(|| T::from_reflect(&DynamicEnum::new(name, DynamicVariant::Unit)))
        .flatten()
}

// Bevy's input enums must stay reflectable for the names above.
const _: fn() = || {
    fn reflectable<T: FromReflect + Typed + Enum>() {}
    reflectable::<KeyCode>();
    reflectable::<MouseButton>();
    reflectable::<GamepadButton>();
    reflectable::<GamepadAxis>();
};

/// Writes changed settings and bindings into [`UiProfile`], and the profile to
/// disk once no Menu or Modal screen takes input (or on exit).
pub(crate) fn sync_profile(
    profile: Option<ResMut<UiProfile>>,
    store: Option<Res<UiSettingsStore>>,
    bindings: Res<UiBindings>,
    stack: Res<ScreenStackRes>,
    mut exit: MessageReader<AppExit>,
) {
    let Some(mut profile) = profile else { return };
    let profile = profile.bypass_change_detection();
    if let Some(store) = store.filter(|s| s.is_changed() && !s.is_added()) {
        profile.dirty |= store.0.save_to(&mut profile.profile);
    }
    if bindings.is_changed() && !bindings.is_added() {
        let table = bindings.to_table();
        let old = profile.profile.bindings.get(BINDINGS_CONTEXT);
        if old.map_or(!table.is_empty(), |t| *t != table) {
            if table.is_empty() {
                profile.profile.bindings.remove(BINDINGS_CONTEXT);
            } else {
                profile
                    .profile
                    .bindings
                    .insert(BINDINGS_CONTEXT.to_owned(), table);
            }
            profile.dirty = true;
        }
    }
    let exiting = exit.read().count() > 0;
    let in_menus = matches!(
        stack.0.input_target().map(|s| s.layer),
        Some(Layer::Menu | Layer::Modal)
    );
    if !profile.dirty || (in_menus && !exiting) {
        return;
    }
    let Some(dir) = profile.dir.clone() else {
        return;
    };
    // On failure the flag clears too: the next change retries, not every frame.
    profile.dirty = false;
    profile.last_error = profile.profile.save(&dir).err().map(|e| e.to_string());
}
