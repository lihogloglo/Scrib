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
