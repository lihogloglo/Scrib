# First person

## The standard technique
- **Body.** Three live strategies. (1) True first person: the camera rides a full body's head (Cyberpunk 2077, Dying
  Light, Kingdom Come, Chivalry, Mordhau). (2) A separate arms rig and clip set (Dishonored, Skyrim, Overwatch,
  Chivalry's separate meshes). (3) UE 5.5+ First Person Rendering: first-person primitives get their own FOV and are
  scaled toward the camera so they cannot touch walls, in one pass; a world-space body invisible to its owner casts
  shadows and shows in reflections ([Epic](https://dev.epicgames.com/documentation/en-us/unreal-engine/first-person-rendering)).
- **Cyberpunk 2077** is "100% first-person" except inventory, driving, mirrors and some cutscenes
  ([VGC](https://www.videogameschronicle.com/news/cyberpunk-will-be-100-first-person-apart-from-some-cut-scenes)); its
  sequel's director says interactive first-person scenes cost about three times a classic cutscene
  ([FRVR](https://frvr.com/blog/news/cyberpunk-2-director-pushes-back-on-third-person-cutscenes-as-it-would-be-a-completely-different-game/)).
- **View model FOV.** Separate FOV for the arms plus a depth range so they do not clip (UE 5.5 per-primitive FOV;
  two-camera layering in the Bevy example). Footnote: OpenMW swaps projection and clears depth per subgraph.
- **Camera motion, two schools.** Stable: Source and Doom keep the camera calm and add small view punch and bob.
  Body-driven: Mirror's Edge and Dishonored let the animation drive the camera (Mirror's Edge hand-authored the
  camera per clip, removed head bob and added a centre dot against sickness,
  [Game Anim](https://www.gameanim.com/?p=2058), [Engadget](https://www.engadget.com/2008/07/17/how-mirrors-edge-fights-simulation-sickness/)).
  The true-first-person recipe is a stabilised head camera: position from the head, rotation from the view, a head
  constraint aimed at a far point
  ([Game Developer](https://gamedeveloper.com/programming/true-first-person-camera-in-unreal-engine-4)).
- **Melee.** Dark Messiah: full-body awareness, view punch, a one-button kick, pervasive physics. Chivalry and
  Mordhau: blade sampled along sockets each tick; view turn "drags" swings; Mordhau caps turn speed during a swing.
  KCD locks on with a five-point star. Pairing a direction input with an aim constraint is the norm
  (`superfargoth/docs/research/animation/07*.md` s2.3, `08*.md` s2).
- **Comfort options** are expected: bob, shake, FOV, auto-camera off (Dying Light, Mirror's Edge).

## Who ships it
Dying Light, Cyberpunk 2077, KCD (true first person); Skyrim, Dishonored (arms rig); Mirror's Edge (camera authored in
the clip); UE 5.5 (engine support). RDR2 can be played start to finish in first person except cutscenes ([GamesRadar, 2018](https://gamesradar.com/red-dead-redemption-2-has-a-full-first-person-mode-just-like-gta-5)); whether it uses a full body is not stated in sources found.
Unity: no built-in equivalent found, so no claim.

## Key numbers
- Dying Light 2 run about 7 m/s with jumps a little higher than real (TechRaptor, via search summary).
- UE's advanced setup adds first-person legs lined up with the world body by a per-vertex alpha gradient.
- A Dark Messiah style system needs the upper body to follow 60-100 percent of view pitch (our inference, research
  note 14, s2).

## Where Scrib stands
Hybrid design, like UE 5.5: one gameplay body, two render bodies. The head goes to a layer only lights and the traced
scene see, the arms are redrawn in a viewmodel pass with their own FOV and a near depth slice in one pass (built, FP-2b
awaiting owner), and the world body supplies shadows. Open: neck line when looking down, per-move camera amount,
kick and view punch, turn cap during swing. See `superfargoth/docs/specs/first-person.md` and
`superfargoth/docs/research/animation/15-audit-and-plan-2026-11-12.md` s4.

## Sources
- `superfargoth/docs/specs/first-person.md` "Industry comparison"; animation research 07, 08, 11 s5-6, 14 s2, 15 s4.
- Web research done 2026-11-12 for the RDR2 and Unity rows only; other rows keep their links above.
