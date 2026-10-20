# Vendored third-party crates

Patched copies of crates.io crates, wired with `[patch.crates-io]` in **both**
`superfargoth/Cargo.toml` and `bevyrrowind/Cargo.toml`, excluded from the superfargoth
workspace (not ours to lint or test). Every changed hunk carries a `superfargoth patch`
comment; `grep -rn "superfargoth patch" vendor/<crate>` lists them.

| Crate(s) | Base | Patch | Drop when |
|---|---|---|---|
| `big_space` | crates.io release | dirty-tree pruning in low-precision propagation (`docs/research/perf/README.md` item 2) | upstream has it |
| `bevy_core_pipeline`, `bevy_pbr`, `bevy_render` | 0.20.0 (crates.io) | bevyengine/bevy [#26149](https://github.com/bevyengine/bevy/pull/26149) + [#26161](https://github.com/bevyengine/bevy/pull/26161) (`patches/bevy-26149-26161.diff`, both PRs' stacked diff at head `4e718df0`) | the Bevy release that ships both PRs |

## The Bevy shadow patch (#26149, #26161)

- **#26149** (`Core3dShadows`): point and spot shadow views run a slim schedule (GPU mesh
  preprocessing + the two shadow passes) instead of the whole `Core3d` camera schedule
  per cube face (bevy issue #25576).
- **#26161** (`SkipRootViewRendering`): per point/spot light, a hash of its visible casters,
  its view/projection and the queued shadow-phase entity count; a shadow view whose inputs
  are unchanged (no caster moved, deforms, or changed mesh; no pipeline still compiling)
  is skipped by the camera driver and keeps last frame's map. Directional lights are not
  covered. `bevy_render` only gains `BinnedRenderPhase::entity_count`.

The diff applied cleanly to the 0.20.0 sources (`patch -p1` from `vendor/`, two hunks
offset by 9 lines).

## Rebase or drop on a Bevy upgrade

1. Check both PRs: merged and released in the new version? Delete the three
   `vendor/bevy_*` folders and their `[patch.crates-io]` lines in both workspaces. Done.
2. Otherwise copy the new version's crates from the cargo registry
   (`~/.cargo/registry/src/index.crates.io-*/bevy_{core_pipeline,pbr,render}-<ver>`) over
   the folders (remove `Cargo.lock`, `.cargo-ok`), fetch the PRs' current diff
   (`gh pr diff 26161 -R bevyengine/bevy`, it includes #26149), strip the `crates/` prefix
   (`sed 's#\(a\|b\)/crates/#\1/#g'`), `patch -p1` it from `vendor/`, re-add the
   `superfargoth patch (#…)` comments, refresh `patches/`, and bump the versions above.
3. Every patched crate must match the version the rest of Bevy resolves to, or cargo
   silently ignores the patch (`cargo tree -i bevy_pbr` shows the path source when it took).
