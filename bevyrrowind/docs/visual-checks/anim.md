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
`plant_speed` 0 (off). Status: awaiting owner (commit of AN1d, 2026-11-26).

## ANIM-9: sit/stand batch 1 candidates (AN26a)

Run: `owner_launch.sh clip-preview --browse "D:/Gamedev/Bevyrrowind/animation packs/generated/sit1"` (absolute folder),
C = candidates, G on a group (groups = `seat.<family>[.old].<piece>.<band>`; verdicts land in `sit1/verdicts.toml`). Questions:
(1) G on `seat.chair.enter.front.seat48`: does any take sit down without the hips sliding or the feet skating? yes/no
(2) G on `seat.stool.enter.left.seat64`: does any take turn in 90 degrees and sit, feet stepping (not gliding)? yes/no
(3) G on `seat.chair.loop.rest.seat48` then `.fidget`: does any loop restart without a visible pop at the seam? yes/no
(4) G on `seat.floor.exit.front.seat14`: does any take stand up from the floor believably (no float, no snap)? yes/no
(5) G on `seat.chair.old.loop.rest.seat48`: does any take read as an old person? yes/no
Proof log: `browse grades: <n> clips graded, 0 gated` (136 clips in the folder) in last.log. Keeps = the hub sources and picks for AN26b.
Status: awaiting owner (AN26a, uncommitted 2026-11-26).

## ANIM-6b: side-by-side group review in clip-preview (AN10)

Run: `owner_launch.sh clip-preview --browse "D:/Gamedev/Bevyrrowind/animation packs/generated/loco2"` (absolute folder),
Down to a walk activity, then G (V = unrated first if you like; verdicts land in the same `verdicts.toml`). Questions:
(1) G on a walk group: do 2-4 takes play side by side, starting and restarting in step, a label over each? yes/no
(2) press 2 then N: does that take vanish and the rest close up, still in step? yes/no
(3) with 2 left, Y on one and M on the other: does the next group open by itself? yes/no
(4) G again: back to one clip, one actor on the mark? yes/no
Proof log: `clip-preview: group <id> <n> takes` (and `clip-preview: group row spacing ...`) in last.log.
Status: awaiting owner (AN10, uncommitted 2026-11-26).

