# Remediation plan (2026-11-09)

**What this is.** One ordered, self-contained work plan for everything the 2026-11-09 audit found wrong:
unapplied platform decisions, doc statements that are now false, missing mechanical guardrails, and real
code defects. Written so a future agent can pick **one slice**, read only that slice, and finish it.

**Baseline.** Every file:line below was verified against **HEAD `80c9a2c`** on 2026-11-09, after the
fast-forward that synced this folder. If `git rev-parse --short HEAD` is not `80c9a2c`, re-verify each
anchor with the `grep` given in the slice before editing. Anchors are quoted text, not just line numbers,
because lines drift.

**How to use it.**

1. `git rev-parse --short HEAD` and `git status --porcelain` (must be empty, or you are on someone's work).
2. Read `.sessions/*.md`. **Do not touch a file another live session owns** (ownership map in §3).
3. Pick the lowest-numbered slice in your part whose `Blocked by` is clear.
4. Register in `.sessions/<name>.md` per `bevyrrowind/CLAUDE.md` "Parallel sessions".
5. Do the slice. Verify with its `Verify` line. Stop. Do not extend scope into the next slice.
6. End-of-session docs per `CONTRIBUTING.md`: ROADMAP status mark on your task's line only, one
   `bevyrrowind/docs/CHANGELOG.md` line, and a dated `DECISIONS.md` bullet if the slice settles something.

**Provenance.** Findings come from four read-only review agents on 2026-11-09 (doc-decision audit,
platform-invariant conformance, gameplay-lane correctness, compile check) plus direct verification by the
coordinating session. Where a claim was not verified, the slice says so explicitly.

---

## 1. Standing rules that constrain every slice

These are the owner's rules. A slice that breaks one is wrong even if it works.

- **Builds are expensive.** The owner's laptop is weak and a Bevy rebuild takes a long time. Run the
  minimum. Every cargo command goes through `bash bevyrrowind/tools/cargo_turn.sh <args>` from the repo
  root, which waits for the single build slot and runs at BelowNormal priority. Check for a running build
  first: `tasklist | grep -iE "cargo|rustc"`.
- **`cargo_turn.sh` exit codes cannot be trusted** (found 2026-11-09: a run reported exit 0 with 5 real
  compile errors). Judge by output: `... 2>&1 | grep -E "^error" -A6`. Never by `$?`.
- **No automated visual checks, ever** (owner 2026-11-01, repeated). No `--screenshot` runs, no pixel
  diffs, no on/off image comparison to decide whether a rendering feature works. Rendering work ends with
  a build ready plus a test-scene entry in `bevyrrowind/docs/visual-checks/<system>.md` and its index line
  in `visual-checks.md`, status "awaiting owner", naming camera, time, flags and what to look for.
- **Agents never launch the game** (owner 2026-11-01). Build, then write the exact run command and the log
  lines to look for in your handoff. The owner launches.
- **Never commit unless the user asks.** When asked: stage explicit paths, never `git add -A`, and commit
  with `git commit -- <paths>`. The index is shared with other live sessions.
- **Don't revert changes you didn't make.** A file changing under you is another session's work.
- **Doc budgets:** ROADMAP <= ~250 lines, a spec <= ~25 KB, a visual-check entry <= ~10 lines.

## 2. Build economy (read before planning your slice)

The single biggest cost in this plan is compilation. Therefore:

- **Part A is doc-only. It needs no build at all.** 14 slices, zero compiles. Do these first and in bulk;
  they are also what stops future agents being misled.
- **Part D (guardrails) is almost free** and makes everything after it cheaper, because CI catches on a
  server what this laptop would otherwise pay for.
- **Parts B, C, E each need builds. Batch them.** Do not compile once per slice. Finish all edits in a
  part, then run one scoped `test -p <crate> -p <crate>` over exactly the touched crates.
- **Never run `--workspace` on this laptop** unless a slice explicitly requires it. Scope with `-p`.
- `Cargo.lock` and the vendored Bevy moved on 2026-11-09, so the **first** Bevy-dependent build after that
  sync is a long one. Expect it once; it is not a bug.
- A slice's `Build` field is one of: **none**, **scoped** (`-p` over named crates), or **full** (workspace
  or the game binary; needs the owner's go because of cost).

## 3. Ownership map (live as of 2026-11-09)

| Session | Owns | Keep out of |
|---|---|---|
| `coordinator-mechanics` (`repo-f2`) | `sf_effects` `sf_rpg` `sf_rpg_ui` `sf_ui_screens` `mw_effects` `mw_rpg` `mw_rpg_bevy` `mw_script` `mw_stats` `mw_ui` + their specs | everything else |
| `coord-static-scene` | `sf_scene` (+`bake.rs`), `mw_bake/src/scene*`, `sf_world_bevy/src/matclass.rs`, `sf_diag/src/views.rs`, `sf_diag_bevy/src/views.rs`, vendored counter + `gpu_image.rs` hunks, `bake_scene` wiring in `mw_boot_bevy` | gameplay crates, AI lane |
| `ai-coordinator` | git hygiene, `docs/briefs/`, AI/nav docs | all `*/crates/` |
| `coord-vision-audit` | this document, `docs/reviews/` | all `*/crates/` |

**Part E is entirely inside `coordinator-mechanics`' scope.** Its findings were sent to that session on
2026-11-09; they are recorded here for the owner, not for another agent to fix in parallel.

---

## PART A — Doc truth (no builds; do these first)

Why first: `CLAUDE.md` tells every agent to start at `docs/SYSTEMS.md` and its row's specs. While those
files are wrong, every future slice starts from bad information. This part is pure text and costs nothing
but care.

### A1. Delete the false GPL clearance in ARCHITECTURE §3 — **do this one first**

- **Build:** none. **Blocked by:** nothing. **Severity:** licence hazard, highest in the plan.
- **Evidence (verified):** `superfargoth/docs/ARCHITECTURE.md:124` reads
  `- Vendored Bevy patches stay MIT OR Apache-2.0 with no GPL-derived lines, so`.
  This is false: the H2 world-unit PCSS hunk in `superfargoth/vendor/bevy_pbr` is adapted from Clockwork
  (GPL-3.0). Our own records say so — `superfargoth/vendor/README.md:99` and
  `superfargoth/THIRD_PARTY_NOTICES.md:51`.
- **Why it matters:** the sentence licence-clears, for upstreaming to Bevy under MIT/Apache, a hunk our own
  vendor README documents as GPL-derived. An agent acting on it could send GPL code to an MIT/Apache
  project. That is the one error in this plan with consequences outside the repo.
- **Do:** replace the claim with the truth: vendored patches are MIT OR Apache-2.0 **except** the
  GPL-derived H2 PCSS hunk in `vendor/bevy_pbr/src/render/shadow_sampling.wesl`, which must never be
  offered upstream and must be stripped before any upstream PR from that file. State that any new vendored
  hunk carrying ported GPL code must be marked in `vendor/README.md` and excluded from upstreaming.
  Ready-to-paste wording: `docs/archive/platform-decisions-2026-11-08.md`, end section.
- **Verify:** `grep -n "GPL-derived" superfargoth/docs/ARCHITECTURE.md superfargoth/vendor/README.md`
  shows the hunk named in both and no blanket "no GPL-derived lines" claim remains.

### A2. Link the vision and decisions docs into the reading path

- **Build:** none. **Blocked by:** nothing. **Severity:** structural — this is why nothing else got applied.
- **Evidence (verified):** `grep -rln "vision-and-platform-2026-11-08\|platform-decisions-2026-11-08"`
  over all `*.md` matches only those two files themselves plus
  `superfargoth/docs/research/npc-ai-2026-11-08.md`. No `CLAUDE.md`, `CONTRIBUTING.md`, `ARCHITECTURE.md`,
  `ROADMAP.md` or `SYSTEMS.md` references them.
- **Why it matters:** the reading protocol is "start at SYSTEMS.md, read only your row". Two documents that
  nothing links to are invisible by construction. Eleven owner decisions taken 2026-11-08 live only there,
  and one day later exactly one had been applied.
- **Do:** add one line to the root `CLAUDE.md` pointing at the vision + decisions docs as the standing
  platform direction, and one line in `CONTRIBUTING.md` "Find your way". Keep both to one line; these files
  are loaded every session.
- **Verify:** the grep above now matches `CLAUDE.md` and `CONTRIBUTING.md`.

### A3. Append the 2026-11-08 owner decisions to DECISIONS.md

- **Build:** none. **Blocked by:** A2. **Severity:** high.
- **Evidence (verified):** of the 8 decision bullets the vision doc §10 asks for, only the Delirium one
  exists. `grep -c "systemic, persistent\|L0/L1\|sample pack\|Signed-off-by\|scrib-platform-steam\|
  formats.md\|Script surface\|adopt every minor" superfargoth/docs/DECISIONS.md` returns 0 for each.
- **Do:** append one dated bullet (<= 3 lines) per decision 1, 1b, 2, 3, 4, 5, 6, 7, 9, 10 from
  `docs/archive/vision-and-platform-2026-11-08.md` §9 "Owner answers". **Append only** — never edit existing
  bullets. Exact text is pre-written at the end of `docs/archive/platform-decisions-2026-11-08.md`.
- **Note:** `DECISIONS.md` is ~160 KB. The vision doc sets a ~200 KB split trigger. Do not split in this
  slice; log it as a follow-up.
- **Verify:** each decision number appears once with its 2026-11-08 date; file still parses as a flat
  bullet list; `node tools/decisions-index.mjs` regenerates `DECISIONS-INDEX.md` cleanly.

### A4. Fix the ARCHITECTURE §5 crate map (12 known errors)

- **Build:** none. **Blocked by:** nothing. **Severity:** high — it misdirects where code goes.
- **Evidence (verified at `80c9a2c`):** §5 names `sf_foliage`, `sf_interaction`, `sf_viewer`, `sf_devui`,
  `sf_ui_lua` — none exist on disk. It omits `sf_surface`, `sf_gi`, `sf_combat`, `sf_stats`, `sf_action`,
  `sf_ids`, all of which do. (`sf_ai` is listed and now genuinely exists as of AI1-AI5, so that entry is
  correct again.)
- **Do:** replace §5's prose list with the corrected L0-L3 layer table. **Use the corrected table from A5,
  not the vision doc's** — the vision doc's own table is incomplete (see A5).
- **Verify:** every crate named in §5 exists (`ls superfargoth/crates/`), and every crate on disk appears
  exactly once in the table. A one-off shell loop over `ls superfargoth/crates` is enough; D3 later makes
  this mechanical.

### A5. Correct the L0-L3 layer table before anyone builds a CI check on it

- **Build:** none. **Blocked by:** nothing. **Severity:** blocks D3.
- **Evidence (verified):** the vision doc §3.1 table omits **11 of 68** framework crates: `sf_anim`,
  `sf_anim_graph`, `sf_audio`, `sf_camera`, `sf_controller`, `sf_effects`, `sf_level`, `sf_model`,
  `sf_ragdoll`, `sf_scene`, `sf_texture`. Several are load-bearing for L0/L1 (`sf_world` depends on
  `sf_model`; `sf_physics` on `sf_controller` and `sf_model`), so the rule cannot be evaluated as written.
  The table also files each genre kit's own thin Bevy shell (`sf_action_bevy`, `sf_combat_bevy`,
  `sf_character_bevy`, `sf_stats_bevy`) under L2, which makes the kit-shell edges look like violations when
  they are correct by design.
- **Do:** place all 68 crates in exactly one layer. Add the rule that a kit's `*_bevy` shell belongs to the
  **same layer as its kit**, not to L2. Record the table in `ARCHITECTURE.md` §5 as the single source of
  truth. Do **not** move any code in this slice.
- **Verify:** count of crates in the table == `ls superfargoth/crates | grep -v workspace-hack | wc -l`.

### A6. ARCHITECTURE §1 — scope wording (owner decision 4)

- **Build:** none. **Severity:** medium, but it shapes every new spec.
- **Evidence (verified):** `ARCHITECTURE.md:13-14` still opens "An open-world action-RPG framework on Bevy,
  with Morrowind as its first and flagship content pack." Owner decision 4 (2026-11-08) was: "an engine for
  systemic, persistent 3D open worlds with characters; the ARPG is the first L3 kit."
- **Context the editor must preserve:** `WISHLIST.md` opens with the owner's own words, *"Morrowind if
  Bethesda made it in 2026 ... Crimson Desert level"*, and ARCHITECTURE §1 keeps that as the **first
  pack's** vision. Decision 4 changes the *framework's* charter, not the game's ambition. Both must survive
  the edit: platform charter at the top, Morrowind vision as the pack's.
- **Do:** add the philosophy and non-goals from vision §1 above the Morrowind vision; keep "Morrowind if
  Bethesda made it in 2026" explicitly labelled as the first content pack's goal.
- **Also update** (same wording, one line each): `CONTRIBUTING.md` opening, root `README.md`,
  `superfargoth/README.md`.
- **Verify:** `grep -rn "action-RPG framework" --include=*.md .` returns only historical/archive contexts.

### A7. ARCHITECTURE §2 principle 2 — one workspace

- **Build:** none. **Severity:** low, but it contradicts `CLAUDE.md`.
- **Evidence (verified):** `ARCHITECTURE.md:82` still says the framework "lives in its own Cargo workspace
  and repo and cannot name a Morrowind type." False since the 2026-10-25 merge; root `CLAUDE.md` says "ONE
  workspace at the root" and "one repo at this root".
- **Do:** keep the principle (the boundary is a compile-time fact) and restate the mechanism: one workspace,
  one repo, boundary enforced by a `cargo metadata` check (D3) rather than by separate workspaces.
- **Verify:** `grep -n "own Cargo workspace and repo" superfargoth/docs/ARCHITECTURE.md` is empty.

### A8. `superfargoth/vendor/README.md` — wrong patch site, and mark the GPL hunk

- **Build:** none. **Blocked by:** A1 (same licence fact). **Severity:** medium.
- **Evidence (verified):** `vendor/README.md:4` says the patches are wired "in **both**
  `superfargoth/Cargo.toml` and `bevyrrowind/Cargo.toml`, excluded from the superfargoth workspace".
  Neither file carries them: `[patch.crates-io]` is in the **root** `Cargo.toml` (verified at root
  `Cargo.toml:278`), patching `big_space`, `bevy_core_pipeline`, `bevy_pbr`, `bevy_render` and
  `bevy_solari`, all as `path =` entries.
- **Do:** correct the patch site to the root `Cargo.toml`; mark the H2 PCSS hunk GPL-derived and
  never-upstreamable (A1); record the Bevy upgrade cadence (owner decision 6: adopt every minor, never skip,
  lane from the RC, ship <= 6 weeks) and that the vendored crates are to move to a git fork branch on the
  next rebase (F6).
- **Verify:** `grep -n "patch.crates-io" Cargo.toml` and the README now name the same single file.

### A9. `LICENSE-EXCEPTION.md` — the Delirium paragraph contradicts owner decision 8

- **Build:** none. **Severity:** medium.
- **Evidence (verified):** `LICENSE-EXCEPTION.md:20-21` ends "Only a package that bundles a vendor runtime
  with the game must be built without Delirium (`--no-default-features`)." Owner 2026-11-08 ruled Delirium
  "stays part of the main game, on by default, **not optional**", explicitly voiding an agent proposal to
  remove it.
- **Do:** rewrite the scope paragraph so third-party GPL code (Delirium included, still default) is named as
  not covered by the permission, and drop the build configuration the owner has excluded. Add the decision-9
  rule: platform SDKs never enter the game process (Steam via a separate helper over a generic IPC); no
  Steamworks extension to this file, since it cannot bind third-party GPL holders.
- **Verify:** `grep -n "no-default-features" LICENSE-EXCEPTION.md` is empty.

### A10. Reconcile the two GI plans and remove the automated-screenshot workflow

- **Build:** none. **Severity:** high — it instructs agents to break a standing owner rule.
- **Evidence (verified):** `docs/archive/next-gen-pbr-gi-rt-plan.md` contains a section
  `# 5. Automated ReadyGate visual regression screenshots (Raster vs RT)` with
  `cargo run ... --screenshot interior_raster.png` / `--screenshot interior_rt.png` command lines, and
  `grep -c "superseded" ` over that file returns 0. It also plans baked `IrradianceVolume` per cell where
  `superfargoth/docs/specs/gi.md` specifies dynamic ray-query probe volumes with no bake.
- **Do:** (a) mark `gi.md` canonical in both files; (b) mark the next-gen plan's Tracks 2-3 superseded
  **where they conflict** — baked volumes vs dynamic probes, and the whole automated-screenshot section,
  which contradicts "no automated visual checks" and must be struck, not merely annotated; (c) keep Track 1
  (PBR materials and tangents), which every tier needs and nothing supersedes; (d) record the no-RT GI
  fallback as a named gap (see F7).
- **Verify:** `grep -rn "screenshot" docs/archive/next-gen-pbr-gi-rt-plan.md` returns nothing that decides whether a
  feature looks right; `grep -n "canonical" superfargoth/docs/specs/gi.md` names itself as such.

### A11. `docs/SYSTEMS.md` — add the missing rows

- **Build:** none. **Severity:** medium. **Note:** the AI row was corrected on 2026-11-09 by another
  session and now points at `specs/ai.md`, `ai-as-built.md` and `ai-adapter.md`. Do not re-fix it.
- **Do:** add rows for **Scalability** (spec pending, F2), **Interaction / smart objects** (spec pending,
  F7), and **Persistence policies + simulation LOD** (spec pending, F7), each with its crates column marked
  `(planned)` so the row exists before the code does. Keep the one-row-per-system format and the file's
  current column set.
- **Verify:** `tools/doccheck.sh links` passes; each new row's spec link either resolves or is marked
  `(planned)` with no link.

### A12. `CONTRIBUTING.md` — tier tie-breaker, DCO, script-ready, net-ready pointers

- **Build:** none. **Blocked by:** A6 (same file, one editing pass). **Severity:** medium.
- **Evidence (verified):** `grep -ci "signed-off-by\|DCO" CONTRIBUTING.md` returns **0**. The tie-breaker
  still reads as a single frame target; `grep -n "small co-op (2-4 players" CONTRIBUTING.md` still matches,
  contradicting owner decision 2 ("no ceiling").
- **Do:** (a) tie-breaker becomes per tier: "fast on the floor tier, beautiful on the top tier", with the
  tier matrix referenced rather than inlined; (b) replace "small co-op (2-4 players, host-authoritative)"
  with decision 2's wording: co-op 2-4 is the first milestone, not the limit; design for server authority, a
  headless build, id namespaces and interest management; (c) add the **DCO** rule (inbound = outbound, plus
  consent to future §7 permissions) per decision 1b; (d) add the **script-ready** rule per decision 7: every
  new spec carries a "Script surface" section (events, commands, formula structs as plain data) while Luau
  stays parked; (e) point "Net-ready" at the lints once D2/D3 exist.
- **Verify:** all five greps present; the file stays readable as working rules, not a changelog.

### A13. `specs/net.md` and `specs/streaming.md`

- **Build:** none. **Severity:** medium.
- **Evidence (verified):** `net.md` title still reads "host-authoritative 2-4 player co-op" and its
  non-goals still list "dedicated servers, more than 4 players", both void under decision 2.
  `streaming.md` states "ring radii scale with the quality preset" without excluding R0 (two places).
- **Do:** in `net.md`: reword the non-goals per decision 2; stop fixing the player count at 4
  (`sf_ids::PlayerId(u8)` already allows 255); add the **one state registry** idea (a component declares
  `saved` / `replicated` / `predicted` once, and the save section and replication rule are generated from
  that single declaration) as the design to adopt before more systems add their own save sections; put Steam
  under "Topology and transport" as the separate helper process (decision 9).
  In `streaming.md`: state explicitly that **R0 (simulation and collision) is fixed per game and never
  scales with the quality preset**; only visual rings scale. Cross-reference C5, which makes it true in code.
- **Verify:** `grep -n "2-4 player\|more than 4 players" superfargoth/docs/specs/net.md` is empty;
  `grep -n "R0" superfargoth/docs/specs/streaming.md` shows the exclusion.

### A14. Clear the dead premises left in research notes, and the stale H1 symptom

- **Build:** none. **Severity:** low each, but they are live instructions.
- **Evidence (verified):** `superfargoth/docs/research/npc-ai-2026-11-08.md:472` still says "GPL quarantine
  (vision §2.3, licence decision pending)". The decision was taken the same day: GPL stays, **quarantine
  dropped**. Separately, `bevyrrowind/docs/hard-problems.md` H1 quotes Balmora at ~12.3-12.5 ms as the live
  symptom, while `bevyrrowind/docs/perf.md` "Frame budget" records 9.5 ms Balmora / 10.5 walk / 11.1 flight
  against an 11 ms gate, measured 2026-10-28 after the shadow resets.
- **Do:** (a) correct the research note's premise line (one line; do not rewrite the research); (b) update
  H1's symptom block to the 2026-10-28 numbers and restate what is still open: the flight bench at 11.1 ms
  median, the per-preset budget table, and the render-thread structure that 1.16 static scene targets. Keep
  H1 **open** — the hypothesis and approach are unchanged.
- **Also flag, do not fix here:** the frame budget has not been re-measured since 2026-10-28, while night
  lighting, surface weather W5-W7, albedo de-lighting, three GI slices, `matclass` and the sun-shadow
  early-out all landed after it. Add a ROADMAP 1.2 note that the table is stale and needs one re-measure
  pass on the owner's machine. Do not run benchmarks from an agent session.
- **Verify:** both greps show the corrected text; ROADMAP 1.2 carries the stale-measurement note.

---

## PART B — Net-ready code defects (scoped builds; batch all edits, then one test run)

**Read first.** These are the only findings in this plan that are live *code* defects rather than drift.
They are **not** visible single-player bugs today; they are the reasons a host-authoritative session would
desync, and they are cheap now and expensive later. Two of them (B1, B4) also make gameplay depend on
presentation in single player, which the architecture forbids on its own terms.

**Owner decision needed before B2/B3:** moving mover and walker stepping onto the fixed tick changes
movement and AI timing, which the owner signs off visually (AL-, AN-, PL- checks). Do B1, B4, B5 first —
they are behavioural corrections with no feel risk — then ask before B2/B3.

**Batching:** make all edits for the slices you are doing, then one
`bash bevyrrowind/tools/cargo_turn.sh test -p sf_controller_bevy -p mw_walkers_bevy -p mw_doors_bevy`
(drop the crates you did not touch). Judge by `grep -E "^error" -A6`, never by exit code.

### B1. The render camera must not be an NPC obstacle

- **Build:** scoped (`-p mw_walkers_bevy`). **Severity:** highest in Part B.
- **Evidence (verified at `80c9a2c`):** `bevyrrowind/crates/mw_walkers_bevy/src/walkers.rs:664`
  `fn camera_obstacle(`, registered at `walkers.rs:775`. It turns the render camera's position into an
  `AvoidObstacle` that walkers steer around, differencing positions with `time.delta_secs()` in `Update`.
- **Why it is wrong:** a purely presentational object becomes part of the collision/avoidance world. In
  co-op each client's camera would shove NPCs differently, and the host's own camera would shove them for
  everyone. It also breaks the architecture's rule 2 ("the picture is a view of the world") in the worst
  direction.
- **Do:** keep the *intent* — NPCs should avoid the player — but source the obstacle from the **player
  character's gameplay transform**, not the camera, and compute it on the fixed tick. In first person the
  camera and player nearly coincide, which is why the camera was a convenient proxy; that coincidence is
  not a licence to read presentation state. If a per-player obstacle is wanted in co-op, key it by
  `sf_ids::PlayerId` so each player contributes one.
- **Verify:** `grep -n "Camera\|GlobalTransform" bevyrrowind/crates/mw_walkers_bevy/src/walkers.rs` shows
  no camera read in the avoidance path; the crate's tests pass; add a test asserting a walker avoids the
  player entity and is unaffected by camera movement.
- **Owner check:** NPC avoidance is visible behaviour — log a visual-check entry, do not judge it yourself.

### B2. Mover stepping runs on frame delta, not the tick

- **Build:** scoped (`-p sf_controller_bevy`, plus whatever fails downstream). **Severity:** high.
- **Blocked by:** owner go (feel risk, see Part B preamble).
- **Evidence (verified):** `superfargoth/crates/sf_controller_bevy/src/lib.rs` documents its own schedule at
  lines 11-20: `ControllerSystems::Drive` (`Update`), `ControllerSystems::Step` (`Update`),
  `ControllerSystems::Feed` (`PostUpdate`); `lib.rs:38` imports `PostUpdate, Update`; the step system takes
  `Res<Time>`. So **all mover locomotion, for every character including NPCs**, integrates on frame delta.
- **Why it is wrong:** net-ready rule 1 (fixed tick for gameplay). `sf_action_bevy`, `sf_combat_bevy`,
  `sf_stats_bevy` and `sf_physics_bevy::step` are already correctly on `FixedUpdate`; this crate is the gap
  in the middle of the chain, so commands arrive on the tick and are then integrated off it.
- **Do:** move `Drive` and `Step` to `FixedUpdate`; keep `Feed` in `PostUpdate` (it is presentation
  write-back and belongs there). Expect to touch the interpolation seam so rendering still reads a smooth
  transform between ticks — that is the point of keeping `Feed` separate. Check `specs/locomotion.md`
  "Movers" and `specs/action.md` "Motion owners" before changing order, and update them.
- **Verify:** scoped tests pass; `grep -n "Update" superfargoth/crates/sf_controller_bevy/src/lib.rs` shows
  only `FixedUpdate` for Drive/Step and `PostUpdate` for Feed.
- **Owner check:** movement feel. Register an entry and ask; never judge from a screenshot.

### B3. Walker AI steering on frame delta

- **Build:** scoped (`-p mw_walkers_bevy`). **Severity:** medium-high. **Blocked by:** same owner go as B2.
- **Evidence (verified):** `mw_walkers_bevy/src/walkers.rs:605` `fn drive(` and `walkers.rs:893`
  `fn stride_monitor(` run in `Update` with `Res<Time>`; `mw_walkers_bevy/src/physics.rs:780` is the same
  shape. `drive` writes `MoveIntent`, i.e. gameplay.
- **Do:** move `drive` and the intent-writing path to `FixedUpdate`. `stride_monitor` is diagnostic — if it
  only observes, leave it in `Update` and say so in a comment; if it feeds gameplay, move it.
- **Verify:** scoped tests pass; no `Res<Time>` remains in an `Update` system that writes gameplay state.

### B4. Door travel must not be picked from the render transform

- **Build:** scoped (`-p mw_doors_bevy`). **Severity:** medium.
- **Evidence (verified):** `bevyrrowind/crates/mw_doors_bevy/src/lib.rs:219`
  `cams: Query<(Entity, &GlobalTransform), With<DoorUser>>` — the `activate` system (registered in `Update`)
  picks the door from the `DoorUser` camera's `GlobalTransform`. Worldspace travel, a gameplay decision, is
  made from presentation state.
- **Context:** a 2026-10-25 decision deliberately made "the streaming camera stays the door traveller and
  script player". The vision doc flags that decision as exactly the kind of single-player shortcut that
  looks harmless and blocks co-op. Treat this slice as revisiting it, and record the outcome in
  `DECISIONS.md`.
- **Do:** pick the door from the player character's gameplay position and cell. Keep `DoorUser` as the
  marker if convenient, but put it on the gameplay entity.
- **Verify:** `grep -n "GlobalTransform" bevyrrowind/crates/mw_doors_bevy/src/lib.rs` shows no read in the
  activation path (the test-only spawn at `lib.rs:451` may keep it); crate tests pass.

### B5. NPC AI wake/sleep must anchor to the player, not the camera

- **Build:** scoped (`-p mw_walkers_bevy`). **Severity:** medium.
- **Evidence (verified):** `mw_walkers_bevy/src/walkers.rs:526` `fn processing_range(` wakes and sleeps NPC
  AI around `ExteriorCamera`. Note the contrast: R0 streaming itself is **correctly** player-anchored —
  `mw_exterior_bevy/src/exterior.rs` spawns `StreamingAnchor::gameplay()` on `Player` — so this is an
  inconsistency, not a missing concept.
- **Do:** anchor to the same gameplay anchor streaming uses. In co-op, wake around the union of player
  anchors.
- **Verify:** crate tests pass, including `processing_range_leaves_held_movers_alone` at `walkers.rs:1194`;
  add a test that camera-only movement does not wake NPCs.

### B6. Dev-only camera raycast — confirm the gate, do not remove

- **Build:** none if only reading. **Severity:** informational.
- **Evidence (verified by review):** `bevyrrowind/crates/mw_delirium_bevy/src/lib.rs` ~960-975 raycasts from
  `Camera3d` / `GlobalTransform` to apply gameplay impulses. It is the documented dev click-to-push, behind
  the `delirium_push` dev feature per ROADMAP 3.2.
- **Do:** confirm the feature gate still holds and that no release path reaches it. If gated, leave it and
  note it as a known, accepted presentation-to-gameplay path for dev only. Do not "fix" a dev tool.

---

## PART C — Layering and platform structure (scoped builds)

### C1. `sf_save_bevy` must not depend on the ARPG kit

- **Build:** scoped, and this one is a real refactor. **Severity:** highest in Part C for the platform goal.
- **Evidence (verified):** `superfargoth/crates/sf_save_bevy/Cargo.toml:39,44,46` =
  `sf_action`, `sf_rpg`, `sf_stats`. The save plugin knows the character sections of the genre kit
  (`sf_save_bevy/src/sections/character.rs`).
- **Why it matters:** this is the one layering edge that defeats the stated platform goal. A new game that
  replaces the ARPG kit **cannot use the engine's save plugin**, because the save plugin depends on the kit.
- **Do:** invert it. `sf_save_bevy` should expose a registration seam (each kit registers its own save
  sections, owner-keyed and versioned, as the save spec already describes); the kit crates depend on the
  save plugin, not the reverse. Do **not** do this as a drive-by: write the seam into
  `superfargoth/docs/specs/save.md` first, then move the sections out in a second slice. Coordinate with
  C2 of the state-registry idea in A13 — one declaration driving save + replication is the end state, and
  this seam should not be designed twice.
- **Verify:** `cargo metadata` shows no `sf_save_bevy` -> `sf_rpg`/`sf_stats`/`sf_action` edge; saves still
  round-trip (`-p sf_save -p sf_save_bevy -p mw_saves_bevy`); existing save files still load (there are
  golden save-section tests — keep them green, and if a migration is needed, add it, do not relax the test).

### C2. `sf_weather` (L1) must not depend on `sf_sky` (L2)

- **Build:** scoped (`-p sf_weather -p sf_sky`). **Severity:** medium.
- **Evidence (verified):** `superfargoth/crates/sf_weather/Cargo.toml:14` `sf_sky.workspace = true`; used in
  `sf_weather/src/frame.rs:12` and `records.rs:12`, and re-exported at `sf_weather/src/lib.rs:35`.
- **Do:** determine what `sf_weather` actually needs from `sf_sky` (likely a small shared type: sun/moon
  direction or a time-of-day value). Move that type down into a lower crate both can depend on, or into
  `sf_weather` itself if `sf_sky` is its only other user. Note the re-export at `lib.rs:35` is public API —
  changing it is a breaking change for `*_bevy` consumers; grep for users before moving.
- **Verify:** `cargo metadata` shows no `sf_weather` -> `sf_sky` edge; both crates' tests pass.

### C3. Two framework crates are Bevy plugins wearing a core crate's name

- **Build:** scoped. **Severity:** medium (compile time + the Bevy-free-core rule).
- **Evidence (verified):** of all non-`_bevy` crates, only three pull Bevy. `sf_ui_css` and `sf_ui_screens`
  pull `bevy_app`/`bevy_asset`/`bevy_ecs`/`bevy_ui`/`bevy_picking`/`bevy_text` and more — they **are** Bevy
  plugin crates. `sf_vfs` pulls `bevy_asset` only, **feature-gated** behind a `bevy` feature, so its core
  stays Bevy-free — that one is fine and needs no change. (In the game pack, `mw_assets` pulls full `bevy`
  with `bevy_pbr`; it is an asset-conversion crate, so the dependency is load-bearing, but it is the worst
  compile-time offender and worth a note in `build-perf.md`.)
- **Do:** either rename `sf_ui_css` -> `sf_ui_css_bevy` and `sf_ui_screens` -> `sf_ui_screens_bevy`, or
  record in `ARCHITECTURE.md` §5 that these two are plugin crates despite the name. **Renaming touches every
  dependant and `hakari`** — the cheap, honest option on this laptop is to document now and rename in a
  dedicated slice later. Pick the documentation option unless the owner asks otherwise.
- **Verify:** the D3 check and §5 agree on which crates are allowed to pull Bevy.

### C4. The quality preset ladder is defined in four places

- **Build:** none for the audit; scoped if you consolidate. **Severity:** medium (second-pack goal).
- **Evidence (verified):** the game's ladder is `mw_game_core::exterior::Quality`
  (`bevyrrowind/crates/mw_game_core/src/exterior.rs:39`, Low/Medium/High/Ultra), mapped to render settings
  in `mw_exterior_bevy/src/exterior.rs`. The framework separately carries `WaterQuality`
  (`sf_water_bevy/src/lib.rs:69`), `UpscaleQuality` (`sf_sky/src/lib.rs:89`, serde-serialized into the
  profile), `AnimTier` (`sf_anim_bevy/src/lod.rs:18`), and `ScenePreset` (`sf_scene/src/cull.rs`).
- **Correction to an earlier claim:** `ScenePreset` is **not dead code**. `sf_scene/src/cull.rs:17`
  documents it as "the only knobs the quality presets turn", and it is used in `cull.rs` and
  `cull/tests.rs`. It is the beginning of the framework-side preset seam, not a duplicate to delete. Treat
  it as prior art for F2, and leave it alone in this part. **`sf_scene` is owned by `coord-static-scene` —
  do not edit it from this plan.**
- **Do (this slice, audit only):** write the inventory into the `sf_scalability` spec stub (F2) — every
  preset enum, where it lives, what it switches, and whether it is player-facing. Change no code.
- **Verify:** the spec stub lists all five with file:line.

### C5. Make it impossible for the quality preset to shrink the simulation radius

- **Build:** scoped (`-p sf_world -p sf_lod_bevy`). **Severity:** low today, high if it ever fires.
- **Evidence (verified):** R0 does **not** scale with the preset today. Defaults are fixed constants
  (`superfargoth/crates/sf_world/src/rings.rs:84`, `active: RingRadii::new(192.0, 256.0)`), and the game
  rewrites only `rings.near`/`rings.far` from calibration flags. **But** the coupling is wired:
  `superfargoth/crates/sf_lod_bevy/src/lib.rs:142-143` do
  `rings.active.load = rings.active.load.min(rings.near.load)` and the same for `unload`, where
  `rings.near` is quality-derived through `sf_lod/src/levels.rs:78` (`handoff(1) = 400.0 * screen.bias`).
  The clamp starts cutting R0 at screen bias < 0.255 (R0 unload at < 0.11); shipped biases are 0.75-1.0.
  Separately `sf_world/src/rings.rs:107-115` `RingConfig::scaled()` scales `active` along with the visual
  rings; nothing calls it today.
- **Why it matters:** if R0 ever scales, how many NPCs a host simulates would depend on its graphics
  settings — gameplay driven by a video option, and divergence between co-op peers.
- **Do:** (a) add a regression test asserting `rings.active == (192.0, 256.0)` across all four presets;
  (b) split `RingConfig::scaled()` so it cannot touch `active` (or assert that it does not); (c) comment
  both clamp lines with why `active` must be excluded. Keep the clamp for the visual rings if it is
  load-bearing there.
- **Verify:** the new test fails if someone makes `active` preset-derived; existing streaming tests pass.

---

## PART D — Guardrails (cheapest high-value work in this plan)

Why this matters more than it looks: ~315k lines were written in about four weeks, mostly by agents, in a
shared working tree, with **no CI**. Every gate — `cargo test`, clippy, `cargo deny`, hakari — depends on an
agent remembering. On 2026-11-09 four sessions were editing one tree at once. Guardrails are how the rules
in this document stay true after it is filed.

### D1. CI running the gates that already exist

- **Build:** none locally — that is the point. **Severity:** highest in the plan after A1.
- **Evidence (verified):** no `.github/` directory; no other runner anywhere in the repo.
- **Blocked by:** F1 (Delirium must be fetchable, or CI cannot build the default game).
- **Do:** a GitHub Actions workflow running, on push and PR: `cargo test --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings` (pedantic), `cargo deny check`, the hakari check,
  and `tools/doccheck.sh links`. Cache the cargo registry and `target` aggressively — CI minutes are the
  whole point of moving these off the laptop. Mark the first run's duration in the workflow comments so
  later agents know the cost.
- **Do not** add any job that renders, screenshots or benchmarks. CI judges code, never looks.
- **Verify:** a deliberate clippy warning on a branch fails CI; a doc-link break fails CI.

### D2. Encode the net-ready rules as lints

- **Build:** none (config only; the lints bite on the next build anyone runs). **Severity:** high.
- **Evidence (verified):** no `clippy.toml` anywhere; `grep disallowed *.toml` returns nothing.
- **Do:** add `clippy.toml` with `disallowed-methods` / `disallowed-types` for the rules the code already
  follows by discipline, so they cannot rot: `rand::random`, `rand::thread_rng` (currently clean — `rand` is
  in no manifest), and `Time<Real>` in gameplay crates. Document each entry with the rule it enforces, so a
  future agent knows why it is denied.
- **Verify:** adding `rand::random()` to a gameplay crate fails clippy.

### D3. The boundary and layer check from `cargo metadata`

- **Build:** none (metadata only). **Severity:** high. **Blocked by:** A5 (the table must be right first).
- **Do:** a small test or script asserting from `cargo metadata --no-deps`: (a) no `sf_*` crate depends on
  any `mw_*` crate (**currently true** — verified across all manifests and dep kinds, so this is a ratchet,
  not a fix); (b) each crate's layer, per the A5 table, depends only on layers at or below it, with the
  documented exception that a kit's `*_bevy` shell sits in its kit's layer; (c) no L0/L1 crate depends on a
  Bevy render crate (**currently true**, verified transitively). Expect the first run to flag exactly C1
  (`sf_save_bevy` -> kit) and C2 (`sf_weather` -> `sf_sky`) — allowlist them with a TODO referencing those
  slices rather than weakening the rule, and remove each allowlist entry as its slice lands.
- **Verify:** the check passes with the two known allowlist entries and fails if a new upward edge appears.

### D4. The schedule lint for presentation-into-gameplay

- **Build:** none for the lint; it reports on the current tree. **Severity:** high — this is the one
  invariant with live defects and no test of any kind behind it.
- **Do:** a check over a named list of gameplay crates asserting that **no system registered in `Update` or
  `PostUpdate` takes `Res<Time>`, `Res<Time<Real>>`, `&GlobalTransform` or `&Camera`**. This single rule
  flags exactly the Part B defects — `sf_controller_bevy::step`, `mw_walkers_bevy::{drive, camera_obstacle,
  processing_range}`, `mw_doors_bevy::activate` — and nothing else. Known legitimate exceptions to
  allowlist explicitly, each with its reason: `mw_scripts_bevy/src/lib.rs` MWScript host polling (an
  allowed exception by design), the `--auto` test harness in `mw_walkers_bevy/src/physics.rs`,
  `mw_player_bevy`'s `await_ground` arrival timeout, and the dev-gated `mw_delirium_bevy` push (B6).
- **Verify:** the check reports the Part B sites before they are fixed and is clean after, with only the
  documented exceptions allowlisted.

### D5. DCO

- **Build:** none. **Severity:** medium, and it expires. **Blocked by:** A12 (same file).
- **Why now:** the owner is still effectively the sole author. The window for setting inbound = outbound
  closes with the first outside contribution.
- **Do:** the DCO paragraph in `CONTRIBUTING.md` (A12d) plus a CI sign-off check (D1).
- **Verify:** an unsigned commit on a test branch fails CI.

---

## PART E — Gameplay-lane correctness (owned by `coordinator-mechanics`)

**Do not fix these from another session.** They sit inside `coordinator-mechanics`' scope
(`sf_effects`, `sf_rpg`, `sf_rpg_ui`, `sf_ui_screens`, `mw_effects`, `mw_rpg`, `mw_rpg_bevy`, `mw_script`,
`mw_stats`, `mw_ui`) and were sent to that session on 2026-11-09. They are listed here so the owner has one
place to see everything, and so a later agent picking up that lane has the list.

**Compile status:** a correctness review found **no compile blockers**, and the ROADMAP now records the
gameplay crates as compiling and passing unit tests and clippy since 2026-11-09. The handoffs' own worry
list (`Commands::write_message`, `ActorKey: Copy`, the `tes3` enum casts, the hand-written postcard golden
bytes at `sf_rpg/src/tests_service.rs:583`) was checked and cleared.

### Read this before acting on any row below: three of eleven were false positives

**Do not treat this list as verified.** It began as eleven defects from one read-only review. A second
review of the same scope disputed three; all three were then checked **against OpenMW's actual source** in
the local clone (`Dev/openmw`, commit `c0790fc`) and **all three turned out to be non-defects — the code
was already correct.** They are struck through below with the citation that settled them, kept visible so
nobody re-raises them.

That is a false-positive rate of roughly one in four, and the failure mode was consistent and instructive:
in each case the reviewer compared our code against *Morrowind's* documented behaviour or against an
assumed rounding convention, rather than against the OpenMW source our module docs actually cite. E9 is the
sharpest example — the reviewer saw two different rounding idioms in neighbouring functions, assumed one
was a slip, and was wrong, because **OpenMW itself uses different rounding in those two branches**.

Consequences for whoever picks up this lane:

1. **Every remaining row needs source verification before it is treated as real.** The surviving rows are
   *candidates*, with the confidence noted per row. E8 is the most certain (two independent reviews
   converged on it); E6 and E7 are structural and checkable without OpenMW; the rest are unverified.
2. **Do not quote a defect count.** "Eleven defects" was wrong within a day. Hand the owner the list with
   per-item status, never a number.
3. **The fix rule in §6 is now doubly important**, and it cuts both ways: derive expected values from the
   cited source — and also check, before changing anything, whether the *code* already matches that source
   and the reviewer did not.

**Will be visible in the owner's test pass:**

| # | Site | Defect |
|---|---|---|
| E1 | `mw_rpg/src/runner.rs:482` | Text defines never expanded on the MessageBox path; every scripted box renders literally `^PCName, ...`. `rules.expand` runs only on dialogue response text (`mw_rpg_bevy/src/dialogue.rs:791`). |
| E2 | `mw_script/src/functions.txt:193` | `GetDisposition` still `stub=50` while Set/Mod are live; a script reading disposition sees a constant. `functions.rs:335` asserts the stub deliberately — a decision to settle, not a slip. |
| ~~E3~~ | `mw_stats/src/gmst.rs:82,86` | **WITHDRAWN — NOT A DEFECT. The code is correct.** Verified in the local OpenMW clone (`Dev/openmw`, commit `c0790fc`): `apps/opencs/model/world/defaultgmsts.cpp:1678` is `0.0f, // fDispCrimeMod` and `:1782` is `1.0f, // fPerTempMult`. Our fallbacks match vanilla exactly. The range table at `:2031` constrains `fDispCrimeMod` to `(-inf, 0]`, so the "believed vanilla 0.25" that prompted this finding was never even a legal value. **The bounty term being inert is intended vanilla behaviour.** |
| E4 | `mw_rpg/src/persuasion.rs:241` | A failed bribe is free; OpenMW debits unconditionally before the roll. Repeatable exploit. Also credits the speaker where OpenMW only debits. |
| E5 | `mw_rpg/src/disposition.rs:148` | Non-member faction branch adopts the first faction's reaction even when positive and takes rank from a foreign faction; OpenMW keeps only negatives. Tests pin the wrong behaviour. |

**Silent, and the tests currently confirm the bug:**

| # | Site | Defect |
|---|---|---|
| E6 | `mw_rpg/src/services.rs:133` | Sell branch drops OpenMW's `min(buyTerm, sellTerm)`. Buy at 170, sell back at 205 on the tests' own fixture — a money printer. `tests_services.rs:69,116` assert the wrong number. **Highest severity in the lane**; latent only because trade is unwired. |
| E7 | `sf_rpg/src/service/purse.rs:12` | The `sf/services` save section is declared but never registered, and `"sf/services"` is a full key where `key()` expects a bare name, yielding `sf/sf/services`. No `SaveSection` impl, no apply-order entry, so "version 1" has no migration hook. An incomplete SV1 deliverable, not deferred work. |
| E8 | `sf_effects/src/step.rs:553` | `ResistQuery.target` hard-coded `None`, so `resist` returns `1.0` always: resistances and weaknesses have no effect in the real pipeline. E5-unbuilt rather than broken — but a green `tests_rules.rs` does not mean resistances work. `LandModel::protect` and the `ItemModel` seam likewise have no call site. |
| ~~E9~~ | `mw_rpg/src/persuasion.rs:321` | **WITHDRAWN — NOT A DEFECT, and the apparent inconsistency is fidelity.** OpenMW `mechanicsmanagerimp.cpp` (commit `c0790fc`) genuinely uses **different rounding in the two branches**: Taunt is `x = floor(-c * fPerDieRollMult)` (floor of a negative), Intimidate's success path is `x = -floor(c * fPerTempMult)` (negative of a floor). For `c*mult = 9.3` those give -10 and -9 respectively. Our `floor(-c*mult)` at :321 and `-((c*mult).floor())` at :299 mirror OpenMW branch for branch. The reviewer saw the asymmetry and assumed a slip; it is a faithful port. (Unchecked by us: OpenMW also clamps Taunt with `if (success && abs(x) < iPerMinChange) x = -iPerMinChange;` — worth confirming our port has it.) |
| ~~E10~~ | `mw_rpg/src/persuasion.rs:304` | **WITHDRAWN — NOT A DEFECT. The "OpenMW, MCP" comment is exactly right.** OpenMW's own source carries the comment *"Deviating from Morrowind here: it doesn't increase disposition on marginal wins, which seems to be a bug (MCP fixes it too). Original logic: x = 0, y = -iPerMinChange"*, then sets `x = iPerMinChange; y = x; // This goes unused.` (`iPerMinChange` default 10, `defaultgmsts.cpp:1961`). Our `(min_change, min_change)` with a dead second value matches, dead value included. The reviewer cited Morrowind's original logic, which OpenMW deliberately rejects. |
| E11 | `mw_effects/src/stats.rs:10` | `MAGNITUDE_STAT_EFFECTS` omits index 84 (Fortify Maximum Magicka) though `mgef.rs:345` gives it that archetype. The test compares the two lists against each other, so it cannot see the omission. |
| E12 | `sf_effects/src/step.rs:330` | Temporary pool Fortify emits `AddModifier` **and** `PoolAdjust`, double-counting on `KeepRatio` pools. Magicka and fatigue are both `KeepRatio`, so Fortify Magicka 100 on a 50/100 pool gives 200/200 where OpenMW gives 150/200. *(Found by `coordinator-mechanics`' reviewers, not ours.)* |
| E13 | `sf_effects/src/step.rs:320` | Drain is modelled as Damage+Restore on a shared counter rather than as a modifier, so it leaks whenever an unrelated Restore lands between inflict and expiry. *(Same source as E12.)* |

**Accuracy note on E12/E13.** Our review reported that temporary pool fortify "round-trips exactly on all
three rebase modes". That statement is about the **save round trip**, which is exact, and must not be read
as clearing the **live arithmetic**, which E12 and E13 show is wrong. Two independent review prompts
converged on E8 (`step.rs:553`), which is why that one is treated as the most certain in the lane.

**Rule for fixing E2, E6, E9, E11 (adopted by `coordinator-mechanics`):** several of these defects are
*pinned by their own tests* — `tests_services.rs:69,116` assert the money-printer value, `functions.rs:335`
asserts the stub, `tests_disposition.rs:379` pins the off-by-one, and the magnitude-stats test compares two
lists against each other. When fixing, **re-derive each expected value from the cited OpenMW source**; never
adjust the code to match the test or the test to match the code.

**Written but unreachable — do not spend test-pass time here:** the trade and service-list screens
(registered in `sf_ui_screens/src/lib.rs` but zero references anywhere in `bevyrrowind/`; visual check
PUI-SV3 is untestable), the entire effects lane (~4,100 lines; nothing depends on `mw_effects`, no
`sf_effects_bevy`, no save section, no startup load), `mw_ui::book` (762 lines, no caller, no book screen),
and the Spells/Travel/Training/Repair buttons from `mw_rpg/src/barter.rs:545` (they appear and do nothing;
the four price rules have no caller). **Correction to the test-pass doc:** the Persuasion menu *is* wired
end to end, so Admire on Fargoth and DJ-2 are real in-game checks.

**Lower severity, grouped:** absolute-tick cooldowns in `sf_effects/src/book.rs:176` (E4 must rebase on
load); `step.rs:69` silently discarding book writes; `.round()` vs OpenMW's possible truncation in
`mw_effects/src/source.rs:248`; per-piece vs per-line floor in `sf_rpg/src/service/price.rs:30`;
non-atomic `apply_move` in `sf_rpg/src/state.rs`; `sf_rpg_ui/src/trade.rs:350` greying all stock until the
host's first `set_prices`; the trade and service-list screens not virtualized; `mw_rpg/src/runner.rs:481`
discarding `Message::report`; the placeholder scan implemented twice
(`mw_script/src/parser.rs:991`, `mw_ui/src/message.rs:261`) with no test coupling them; and one net-ready
nit — `sf_rpg/src/service/mod.rs` `service_seed` mixes an unsaved wrapping counter, so rolls repeat across
a reload.

---

## PART F — Platform gaps (larger; owner picks timing)

These are the items the 2026-11-08 vision doc proposed and the owner accepted, none of which exist in code.
They are ordered by "cheap now, expensive later", not by size.

### F1. Delirium: own the fork, commit the pin

- **Build:** scoped, and it unblocks D1. **Severity:** high — it blocks CI.
- **Status (verified):** the re-pin to `f4ff84d` is now committed (root `Cargo.toml`; `3e219bf`), so the
  immediate "HEAD cannot be cloned and built" problem is **resolved**.
- **Left:** the pin still points at the upstream `PVCPC/delirium-physics`, which has already rewritten its
  history once (the previous rev `91ee246` vanished). Decision 8 keeps Delirium default and non-optional, so
  a fresh clone's ability to build the default game depends on a third party not force-pushing.
- **Do:** mirror the pinned rev to a project-owned fork and pin that. Keep upstream as a remote for pulling
  fixes.
- **Verify:** a clean clone on a different path builds `mw_delirium` with default features.

### F2. `sf_scalability` and the tier matrix

- **Build:** scoped once it exists. **Severity:** high for "runs on a lot of hardware".
- **Status (verified):** no `sf_scalability` crate. The preset ladder lives in the game
  (`mw_game_core::exterior::Quality`), with four more scattered through the framework (C4). The owner
  accepted Steam Deck as a supported floor tier with its own gate (decision 3); there is **no floor-tier
  benchmark and no FSR/XeSS path** — `sf_sky::Upscaling` offers `Off`/`Taa`/`Dlss` only, so non-NVIDIA
  hardware falls back to TAA at a higher internal resolution and would miss a gate tuned for DLSS.
- **Do, in order:** (1) write the spec, including the C4 inventory and the tier matrix (floor / mainstream /
  reference / top, each with its device, target and path); (2) presets become **records** (RON, moddable,
  overridable per game) driving typed console variables, not Rust enums — `sf_console` already has typed
  variables every plugin registers, and `sf_scene::ScenePreset` is prior art for the seam; (3) add an
  `sf_upscale` seam with FSR beside DLSS and TAA (FidelityFX is MIT, so no plugin package is needed);
  (4) add a floor-tier benchmark JSON next to the eight in `bevyrrowind/benchmarks/`.
- **Note:** express budgets **per tier, CPU and GPU separately**. Today's single "<= 11 ms" hides the
  render-thread bound (H1), which is the axis that hurts a handheld most.
- **Verify:** the Options screen (ROADMAP 2.2b) can render groups from records; a floor-tier gate exists and
  is measured on real hardware by the owner, not by an agent.

### F3. The sample pack, and with it the second consumer

- **Build:** scoped (an `sf_*`-only app). **Severity:** high for the platform claim.
- **Status (verified):** no `samples/`. The framework/game split holds in code — zero `mw_*` dependencies in
  any `sf_*` manifest, verified across all manifests — but it is checked only by reading. There is no second
  consumer, so Morrowind-shaped assumptions can leak in silently.
- **Do:** `samples/greybox` per decision 5: S0 now, headless and minimal, an `sf_*`-only app that runs in
  CI and hosts the F4 loopback test; S1 (the rendered CC0 valley, Blender -> glTF levels, JSON Schemas for
  records and `mod.toml`, the human guide written from it) after the Morrowind vertical slice.
- **Verify:** the sample app builds and runs headless in CI with no `mw_*` crate in its dependency graph.

### F4. Two-peer loopback co-op test

- **Build:** scoped. **Severity:** high — it is the only way the net-ready rules stop being prose.
- **Status (verified):** there is **no netcode at all**: no `lightyear`, no `replicon`, no `sf_net`, no
  headless app build. Principle 3 is "multiplayer is the default topology" and decision 2 set no ceiling,
  yet N1-N5 wait until after M6. Meanwhile Part B shows four single-player shortcuts already in the tree.
- **Do:** after F3, two peers in one process (lightyear 0.31 supports Bevy 0.20 and is built on replicon;
  both offer in-memory transports), one capsule each, commands on the tick, asserted in CI. Then make
  "the simulation runs with no renderer" a CI target — a `--headless` build without `bevy_render`, which is
  what a dedicated server is and also gives fast gameplay tests.
- **Verify:** the test fails if a gameplay system reads presentation state or runs off the tick. That is the
  point: it turns Part B and D4 into a failing test rather than a review finding.

### F5. One state registry for save + replication + prediction

- **Build:** scoped. **Severity:** medium-high, and it gets more expensive per system added.
- **Status:** `sf_save` sections and any future replicated components are separate registries; Unreal marks
  one property declaration as both. Every new system currently invents its own save section.
- **Do:** design it in `net.md` and `save.md` together (A13), then implement so one declaration drives the
  save section, the replication rule and the prediction class, sharing one serializer and one schema
  version. Do this **before** more systems add sections — `sf_effects` (E7) is already a half-registered
  example of the cost of not having it. Sequence it with C1, which touches the same seam.

### F6. Bevy upgrade lane

- **Build:** full, once, by the owner's choice. **Severity:** medium now, high at 0.21.
- **Status (verified):** five crates are vendored as `path` patches in the root `Cargo.toml`
  (`big_space`, `bevy_core_pipeline`, `bevy_pbr`, `bevy_render`, `bevy_solari`) — ~104k lines carrying
  roughly 10 of our own render features plus upstream PRs. Decision 6 called for moving them to a git fork
  branch on the next rebase; they are still paths.
- **Do:** (1) move the vendored crates to a git fork branch so rebases are git operations, not directory
  edits; (2) upstream the two generic hooks most of our patches need — a view-level material extension point
  and a pluggable diffuse-GI provider — after which most vendored hunks become plain plugin code;
  (3) adopt each Bevy minor within 4-6 weeks, never skipping, starting from the RC. Keep the A1/A8 rule in
  view: the GPL-derived H2 hunk must never go upstream.

### F7. The systemic layer — the actual grand vision

- **Build:** specs first, no build. **Severity:** this is what "a world that feels real" means.
- **Status (verified):** `sf_ai` and `sf_ai_bevy` now exist (AI1-AI5: HTN planner, directives, save
  section, debug overlay) and `specs/ai.md` + `nav.md` are written. Everything else in vision §7 is absent:
  no `sf_interaction`, no perception/stimuli layer, no schedules/calendar, no simulation LOD (actors are
  simply dormant outside R0), no material/element rules, no per-NPC memory. `PlacementOverlay` is the one
  piece of persistence that exists.
- **Do, in the vision's order, research first (CONTRIBUTING rule 1), one spec per slice:** persistence
  policies (extend `PlacementOverlay`: moved/removed/spawned, per-entity state blobs, reset policy per cell)
  -> schedules + simulation LOD (full / coarse-analytic / statistical tiers, so an NPC is where their
  schedule says when you arrive) -> interaction and smart objects (affordances, claims, animation hooks)
  -> perception and stimuli -> material and element rules. Each spec carries a "Script surface" section
  (A12d) and a "Co-op" section. Lesson from Radiant AI: systemic but **bounded** — authors need knobs that
  keep emergent behaviour from breaking quests.
- **Also here:** the no-RT GI fallback (A10d). Share the probe-volume data model and change only the update
  source (rays vs a one-time bake), so the look stays consistent across tiers instead of cliff-edging to
  "off" on hardware without ray queries.

### F8. One authoritative game clock (already ROADMAP 1.17 — cross-lane, owner has not assigned it)

- **Build:** scoped. **Severity:** high, and it is a live gameplay bug, not only a net-readiness issue.
- **Status (verified at `80c9a2c`):** ROADMAP 1.17 (owner 2026-11-09, "an obvious fix, needed for
  multiplayer") records **three** clocks: `sf_sky::SkyClock` (saved `sf/clock`, `f64` days),
  `sf_rpg::GameTime` (`sf_rpg/src/quest.rs:66`, integer minutes; AI, dialogue, journal) and
  `sf_effects::GameMinutes` (`sf_effects/src/book.rs:21`). Nothing converts between them, so **AI directive
  timers cannot advance in game**. `sf_effects/src/book.rs:18` documents that the host is expected to
  convert from `sf_rpg::GameTime::total_minutes` and that `sf_effects` does not — so the seam is known, just
  unimplemented.
- **Why it belongs in this plan:** it is the same seam as the E-lane's absolute-tick cooldown problem
  (`sf_effects/src/book.rs:176`), which breaks if the tick counter restarts on load. Fixing the clock first
  makes the cooldown rebase trivial; fixing them separately means doing the conversion work twice.
- **Ownership:** assigned by the owner on 2026-11-09 to `coordinator-mechanics` (`repo-f2`). It crosses that
  lane (`sf_rpg`, `sf_effects`) and the sky/world lane (`sf_sky`), so coordinate on `sf_sky::SkyClock`.
- **Do:** research and spec first per ROADMAP 1.17 — one host-authoritative integer clock in a low-level
  Bevy-free crate, saved once, replicated by `sf_net` later, with the other two derived from it. Brief at
  `docs/briefs/ai-next.md`.
- **Related:** ROADMAP 1.18 (IK keeps running while paused, owner 2026-11-09, not diagnosed) is a separate
  pause-gate bug in the same general area; do not bundle them.

---

## 4. What changed on 2026-11-09 (so later readers can date this document)

- This folder was **20 commits behind** `origin/main` with a dirty tree holding an older copy of work
  already pushed. Verified independently from two sessions: every changed and untracked file was
  byte-identical to `origin/main` once line endings were normalized, so **nothing unique was ever at risk**.
  The tree was stashed (`stash@{0}`) and fast-forwarded to `80c9a2c`; a remote backup also exists
  (`origin/backup/coord-repo-worktree-2026-11-09`, and `origin/backup/laptop-2026-11-09`).
- Landed in those 20 commits: `sf_ai` + `sf_ai_bevy` (AI1-AI5), static-scene S0-S3a including the
  `bevy#25978` `GpuBatchedMesh3d` port into vendored `bevy_pbr`/`bevy_render`, the Delirium re-pin, the
  gameplay lane (stats S7, disposition + Persuasion menu, effects E1-E3, services SV1-SV3a, MessageBox),
  and most docs. Any analysis based on files read before the sync may be stale — including parts of the
  2026-11-08 vision doc's own §0 facts.
- A compile check of the pre-sync tree was **inconclusive and should not be cited**: it reported 9 errors in
  `mw_rpg` and `sf_rpg_ui`, but every symbol they named existed, because cargo was reusing stale rmeta from
  backdated laptop-written files. It was stopped on the owner's instruction to economise on builds. The
  authoritative statement is the ROADMAP's: the gameplay crates compile and pass unit tests and clippy as of
  2026-11-09.

## 5. Open questions for the owner

1. **Part B scheduling (B2, B3).** Moving mover and walker stepping to the fixed tick is correct per the
   net-ready rules, but it changes movement and AI timing and so needs your eyes. Do it now while the code
   is small, or defer to the co-op phase and accept a larger change later?
2. **Charter wording (A6).** Confirm that `ARCHITECTURE.md` should lead with the platform charter
   ("an engine for systemic, persistent 3D open worlds with characters") while keeping "Morrowind if
   Bethesda made it in 2026" as the first pack's vision. The current docs lead with the ARPG framing, and
   every new spec is written to whichever one is on top.
3. **E2 and E10** are deviations from OpenMW that a previous agent made deliberately and documented. They
   need a ruling, not a fix: `GetDisposition` as a constant stub, and the Intimidate marginal-win branch.
4. **Frame budget re-measure (A14).** The passing table is from 2026-10-28 and roughly a dozen rendering
   features have landed since. One re-measure pass on your machine would tell us whether Phase 1 is still
   green before more is built on top of it.
5. **F2 ordering.** `sf_scalability` is a precondition for the Steam Deck floor tier you accepted, but it is
   also a refactor across five preset enums. Before or after the Morrowind vertical slice?
6. **Install-gated tests (§6a) — two separate questions, please answer them separately.**
   (i) The ~100 non-ignored gated tests that pass having asserted nothing: may we make them fail loudly or
   report as skipped? This needs no licence decision and no CI, it is a local change, and until it lands
   every green test run you see is partly fiction. The only reason it is a question at all is that
   `census_real_install.rs`'s own doc says skipping-as-passing is intended, so it is your design call.
   (ii) Can CI hold a licensed Morrowind install? If not, the install-gated suite never runs anywhere, and
   every OpenMW-port claim stays unverified by test — which changes how much D1 is worth.

## 6a. The strongest finding in this document: "green tests" currently prove very little

**There is no Morrowind install on this machine.** Verified 2026-11-09 by `coordinator-mechanics` and
confirmed directly by the owner: `BEVYRROWIND_MORROWIND` is unset, no install in any standard Steam/GOG/
Bethesda location, no `Morrowind.esm` anywhere under the user folder. The game cannot be launched here; the
machine can compile, slowly, and that is all.

Consequence: install-gated tests **skip and report success**, and this happens on *both* paths.

**Certain (verified directly at `80c9a2c`):**

- `bevyrrowind/crates/mw_bake/tests/census_real_install.rs` is a plain `#[test]` at line 10 with **no
  `#[ignore]` anywhere in the file**, which early-returns at lines 12-14 after printing "no Morrowind
  install detected; skipping". It therefore runs and passes in an ordinary `cargo test --workspace`, having
  asserted nothing. Its own module doc says "Skips (passes) without a Morrowind install" — so this is a
  **documented design choice**, not an oversight, which is why it needs an owner ruling rather than a fix.
- The deliberate verification path is no better: `coordinator-mechanics` ran
  `cargo test -p mw_stats --test real_install -- --ignored --nocapture` and got **1 passed, 0 failed,
  0.01 s, nothing asserted**. So someone running `--ignored` specifically to check Morrowind behaviour is
  told "pass".
- Scale: 45 files under `bevyrrowind/crates` contain the skip-and-return pattern.

**Not pinned down — the honest answer is a range, and three counting passes failed to narrow it.** The
difficulty is real, not sloppiness: the gate often lives in a **shared helper**, so tests that pass
vacuously need not contain the gate string themselves. Verified examples of such helpers:
`mw_bake/tests/anim_real_install.rs:30` (`fn content() -> Option<Content>`) and
`mw_actors/tests/equipment_audit.rs:34` (`fn setup() -> Option<...>`).

| Counting method | Result | Bias |
|---|---|---|
| Tests whose own body contains the gate | 30 non-ignored across 19 files (9 in `mw_bake`) | **under** — misses every helper caller |
| Every non-ignored test in a gated file | ~106 across 37 files (20 in `mw_bake`) | **over** — a gated file may also hold ungated tests |
| Broader gate pattern, same file-level rule | 122 across 46 files | over, more so |

**So: between 30 and ~106, true figure in between. Quote the range with its method, never a bare number**
— including the `mw_bake` subtotal, which is 9 by the inline method and 20 by the file-level one. For
anyone citing the static-scene lane's "191 tests pass", the honest form is "191 passed, of which 9-20
asserted nothing without an install".

**The one cheap way to settle it exactly, when a build is affordable** (not today):
`mw_bake/src/inputs.rs:164` already fabricates a synthetic install. One run with a fabricated install,
diffed against one run without, gives the exact set of tests whose result depends on install presence.
Definitive, one build. *(Note: `inputs.rs:164` is a fixture builder, not a gate — do not count it as one.)*

Put together with §6 (four tests that assert their own bug), the honest reading is:

> A green `cargo test --workspace` on this machine is **not** evidence for any Morrowind-behaviour claim.
> It shows the code compiles and is self-consistent. It does not show the ports match Morrowind.

This is the single strongest argument in this document for **D1 (CI)**, and it changes what CI is for: CI
with a licensed Morrowind install is the only way the install-gated suite ever actually runs. Until then,
every "ported from OpenMW, tests pass" claim in the repo rests on tests written from the implementation.

**Do — and note (a) needs no decision from anyone:**

- **(a) Fix the silent pass now. This is a local change and is not blocked on the licence question.** The
  ~100 non-ignored gated tests should either fail loudly without an install, or report as skipped somewhere
  a human sees it. Until then they silently corrupt every green run the owner looks at, including runs used
  to decide whether a slice is done. Cheapest honest form: keep the gate, but make the absence of an install
  a hard error unless an explicit opt-out env var is set, and `#[ignore]` everything that genuinely needs
  the data. This is the highest value-per-effort item in the whole document.
- **(b)** In D1, decide whether CI can hold a licensed install (the owner's own copy cannot be
  redistributed — see the clean-room rule and "Morrowind's own data is never redistributed"). If it cannot,
  say so plainly in the CI docs so nobody mistakes a green CI for verified Morrowind behaviour.
- **(c)** Treat every OpenMW port's expected values as source-derived (§6) — without an install that is the
  only verification available. `coordinator-mechanics` has OpenMW cloned locally (`Dev/openmw`, commit
  `c0790fc`) and is settling E3 from source, which is the pattern to copy.

## 6. One process finding, not a code finding

Four separate slices each shipped a **test that certifies its own bug**:

- `mw_rpg/tests_services.rs:69,116` assert the buy-at-170-sell-at-205 money printer (E6).
- `mw_script/src/functions.rs:335` newly asserts the `GetDisposition` stub (E2).
- `mw_rpg/tests_disposition.rs:379` pins the Taunt off-by-one (E9); `:97,100,242` pin the faction-standing
  deviation (E5).
- `mw_effects`' `magnitude_stats_match_the_mgef_table` compares two of our own lists against each other, so
  it cannot see the omitted index (E11).

In each case the test was written from the code rather than from the cited OpenMW source, so a green test
run is evidence that the code does what the author believed, not that it matches Morrowind. This is why the
2026-11-10 pass could have come back clean on a lane with a money printer in it.

The mechanical fix is a rule, now adopted in Part E: for any formula ported with a `cite file + commit`
module doc, the test's expected value must be **derived from the cited source**, and the citation must name
the line that produced the number. `CONTRIBUTING.md` already requires exact assertions for logic and data
(as against sanity-only for look and feel); what is missing is the requirement that the expected value's
provenance be the source, not the implementation. Worth one line in `CONTRIBUTING.md` "Tests and visual
checks" — it is cheaper than any of the bugs it would have caught.
