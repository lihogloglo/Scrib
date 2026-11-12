# Lights

## What it does

Every Morrowind light record becomes a Bevy `PointLight` with inverse-square falloff, placed at the model's
`AttachLight` node and flickering per its record. Bevy's GPU clustering feeds them to the main pass. A few lamps
get shadow maps (see [shadows](shadows.md)). Night readability comes from an ambient floor, not from wide-open exposure.

## How it works

- **From content.** `LightCalibration::point_light` (`bevyrrowind/crates/mw_assets/src/lights.rs:126`):
  `intensity = lumens_per_radius_sq (8000) x radius^2`, `radius` 0.05 m emitter, `shadow_maps_enabled: false`,
  shadow near plane 0.25 m (`SHADOW_NEAR_Z` `:145`). `spawn_light` (`:148`) adds `BudgetedShadow`, `ShadowReach` and,
  for flicker or pulse records, `AnimatedLight` (`:226`, rewrites `intensity` only past 1.5 % change, skips hidden lights).
  Callers: `mw_exterior_bevy/src/exterior.rs:634` (placements), `level.rs:304`. `attach_lights` (`:174`) reparents the light
  under the model's `AttachLight` node. Held torches: `mw_actors_bevy/src/held.rs` (`HeldLightBudget` 12 nearest held lights, `:64`).
- **Reach follows shadow state.** `PointLight::range` is written each frame by the shadow budget as a blend of
  `ShadowReach::unshadowed` and `shadowed` by the shadow fade (`sf_sky_bevy/src/shadow_budget.rs:134`). Both scales
  are 4 x radius today (`LightCalibration::default` `lights.rs:80`; `reach` `:121`).
- **Lamp bodies.** Emissive meshes of light carriers get `GiSkipEmissive` so probe GI does not light twice
  (`mark_lamp_bodies`, `lights.rs:194`).
- **Clustering.** Bevy 0.20 GPU clustering is the default on desktop; we set no `ClusterConfig` (grep finds none).
  Extraction re-runs every frame for animated lights; per [lights spec](../../../superfargoth/docs/specs/lights.md) neither
  that nor `prepare_lights` showed up above 0.4 ms. No light budget or draw distance exists beyond the shadow budget.
- **Night brightness and ambient.** The exterior ambient floor is `night_ambient_nits` 4 (sun below -12 deg, fading
  out by -4 deg; `LightCalibration::apply_sky` `lights.rs:93`); the exposure floor is EV100 3.5 (same default); interior cell
  ambient is 300 nits x cell colour (`ambient` `:106`). The flat ambient lights diffuse only
  (`AmbientLightSpecular(false)`, `mw_environment_bevy/src/render.rs:183`; vendor row N1, `vendor/README.md:27`).
  Moonlight is 2.5 x pi x the floor in lux, artistic (`FULL_MOONLIGHT_FLOOR_RATIO`, `lights.rs:77`).
- **Sky-side lights.** Sun, moon, lightning flash (`sf_weather_bevy/src/fx.rs`) and the cloud-shadow light are
  directional lights; see [sky-atmosphere-weather](sky-atmosphere-weather.md).
- **Ray-traced path.** `PointLightProxy` turns point lights into emissive spheres for Solari
  (`sf_sky_bevy/src/proxy.rs`); GI reads a 64-light list (`sf_gi/src/lights.rs`).

## Bevy features used

`PointLight`, GPU clustering (`bevy_pbr` cluster passes), `GlobalAmbientLight`, `AmbientLight`, `Exposure`.
Vendored: DLSS cluster lookup fix #26236 (`vendor/README.md:14`), ambient specular toggle (N1).

## Switches and counters

`--off lights`; `--range-unshadowed X` (reach scale without a map); live `flip ambient_specular`, `flip gi`.
Counters: `PointShadowBudget` state, perf report light counts via `mw_devtools_bevy/src/perf.rs`.

## Known limits

- A lamp without a map lights through walls at 4 x radius; shortening to 1.5 x was rejected by the owner
  (towns and interiors darkened, [ledger](../../ledger/shadows.md), LT-reach).
- A lamp whose short reach misses every frustum never ranks for a shadow (Bevy visibility uses the current range).
- Night "too lit versus OpenMW" and the unconfirmed night glow are open (RO-16, [sky ledger](../../ledger/sky-weather.md)).

## Specs

[lights](../../../superfargoth/docs/specs/lights.md), [sky-lighting](../../../superfargoth/docs/specs/sky-lighting.md),
Morrowind light rules `bevyrrowind/docs/research/`.
