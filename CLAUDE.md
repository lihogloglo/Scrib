# Bevyrrowind

Morrowind content pack and game binary on the superfargoth framework.
**GPL-3.0-or-later.** Sibling repo `../superfargoth` (MIT/Apache) is a path dependency.

## Start of every session

1. Read `docs/ROADMAP.md` ("Next up" and the current milestone).
2. For structural work, read `../superfargoth/docs/ARCHITECTURE.md`.
3. For the system you touch, read its spec in `docs/specs/` (or
   `../superfargoth/docs/specs/`). No spec yet for a major system? Write it first.

## End of every session

Update `docs/ROADMAP.md`: status marks, "Next up", one session-log line.
Record decisions with lasting effect in ARCHITECTURE.md §10.

## Parallel sessions

Several Claude sessions may work in this folder at once, **in the same
working tree**. Protocol:

1. **Register.** At start, create `../.sessions/<short-name>.md` (the
   parent folder is not a repo, so this is never committed) with your scope:
   the crates/files you own, and what you're doing. Read the other files in
   `../.sessions/` and stay out of their scope. Delete your file when you finish.
2. **One build at a time.** Cargo's lock prevents corruption but not
   two sessions fighting over the CPU (a Bevy rebuild can take minutes).
   Before any `cargo build/test/clippy/run`, check for a running build
   (`tasklist | grep -iE "cargo|rustc"` in bash). If another session's build
   is running, wait for it, using a background `until` loop rather than
   sleeping. Keep the build/test/run cycle short so you don't hold the
   machine for long; spec-only sessions shouldn't need to build at all.
3. **Shared files.** `docs/ROADMAP.md`: edit only your milestone's section
   plus one session-log line. ARCHITECTURE.md §10: append only. Workspace
   `Cargo.toml` / `Cargo.lock`: change only when your scope needs a new
   crate or dependency, and note it in your session file.
4. **Never commit unless the user asks.** If asked, stage explicit paths
   (`git add <your files>`), never `git add -A` or `commit -a`: the other
   session's work is in the same tree.
5. **Don't revert changes you didn't make.** Files changing under you are
   likely the other session's work; check `../.sessions/` and adapt.

## Hard rules

1. **Dependency direction is one-way.** superfargoth never depends on this
   repo and never names Morrowind formats, IDs, or conventions. Need
   Morrowind knowledge in the framework? Add an adapter hook.
2. **No GPL-derived code in superfargoth.** GPL reuse (e.g. OpenMW) is fine
   *here*, never there.
3. **Everything is a mod.** Content flows through `sf_vfs` / `sf_records` /
   `sf_mods`. No privileged content paths.
4. **Industry standard, never kludge.** Name the canonical technique first.
   Temporary scaffolds are named, isolated, and come with a removal path.
5. **Verify visually, but the owner's eyes are the judge.** Rendering work
   isn't done until it ran and was seen. Take screenshots for sanity
   (`cargo dev view "<cell>" --screenshot <png>`, then look at the PNG),
   but at most ~3 screenshot iterations per issue: no long screenshot-debug
   loops. Then add a test scene that isolates the behavior (named camera,
   time, flags, "what to look for") to `docs/visual-checks.md` (status
   "awaiting owner") and move on; the owner confirms in batched passes.
   Test scenes come in two kinds: real game locations (`view` scenes) and
   isolated synthetic scenes (framework examples), as in Godotwind.
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

## Commands

```sh
cargo dev view "Seyda Neen, Census and Excise Office"   # Bevy dynamically linked
cargo run --release -p bevyrrowind -- info              # load all content, report
cargo dev scene --list                                  # test scenes (scenes.ron); `scene <name> [view flags]` opens one, F1 shows the A/B keys (docs/visual-checks.md)
cargo run -p bevyrrowind --features dev,solari,dlss -- view "<cell>" --rt [--dlss quality]   # ray traced + DLSS (SDK in ../sdk)
cargo test --workspace && cargo clippy --workspace --all-targets --features bevyrrowind/dev
```

Clippy runs at `pedantic`; keep it warning-free.
