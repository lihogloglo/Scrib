# Main menu, pause menu and settings: industry research (2026-11-11)

Research only. Items marked (unverified) come from memory, not a fetched page.

## 1. Settings model in shipped engines

**UE5 Lyra GameSettings** ([docs](https://dev.epicgames.com/documentation/en-us/unreal-engine/lyra-sample-game-settings-in-unreal-engine)):
- `UGameSettingRegistry` holds all settings; `UGameSettingCollection` groups them (page, section). A second, hidden registry can hold non-UI settings.
- `UGameSettingValue` subclasses: discrete (bool, int, enum, option list) and scalar (range, with getters/setters bound to the local player).
- Each setting has a unique `DevName`, a `DisplayName`, a rich-text description and a default.
- **Edit conditions** decide per setting: enabled, disabled (with a visible reason), or hidden. They can also block reset and exclude the setting from analytics. Built-ins: inline lambda, platform trait, "primary player only".
- **Dependencies** (`AddEditDependency`): setting A re-evaluates when B changes (Resolution depends on Window Mode).
- Platform traits (keyboard/mouse present, resolution change allowed) replace `#if PLATFORM_*`, so they can be tested live.
- Apply/revert/"dirty" and scalability groups live in `UGameUserSettings` (not on the fetched page; unverified detail): quality groups (shadows, textures...) plus an overall level that reads back as Custom when groups disagree; `ApplySettings` vs `ConfirmVideoMode`/`RevertVideoMode`.
- Lyra pairs this with CommonUI (screen stack, input routing) and a separate input-settings page ([Lyra input settings](https://dev.epicgames.com/documentation/unreal-engine/lyra-input-settings-in-unreal-engine)).

**OpenMW Lua `I.Settings`** (the modding reference; [docs mirror](https://elsid-openmw.readthedocs.io/en/latest/reference/lua-scripting/interface_settings.html)):
- `registerPage{key, l10n, name, description}`, `registerGroup{key, page, l10n, name, description, permanentStorage, settings=[{key, renderer, default, argument, name, description}]}`, `registerRenderer(key, fn(value, set, argument))`, `updateRendererArgument`.
- Built-in renderers: `textLine`, `checkbox`, `number`, `select`, `inputBinding`, etc. Custom renderers are Lua UI functions.
- Groups are `global`, `player` or `menu`-scoped storage; pages register from player scripts only. Values persist in a settings file; scripts subscribe to group changes.
- Takeaway: the mod surface is page, group, setting, renderer. Mods never draw the screen, they declare data and optionally supply a renderer.

**SkyUI MCM** (unverified): page-based DSL with toggle, slider, menu, keymap, text, header; per-option info text; `OnOptionSelect/Highlight` callbacks; option enable flags set from script (`OPTION_FLAG_DISABLED`); a "mod config" screen separate from the game's own options. Newer MCM Helper reads a JSON/Config definition with `ShowIf`-style conditions, which is the same declarative direction as Lyra edit conditions.

**Unity / Godot** (unverified): neither ships a settings framework. Unity has Quality Settings levels plus `PlayerPrefs`; Godot has `ProjectSettings`/`ConfigFile` and `InputMap` remap by hand. Nothing to copy beyond "quality level + per-group overrides".

### Common data model (union of the above)

| Field | Notes |
|---|---|
| id, label, description, keywords | Localisable; keywords feed search |
| type | bool, int, float, choice, text, key/button binding, action (button), custom renderer |
| default, range, step, options | Range and step per type; options may be dynamic (monitors, resolutions, GPUs, languages) |
| scope | global (machine), per-profile/user, per-save (gameplay difficulty), per-device (graphics) |
| apply timing | live (slider, preview), on-confirm (video mode, with revert timer), restart-required |
| enable/visibility conditions | Expression over other settings and platform traits; disabled state carries a reason string |
| dependencies | Change in B re-evaluates A (and may clamp or reset A) |
| platform visibility | Desktop only, DLSS-capable GPU only, controller present |
| perf/VRAM hint | Low/Med/High cost tag, optional estimated VRAM |
| page, group, order | Layout |
| preset membership | Which preset levels set which value |
| reset policy | Allowed in "reset page"; excluded from reset (language, key rebinds) |

State, not data: current value, saved value, dirty flag, error/unsupported state.

## 2. Modern options UX: standard vs nice-to-have

Evidence is thin from search (secondary reviews only). [A review of the Spider-Man PC port](https://eugames.pages.dev/posts/marvel-s-spider-man-pc-a-patch-or-two-away-from-perfection) praises the pre-launch setup menu and in-game options that apply instantly while paused; [Nixxes' Spider-Man 2 tuning page](https://support.nixxes.com/hc/en-us/articles/49284471919635-Marvel-s-Spider-Man-2-PC-Performance-Tuning) shows display/graphics reachable from both launcher and pause menu. The rest below is established practice from shipped titles (unverified per title).

**Standard (expect from any PC release):**
- Preset (Low..Ultra) that shows "Custom" once any sub-option diverges; derived, never stored as a separate truth.
- Display mode, resolution, refresh, vsync, frame cap, upscaler mode, render scale.
- Resolution/mode change applies, then a ~15 s keep/revert countdown (Windows and most engines).
- Separate "Apply"/"Revert" for risky settings; instant apply for safe ones.
- Reset page to defaults; per-setting description area; focus/hover shows the description.
- Full keyboard, mouse and gamepad navigation of the options screen itself.
- Rebind keyboard/mouse and gamepad, with conflict detection (swap or unbind), per-action hold vs toggle for sprint/crouch/aim.
- Subtitles on/off, text size, UI scale, colour-blind modes, camera shake and motion blur toggles, separate volume buses.
- Settings reachable from both the main menu and the pause menu, same screen.
- Settings persisted between runs, applied before the first frame (window size, vsync).

**Nice-to-have (differentiators):**
- Live preview: translucent options panel over the paused world (Nixxes, Cyberpunk, Forza); needs the game visible behind the pause menu, which suits pause but not main menu.
- Perf/VRAM impact estimates, "restart required" badges, search box, per-setting preview image (Cyberpunk-style comparisons), benchmark button (Forza, Cyberpunk).
- Remap profiles, per-device bindings, stick deadzone curve editors with live visualiser.
- Xbox Accessibility Guidelines coverage beyond basics: narration/TTS of the UI, high-contrast UI, remappable everything, one-handed presets. ([XAG](https://learn.microsoft.com/en-us/gaming/accessibility/guidelines) (unverified, page not fetched))

**Counter-example:** Elden Ring ships minimal graphics options and no preview/benchmark; players complain about missing hold/toggle and no controller remap of some actions. Lesson: gaps in rebinding and accessibility cost more goodwill than missing eye-candy.

## 3. Menu flow and front end

- **Front-end is a separate mode, not a gameplay overlay.** Lyra's frontend experience loads a minimal front-end map and UI layers; Bethesda, FromSoft and most AAA show title art or a lightweight scene, and load the world only on New/Load. This matches the owner's rejection of a menu over the live world.
- Typical flow: logo/intro videos (skippable, once per boot) -> press-any-key title (picks the active input device, needed for gamepad vs keyboard glyphs) -> main menu (Continue = newest save, New, Load, Options, Mods, Quit) -> loading screen -> gameplay. Pause menu: Resume, Save, Load, Options, Quit to menu.
- Continue = newest save is the default button; disabled when no saves.
- Pause and main menu share one `OptionsScreen`; differences are driven by edit conditions (e.g. "restart required" or "language" hidden in-game; display tab hidden when not applicable), not forked screens.
- **Bevy 0.20 idiom** ([0.20 notes](https://bevy.org/news/bevy-0-20/)): top-level `States` (`Boot, Frontend, Loading, InGame`), `SubStates` for `InGame` (`Playing, Paused`) and for menu pages, `ComputedStates` for "any menu visible" (input mode, cursor, time pause). `DespawnOnExit`/`DespawnOnEnter` scope menu entities; 0.20 makes them faster via a batched `despawn_all`. Defer world spawn until `OnEnter(Loading)` from New/Load. Pause: pause `Time<Virtual>` and gameplay schedules on `Paused`, keep UI running. Scene spawning gets a `Ready` event in 0.20, usable for "loading finished" gates.

## 4. Bevy 0.20 crates and built-ins

Checked via crates.io API / docs.rs on 2026-11-11.

| Item | Version, date | Licence | Verdict |
|---|---|---|---|
| **`bevy_settings`** (built into Bevy 0.20 behind a `bevy_settings` feature; [docs](https://docs.rs/bevy/0.20.0/bevy/settings/index.html)) | 0.20.0 | MIT/Apache | **Use.** `SettingsPlugin`, `#[derive(SettingsGroup)]` reflected resources, `SaveSettings`/`SaveSettingsDeferred` (debounced, default 1 s)/`SaveSettingsSync`, filesystem or browser storage. File format and paths not stated on the page: verify in source before relying on it. Fits engine-level settings (window, audio, input). Mod-defined `ui.setting` values still need our own record-backed store (reflection needs compile-time types). |
| `bevy_flair` | 0.9.0, 2026-10-14, Bevy 0.20 | MIT | Already chosen. |
| `bevy_enhanced_input` | 0.27.2, 2026-10-26 | MIT/Apache | Already chosen. Provides context-based remap; remap UI is ours. |
| `bevy-persistent` | 0.12.0, 2026-10-24 | MIT/Apache | Maintained; Bevy 0.20 support not confirmed. Redundant if `bevy_settings` covers it. |
| `bevy_pkv` | latest listed 0.6.0, no 0.20 seen | MIT/Apache | Skip (stale, KV store for saves). |
| `bevy-settings` (tecbeast42) | 0.7.2, Jul 2025, table ends at Bevy 0.14 | | Skip, stale. |
| `bevy-alt-ui-navigation-lite` | 0.5.0, 2025-09-30 | MIT/Apache | Skip: stale, and Bevy now ships directional navigation (below). |
| `haalka` | 0.7.1, 2026-02-11 | MIT/Apache | Skip: reactive UI layer overlapping our MVVM view-models; not 0.20 confirmed. |
| `bevy_cobweb_ui` | 0.22.2, 2026-01-14 | MIT/Apache | Skip: own asset-driven UI framework, overlaps bevy_flair/Fluent stack. |
| `bevy_hui`, `bevy_lunex`, `sickle_ui` | not checked | | Skip: competing UI stacks; sickle_ui is superseded by Bevy Feathers (unverified). |

**Bevy's own UI in 0.20** ([notes](https://bevy.org/news/bevy-0-20/)):
- Feathers: colour input, scrollable list view with scrollbar, dropdown selection, lazy menu, scrubbable number input (hard/soft limits, step, precision), `ThemeContext` with semantic tokens. Feathers is editor-flavoured, so treat as optional building blocks, not our look.
- `bevy_ui_widgets`: headless `TabList`/`Tab` (new in 0.20, with `ValueChange`, keyboard shortcuts, focus integration). The release notes do not mention slider/checkbox/radio/text input; those were in earlier releases (unverified for 0.20: grep source for exact status). Our settings tabs map directly onto headless tabs.
- Focus/navigation: `bevy_input_focus` has `AutoDirectionalNavigation` (positions-based, no manual graph) plus a manual override layer; docs.rs examples exist ([auto nav example](https://bevy.org/examples/ui-user-interface/auto-directional-navigation)). 0.20 changelog not found; confirm API in source. Suits gamepad navigation of settings rows.
- New in 0.20: `FixedNode`, em/rem units, elliptical border radius.
- **BSN**: landed in 0.19, 0.20 cleans syntax (`@` scene refs, `--` list separator, `bsn_list!`, `Ready` event). `.bsn` asset format is **not shipped** (planned). So no `.bsn` menu files yet; keep layouts in Rust code or our records.

## 5. Gaps: `ui.setting` (`SettingDef`) vs the model

Current fields (`superfargoth/crates/sf_ui/src/records.rs:224`): `label`, `description`, `page`, `group`, `kind` (Bool, Int{min,max,step}, Float, Choice{options}, Text{max_len}). Values live per profile under `[settings.<owner>]`.

Missing:
1. **Apply timing**: no live / on-confirm / restart-required flag; no revert-countdown policy.
2. **Scope**: only per-profile; no global (machine/graphics) vs per-save split.
3. **Enable/visibility conditions** and dependencies; no disabled-reason text; no platform/hardware trait gating.
4. **Dynamic option lists** (monitors, resolutions, languages, input devices): `Choice.options` is static `Vec<String>`.
5. **Dirty/default/reset** semantics: no `reset_excluded`, no per-page reset, no presets membership (preset level -> value map), no derived "Custom" preset.
6. **Search keywords**, ordering within group, icon, perf/VRAM cost tag, preview image.
7. **Binding kind** (key/button/axis) with conflict rules and hold/toggle companion; presumably handled by bevy_enhanced_input contexts, not records.
8. **Action kind** (button/benchmark/open sub-screen) and **custom renderer** hook (OpenMW `renderer`, `argument`).
9. **Value validation / clamping on dependency change**; units/format string (ms, %, x).
10. **Change subscription contract** for mods (group-level on-change event) and a stable storage key for `bevy_settings` mapping.
11. Float default/range are `f64` without a display scale (logarithmic sliders, e.g. view distance).

## Recommendations

- Add `scope` (global/profile/save) and `apply` (live/confirm/restart) to `SettingDef`; build the keep/revert 15 s countdown for display-mode changes as a generic confirm-apply flow.
- Add a small declarative `condition` (enabled/visible expression over other setting ids and named platform traits) with a disabled-reason Fluent key, modelled on Lyra edit conditions; keep it data, not code.
- Make `Choice` options sourceable from a registered provider (monitors, resolutions, languages) in addition to static lists.
- Define presets as data: a preset record maps setting ids to values; "Custom" is computed when current values match no preset.
- Use Bevy's `bevy_settings` for engine-side persistence; confirm file format and path in source first, and keep the record-backed profile store for mod settings.
- Use `bevy_ui_widgets` headless tabs and `bevy_input_focus` auto directional navigation for the options screen; do not adopt haalka, cobweb, or the stale navigation crate.
- One `OptionsScreen` shared by main and pause menus; differences via conditions. Pause shows the live world behind a translucent panel (preview); the main menu uses title art/video, no world.
- Front end as a top-level `States` split (Boot, Frontend, Loading, InGame) with `SubStates` for pause and `DespawnOnExit` scoping; spawn the world only on New/Load; Continue = newest save.
- Ship a checklist baseline before extras: rebinding with conflicts and hold/toggle, subtitles/UI scale/colour-blind, search box, reset page. Defer perf/VRAM hints and benchmark button.
- Do not wait for `.bsn` assets; revisit once Bevy ships them.
