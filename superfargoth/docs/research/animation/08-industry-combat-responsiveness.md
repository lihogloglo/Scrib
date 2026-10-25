# R8: responsiveness vs weight, first-person melee, combat in both views (industry)

Status: done (r8, 2026-10-18).

Study note, slice R8 (README.md). Research only, from published sources: talks, dev blogs, official
patch notes, engine docs, and (marked) mod documentation and press. R6 (`06-souls-combat-feel.md`)
already covers souls windows, buffering, hit-stop and paired kills from code, and R2 covers the
faith-runner first-person body; this note does not repeat them. R7 covers traversal.
Inference is marked *(inference)*. Where only a summary of a talk was reachable, that is said.

## 1. Responsiveness vs weight: how it is measured and tuned

### 1.1 Input latency budgets

- **Measured, not felt.** Mick West filmed controller and screen and counted frames from press to
  first visible response ([Measuring Responsiveness, 2008](https://www.gamedeveloper.com/design/measuring-responsiveness-in-video-games)).
  - Best measured 60 fps games: **~50-67 ms** (3-4 frames at 60 Hz). 30 fps games: **~100-167 ms**;
    worst cases up to ~300 ms. His bar: a 60 fps game should hit ~3/60 s and anything over ~4/60 is a
    problem; ~200 ms "is too long to wait for a gun to fire". TVs add ~2 frames on top.
  - **Animation is a named cause**: one game started attacks in 7 frames but turns in 18 ("I would
    consider that a bug... all moves should start in 7").
  - Skate felt responsive at 10 frames because motion *began before the gesture finished*.
- **Where animation adds frames** ([Programming Responsiveness](https://www.gamedeveloper.com/programming/programming-responsiveness)):
  anim events read one loop late; a new clip not advancing until the next frame. Fix: the velocity
  impulse **on the first frame** of the clip; a late impulse "might look better, but it feels bad".
- *(inference)* The budget splits in two: **first visible response** (pose change, camera kick, sound)
  within ~1-3 sim ticks at 60 Hz; the **effect** (hit, displacement) may come much later if the wind-up
  is visibly under way. That is how a 500-1,000 ms heavy can still feel responsive.

### 1.2 Who moves the body: the responsiveness/weight dial

Daniel Holden's [Code vs Data Driven Displacement](https://theorangeduck.com/page/code-vs-data-driven-displacement)
(Ubisoft La Forge alumnus) is the clearest statement of the locomotion trade-off. A **simulation object**
(capsule, moved by code from input, typically through a critically damped spring) and the **character**
(moved by its animation's root) must be reconciled:
- *Character snapped to simulation*: precise, but floaty, foot sliding, "sudden spinning"; designers then
  make the sim *less* responsive to hide it.
- *Simulation follows the character* (pure root motion): weight, no sliding, but "sluggish and slow to
  respond".
- Blends of the two trade one constraint for the other. His preferred tools: **damped adjustment** of the
  character toward the sim (halflife-based, rotation slower than position); **velocity-limited
  adjustment** (correct at most ~0.5× the character's own speed, so feet don't skate at a plant or a
  start/stop); **clamping** to a max distance/angle (predictable for networking and collision, but feels
  "dragged" at the bound); foot locking only as last resort.
- **Position and rotation responsiveness are separate dials**; setting the desired-rotation response too
  high is "a common mistake seen in many games" (spinning on the spot). Procedural look-at raises
  *perceived* responsiveness without moving the body.
- His closing recommendation: **label regions of the animation data with the displacement behaviour
  designers want**, rather than one global mode.

Unreal takes the code-driven side: the capsule moves by the movement component, and **distance
matching** picks the clip frame from distance to the stop, start, pivot or ground instead of time
([UE docs](https://dev.epicgames.com/documentation/en-us/unreal-engine/distance-matching-in-unreal-engine));
a pivot is a stop whose speed never quite reaches zero. Avalanche framed Just Cause 3's version as
"highly responsive movement, versus animation that requires time to convey energy, momentum and
intention", answered by "correcting and manipulating motion" ([GDC 2015 abstract](https://gdcvault.com/play/1021981/Finding-Balance-Realizing-Responsive-High);
slides not public).

### 1.3 Combat: asymmetric speed and cancels

- **Ghost of Tsushima** (Chris Zimmerman, [PlayStation Blog 2020](https://blog.playstation.com/2020/06/23/ghost-of-tsushima-mastering-the-katana/)):
  attacks are mocapped at realistic speed, and humans need "about 0.3 seconds to respond to a visual
  stimulus". So **"your attacks can be arbitrarily fast, but Mongol attacks can't be faster than the
  player can react"**. Enemy strings make only the *first* blow slow enough to read; follow-ups come
  fast. The player's heavy attacks "can be instantly cancelled at any point", an "important part of
  high-level play". Block pressed just before impact becomes a parry; pressed on impact, a perfect parry.
- **For Honor** publishes its timings in official patch notes, so the ranges are public
  ([patch 2.38](https://www.ubisoft.com/en-us/game/for-honor/news-updates/6yK8Z1hgpAoScO2THLLdYD/patch-notes-2380-for-honor);
  [2.28 summary](https://www.dualshockers.com/for-honor-update-2-28-patch-notes-today-june-10-year-5-season-2/)):
  lights ~400-600 ms to impact, heavies ~700-1,200 ms, miss recoveries ~600-1,100 ms; dodge attacks come
  out partway through the dodge with i-frames; cancel and follow-up windows are tuned in **~33-100 ms
  steps**, and patches mostly move them by 100 ms. Its stated balance aims are about what is
  *punishable* and *what beats mashing*, i.e. commitment is a tuned per-move cost, not a global feel.
  The ~500 ms light floor sits just above Zimmerman's ~300 ms reaction time *(inference)*.
- **God of War (2018)**, closer camera (Mihir Sheth, [GDC 2019](https://gdcvault.com/play/1026423/Evolving-Combat-in-God-of);
  details only via an [AI summary of the recording](https://videohighlight.com/v/hE5tWF-Ou2k)): with no
  target Kratos attacks straight; with one, attacks **auto-orient** to it and a "suck to target" pulls him
  in, its range shrinking at wider angles. Lock-on was added late after playtests, with free aim kept.
- **Celeste-style forgiveness** carries over directly: a jump pressed shortly before landing fires on the
  landing frame (buffer); a jump shortly after leaving a ledge still works (coyote time, about 6 frames
  at 60 Hz per community measurement) ([Thorson, "Celeste & Forgiveness"](https://maddymakesgames.com/articles/celeste_and_forgiveness/);
  [TASVideos](https://tasvideos.org/Forum/Topics/22956)). R6's clip-gated buffers are the combat form of
  the same idea.

### 1.4 Hit-stop and shake

- **Sakurai** (Smash, [Famitsu column, translated](https://sourcegaming.info/2015/11/11/thoughts-on-hitstop-sakurais-famitsu-column-vol-490-1/)):
  hit-stop grows with damage up to a **hard cap**; attacker and victim freeze **equally**; the victim
  **vibrates** (horizontally on the ground, vertically in the air) with amplitude decaying over the stop
  and scaled by camera distance, while **hurtboxes stay where they were**; the victim's pose blends into
  its hurt pose during the stop. Many-enemy games (Dynasty Warriors) use little hit-stop because it slows
  the whole game; free-for-alls limit it because a frozen fighter is a free target for a third.
- *(inference, with R6)* R6's whole-character clock hold of ~60 ms plus ease-out agrees: freeze both
  sides briefly, never the world; shorter when crowds fight.
- **Shake as trauma**: a decaying trauma value driving noise-based shake is the common indie/AA form
  (Eiserloh, GDC 2016, [video summary](https://www.gamedeveloper.com/programming/video-sprucing-up-cameras-with-math);
  faith-runner's squared trauma in R2 §7 is the same idea). A second-hand account of his camera material
  gives a useful 3D rule: **shake rotation only, not translation**, so the camera can't jitter into a wall
  ([blog](https://blogs.jccc.edu/dpatter/tag/mathematics), unverified against the talk).
- **Comfort rules** (Xbox Accessibility Guideline 117, [devdocs](https://devdocs.xbox.com/gaming/accessibility/xbox-accessibility-guidelines/117)):
  let players disable shake, bob and motion blur; avoid repetitive sway and bob outside core gameplay;
  adjustable FOV; disable automatic camera movement; "allow players to choose between 1st and 3rd
  person". Examples: Halo Infinite shake sliders, ESO head-bob intensity.

## 2. First-person melee

Primary sources here are thin: no studio has given a full first-person melee talk that is publicly
readable (KCD, Mordhau, Chivalry 2 and Dying Light combat talks either don't exist or are paywalled).
What follows is assembled from dev posts, patch notes, engine docs and mod docs.

**Body strategies.**
- **Separate arms rig** (Oblivion, Skyrim): first person uses its own skeleton with a camera bone the
  world skeleton lacks ([Nexus forum](https://forums.nexusmods.com/topic/722270-reqz-1st-person-weapon-anims)),
  and its own clips. Mods that swap in third-person clips for first person report the feet not moving
  during attacks: the two clip sets never agreed ([Joy of Perspective](https://www.nexusmods.com/skyrim/mods/6002)).
- **Separate presentation, exaggerated** (Overwatch, Matt Boehm, GDC 2017, [summary](https://www.invenglobal.com/articles/1187/how-overwatchs-first-person-animation-breathed-life-into-heroes)):
  first-person and third-person animation are made and rendered differently. First person stretches
  weapons and limbs "like rubber" for impact, keeps the reticle and screen centre clear (reloads were
  moved to the other hand when they blocked the view), and looks broken seen from outside (hands passing
  through each other).
- **True first person** (one full body, camera on it): Chivalry, Mordhau, KCD. Triternion: "keeping the
  player immersed in first person is a big deal", and third person in their videos is switched on
  "to show how things look to other players" ([dev posts](https://forums.unrealengine.com/t/mordhau/50400)).
- **Engine middle ground**: UE 5.5+ renders chosen primitives with their own FOV and a scale toward the
  camera so they don't clip walls, morphed before projection in the same pass. The owner sees the
  first-person primitives; a separate **world-space representation** (owner-no-see) casts shadows and is
  seen by others; a per-vertex alpha blends world and first-person space so **feet stay on the ground**;
  self-shadowing is screen-space ([UE docs](https://dev.epicgames.com/documentation/en-us/unreal-engine/first-person-rendering)).

**Hit detection follows the weapon, not a cone.**
- Vanilla Skyrim tests **one frame**, on a hit event in the clip, and picks the best target in front:
  the swing's path doesn't matter. The popular *Precision* mod replaces it with weapon collision,
  hit-stop with camera shake, weapon trails and recoil off hard materials, in **both views**
  ([mod page](https://www.nexusmods.com/skyrimspecialedition/mods/72347/), via search summary).
- Chivalry (2012) samples points along the blade between hilt and tip sockets each tick and traces from
  each point's previous position to its current one, which fixed misses when the player pivots
  ([dev blog #1](https://www.moddb.com/games/chivalry/news/developer-blog-1-hit-detection), via
  search snippet; page blocks fetch). R6's ER and SK sweeps are the same idea.
- Mordhau: a **parry** happens when the weapon strikes a player who is in a parry state *and aiming at
  one of the weapon's traces, with some angle tolerance*; they rejected a box collider in front of the
  player "to have more control". Authoritative server; "frame-perfect" hit detection is the priority
  ([dev posts](https://forums.unrealengine.com/t/mordhau/50400)).
- Chivalry 2's post-launch hit registration problems (hits "not landing or landing too early") were
  blamed on **low animation update rates** of off-screen players
  ([Torn Banner, 2021](https://chivalry2.com/2021/10/28/post-launch-update-re-patch-2-2-0-fight-knight/)),
  and some stab specials had "early tracers" removed so they no longer felt "like an instant and
  unreactable hit" ([2.3 notes](https://chivalry2.com/2021/11/26/merry-chivmas-patch-notes-2-3/)). Hit
  windows that depend on an evaluated pose break under animation LOD (R6 adopt 1 avoids this).

**The view drives the weapon, and that is the main FP melee failure.**
- In true first person the weapon hangs off a body that turns with the mouse, so turning during a swing
  changes where and when it lands. Chivalry 1's community called this **dragging**; Torn Banner: it
  caused "animation issues that made the combat difficult to read. We should have fixed it, and never
  did" ([Code of Chivalry](https://chivalry2.com/2019/10/08/498)). The same post defines *clunky* as
  "feels unresponsive or sluggish **without clear indication**".
- Chivalry 2 answered "spin to win" by simulating "more bones and muscles", so turning builds "torque
  and tension" instead of spinning ([Piggott, Wccftech](https://wccftech.com/chivalry-ii-interview-with-torn-banner-studios-steve-piggott-and-alex-hayter//amp/)).
  Mordhau kept drags (turn away to delay the hit) and accels (turn into it) as skill, plus feints
  during the wind-up "before your character is fully committed" (players,
  [Steam](https://steamcommunity.com/app/629760/discussions/0/2653116024331733385); community wikis put
  the feint cut-off near the end of the wind-up).
  *(inference)* For a single-player RPG the lesson is a **turn-rate cap during active frames** (R6's
  no-turn windows), so the mouse can aim the swing at its start but can't whip it.

**Direction and readability.** KCD fights **locked on** in first person: a five-point star appears on
the target and the stick or mouse picks the point to strike from. A **perfect block** is timed to a
**green shield icon** over the reticle, so the timing cue is UI, not only the enemy's body; a master
strike (a riposte) needs the block in the first frames of that cue ([Push Square guide](https://www.pushsquare.com/news/2018/02/guide_kingdom_come_deliverance_combat_tips_and_tricks_-_how_to_survive_in_battle);
[KCD2 guide](https://beebom.com/how-to-master-strike-in-kingdom-come-deliverance-2/amp/)).
*(inference)* Lock-on plus a UI cue is how true first person compensates for not seeing the
opponent's whole wind-up at melee range. KCD2 kept the system with fewer directions and more weight
per blow (reviewer, [Newsweek](https://www.newsweek.com/entertainment/video-games/kingdom-come-deliverance-2-review-2025183)).
Dark Messiah made **one-button kicks and pervasive physics** its first-person melee identity
([Colantonio Q&A](https://gamebanshee.com/kkcdd)); Dying Light credits weight to "timing, weight, and
enemy reactions... wrapped in brutal audio feedback", with body-part hits that stun, trip or dismember
([Prima](https://primagames.com/featured/franchise-director-tymon-smektala-talks-dying-light-the-beast)).

**Camera and comfort.** No studio numbers for FP melee cameras were found; the rules are XAG 117
(§1.4) and rotation-only shake. faith-runner (R2 §5-7) stays the most concrete reference.

## 3. Both views: which one is truth

- **The world body is truth in every multiplayer case.** Mordhau's server is authoritative and
  third person shows "how things look to other players" *(inference: its traces are on the world body
  others see)*. Overwatch's first-person animation is admittedly wrong from outside, so gameplay can't
  read it. UE splits the same way: first-person primitives are presentation, the world-space
  representation is what others see.
- **Skyrim sidesteps the question**: a hit is an event plus a target search, so either clip set can fire
  it. The cost: the swing doesn't matter, and the two clip sets drift (feet, timing).
- **What breaks when switching.** Third person gives awareness; Chivalry 2's third person works "similar
  to the Elder Scrolls games" but suits competitive play poorly ([review](https://massivelyop.com/2021/05/31/fight-or-kite-chivalry-2-is-a-blood-soaked-action-packed-experience-of-absolute-mayhem/)).
  Mordhau's view-driven swings look like "a spinning toy" from outside (players, [Steam](https://steamcommunity.com/app/629760/discussions/0/1645418261863641315/?ctp=4)).
  God of War's close camera needed off-screen attack indicators, enemies held in their last-known
  quadrant, ~2 aggressive enemies at once, and hits that knock enemies **toward the view**
  ([AI summary of Sheth's talk](https://videohighlight.com/v/hE5tWF-Ou2k)): the closer the camera, the
  more AI and reactions are shaped around it, and first person is the extreme case.

## 4. The R11a conflicts

**Who owns motion.** *For code-driven*: precise control, needed by networking and collision
(Holden's clamping; UE's capsule plus distance matching). *For clip-driven*: weight and no sliding,
at the price of sluggishness. *Evidence for both*: Holden's answer is per-region labels in the data plus
velocity-limited correction, which is "per move kind" by another name. Locomotion: code leads, the
clip catches up within clamps. Attacks, dodges, takedowns: the clip leads (R6), with lunge and turn
windows as the code's say.

**Commitment vs responsiveness (main conflict).** *For commitment*: For Honor's per-move wind-ups and
recoveries are its balance tool (~400-600 ms lights, ~700-1,200 ms heavies, recoveries moved in 100 ms
steps); souls (R6). *For responsiveness*: Ghost of Tsushima lets the player cancel heavies "at any
point" and makes player attacks "arbitrarily fast"; Torn Banner's "clunky" is sluggish **without
clear indication**; West: the first visible response must come within ~50-67 ms. *Reconciliation the
sources support*: (1) split **first response** (within a few frames: pose starts, sound, camera kick)
from **commitment** (the effect and the recovery may be long); (2) make commitment **asymmetric**:
the player's attacks are fast and cancellable into defence, enemies' first blow in a string is slower
than ~0.3 s reaction time; (3) tune commitment **per move** in data, with cancel windows (R6); (4)
forgive input timing (buffers, coyote time). Momentum moves (free-running) sit at the responsive end,
heavy attacks and paired kills at the committed end.

**First person.** *For a separate arms rig*: exaggeration for impact and screen clarity (Overwatch),
Skyrim's long practice, no head-bob from third-person clips. *Against*: clip sets drift (Skyrim's feet
and timing), double authoring, and what you see isn't what hits. *For one body*: Chivalry, Mordhau and
KCD fight on the world body with world hit traces; UE 5.5 ships per-primitive first-person FOV/scale
and a feet blend so one body can serve both. *Against*: the view drives the weapon (dragging, "spin to
win"), and you can't read wind-ups at sword range, so KCD adds lock-on and a UI timing cue. *Evidence
points to*: one body is the truth, first-person drawing is a presentation transform (FOV, scale, small
offsets) on that body, never different timing.

**Camera.** *Lock-on/assist*: God of War added lock-on late after playtests and auto-orients attacks;
KCD locks on in first person; souls (R6). *Free look*: Mordhau and Chivalry leave aim to the mouse and
pay in readability (drags, spins). Closer cameras need off-screen support (God of War). Comfort: XAG
117 switches, rotation-only shake, both views offered.

**Pace and scale.** Hit-stop must shrink as enemy count rises (Sakurai, Dynasty Warriors); the
player's speed may exceed realism, enemies' may not exceed reaction time (Ghost of Tsushima). One
tuning won't serve a duel and a crowd: scale hit-stop, aggression tokens and wind-ups by encounter
*(inference)*.

**What clips must carry.** Weapon-swept hits need a weapon segment per frame (Chivalry, Mordhau,
Precision); Skyrim's single hit event is the minimum and loses the swing. First-person presentation
clips (Overwatch) carry no gameplay data.

## 5. Situations

- **Sword fight, third person.** R6's clip-led attacks and cancels; first response within a few
  frames; enemy first blows slower than ~0.3 s; auto-orient and suck-to-target when unlocked (God of
  War); short shared hit-stop, victim shake, fixed hurtboxes (Sakurai).
- **Sword fight, first person.** Same sim and windows; sweeps from the world body; turn-rate cap in
  active frames; soft lock or KCD-style lock with a block timing cue; small rotation-only kicks; arms
  drawn with their own FOV/scale, exaggerated only in presentation.
- **Stealth kill, first person.** No first-person source found. Deus Ex: Human Revolution cut to third
  person for takedowns to show the moves ([Game Informer](https://www.gameinformer.com/b/news/archive/2010/03/11/deus-ex-human-revolution-details-divulged-at-gdc-panel.aspx);
  [Siliconera](https://www.siliconera.com/deus-ex-human-revolution-and-the-soul-of-deus-ex-1/)), and
  previews found the switch jarring. *(inference)* With one body and R2's baked camera bone, the
  paired kill (R6 §9) can play in first person with look limits, and a third-person cut stays an
  option per kill.
- **Sneak, roofs, climbing, sitting, doors**: out of scope (R7); the locomotion dial (§1.2) applies.

## 6. For us

Mapped to `sf_controller`, `sf_anim_graph`, `sf_anim`, the missing `docs/specs/combat/` (R6) and the
missing camera spec (R2).

**Adopt**
1. **A latency budget and test.** First visible response within 1-3 fixed ticks of a press for every
   action (pose change, sound or camera kick), measured by a scripted-input test like R6's: input
   tick → first frame the pose or camera differs. Root impulses on the first frame of clips that need
   them. Check that our action layer doesn't add a frame by firing events after the anim update.
2. **Holden's displacement model for locomotion**: code-driven `sf_controller` capsule, the character
   corrected toward it with a halflife, velocity-limited (~0.5× own speed) and clamped; separate
   position and rotation responses; and a **per-clip-region displacement label** (code, clip, blend)
   instead of one root-motion mode per graph. This refines animation.md's Drive/Match/Ignore.
3. **Asymmetric combat timing as a spec rule**: player moves fast with cancels into defence; enemy
   first blows readable (≥ ~0.3 s cue), follow-ups may be fast; per-move commitment data (R6 model).
4. **One body is truth in both views.** Hit sweeps from the shared action clock and baked weapon tracks
   (R6 adopt 1). First person is a presentation transform: own FOV and scale for arms and weapon, feet
   blended to world, world body for shadows (the UE 5.5 model, built on Bevy render layers). Turn-rate
   cap during active frames.
5. **Hit-stop scaled by crowd**: R6's curve, shorter or off when many enemies are engaged; victim
   shake with hurtboxes fixed; both sides frozen.
6. **Comfort settings from XAG 117** in the camera spec: shake, bob, auto-camera off; FOV slider;
   rotation-only shake; view choice.

**Adapt**
- **Lock-on in first person** as a soft aim with an optional hard lock and a UI timing cue (KCD), not
  a camera that turns for you.
- **God of War's close-camera kit** (off-screen indicators, aggression tokens, reactions toward the
  view) for third person; tokens apply to both views.

**Reject**
- A separate first-person rig with its own timing (Skyrim): the drift is exactly what we want to avoid.
  Morrowind's first-person clips may still be presentation-only on top of the shared clock (R6).
- Free view-driven swing physics (Chivalry 1 dragging) in single-player combat.
- Single-frame cone hits as the default; keep them only as a fallback when a clip has no weapon track.

**Docs that are thin or missing**
- animation.md has no latency budget and no displacement-correction model (halflife, velocity limit,
  clamp); locomotion.md's capsule/clip relationship should cite Holden.
- No camera spec covers comfort settings, first-person FOV/scale, or close-camera combat support.
- WISHLIST §9's "hitstop" needs the crowd rule; combat spec needs the asymmetric timing rule.
