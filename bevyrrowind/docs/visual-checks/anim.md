# Visual checks: anim (animation lane, `docs/roadmaps/anim.md`)

Open visual checks of the animation lane (ANIM-*). Index and how-to: [`../visual-checks.md`](../visual-checks.md).

## ANIM-4: NPCs turn their heads to you (AN8 look-at + AN8b OpenMW head tracking)

Run: `owner_launch.sh view "Seyda Neen, Census and Excise Office"` (streamed with its actors, you start as the player;
Socucius at his desk, interior range 2.9 m; from the next dev build also `scene anim-look`, same setup), then
`scene wk-balmora` and press F11 to walk the player (a fixed-camera scene starts in free camera: nobody to look at;
exterior 5.7 m). Not `scene census`: it is the render scene (`--single-cell` skips actors; 2026-11-26 launch had no NPC).
Walk up to an NPC from the front. Look for: chest, neck and head turn smoothly to follow your eyes (head most, chest a
little), no snap when it starts or stops; walking past its side lets go (it releases at 90° from its facing, and the op
itself at 120° from the animated aim), no flip-flop when you stand behind it; sneaking unseen or stepping out of range
lets go; pick-up/let-go may lag up to 0.3 s (Morrowind's refresh). Close (A0) NPCs only. A/B: tunable
`mw.anim.head_track.on` (dev tunables panel). Known: beast heads aim ~5° low (head axis measured on human rigs).
Status: OK (owner 2026-11-26, exe c0df7ec9): interior (view Census and Excise Office) "yep all good"; exterior (wk-balmora + F11) "the walkers are fine".

## ANIM-1: loco2 gait picks + feet planted on the walk/jog blend (AN5b short list, AN1d)

Run 1: `owner_launch.sh clip-preview --browse "D:/Gamedev/Bevyrrowind/animation packs/generated/loco2"` (absolute
folder; C = candidates only). Short list (ledger AN5b): walk p16.cfg23.03/.01, walk back.free.00, strafe.l.s10.04/.05,
jog p45.05 + stride.1.03, jog back.s20.04, jog strafe.l.free.02; no usable sprint. Questions: (1) is one walk of
p16.cfg23.03/.01 good enough to ship? (2) one jog of p45.05/stride.1.03? (3) do the arms swing enough (MW swings wider)?
Run 2 (next dev build, GRAPH_FORMAT 16): `scene wk-balmora`, F11, walk then speed up into a jog. (4) no pelvis dip or
knee buckle while walk blends into jog? (5) feet stay put on sidesteps and backpedals (AN1d slides 2-3 cm there)?
Proof log: `walkers feet: ... unplanted` and `head track: player body present` lines in last.log. A/B: graph guard
`plant_speed` 0 (off).
Status: partial (owner 2026-11-26 sitting, run 2 on wk-balmora, dev exe 1c699df9, bake current, both proof lines
seen): (5) YES, "planted feet seem to be planted". (4) NOT JUDGED: the keyboard has no walk input, so the walk clip
and the walk->jog blend were never on screen ("the walk doesn't work because we don't have a specific button to just
walk, but it's alright"); stays open until the player can walk. In passing: "the animations (jog and sprint) are
still wonky, we'll have to find better ones (or generate them)". Run 1 (questions 1-3): NOT ANSWERED at the same
sitting: the review tool's UI was rejected (ANIM-6b); asked again once its overlay is redone.

## ANIM-9: sit/stand batch 1 candidates (AN26a)

Run: `owner_launch.sh clip-preview --browse "D:/Gamedev/Bevyrrowind/animation packs/generated/sit1"` (absolute folder),
C = candidates, G on a group (groups = `seat.<family>[.old].<piece>.<band>`; verdicts land in `sit1/verdicts.toml`). Questions:
(1) G on `seat.chair.enter.front.seat48`: does any take sit down without the hips sliding or the feet skating? yes/no
(2) G on `seat.stool.enter.left.seat64`: does any take turn in 90 degrees and sit, feet stepping (not gliding)? yes/no
(3) G on `seat.chair.loop.rest.seat48` then `.fidget`: does any loop restart without a visible pop at the seam? yes/no
(4) G on `seat.floor.exit.front.seat14`: does any take stand up from the floor believably (no float, no snap)? yes/no
(5) G on `seat.chair.old.loop.rest.seat48`: does any take read as an old person? yes/no
Proof log: `browse grades: <n> clips graded, 0 gated` (136 clips in the folder) in last.log. Keeps = the hub sources and picks for AN26b.
Status: awaiting owner (AN26a, uncommitted 2026-11-26). Skipped at the 2026-11-26 sitting: same tool, UI rejected
(ANIM-6b); waits for the overlay redo.

## ANIM-6b: side-by-side group review in clip-preview (AN10)

Run: `owner_launch.sh clip-preview --browse "D:/Gamedev/Bevyrrowind/animation packs/generated/loco2"` (absolute folder),
Down to a walk activity, then G (V = unrated first if you like; verdicts land in the same `verdicts.toml`). Questions:
(1) G on a walk group: do 2-4 takes play side by side, starting and restarting in step, a label over each? yes/no
(2) press 2 then N: does that take vanish and the rest close up, still in step? yes/no
(3) with 2 left, Y on one and M on the other: does the next group open by itself? yes/no
(4) G again: back to one clip, one actor on the mark? yes/no
Proof log: `clip-preview: group <id> <n> takes` (and `clip-preview: group row spacing ...`) in last.log.
Status: REJECTED (owner 2026-11-26 sitting, exe 1c699df9, `--browse` loco2): "woah that UI is terrible and it's hard to
understand what's happening. No more wall of text in the test scenes please. I have no idea if my changes were
validated or not...". Questions 1-4 not answered. Evidence (`%TEMP%/bw-owner-2026-11-26-1957/last.log`, 3 min): 9
groups opened in list order, zero `browse verdict:` lines (every Y / N / M / Backspace / U / Z press prints one,
refused ones too) and no `verdicts.toml` in loco2: no verdict key reached the tool, and nothing on screen said so.
Follow-up before any further owner rating: overlay redo (clip name, verdict state, one line of keys, a visible
confirmation on every verdict).


## ANIM-11: the player sits on a seat (AN26c2)
`owner_launch.sh view "Seyda Neen, Census and Excise Office"` (game location, player mode, default flags, third person). Picks are the provisional
ANIM-9 ones: judge the wiring (starts, lands, holds, leaves), not the takes. Questions:
(1) Stand in front of a chair, prompt "Sit", press E: does the character sit down onto the seat without a jump or a snap at the start? yes/no
(2) Seated: does the body rest on the seat (not floating above it, not sunk into it) in a calm loop? yes/no
(3) Press E or push the stick: does the character stand up and end beside or in front of the chair, without sliding through it? yes/no
(4) Sit again from the chair's left side: does the sit-down start from where you stand (no spin to the front first)? yes/no
Proof log: `player clips: N of M slot clips baked` in last.log, N and M both 48 higher than before (the 48 seat slots; N lower means the seat
bake is missing: `bevyrrowind bake`).
Status: FAIL (owner 2026-11-26 sitting, exe 1c699df9, bake of 19:28): "sitting doesn't work, I tried on many different
seats". Questions 1-4 not reached. Log `%TEMP%/bw-owner-2026-11-26-2003/last.log`: `player clips: 77 of 77 slot clips
baked`, 15 furniture placements indexed, 12 x `activate: Sit (furn_com_rm_chair_03 | furn_com_r_chair_01 |
furn_com_rm_bench_02, 0.9-2.1 m)`, each followed by `activation of <record>: default action`, then nothing: the use
reaches the default action and the sit never starts. The seat path after it (`sf_interaction_bevy` `activate.rs`
finish, `seat.rs` seat_requests / seat_offers / seat_events) has no log line and several silent exits (use refused,
no def or pose slot, seat use already pending, claim refused, no free entrance, `SeatRefused`): which one is not
known. Next: a headless repro in this cell + a log line per exit, then the fix (interaction crates). First or third
person at the time: asked. Replaces IN-32 question (4) "E leaves you standing".

## ANIM-12: attacks and hit reactions on Morrowind clips (AN34 + AN35)
`owner_launch.sh scene level-sparring` (synthetic greybox, muted; F11 player, F to draw). Not built yet: launch only after AN34 and AN35 are
committed. Look at the player's light attacks (LMB, each stick direction) and at the sparring dummy's swings landing on you. Questions:
(1) LMB: does the wind-up read before the strike (a visible pull-back) without looking slow? yes/no
(2) Does the strike snap (faster than the wind-up), the blade carrying a little past the hit (overshoot)? yes/no
(3) Does the recovery play out with no pop between the strike and the follow-through? yes/no
(4) Let the dummy hit you: is the reaction a short flinch that frees you quickly (~0.4 s), its side readable from the body kick? yes/no
Tune live if "no": `mw.combat.clips.rate.*`, `.accent.*`, `.react.*`. Proof log: `sparring: the player: hit N <tier> from the <side>` in last.log
(reactions fire); the attack-clip proof line is named when AN34 lands.
Status: planned (after AN34 + AN35; spec combat-adapter.md "Morrowind attack clips").

## ANIM-13: missing moves as key-pose clips (AN37)
`owner_launch.sh scene level-sparring` (synthetic greybox, muted; F11 player, F to draw). Not built yet: launch only after AN37 is committed;
keys for backstep, kick and parry are named in this entry at build time. Questions:
(1) Backstep: does it read as a quick hop back with the roll's weight? yes/no
(2) Hold guard with a shield, then with a 2H weapon: a braced guard, not a frozen frame? yes/no
(3) Kick: does it read as a kick (weight shift, foot out, back)? yes/no
(4) Parry then riposte (only if C2d is built): one deflect, then one stab? yes/no
Status: planned (after AN36 + AN37).
