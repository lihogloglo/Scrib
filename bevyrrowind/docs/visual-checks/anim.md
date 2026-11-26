# Visual checks: anim (animation lane, `docs/roadmaps/anim.md`)

Open visual checks of the animation lane (ANIM-*). Index and how-to: [`../visual-checks.md`](../visual-checks.md).

## ANIM-4: NPCs turn their heads to you (AN8 look-at + AN8b OpenMW head tracking)

Run: `scene census` (Socucius at his desk, interior range 2.9 m), then `scene wk-balmora` (walkers, exterior 5.7 m).
Walk up to an NPC from the front. Look for: chest, neck and head turn smoothly to follow your eyes (head most, chest a
little), no snap when it starts or stops; walking past its side lets go (it releases at 90° from its facing, and the op
itself at 120° from the animated aim), no flip-flop when you stand behind it; sneaking unseen or stepping out of range
lets go; pick-up/let-go may lag up to 0.3 s (Morrowind's refresh). Close (A0) NPCs only. A/B: tunable
`mw.anim.head_track.on` (dev tunables panel). Known: beast heads aim ~5° low (head axis measured on human rigs).
Status: awaiting owner (build 2026-11-26).
