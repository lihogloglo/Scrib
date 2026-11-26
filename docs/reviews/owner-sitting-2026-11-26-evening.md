# Owner sitting, 2026-11-26 late evening (dev exe at 59f25af7, bake at e569fde0 + launch-time rebakes)

Verdicts and remarks in the owner's words, by lane. The index `bevyrrowind/docs/visual-checks.md` carries the statuses;
the entry of each check is updated by the next session of its lane. Logs: `.sessions/out/sitting2-*.log` (local, untracked).

## Worldgen / water

- **WA-16 player swims: PASS** (Q1 yes, Q2 yes, Q3 yes).
  Owner: "the swimming doesn't trigger any swimming animation though." -> new bug (animation / player lane): the
  swim state plays no swim clip.
  Found in the log: `swim: body ... stops swimming` is written every frame while the body is not swimming (2273
  lines in one run, 5 `starts swimming`): `mw_environment_bevy::swim` should log on the change only.
- **WA-13 river joins (scene river-join-ab, FL-4b / FL-4c): FAIL.** Owner: "the river is still doing a hard
  cutoff. in one instance the transition felt a bit smoother but honestly, not by much. I spotted z fighting. Also,
  the river is always the same width (a ribbon, basically). Also it more often than not is floating above ground
  (sometimes, dozens of meters above). Also, it's not correctly following the terrain. It should have a groove. It
  should not be homogenous. Also still no flowmap ?. Meh."
  Reading: this is a verdict on generated rivers as a whole, not on the join fade. Six defects: (a) the fade does
  not read, (b) z-fighting, (c) constant width, (d) floats above the ground, up to dozens of metres, (e) does not
  follow the terrain, no carved bed ("should have a groove": the parked river bed carve is wanted), (f) uniform
  surface, no flow map. Next step is research + a spec for rivers (industry standard first), not more join work.

## Big world / rendering

- **FF-7 far impostor depth (scene impostor-depth-ab): PASS.** Owner: "there is absolutely no difference between
  A and B, no holes etc". `far.impostor.depth_equal = on` stays the default; off is the A/B arm.

## Animation (clip browser on loco2, launch 4)

- Owner, while rating (the tool questions ANIM-6b / ANIM-6c were not answered one by one; the owner is using the
  browser to rate "all the anims", which takes a while):
  "A LOT of the animations don't go straight (massive bias on one side so they end up running obliquely). This is
  the case for the current sprint anim btw. We should do something for that, automatically."
  "a lot of pollution from the prompts. Some of the anims are ALL bad for a given prompt (they have their hands in
  front of their face, etc)."
- Context from the ledger (`docs/ledger/animation.md`, WALK-1, 2026-11-12): the heading offset was measured only on
  the three picked loco1 takes (walk under 1 deg, jog 2.8, sprint 4.5 deg sweep) and the clip was ruled out as the
  cause of "walk goes diagonal". Nothing measures the travel direction of every take at grading, and the cook does
  not straighten it.
- Owner request, two automatic steps for the animation lane (to scope, industry standard first):
  1. Straightness: measure each take's travel direction against its facing (and against the direction its
     activity names: forward, back, strafe) at grading; show and gate on it in the browser; straighten at cook
     (rotate the root path onto the nominal direction, spread the leftover sideways drift over the loop).
  2. Prompt pollution: a pose gate at generation (for locomotion: hands near the head, and the like), counted per
     prompt; a prompt whose takes all fail is reported and regenerated with another wording instead of reaching
     the owner.
- **ANIM-6c mirror preview (X): FAIL on the look.** Owner, on a `strafe.l` group: "it's terrible, twisted limbs
  etc". The key works (4 `browse mirror: ... on` lines); the mirrored pose is wrong. For the animation lane: check
  the mirror maths of AN46d / the retarget of a mirrored take before any slot uses `#mirror`.
- **ANIM-1 run 1 ratings were made** in this launch: 142 `browse verdict:` lines in the log;
  `animation packs/generated/loco2/verdicts.toml` at 23:12 holds 40 keep, 1 maybe, 92 reject (133 of 246 takes).
  The owner asked how many takes there are per style and felt there were too many ("I feel like I've seen 30 walk
  anim at least"): loco2 = 246 takes in 41 variants of 6 takes: walk 108 (18 variants, 8 of them plain forward
  cycles = 48 takes), jog 60 (10), run 42 (7), sprint 24 (4), turn 12 (2). For the lane: smaller batches per
  variant reach the owner (memory "owner rates by elimination": small batches), the gate removes the rest first.
- ANIM-6b overlay questions: not answered one by one. The owner rated 133 takes with it without complaint about
  the overlay; ask the three overlay questions again only if the next session needs the explicit yes.

## AI / gameplay (Seyda Neen, noon, with sound, launch 5; log copy .sessions/out/sitting2-seydaneen-last.log)

Owner: "crouch is good. I've seen no double. wanderer do stop and greet. They do a weird little trajectory though
like a semi circle to come back talk to me. Unnatural.."
- **PL-11 standing crouch (2.1f): PASS** ("crouch is good"; log: `player: sneak on`).
- **AI-13 greeting re-ask: PASS on Q1** (wanderers stop and greet; log: `greeting: 4 greeted (0 without a voice)`,
  3 greeting rounds). **New remark, a look defect:** before greeting, the wanderer walks "a weird little
  trajectory ... like a semi circle to come back talk to me. Unnatural." For the AI lane: the greeter should turn
  in place toward the player (or stop where it is), not walk an arc back. Q2-Q4 (no second greeting close by,
  greets again after 8 m, turn from behind) were not answered one by one.
- **AI-16 NPC across a cell border: PASS on Q3** ("I've seen no double"; log: `actors: 1 re-homed`). Q1-Q2
  (still there, walks smoothly while the village unloads) not answered one by one.
- **AI-15** (pose kept after unload): not answered; log shows `actors: 1 poses kept at unload`.

## Interaction (Seyda Neen, Census and Excise Office, launch 6; log copy .sessions/out/sitting2-sit-last.log)

- **ANIM-11 re-ask, the player sits: PARTIAL.** Owner: "yes sitting works, but not for all the chair. It worked for
  the bench, not for the stool. It didn't work for the big chair because the character needed to actually pull the
  object to sit on it."
  - Bench: works (log: 6 `seat: player entered` + 6 `exited`, all on placement 33c1a668... slot 0).
  - Stool: fails. Big chair: fails, it stands pushed in at a table, so there is no room to walk to its front; the
    owner expects the character to pull the chair out.
  - Log: the 7 failed presses all end `seat: use dropped: enter refused by the action layer
    (interact.sit.approach: ApproachCancelled)`, 4 on placement f81..., 3 on 112... (which of the two is the stool
    is not read from the log). The headless repro (IN2c6, 21 of 21) did not cover these two cases as the game runs
    them: its stances start on free floor.
  - For the interaction lane: (a) why the stool cancels the approach (diagnose first, from this log); (b) a seat
    blocked by a table: the industry answer is the chair slides out as part of the sit (seat furniture moved by
    the enter clip, back in on exit), a design slice, not a tolerance tweak. Q2 (rests on the seat) and Q3 (stands
    up cleanly) were not answered one by one.
