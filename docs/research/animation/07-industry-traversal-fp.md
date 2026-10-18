# R7: Industry practice for traversal and first/third person

Slice R7 of the animation study (`README.md`). Web sources only, research done 2026-10-18. R1-R5 read
open-source code. This note reads what the studios themselves published, to check those code studies
against shipped games.

**How strong the evidence is.** The most detailed talks are behind the GDC Vault login (Mirror's Edge
2009, Dying Light 2018, Spider-Man 2019) or a 3.8 GB download (TLOU2 2021); only their abstracts
were read. The mechanisms come from developer interviews, Jonathan Cooper's own write-up of his AC III
talk, engine docs (UE Motion Warping, First Person Rendering, GASP) and Ubisoft's parkour post.
"Inference" marks my reading; "community" marks modder or player sources.

## 1. Traversal: how shipped games do it

### 1.1 Finding what can be climbed: markup, scanning, or both

**Dying Light (Techland): moved from markup to a live scan.**
- **The first approach failed.**
  - Designers hand-placed invisible "hooks" on ledges, walls and fences, on the rule that
    "anything that looked climbable will be climbable".
  - One map segment, about a quarter of the final size, reached **more than 50,000 hooks**, and
    every level edit broke them ([MCV](https://www.mcvuk.com/development-news/the-vaulting-dead-implementing-first-person-parkour-in-dying-light/),
    [Game Developer deep dive](https://www.gamedeveloper.com/design/game-design-deep-dive-i-dying-light-i-s-natural-movement-system)).
- **The replacement: a live scan.**
  - A programmer's prototype "analyses the environment in front of you in real-time". If the
    geometric conditions hold, "the climb animation is triggered".
  - Designers then only had to make assets meet those conditions.
  - Getting to the final system took about 1.5 years.
  - In Dying Light 2 it is still live, "no offline calculations, it simulates scanning in real
    time" ([Game Developer, DL2](https://www.gamedeveloper.com/programming/climbing-to-new-heights-with-dying-light-2-s-improved-parkour-system)).
- **What the scan cost.** Without strict control of grabbable surfaces, many animations clipped
  through objects and were redone. Moving and destructible objects needed dedicated support, and
  thousands of hand-placed "helpers" were "handled separately" (DL2). Wall runs must skip
  surfaces with climbable ledges ([PCGamesN, Kulon](https://www.pcgamesn.com/dying-light-2/parkour-interview));
  DL2 disabled side wall runs since "the city was not supporting it". Level design had to dress
  every reachable rooftop, rewrite quests, and teach enemies to climb.

**The UE5 Game Animation Sample (GASP): a scan that finds authored ledges.**
- Epic's docs give only the outline:
  - an obstacle course of "vaulting, climbing, and dropping";
  - traversal montages injected through a slot;
  - "Motion Matching can be used to select montages and entry frames for gameplay actions"
    ([Epic, GASP](https://dev.epicgames.com/documentation/en-us/unreal-engine/game-animation-sample-project-in-unreal-engine)).
- A C++ port of its Blueprint fills in the detail ([velog dev log](https://velog.io/@uji06134/11.14-TIL);
  community, so names and values are one port's):
  - **Ledges are authored.** Each traversable actor carries spline ledges paired with their
    opposite ledge.
  - **The check** is a capsule trace forward (about 75 cm ahead, radius 30, half-height 86). It
    yields the front ledge, back ledge, back floor, obstacle height and depth.
  - **A chooser** takes those measurements plus gait and speed, and picks Hurdle, Vault or Mantle.

**Assassin's Creed Shadows (Ubisoft, 2025): physical handholds only.**
- Ubisoft's own post says characters "can only climb surfaces with physical handholds".
- Designers then build parkour routes ("highways") around those limits
  ([Ubisoft parkour overview](https://www.ubisoft.com/en-us/game/assassins-creed/news/4TA6gKaTvtOC1mOjZIxCZd)).
- This reverses the climb-anything Odyssey/Valhalla era, which critics called less meaningful
  ([Outer Haven](https://www.theouterhaven.net/?p=284749)).

**Breath of the Wild** climbs any surface, limited by stamina. No developer source on its climbing
animation was reachable. An analyst counts about 8 directional cycles blended by stick direction,
plus hand and foot IK ([New Frame Plus](https://amara.org/subtitles/0IXJ0B4bKKKN/en/1/download/How%20Links%20Climbing%20Animation%20Works%20in%20Breath%20of%20the%20Wild.en.txt);
analysis).

**Inference.**
- The industry settled on **scanning geometry for affordances**, then limiting the result.
- AC Shadows limits it by authored routes and handhold geometry. GASP limits it by authored ledge
  splines on blocks. Dying Light limits it by asset rules plus hand-placed helpers.
- Pure scanning (Dying Light 1) cost the most in clip rework and level dressing.

### 1.2 Choosing the move

**GASP: chooser first, then motion matching.**
- The chooser filters by context: action type, height bands, gait.
- Motion matching then picks the montage *and its start frame* from the current pose.
- A `JustTraversed` flag selects the tail of the traversal clips when blending back to
  locomotion ([Epic, GASP](https://dev.epicgames.com/documentation/en-us/unreal-engine/game-animation-sample-project-in-unreal-engine)).
- This is the "authored backbone plus pose search" hybrid animation.md already names.

**AC III jumps: parametric blending failed.**
- The old system blended all distances parametrically. That "requires very similar kinds of
  actions in order to blend correctly", so it could not be extended.
- After "many months" of prototypes the team built a hybrid:
  - clips grouped by height and distance;
  - only similar actions blended together;
  - some jumps played alone and stretched to fit.
- Level design agreed to vary gaps so the same jump rarely played twice in a row
  ([Cooper, Animating the 3rd Assassin p.4](https://www.gameanim.com/?p=5864&page=4)).

**Uncharted 4: hundreds of climb clips.**
- A reach mechanic points Drake's hand at the next hold; the combinations "required us to have
  hundreds of new climbing animations", more memory than his whole Uncharted 3 move set. Hold size
  changes the clip (big: fast; small: slow and shaky); full-body IK and physics smooth the rest
  ([Game Informer, Yates](https://gameinformer.com/b/features/archive/2015/01/23/how-uncharted-4-is-taking-game-technology-to-the-next-level)).
- Naughty Dog's NPC traversal had to **choose the next action before the current one finished**,
  in order to play it ([ND retrospective](https://www.naughtydog.com/blog/uncharted_4_a_thiefs_end_the_5_year_anniversary_retrospective)).
- Inference: a chooser for continuous parkour needs lookahead, not only the current probe.

**The Last of Us Part II: motion matching beyond locomotion.**
- Michal Mach's reel says it drove locomotion and also traversal, melee and cover
  ([reel](https://vimeo.com/433024043)).
- The GDC abstract promises "initial joy, later frustration and ultimate fear", and says "not
  everything went according to plan"
  ([ND at GDC 2021](https://www.naughtydog.com/blog/naughty_dog_at_gdc_2021)). The slides, not
  read here, are the primary source on what went wrong.

### 1.3 Aligning the clip to the world

**Motion warping (UE).**
Windows in the montage name a target; *skew warp* bends the remaining root motion so the window ends
on it, with translation, rotation and Z switchable, `Default`/`Facing` rotation, a rotation time
multiplier and a bone as warp point ([Epic, Motion Warping](https://dev.epicgames.com/documentation/en-us/unreal-engine/motion-warping-in-unreal-engine)).
For a mantle the window starts at hand contact. GASP warps to three targets, front ledge, back ledge
(hurdle and vault only) and back floor (velog port).

**IK and physics on top (AC III**, Cooper [p.4](https://www.gameanim.com/?p=5864&page=4),
[p.5](https://www.gameanim.com/?p=5864&page=5)): IK adjusts tree poses "to cater for any possible angle
of trunk or branch"; procedural foot placement anticipates turns; physics follow-through lets momentum
play out without stopping, which removed the follow-through clips (ragdoll was rejected as "silly");
45° climb clips cover diagonals.

**Elsewhere.** AC II credited Autodesk HumanIK at runtime for climbing ([MCV](https://mcvuk.com/?p=132628), reported).
Overgrowth (Rosen, GDC 2014, free video) builds animation from "very few key frames" and asks "what
exactly is the difference between a playable character and a vehicle?" ([GDC Vault](https://gdcvault.com/play/1020583/Animation-Bootcamp-An-Indie-Approach));
only the abstract was read.

### 1.4 What made it feel good, and what failed

**Dying Light.**
- Animations locked the player in until they finished, and chained moves felt like a "traffic
  jam". The team **shortened them**.
- **Releasing the camera during moves**, instead of centring and locking it, made a noticeable
  difference to how much control players felt.
- Testers pressed the parkour button too early, because the ledge was out of view at the critical
  moment. The team **deliberately delayed the button response** to a tuned "sweet spot" (all
  [MCV](https://www.mcvuk.com/development-news/the-vaulting-dead-implementing-first-person-parkour-in-dying-light/)
  and the [Game Developer deep dive](https://www.gamedeveloper.com/design/game-design-deep-dive-i-dying-light-i-s-natural-movement-system)).
- The team considered switching to third person and kept first person: "Getting first-person
  parkour to work saved Dying Light."

**Mirror's Edge**: momentum "is like the fuel"; the jump is "context sensitive"; "runner vision"
tints usable objects red ([Game Developer, Channon](https://www.gamedeveloper.com/design/the-philosophy-of-faith-a-mirror-s-edge-interview)).
**AC Unity** merged climbing and free running "so it really feels like parkour", with explicit up
and down ([GamingBolt](https://gamingbolt.com/assassins-creed-unity-parkour-system-combines-climbing-and-free-running),
[DualShockers](https://www.dualshockers.com/learn-how-the-movement-has-been-improved-in-assassins-creed-unity/)).

**AC Shadows** ([Ubisoft](https://www.ubisoft.com/en-us/game/assassins-creed/news/4TA6gKaTvtOC1mOjZIxCZd)):
- Holding parkour-up reaches for "the highest or furthest point". Holding parkour-down goes to "the
  lowest safe point", avoiding leaps into the void.
- Parkour-up is **loud**, so climbing, jumping and landing can get a shinobi spotted. Parkour-down
  landings are quiet.
- Stance change was **disconnected** from parkour-down, so players change stance "without
  accidentally dropping off a roof".
- Small obstacles get automatic passovers while sprinting.
- A drop that would hurt makes the character **hang and wait** for input.

**Spider-Man** runs movement on "custom hero states", with physics mainly for collision tests
([Game Developer](https://gamedeveloper.com/design/making-insomniac-s-i-spider-man-i-do-what-a-spider-can)).
Bumping a building does not kill momentum ([NME analysis](https://www.nme.com/features/how-the-web-swinging-of-marvels-spider-man-makes-its-superhero-fantasy-soar-3287136)).

## 2. First and third person

### 2.1 Body strategies

| Strategy | Who | Mechanism and cost |
|---|---|---|
| **True first person, camera authored in the clip** | Mirror's Edge, Catalyst | ME1: a camera on the mocapped head "didn't work out", so animators hand-made the camera ([Game Anim](https://www.gameanim.com/?p=2058)). Catalyst: "The camera is attached to the actor's head and we have to make sure that all moves read well from that angle". Also: "You can't just translate third-person into first and expect it to work" ([EA, Odeldahl](https://www.ea.com/en-au/amp/news/the-world-from-faiths-perspective)) |
| **True first person, camera on the head bone, stabilised** | KCD; generic UE recipe | The body animation drives the camera, look comes from about 10 additive aim poses, and a head constraint aimed at a distant point steadies walk and run. Pitfalls: a frame of lag from tick order, and full-body montages that double the head rotation ([Game Developer, UE4 true FP](https://gamedeveloper.com/programming/true-first-person-camera-in-unreal-engine-4)). KCD players report head bob with no off switch ([Steam](https://steamcommunity.com/app/1771300/discussions/0/601895662819280659); community) |
| **Separate first-person arms rig** | Skyrim, Morrowind, Chivalry | Skyrim's first-person skeleton has its own camera bone, the third-person skeleton has none ([Nexus forum](https://forums.nexusmods.com/topic/722270-reqz-1st-person-weapon-anims); community). Chivalry needs "a separate first person and third person skeletal mesh", each skinned to its own skeleton ([Torn Banner wiki](https://tornbannerjira.atlassian.net/wiki/spaces/CHIVCOM/pages/5799942/Animation)) |
| **Hybrid, engine-supported** | UE 5.5+ First Person Rendering | First-person primitives get their own FOV and are **scaled toward the camera** so they cannot intersect the world. They cast no scene shadows. A full-body **World Space Representation**, invisible to its owner, casts the shadow and shows in reflections. The "advanced" setup adds first-person legs, lines their feet up with the world body, and uses a material gradient so the feet stay in world space ([Epic](https://dev.epicgames.com/documentation/en-us/unreal-engine/first-person-rendering)) |
| **First person with third person in places** | Cyberpunk 2077 | "100% first-person", except the inventory, driving, mirrors and some cutscenes ([VGC](https://www.videogameschronicle.com/news/cyberpunk-will-be-100-first-person-apart-from-some-cut-scenes)). Its sequel's director says interactive, uncut first-person scenes are "3x as difficult" and as expensive as old-school cutscenes ([FRVR](https://frvr.com/blog/news/cyberpunk-2-director-pushes-back-on-third-person-cutscenes-as-it-would-be-a-completely-different-game/)) |

**Third-person readability is a separate problem.** Chivalry 2 rebuilt its player animation because
"what might have looked good for the player typically wound up looking janky for the opponents"
([Wccftech preview](https://wccftech.com/chivalry-ii-preview-dumb-fun/amp/)).
- Inference: animation built for the first-person camera does not automatically read in third
  person, and the reverse is just as true.

### 2.2 Comfort

**Mirror's Edge** treated simulator sickness as a risk from the start
([Engadget](https://www.engadget.com/2008/07/17/how-mirrors-edge-fights-simulation-sickness/)):
- **A centre dot**, inspired by how dancers spot during spins, and it can be switched off.
- **Head bob removed.**
- **The camera at the eyes**, "not your head".
- A reticle "partly attached to the camera controlled by a focal point in the distance"
  ([search summary of a 2008 interview](https://www.indianvideogamer.com/features/the-most-comprehensive-mirrors-edge-interview/333);
  the page's certificate had expired, so it was not verified).

**Dying Light** tuned motion blur, camera movement, reticle, FOV, filters and the speed of parkour
clips, and commissioned an external study because most of its developers did not get sick
([MCV](https://www.mcvuk.com/development-news/the-vaulting-dead-implementing-first-person-parkour-in-dying-light/)).
DL2 added slow-motion moments and longer airtime, with a run of about 7 m/s and jumps a little
higher than real ([TechRaptor](https://techraptor.net/gaming/news/techland-explains-how-dying-light-2-parkour-works),
via search summary), and later a choice of **assisted or manual parkour** ([KitGuru](https://www.kitguru.net/page/251/!https:/www.kitguru.net/tech-news/mustafa-mahmoud/dying-light-2-update-improves-parkour-and-overhauls-the-night-experience/)).

**Clipping**: two industry answers.
- Depth-cleared arms drawn by a second camera (Skyrim era; R2).
- Primitives scaled toward the camera (UE 5.5). This costs the first-person mesh its scene shadows
  and ray-traced reflections, hence the world-space duplicate.

### 2.3 First-person melee and climbing

- **Melee.**
  - KCD mocapped historical fencers, then mapped them onto a 5-direction strike and guard system
    driven by mouse look ([Czech Radio](https://english.radio.cz/node/8839276)). A critic argues
    the inputs cannot carry the captured finesse ([critpoints](https://critpoints.net/2016/03/17/kingdom-come-deliverance-combat-review/)).
  - KCD players report that lock-on keeps targeting defeated enemies (community).
  - Mordhau caps turn speed during a swing; the source is player threads, not the developer
    ([Steam](https://steamcommunity.com/app/629760/discussions/0/1645418261863641315/?ctp=4)).
  - Inference: first-person melee always pairs a directional input with *some* aim constraint
    (lock-on, a turn cap, or a slowed look), because the camera *is* the weapon's aim.
- **Climbing and mantling.**
  - Dying Light shows hands and sometimes feet, else "your character was a floating camera"
    ([MCV](https://www.mcvuk.com/development-news/the-vaulting-dead-implementing-first-person-parkour-in-dying-light/)).
  - Catalyst: "You need to see what is happening", especially for skill rolls after long jumps
    ([EA](https://www.ea.com/en-au/amp/news/the-world-from-faiths-perspective)).
  - Look limits per move, eased in, are covered in R2 from faith-runner.

## 3. The R11a conflicts: industry evidence

**Who owns motion.**
- *Root motion warped onto a target*: GASP, UE motion warping, and (from its windows) AC. It is
  exact at contact, but needs root motion in the clip and windows authored per clip. GASP also
  documents release-plus-warp jitter as a known issue.
- *The sim drives and the animation dresses it*: Dying Light's "make the player stick to it, and
  play a good animation"; Spider-Man's "custom hero states"; Overgrowth's "character vs vehicle".
  It is cheap and responsive, but clips slide and clip unless IK and physics clean up (AC III,
  Uncharted 4).
- *Industry reading*: **per move kind**. Locomotion and swinging are sim-owned. Contact moves
  (mantle, vault, ledge, sit, door) are warped root motion.

**Commitment vs responsiveness.**
- Dying Light learnt that locked moves pile up into a "traffic jam" and shortened them.
- AC III's goal was "more responsive, without sacrificing any of the animation quality", reached
  through physics follow-through rather than extra recovery clips ([p.1](https://www.gameanim.com/?p=5864)).
- AC Shadows makes the *risk* explicit instead: a loud parkour-up, and hang-and-wait on a
  dangerous drop.
- *Reading*: traversal is short and cancellable at its tail; commitment belongs to combat (R6/R8).

**Context moves vs explicit buttons.**
- Context: Mirror's Edge's context-sensitive jump; AC Shadows' sprint passovers; GASP's jump
  that becomes a vault.
- Explicit: Unity added parkour-down; Shadows split *up* (loud) from *down* (quiet, safe), and
  **separated stance from parkour-down** so sneaking never drops you off a roof.
- Dying Light *delays* the button on purpose.
- *Reading*: two buttons with intent (up, down) plus small automatic passovers only while
  sprinting. Stealth input stays apart from traversal input.

**Blending.**
- GASP motion-matches the montage's **entry frame** and tail, a time search rather than a
  crossfade.
- AC III blends only like with like; unlike jumps play alone and are stretched.
- There is no published evidence that instant switching is preferred outside fighting games.
- *Reading*: keep inertialization and add an **entry-time search** (pose match) for traversal
  clips.

**First person.**
- Every shipped full-body first-person game authors its camera *for* first person: Mirror's Edge
  by hand in ME1, through mocap checked from the head in Catalyst.
- A raw head camera fails without stabilisation.
- Games with both views either keep two rigs (Skyrim, Chivalry) or are first-person-only with
  third person in narrow cases (Cyberpunk: vehicles, mirrors).
- UE 5.5 now ships the hybrid: first-person arms and legs plus a world-space full body for shadows.
- *Reading*: one gameplay body, two **render** bodies. Our R2 "virtual camera baked per clip" is
  close to Mirror's Edge's hand-made camera, done automatically. Nobody has published that it works
  on third-person mocap; Odeldahl's warning says it needs per-move review.

**Camera.**
- Dying Light *released* the camera during moves.
- Mirror's Edge keeps the reticle on a far focal point.
- AC III needed situation-specific rules for its over-the-shoulder camera and added a framing
  combat camera ([p.6](https://www.gameanim.com/?p=5864&page=6)).
- *Reading*: free look by default; limits only where the body can't follow (R2's per-move limits).

**What clips must carry.**
- UE warping needs root motion plus a window per clip.
- GASP needs a montage per traversal kind, with chooser metadata (height and depth ranges).
- Uncharted 4 needed hundreds of hold-specific clips.
- Dying Light redid many clips that clipped once grabbing went geometry-driven.
- *Reading*: the minimum is root motion, a contact or warp window, and the obstacle range a clip
  was made for. A bake can infer contacts. The obstacle range must come from the source manifest
  or a calibration pass.

**Pace and scale.**
- Dying Light 2 chose a realistic run (about 7 m/s) with a slightly exaggerated jump and stretched
  airtime for comfort.
- Mirror's Edge sells speed through momentum.
- AC III tuned climbing speed against real climbers' footage.
- *Reading*: one physical scale, with per-mode multipliers (jump height, airtime), not separate
  worlds.

## 4. The owner's situations

| Situation | Industry practice | First person |
|---|---|---|
| Sneak behind a guard | Stance input separate from traversal (AC Shadows); parkour noise as a stealth cost (Shadows, DL2) | Crouch lowers the eye; legs visible looking down (UE advanced setup) |
| Roof-to-roof jump | Shadows' parkour-up to "furthest point"; AC III's height/distance clip groups with stretch; DL2 run about 7 m/s and longer airtime | Landing dip and comfort (R2); no head bob (Mirror's Edge) |
| Ledge grab / mantle / climb | Scan, classify by height and depth, choose, warp to front ledge, back ledge and back floor (GASP); hold-size clips (Uncharted 4); IK on top (AC III) | Hands visible on the lip (DL, Catalyst); delayed grab input (DL) |
| Sword fight | Out of scope here (R6, R8) | Directional input plus an aim constraint (KCD, Mordhau); a framing camera in third person (AC III) |
| Sit at a bar | Warp to a seat target (UE warping, Smart Object slots, animation.md layer 3) | The camera follows the seated head with stabilisation; there is no published first-person sitting practice |
| Door or lever | Warp to the interaction point | Arms layer |
| First and third person | Two render bodies on one sim (UE 5.5; Skyrim's arms) | — |

## 5. For us

These map the findings to `animation.md`, `locomotion.md` and `character.md`.

1. **Adopt: chooser with ranges, then pose-matched entry.**
   - `ContextProbes` already return height, depth and clearance (locomotion.md "Context probes as
     built"). That is GASP's check.
   - Missing pieces:
     - (a) a **chooser asset** mapping affordance × height/depth band × gait to clips;
     - (b) a **start-frame search** for the chosen clip against the current pose (a narrow pose
       search, before the full motion-matching node of animation.md "Later");
     - (c) the GASP tail rule for blending back to locomotion.
   - Animation.md's roadmap names choosers only in passing; this belongs in R11b.
2. **Adopt: three named warp targets for traversal** (front ledge, back ledge, back floor), on
   top of animation.md layer 1's warp windows. `ContextHit` has `warp_target` and `hang_point`
   only. Add the back ledge and the landing point for vault and hurdle.
3. **Adapt: markup as a filter, not the source.**
   - The probes scan geometry (Dying Light's lesson).
   - Add optional `sf_interaction` tags: "no parkour" and "route", in the spirit of AC's
     highways. Content can then block or favour surfaces without 50,000 hooks.
   - Moving and destructible objects need explicit support (DL2). Leave them out of M4.
4. **Adopt: input split.**
   - Explicit up and down intents. Automatic passover only while sprinting.
   - Stance input never triggers a drop.
   - A short tuned delay before a ledge grab in first person (DL).
   - Parkour noise events go to `sf_ai` perception.
   - This is a locomotion.md and action-layer item.
5. **Adapt, first person: two render bodies, one simulation.**
   - Character.md's plan (view-model arms plus a shadow body) matches Skyrim and UE's *basic*
     setup.
   - Add UE's *advanced* mode for traversal: first-person legs visible, feet aligned with the
     world body.
   - Prefer scale-toward-camera to depth clearing where Bevy allows. It keeps first-person
     geometry in the main pass, so it can be lit and shadowed. Depth clearing needs a second
     camera.
   - Keep R2's baked virtual camera per clip, but **review it per move** (Odeldahl), and stabilise
     the head as the UE true-FP recipe does (a far-point aim).
   - Character.md's "stabilised head socket is enough" is too thin. ME1 is direct evidence that
     a raw mocap head camera fails.
6. **Adopt: comfort as settings.**
   - A centre dot.
   - No procedural head bob by default.
   - First-person FOV.
   - Assisted or manual parkour (DL2).
   - Owner: the camera spec (missing; R2 and R5).
7. **Reject:**
   - **pure parametric blending across unlike jumps** (AC III's failure). Blend spaces only within
     a clip family;
   - **climb-anything with no limit** (AC's own reversal in Shadows).
8. **Open for R11a:** whether to have hold-level climbing (Uncharted 4's hundreds of clips) or
   ledge-only traversal. Our clip sources (UAL, Mixamo) suggest ledge-only plus IK.
