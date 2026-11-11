# OpenMW settings window map (research, 2026-11-11)

Purpose: author our Classic-option options screen as data, mirroring OpenMW. Source: github.com/OpenMW/openmw
master at commit `97722d48f98f14b323c3880a698e8ce81babf7b3` (read 2026-11-11). Files:
`files/data/mygui/openmw_settings_window.layout` (widgets + `SettingCategory`/`SettingName` UserStrings),
`apps/openmw/mwgui/settingswindow.cpp` (logic), `files/data/l10n/OMWEngine/en.yaml` (labels),
`apps/launcher/settingspage.cpp` + `apps/launcher/ui/settingspage.ui` (launcher).
Key form: `[Section] key` in `settings.cfg`. Vanilla/OpenMW marks come from our knowledge of the original Morrowind
Options (Prefs, Audio, Controls, Video) and OpenMW docs, not from the source: "vanilla" = exists in original
Morrowind (possibly renamed); "OpenMW" = added. Best effort.

## Tab order (top-level `SettingsTab`)
Prefs, Audio, Controls, Video (itself an inner tab control: Video, Detail Level, Water, Lights, Shadows),
Scripts, Language. OK button bottom right (saves via `Settings::Manager::saveUser` to the user `settings.cfg`).
Control types: Slider = MyGUI ScrollBar (`SettingType=Slider`, `SettingValueType` Float/Integer), CheckButton =
On/Off toggle button, ComboBox = dropdown. Most widgets bind generically through `SettingCategory` +
`SettingName` UserStrings in the layout, so the page is already data-driven in OpenMW (our target shape).

## 1. Prefs
| Label | Type | Key | Origin |
|---|---|---|---|
| Menu Transparency | slider (float) | `[GUI] menu transparency` | vanilla |
| Menu Help Delay | slider (float) | `[GUI] tooltip delay` | vanilla |
| Difficulty | slider (int) | `[Game] difficulty` | vanilla |
| Actors Processing Range | slider (int) | `[Game] actors processing range` | OpenMW (vanilla: AI distance in INI) |
| Auto-Save on Resting | check | `[Saves] autosave` | OpenMW |
| Always Use Best Attack | check | `[Game] best attack` | vanilla |
| Subtitles | check | `[GUI] subtitles` | vanilla |
| Crosshair | check | `[HUD] crosshair` | vanilla |
| Mouse Emulation Cursor | check | `[HUD] mouse emulation cursor` | OpenMW |

## 2. Audio (all float sliders, `[Sound]`)
| Label | Key | Origin |
|---|---|---|
| Master | `master volume` | vanilla |
| Voice | `voice volume` | vanilla |
| Effects | `sfx volume` | vanilla |
| Footsteps | `footsteps volume` | vanilla |
| Music | `music volume` | vanilla |

## 3. Controls
| Label | Type | Key | Origin |
|---|---|---|---|
| Mouse/Keyboard, Controller | two buttons switching the binding list | none (bindings saved in the input bindings xml, not settings.cfg) | vanilla / OpenMW |
| Action binding list (click to rebind; includes Always Run) | scroll list | input bindings xml | vanilla |
| Reset Controls | button | resets bindings | OpenMW |
| Invert X Axis | check | `[Input] invert x axis` | vanilla |
| Invert Y Axis | check | `[Input] invert y axis` | OpenMW |
| Camera Sensitivity | slider (float) | `[Input] camera sensitivity` | vanilla |
| Enable Controller | check | `[Input] enable controller` | OpenMW |

## 4. Video (inner tabs)
### 4a. Video
| Label | Type | Key | Origin |
|---|---|---|---|
| Resolution list | listbox | `[Video] resolution x` / `resolution y` | vanilla |
| Window Mode | combo (Fullscreen / Windowed Fullscreen / Windowed) | `[Video] window mode` | vanilla (fullscreen toggle) |
| VSync | combo | `[Video] vsync mode` | OpenMW |
| Window Border | check (enabled only in Windowed) | `[Video] window border` | OpenMW |
| Post Processing | check | `[Post Processing] enabled` | OpenMW |
| Field of View | slider (float) | `[Camera] field of view` | OpenMW |
| Gamma Correction (Dark..Light) | slider (float) | `[Video] gamma` | vanilla (Brightness) |

Text hints only: frame-rate hint (F3), window-mode hint.

### 4b. Detail Level
| Label | Type | Key | Origin |
|---|---|---|---|
| Texture Filtering | combo | `[General] texture mipmap` + `texture min filter` | OpenMW |
| Anisotropy | combo | `[General] anisotropy` | OpenMW |
| View Distance | slider (float); a second large-distance slider replaces it when distant terrain is on | `[Camera] viewing distance` | vanilla-ish |

### 4c. Water (all OpenMW, `[Water]`)
| Label | Type | Key |
|---|---|---|
| Shader | check | `shader` |
| Refraction | check | `refraction` |
| Sunlight Scattering | check | `sunlight scattering` |
| Wobbly Shores | check | `wobbly shores` |
| Texture Quality | combo | `rtt size` |
| Reflection Shader Detail | combo | `reflection detail` |
| Rain Ripple Detail | combo | `rain ripple detail` |

### 4d. Lights (all OpenMW, `[Shaders]`)
Clustered Lighting (check, `clustered lighting`); Force Per-Pixel Lighting (`force per pixel lighting`); Particle
Point Lighting (`particle point lighting`); Clamp Lighting (`clamp lighting`); Match Sunlight To Sun
(`match sunlight to sun`); Classic Falloff (`classic falloff`); Max Lights (combo, multiples of 8, `max lights`);
Maximum Light Distance (slider 0..8192, `maximum light distance`); Fade Start Multiplier (slider, key `light fade
start` inferred from `mLightFadeStart`); Light Radius Multiplier (slider 1..5, `light radius multiplier`); Minimum
Interior Brightness (slider, `minimum interior brightness`); Reset button (resets all of these; clustered vs classic
mode hides some rows).

### 4e. Shadows (all OpenMW, `[Shadows]`)
Actor / Player / Terrain / Object / Indoor shadows (checks: `actor shadows`, `player shadows`, `terrain shadows`,
`object shadows`, `enable indoor shadows`); Shadow Planes Computation Method (combo, `compute scene bounds`);
Shadow Map Resolution (combo, `shadow map resolution`); Shadow Distance Limit (slider, `maximum shadow map distance`).

## 5. Scripts (OpenMW)
Shown only with a game loaded (otherwise "Load a game to access script settings."). Filter box + list of Lua pages
on the left, the selected page rendered in a scroll view (`LuaAdapter`) on the right. No settings.cfg keys.

## 6. Language (OpenMW)
Primary Language, Secondary Language (combos, `[General] preferred locales`), "GMST Overrides L10n" (check,
`[General] gmst overrides l10n`), plus a note text.

## Launcher (`settingspage.ui`, Advanced page): mechanics toggles for our classic-rules seam
Tabs: Gameplay, Visuals (sub-tabs Animations, Shaders, Fog, Terrain, Post Processing), Audio, Interface,
Miscellaneous (Saves, Screenshots, Testing, config dirs). They write `settings.cfg` (gameplay ones in `[Game]`).

| Tab | Notable toggles |
|---|---|
| Gameplay | Classic Reflected Absorb Spells Behavior; Classic Calm Spells Behavior; Followers Defend Immediately; Uncapped Damage Fatigue; Enchanted Weapons Are Magical; Only Magical Ammo Bypass Resistance; Factor Strength into Hand-to-Hand; Affect Werewolves; Permanent Barter Disposition Changes; Racial Variation in Speed Fix; Soulgem Values Rebalance; Merchant Equipping Fix; Trainers Choose Offered Skills by Base Value; Can Loot During Death Animation; Steal from Knocked Out Actors; NPCs Avoid Collisions; Use Navigation Mesh; Always Allow Actors to Follow over Water; Graphic Herbalism; Day Night Switch Nodes; Swim Upward Correction; Actor Collision Shape Type; Background Physics Threads |
| Visuals | Animations (Smooth Movement, Weapon/Shield Sheathing, Turn to Movement Direction, Magic Item Animation), Shaders (auto normal/specular maps, soft particles), Fog (Exponential, Radial, Sky Blending), Terrain (Distant Land, Object Paging, Viewing Distance), Post Processing |
| Audio | HRTF, Doppler Factor, Camera as Listener, Audio Device |
| Interface | Show Effect Duration, Enchant Chance, Owned Objects, Melee/Projectile info, Font Size, GUI Scaling, Stretch Menu Background, Zoom on Maps, Dialogue Topic Color |
| Miscellaneous | Maximum Quicksaves, Screenshot Format, Grab Cursor, Testing |

## Lua settings pages
Mods call `I.Settings.registerPage` (menu-script interface, `files/data/scripts/omw/settings/menu.lua`) with a page
key and name, then `registerGroup` for the groups on it (renderers such as checkbox, number, select). Registered
pages are listed in the in-game window's Scripts tab (filterable list, page body on the right). Values live in Lua
storage sections (`storage.playerSection(groupKey)` or global), not in `settings.cfg`; the tab is only populated
once a game is loaded because the player/global scripts must be running.

## Notes for the build slice
- "Always Run" is only a control binding in OpenMW, not an options row.
- Origin marks and the Fade Start key name are not verified line by line against the source.
