# Meta-coordinator playbook

How to run several lane sessions at once with one coordinator between them and the owner (owner 2026-11-26).
The working rules are still `CONTRIBUTING.md`; this page only says how the coordinator applies them.

## Roles

- **Owner:** talks only to the meta-coordinator. Answers design and taste questions, does the visual checks.
- **Meta-coordinator** (one Claude Code session): writes no code. Opens lane sessions, relays questions, verdicts
  and unblocks, keeps crates from being edited by two lanes at once, launches the owner's scenes, logs everything in
  `.sessions/meta.md`.
- **Lane orchestrators** (one session per lane: anim, AI, interaction, specs, bug hunts): each spawns its own
  sub-agents with prepared briefs (CONTRIBUTING "Working with coding agents"), one build agent at a time, and never
  talks to the owner directly.

## Capacity

- **Build lanes: about 4.** Every compile goes through one queue (`cargo_turn.sh`). On 2026-11-26, agents waited
  about as long as they built (264 min of waiting, 279 min of building in 24 h). A fifth build lane would mostly queue.
  Check with `bash bevyrrowind/tools/cargo_turn.sh --stats 1`.
- **Docs, spec and research lanes are nearly free:** they never compile, so add them whenever there is spec work.
- **Cost:** the meta layer costs a few thousand tokens per relay, small next to the 100-170k per slice that lane
  sub-agents spend. Output grew about in proportion to lanes (2026-11-26: 17 commits in the first hour with 4 lanes).

## Opening a lane session

1. Write its prompt to `.sessions/_prompts/<name>.md` (untracked). Template:
   - "You are orchestrator `<name>` for `<lane>`. Register `.sessions/<name>.md` (Scope/Status), read `<previous
     handoff>` and archive it."
   - **Communication:** "The owner talks ONLY to the meta-coordinator, session `<meta name>`. Never ask the owner
     anything in this window. Questions (with a recommended answer), owner launches (scene + what to look at, sent to
     meta BEFORE launching: meta launches), verdicts, commits and handoff go to `<meta name>` via SendMessage."
   - The queue (slice IDs from the lane roadmap), the crates it must stay out of, owner answers it needs.
   - Commit rule (below), "stage only your own hunks in shared files", hand off at ~200k context.
2. Launch it (PowerShell):
   `Start-Process cmd.exe -ArgumentList '/k','title <name> && claude --permission-mode auto "Read .sessions/_prompts/<name>.md and follow it exactly."' -WorkingDirectory D:\Gamedev\Bevyrrowind`
3. Sessions you launch may not show in `ListAgents`. Their first message carries a `from="uds:..."` address:
   SendMessage to that address works.

## Keeping lanes apart

- **One lane per crate at a time.** Before saying yes to a slice in a crate another lane touched, check
  `git status --short <crate>` and the Scope lines (`head -2 .sessions/*.md`). A lane claims a crate in its Scope
  and tells meta when its commit frees it. Meta relays "X committed" to whoever waits on it.
- **Hot crates** (2026-11-26, `docs/roadmaps/order.md`): `mw_walkers_bevy`, `sf_controller`, `sf_action`,
  `mw_rpg(_bevy)`, `mw_combat`, `mw_bake`. Two lanes may share a crate only in separate new files, and each commits
  only its own paths.
- **Shared files** (ROADMAP, DECISIONS, CHANGELOG, roadmaps, `mw_devtools_bevy` tunables catalog): stage only your
  own hunks. 2026-11-26: a whole-file commit swept another lane's line in and broke main for about 10 min.
- Meta's own doc edits ride along in a lane's next commit, named in the message to it.

## Commits and CI

- Lanes may commit and push their own paths once their crate tests and clippy pass locally (`-p`) and the CI jobs
  clippy and policy (deny/hakari) are green (`ci_push.sh <lane-slice> <paths>`).
- Until the CI test job is warm and proven on main, don't wait for it; read it afterwards and fix in a follow-up.
  Once the CI session reports a green main test job, switch lanes to waiting for it.

## Owner checks

- **Batch them.** Collect checks that are ready and launch them one after another in one sitting, not one at a time
  across the day.
- **Ask yes/no questions about circumscribed things:** "Walk up to Socucius: does his head follow you without a
  snap? yes/no". Two to five questions per scene, each naming where to stand and what to watch. No open-ended "how
  does it look".
- **Verify before launching:** the lane gives the exact command plus a log line that proves the feature is in the
  scene (e.g. `walkers: N movers` with N > 0). Meta launches with `bash bevyrrowind/tools/owner_launch.sh ...`, then
  checks `last.log` in the newest `%TEMP%\bw-owner-*` folder for that line before telling the owner what to look at.
  2026-11-26: a render-only scene with no NPCs was sent for a head-tracking check.
- Relay each verdict to the lane, which records it in its `visual-checks/<system>.md` entry.
- A bug the owner reports in passing: log it in the ROADMAP with the evidence (log path, scene, exe commit), offer
  a lane for it, don't guess its cause in the docs.

## Context and succession

- **Watch every orchestrator's context**, yours included: `bash tools/session_context.sh` prints each recent
  session's context (last usage record of its transcript). Lane prompts ask for "ctx Nk" in every message to meta.
  Check at least every hour and whenever a lane reports a commit. 2026-11-26: all four orchestrators went past 200k
  in about 2 h, unnoticed until the owner asked.
- At ~200k, tell the lane: no new slices, collect the running one, commit, hand off, archive, message meta. Then open
  its successor (`<lane>-<n+1>`, prompt in `.sessions/_prompts/<name>.md`: continue from the archived handoff, same
  communication and commit rules), so a finished session is always replaced while its lane has a queue.
- Meta's own successor: update the "Current meta:" line in `.sessions/meta.md`, open the new meta window, and the new
  meta announces its name to every lane. Lanes always address the meta named on that line.

## Meta's log and handoff

- `.sessions/meta.md`: Scope, Status, then one line per relay, commit, decision and verdict, plus the map from
  session names to lanes. A fresh coordinator continues from it.
- Hand off at ~200k context like any session: finish the log, list who is running what, archive the file.
