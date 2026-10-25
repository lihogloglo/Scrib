# Bevyrrowind

Morrowind content pack and game binary on the superfargoth framework.
**GPL-3.0-or-later** (+ GPU vendor runtime permission, `LICENSE-EXCEPTION.md`). Sibling repo
`../superfargoth` (also GPL since 2026-10-25) is a path dependency.

## Start of every session

System map: open `docs/SYSTEMS.md`, find your system's row (specs and sections, crates, ROADMAP and visual-check IDs), read only those.

**Sub-agent with a brief** (most sessions): the brief is your context. Read only what it names, and
specs by section (`grep -n "^#" <spec>`, then read that heading's lines). Don't read the ROADMAP,
ARCHITECTURE, DECISIONS, visual-checks or perf.md whole: grep them for your task's ID or system.
To update the ROADMAP, edit only your task's line (grep for it).

**Orchestrator or a session without a brief:**
1. Read `docs/ROADMAP.md` (owner decisions, current phase).
2. For structural work, read `../superfargoth/docs/ARCHITECTURE.md` (~480 lines).
3. For the system you touch, read its spec in `docs/specs/` (or
   `../superfargoth/docs/specs/`). No spec yet for a major system? Write it first.
4. **Never read whole:** `docs/archive/`, `../superfargoth/docs/DECISIONS.md` (append-only, ~36k
   tokens), `docs/visual-checks.md` (index, ~3k tokens; add an entry to the right `docs/visual-checks/<system>.md` and its index line, grep for IDs), `docs/perf.md`.

## End of every session

Update `docs/ROADMAP.md`: your task's status mark and one line in `docs/CHANGELOG.md`.
As-built detail goes in the spec and the commit message, never the roadmap.
Record decisions with lasting effect as one dated bullet (≤ 3 lines) in
`../superfargoth/docs/DECISIONS.md`.
**Doc budget:** ROADMAP ≤ ~250 lines, a spec ≤ ~25 KB (split "As built" history into an archive
file next to it), a visual-check entry ≤ ~10 lines. Agents load these files; size is context.
Doc cleanup slices run `../tools/doccheck.sh snapshot` before and `verify` + `links` after (no lost line or ID),
and put its final `doccheck: ...` line in the commit message.

## Parallel sessions

Several Claude sessions may work in this folder at once, **in the same
working tree**. Protocol:

1. **Register.** At start, create `../.sessions/<short-name>.md` (the
   parent folder is not a repo, so this is never committed) with your scope:
   the crates/files you own, and what you're doing. Read the other files in
   `../.sessions/` and stay out of their scope. Delete your file when you finish
   (`../.sessions/_archive/` holds old handoffs; not live sessions, ignore it).
2. **One build at a time.** Cargo's lock prevents corruption but not
   two sessions fighting over the CPU (a Bevy rebuild can take minutes).
   Before any `cargo build/test/clippy/run`, check for a running build
   (`tasklist | grep -iE "cargo|rustc"` in bash). Count builds from **any**
   project, not only ours: the owner builds other projects on the same
   machine (e.g. Ovid, whose cargo lives in `%TEMP%\ovid-build-tools`).
   If another build is running, wait for it, using a background `until` loop rather than
   sleeping. **Run every cargo command through `bash tools/cargo_turn.sh <cargo args>`**
   (from the repo root: `bash bevyrrowind/tools/cargo_turn.sh ...`; from superfargoth:
   `bash ../bevyrrowind/tools/cargo_turn.sh ...`; same workspace from anywhere): it waits for the
   slot and runs at BelowNormal priority, so a check and its build can't drift apart.
   Keep the build/test/run cycle short so you don't hold the
   machine for long; spec-only sessions shouldn't need to build at all.
3. **Shared files.** `docs/ROADMAP.md`: edit only your task's lines plus
   one "Recent" line. `DECISIONS.md`: append only. The workspace
   `Cargo.toml` / `Cargo.lock` (repo root, shared with superfargoth): change only when your scope
   needs a new crate or dependency, and note it in your session file.
4. **Never commit unless the user asks.** If asked, stage explicit paths
   (`git add <your files>`), never `git add -A` or `commit -a`: the other
   session's work is in the same tree. The index is shared too: commit with
   `git commit -- <paths>` (commits only those paths, whatever else is staged;
   `git add` new files first), never a bare `git commit` after `git add`.
5. **Don't revert changes you didn't make.** Files changing under you are
   likely the other session's work; check `../.sessions/` and adapt.
6. **Small slices, lean context (owner 2026-10-18, 2026-10-25).** Tokens grow with what an agent
   pulls into its context (every call re-sends it), not with its number of tool calls, so the
   budget is about scope and reading habits, not a call cap.
   - **Whoever writes a brief prepares it.** One deliverable plus its test; if it needs "and", it's
     two slices. Give the exact files, functions and line ranges, the previous handoff, what's
     already ruled out, and how to verify, so the agent doesn't explore. Research/diagnosis and
     building are separate slices.
   - **Read lean.** Grep first, then read only the lines needed; never a whole large file or spec
     (specs over ~25 KB are split into design + "As built"). Cargo output filtered to
     errors/warnings (`2>&1 | grep -E "^(error|warning)" -A6`), tests to summary lines, logs with
     `tail`/`grep`.
   - **Hand off on context, not on a count.** Write the handoff in `../.sessions/<name>.md` as you go.
     Once you've read a lot (several large files or long logs) or your context was summarised, stop
     at a compiling state, finish the handoff and report; a fresh agent continues from the file.
     Target: done or handed off by ~100k tokens of context (owner 2026-10-25): past that, every
     step re-sends the whole context, so cost climbs fast. Orchestrators split slices to fit it.
   - Orchestrators log tool calls and tokens per slice in their session file to see what works.

## Hard rules

1. **Dependency direction is one-way.** superfargoth never depends on this
   repo and never names Morrowind formats, IDs, or conventions. Need
   Morrowind knowledge in the framework? Add an adapter hook.
2. **Morrowind-specific ports live here.** Both repos are GPL; OpenMW ports
   belong here because they are Morrowind-specific, not for licence reasons.
   Cite file + commit. No NVIDIA/proprietary SDK code in the game binary
   (DLSS is a separate plugin package). For Morrowind content mechanics (MWScript, dialogue,
   magic effects, levelled lists, crime), port OpenMW (cite file + commit)
   instead of re-deriving; feel systems are modern first
   (`../superfargoth/docs/ARCHITECTURE.md` §1b).
3. **Everything is a mod.** Content flows through `sf_vfs` / `sf_records` /
   `sf_mods`. No privileged content paths.
4. **Industry standard, never kludge.** Name the canonical technique first.
   Temporary scaffolds are named, isolated, and come with a removal path.
5. **Verify visually, but the owner's eyes are the judge.** Rendering work
   isn't done until it ran and was seen. Take screenshots for sanity
   (`cargo dev view "<cell>" --screenshot <png>`, then look at the PNG),
   but at most ~3 screenshot iterations per issue: no long screenshot-debug
   loops. Then add a test scene that isolates the behavior (named camera,
   time, flags, "what to look for") to the right `docs/visual-checks/<system>.md` plus its index line in `docs/visual-checks.md` (status
   "awaiting owner") and move on; the owner confirms in batched passes.
   Test scenes come in two kinds: real game locations (`view` scenes) and
   isolated synthetic scenes (framework examples), as in Godotwind.
   **Tests for look and feel (owner 2026-10-25):** physics, animation, rendering and UI-look
   tests assert only sanity (no explosion or NaN, nothing through the floor, comes to rest, no
   crash); tuned quality numbers (stretch cm, settle time, ms, pixel diffs) are printed by
   `#[ignore = "measurement: ..."]` tests, never pass/fail limits. Quality is the owner's
   visual check. Logic and data (saves, quests, scripts, parsers) keep exact assertions.
6. **Not a 1:1 port.** Prefer modern behavior; keep rules data/adapter-driven.
7. **Compile time is a constraint** (ARCHITECTURE.md §8b): logic in
   Bevy-free crates, thin Bevy plugin crates, wide shallow crate graph,
   minimal Bevy features per crate.
8. **Research first, and challenge our docs.** Before designing or building
   a system, research how the industry does it today: papers, GDC/SIGGRAPH
   talks, engine docs, OpenMW and the modding community, and the current
   crate ecosystem (versions, Bevy compatibility, maintenance). Specs and
   ARCHITECTURE.md are not gospel. If research shows a better way, update
   the spec and log the change in ARCHITECTURE.md §10.
   Prior projects (Godotwind on Godot, ECSwind) are anecdotes, not designs:
   Godot's node tree, GDScript and renderer quirks don't transfer to Bevy,
   so never copy their tiers or constants; calibrate from our own Bevy
   measurements.
9. **Reuse before building, in this order: Bevy, then crates, then our
   own.** First use what Bevy itself provides (check its docs, examples and
   source for the version we're on). Then maintained crates and tools (GPL
   is fine in this repo; superfargoth needs MIT/Apache/BSD/zlib). Write our
   own only when neither fits, and say in the spec why each earlier step
   didn't.
   **Don't reinvent the wheel: this covers whole projects, not just
   crates.** Before starting a system, look for open-source games, engine
   ports and Bevy projects that already built it (e.g. Mudcrab, a Skyrim
   engine on Bevy with Luau scripting; OpenMW for Morrowind behaviour).
   If one is building what we need and isn't on our critical path, wait
   for it and adopt or port it (license permitting) instead of racing it.
10. **Finish before starting.** Drive started tasks (`[~]`) to verified and
   committed before opening new systems; ask the owner before starting a
   new lane. Parked (owner 2026-10-18): further Lua/Luau work after M5 L3
   (see ROADMAP "## M5").

## Commands

```sh
cargo dev view "Seyda Neen, Census and Excise Office"   # Bevy dynamically linked
cargo run --release -p bevyrrowind -- info              # load all content, report
cargo dev scene --list                                  # test scenes (scenes.ron); `scene <name> [view flags]` opens one, F1 shows the A/B keys (docs/visual-checks.md)
cargo dev view level:ledges [--camera beam]             # synthetic greybox test level (levels/*.ron, docs/specs/test-levels.md); scenes `level-*`
cargo run -p bevyrrowind --features dev,solari,dlss -- view "<cell>" --rt [--dlss quality]   # ray traced + DLSS (SDK in ../sdk)
cargo test --workspace && cargo clippy --workspace --all-targets --features bevyrrowind/dev
```

Clippy runs at `pedantic`; keep it warning-free.

## Live inspection (BRP)

Dev builds (`remote` feature, part of `dev`) serve the Bevy Remote Protocol
on 127.0.0.1 in interactive `view`/`scene` runs, for the `bevy_brp` MCP
server (`bevy_brp_mcp`, registered for this workspace; owner 2026-10-18).
Port `BEVYRROWIND_BRP_PORT`, default 15712 (Ovid uses 15702), shown in the
window title. Code: `apps/bevyrrowind/src/remote.rs`.

- **Attach only.** Build (`cargo build -p bevyrrowind --features dev`),
  then start it with `powershell -File tools/run_dev.ps1 view "<cell>" > target/run_dev.pid`
  (or `scene <name>`; it prints the pid; from bash redirect to a file, since a
  `$(...)` pipe stays open while the game runs), not `cargo dev`: a `cargo run` stays alive while
  the game is open and stalls every other session's build check. Then
  talk to it. `brp_launch`/`brp_shutdown` are denied:
  launching builds behind our back, and the owner may be looking at the
  window. Don't call `brp_extras/shutdown` through `brp_execute` either;
  close only processes you started.
- **Game tools first:** `brp_list_agent_tools` lists `bevyrrowind/status`
  (scene, hour, A/B switches, camera, `ready`) and `bevyrrowind/set`
  (switches, hour, clock, clouds, camera); call them with `brp_execute`.
  Poll `ready` before `brp_extras_screenshot`. Other A/B keys (F4 DLSS, B
  bodies, ...) via `brp_extras_send_keys`. `world_query` and friends only
  see reflected, registered types (Bevy's, and few of ours yet).
- **Not a replacement** for headless tests, `--screenshot` runs,
  benchmarks or the owner's sign-off (rule 5): the screenshot cap still
  holds. Never benchmark with BRP on (`--bench` runs don't start it).
- Need a new live control? Add it to `bevyrrowind/set` or a new
  `bevyrrowind/*` agent tool in `remote.rs` (one place, typed schema),
  not another env-var hook.
