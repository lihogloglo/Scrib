# Animation from a limited free clip set (research slice, 2026-11-25)

Research only. Extends `docs/wiki/industry/character-animation.md`, `docs/wiki/engine/animation.md`, `docs/ledger/animation.md`
(not repeated). Web facts carry a URL; "(judgment)" = my ranking, not a source; "(memory)" = not re-checked this session.

## 0. Findings

1. **100STYLE is the best unused free locomotion source: real mocap, CC BY 4.0, shippable.** Kimodo locomotion failed its gates (ledger 2026-11-12).
2. **Motion matching: park.** Not for combat; locomotion-only spike later on 100STYLE (our mover already predicts a `Trajectory`).
3. **`OrientationWarp` and `LookAt` ops exist but are unwired; acceleration lean is absent.** Starts/stops need no new clips: phase-gated inertialized transitions.
4. **Souls numbers set the roll**: DS3 433 ms i-frames from frame 1, Elden Ring 14 i-frames (0.47 s); ours 0.38 s. "Roll starts late" is the shared sprint/roll button.
5. **Morrowind text keys are a free combat timing track**; OpenMW 0.49 blending equals our inertialization.

## 1. Procedural layers: quality per clip

Judgment ranking by player-visible gain per clip supplied. "Have" = exists in `sf_anim_graph` (wiki ops list).

| # | Layer | Standard | Saves or rescues | State |
|---|---|---|---|---|
| 1 | Gait lanes at authored speed, rate clamp 0.8-1.25, sync groups | Lyra, Mecanim | rescues bad speed fit | have (A3) |
| 2 | Inertialization on every switch | Bollo, GDC 2018: one pose evaluated, offset decays, quintic ([GDC Vault](https://gdcvault.com/play/1025165/Inertialization-High-Performance-Animation-Transitions)) | blend-time authoring | have |
| 3 | **Phase-gated transitions + root decel for starts/stops** | UE distance matching needs a root distance curve, XY axis for start/stop/pivot ([UE docs](https://dev.epicgames.com/documentation/en-us/unreal-engine/distance-matching-in-unreal-engine)) | start/stop clips (first version) | **missing** |
| 4 | **Orientation warping** (lower body toward travel, upper body on aim) | Lyra: 4 cardinal cycles + OW covers diagonals (memory) | 8-way strafe from fwd/back/side cycles; drops the Morrowind directional set | op exists, **not in the stack** (SpeedMatch, StrideWarp, FootPlacement, Lean, HandIk) |
| 5 | Acceleration and turn lean (additive, from capsule accel and yaw rate) | all shipped third-person games (memory) | hides the 720-1080 deg/s capsule turn (audit cause 4) | `Lean` is slope only per wiki: check |
| 6 | Look-at (head then eyes, limits, spring) | Lyra, Unity Animation Rigging | life at zero clips | op exists, **unwired** |
| 7 | Stride warp, foot lock, pelvis | UE Foot Placement | done; player never signed off (AL-2, PL-7) | have |

**Overgrowth** (Rosen, GDC 2014, ["An Indie Approach to Procedural Animation"](https://gdcvault.com/play/1020049/Animation-Bootcamp-An-Indie-Approach)):
few keyed poses, spring interpolation, procedural root and feet; the "13 keyframes" figure is a forum claim
([thread](https://discussions.unity.com/t/an-indie-approach-to-procedural-animation-gdc-video-talk/538228)). Lesson: **short
actions (roll, backstep, hop-dodge, block raise, flinch) can be 4-8 key poses plus springs and a root arc**, no mocap and
no Quaternius. Candidate slice: a `PoseSeq` asset (key poses, eases, root arc).

### Starts and stops without clips (design)
- Stop: stick released, mover brakes (half-life 0.05 s). Keep the cycle speed-matched (it is), switch to idle by an
  inertialized transition **only at the next `down.*` marker**. Slide is bounded by half a stride (~0.4 m at jog); audit:
  14 of 16 stops unplant a foot, 40-100 cm.
- Start: enter the cycle at the marker phase whose foot is forward, rate 1.25 for 0.3 s, additive back-lean. 100STYLE TR clips replace it later.

## 2. Motion matching with a small dataset

- **Needs** a feature database (foot pos/vel, hip vel, 2-3 future trajectory points) over varied clips. Vendor guidance:
  "10+ minutes of varied mocap"; 10 varied clips beat 50 similar ones ([Unity MM FAQ](https://motion-matching-for-unity.readthedocs.io/en/latest/faq/)).
  It only replays what is in the library ([Ubisoft La Forge](https://www.ubisoft.com/en-us/studio/laforge/news/6xXL85Q3bF2vEj76xmnmIu/introducing-learned-motion-matching)).
  Epic: more data, more fidelity, more memory ([UE docs](https://dev.epicgames.com/documentation/unreal-engine/motion-matching-in-unreal-engine)).
- **Our clips fail it**: Kimodo run/sprint skate 83/179 cm/s, UAL has no starts/stops. MM exposes clip quality; a graph hides it.
- **100STYLE changes this**: 4M+ frames at 60 fps (~18.5 h, my arithmetic) of clean locomotion, enough for a locomotion-only DB.
  Combat stays montage-based (GASP: MM for locomotion, actions separate; memory).
- **Open code**: `orangeduck/Motion-Matching` (Holden; code **MIT**, C/C++, includes Learned MM; dataset CC BY-NC-ND; [repo](https://github.com/orangeduck/Motion-Matching)); `JLPM22/MotionMatching` Unity, **MIT** ([docs](https://jlpm22.github.io/motionmatching-docs)). Bevy: `voxell-tech/bevy_motion_matching` (MIT/Apache; Bevy version and maturity **unverified**). **No verified Bevy 0.20 MM crate.** Learned MM (Holden 2020) saves memory on huge data: no gain for us.
- **Recommendation**: park. Later one spike (Bevy-free `sf_motionmatch`, ~600 lines): brute-force search over a cooked
  100STYLE DB (small, no KD-tree), search every 0.1 s, inertialization (have), trajectory from `KinematicMover` (have). Gate: beats
  the blend graph on stops, pivots, turn-in-place in an A/B the owner judges.

## 3. Souls-like action animation

**Public numbers** (Elden Ring 30 fps, frame 0.033 s):
- DS3: roll i-frames from frame 1, 433 ms (medium); enemy attack signal + attack >= 340 ms or nearly undodgeable; hitbox active
  2-4 frames; end pose >= 240 ms as bait; measured input response 60-100 ms
  ([Anatomy of an Enemy Attack in DS3](https://www.gamedeveloper.com/game-platforms/anatomy-of-an-enemy-attack-in-dark-souls-3)).
- Elden Ring: roll 14 i-frames light/medium, 13 heavy (+3 with Windy Crystal Tear or Crucible Feather)
  ([wiki.gg](https://eldenring.wiki.gg/wiki/Invincibility_Frames)). Total roll length and recovery figures are unverified (mirror sites): tune to feel.
- **Cancel model** ([Combat Canceled](https://www.gamedeveloper.com/game-platforms/combat-canceled-i-god-of-war-i-action-game-design)):
  frame-range windows; *buffer* (store, fire when the window opens, a later input can overwrite) vs *instant* (fire on the input frame
  inside the window). Pre-hit cancels read responsive (God of War), post-hit strict (Ninja Gaiden, DMC). Ours (`step.rs` two-stage
  buffer + `Accept`/`Cancel` windows): **match**.
- **Root motion vs capsule**: attacks and rolls clip-driven (`Owner::Clip`/`Warp`/`Progress`, `root_scale`), locomotion sim-driven;
  same split as Lyra montages and FromSoftware per-animation events (TAE internals not verified). **Match.**

**Spammable roll** (`sword_1h.ron` "roll": i-frames 0.02-0.40 move-s, rate 1.25, Dodge cancel 0.72, Move 0.74):
1. Fire on press. A shared Sprint/Roll button forces tap-vs-hold (fire on release) = the CB-3a lateness. Dedicated key (Q1).
2. i-frames from tick 0 (ours 1 tick), 0.35-0.45 s (ours 0.38). Fine.
3. Open `Cancel([Dodge])` at 55-65% of the move, not at 99% of travel: spam is then limited by stamina and buffer, not the clip.
   Buffer a roll pressed up to ~0.2 s early; check Dodge uses the buffer.
4. Front-load travel (ease-out arc) so the body leaves the swing in the first 40%.
5. Inertialize in and out, 0.06-0.10 s, never crossfade.
Base clip stays UAL `Roll_RM`; a key-pose roll (section 1) is the no-mocap upgrade.

**Hit reactions** (judgment): additive flinch (`HitReact`, have) for light hits so a swing is never interrupted; full stagger only on poise break (Elden Ring); player attacks carry armor frames. **Hitstop**: Smash 8-15 frames by damage ([SSBWiki](https://www.ssbwiki.com/Hitlag)); guide: light 2-4, heavy/parry 8-12, finisher 12-20 ([gamejuice](https://gamejuice.pages.dev/articles/hit-stop-frame-trick)). Ours 0.05 s hold + 0.10 s ease: right for light. Add tiers (Small 3, Medium 6, Heavy 10-12 frames), freeze both sides, slow to ~5% not 0.

## 4. Morrowind's own clips as the combat base

**What exists** (UESP animation groups; page 403 here, so from a search digest and memory): per weapon family `WeaponOneHand`,
`WeaponTwoHand`, `WeaponTwoWide`, `HandToHand`, `BowAndArrow`, `Crossbow`, `ThrowWeapon`, each Chop/Slash/Thrust with keys **Start,
Min Attack, Max Attack, Hit, Follow Start/Stop** ([UESP](https://en.uesp.net/wiki/Morrowind_Mod:Animation_Groups); OpenMW picks
the style from the weapon's strongest range, [character.cpp](https://git.cijber.net/mirror/openmw/src/commit/bf8fafa79aa57ca605eeece4cee18461d9e59b02/apps/openmw/mwmechanics/character.cpp)).
Also `Shield` (block), `SpellCast`, `Hit1-5`, `KnockDown`, `KnockOut`, `Death1-5`, stance idles, 4-direction walk/run/sneak, swim, jump, turn. Our adapter maps these (`combat-adapter.md:35`).

**Known problems** (ledger, memory): walk/run authored at ~2.2 and 2.7-3.2 m/s; no start/stop/pivot; three stiff attack variants (hold-to-charge), little anticipation or follow-through weight; poor strafe contacts.

**Modern practice**
- **OpenMW 0.49 smooth transitions**: rules `from`, `to`, `easing`, `duration` (YAML/JSON), end wildcards, bottom rule wins, 0.2-0.4 s,
  Out easings for organic motion, mods ship rules beside the `.kf` ([docs](https://openmw-trav.readthedocs.io/en/latest/reference/modding/animation-blending.html)).
  We have the better tool; take only the idea of **per-pair transition rules as data** (kind + duration).
- **ReAnimation**: alternate attacks play on top of the vanilla group while vanilla **text keys keep driving damage and stage
  timing**; alts must match windup/attack/follow durations ([repo](https://github.com/fallchildren/OpenMWReAnimation)). Same architecture
  as our sheets (timing in data, clips are visuals): confirms the design.
- Simply Walking, MCAR, Animated Morrowind exist as clip replacements ([lists](https://modding-openmw.com/lists/just-beautiful-morrowind-animations)); not read; per-mod terms, BYO only.

**Retiming/augmenting recipe** (existing machinery; judgment)
1. Phase map from text keys: anticipation Start..Min Attack, active = Hit window, recovery = Follow.
2. Per-phase rate in the sheet: anticipation slower (readable), strike 1.3-1.6x, recovery 1.0x.
3. Additive anticipation (spine twist back, arm pull, 0.08-0.12 s) on the Min Attack pose; additive strike overshoot.
4. `HandIk`/`FullBodyIk` keeps the blade plane on the lock-on target (soft aim exists).
5. Cancel exit: inertialized blend-out 0.08-0.12 s at the cancel tick, recovery truncated.

## 5. Free clip sources and licences

| Source | Content | Licence (source) | Ship in GPL repo? |
|---|---|---|---|
| **100STYLE** (Ian Mason) | 100 styles x FW FR BW BR SW SR (fwd/back/side walk+run), ID, TR1-3; Neutral has BR BW FR FW ID SR SW TR1; BVH 60 fps, 28 bones | **CC BY 4.0**, credit "The 100STYLE Dataset - Ian Mason" ([page](https://www.ianxmason.com/100style/)) | **Yes**, attribution entry. No combat, no sprint named. Long takes: loop cutter applies |
| **KayKit Character Animations** (Kay Lousberg) | 161 humanoid clips (150+ free): 1H, 2H, unarmed, dual, block, dodge, hit, death, walk/run/sneak, spells, bows; Rig_Medium/Large, FBX+glTF | **CC0** ([itch](https://kaylousberg.itch.io/kaykit-character-animations)) | **Yes**. New to our docs. Quality unjudged (stylised), no roll confirmed; needs a clip-preview pass |
| Quaternius UAL1/2 | in repo | CC0 | yes (owner rates poor) |
| Mixamo | big library | royalty-free in projects, no redistributing raw files, no ML training ([Adobe FAQ thread](https://community.adobe.com/questions-696/mixamo-faq-licensing-royalties-ownership-eula-and-tos-589400)) | **No**; player downloads |
| LaFAN1 | fights, locomotion | CC BY-NC-ND 4.0 | **No**, BYO |
| Bandai Namco Motiondataset 1/2 | 1: 17 contents x 15 styles incl. fighting (36,673 frames); 2: locomotion, hands x 7 styles (384,931) | README **CC BY-NC 4.0** ([repo](https://github.com/BandaiNamcoResearchInc/Bandai-Namco-Research-Motiondataset/blob/master/README.md)); a 2022 article said NC-ND ([Automaton](https://automaton-media.com/en/news/20220430-12213/)): read the repo `LICENSE` | **No** (NC). BYO combat reference |
| Rokoko free fight pack (10), Mocap Online T.C. Sword | sword/fight mocap | terms not stated on the page / rate-limited | unknown: BYO until read |
| Kimodo (ours) | generated | code Apache-2.0 (secondary), SOMA weights NVIDIA Open Model License, card says commercial-ready; SMPL-X variant differs ([HF](https://huggingface.co/nvidia/Kimodo-SOMA-SEED-v1.1)) | outputs `private_only` until the training-data check; ambient and NPC filler |
| Own key poses (PoseSeq) | roll, dodge, flinch | ours, GPL | yes |

## 6. Gaps by player impact, recommendations

| Rank | Gap | Action | Why |
|---|---|---|---|
| 1 | Locomotion clips bad (Kimodo cycles, Morrowind speed mismatch) | **Change**: cook 100STYLE Neutral FW/FR/BW/BR/SW/SR + ID + TR into `loco.*` slots (loop cutter); Kimodo = ambient filler | real mocap, shippable, 8 directions |
| 2 | No starts/stops/pivots; stops slide 40-100 cm | **Adopt** marker-gated inertialized transitions + root decel now; 100STYLE TR clips next; MM spike later | top gap in the 2026-11-12 audit |
| 3 | Roll late, not spammable | **Change** input (Q1), earlier Dodge cancel, front-loaded travel, check Dodge buffer | Elden Ring target |
| 4 | Lock-on strafe uses Morrowind directional set | **Adopt** `OrientationWarp` in the humanoid stack with 100STYLE side/back cycles | op exists |
| 5 | Combat clips very bad | **Keep** Morrowind groups (owner), retime per phase, additive anticipation; **try** KayKit 1H/block/dodge in clip-preview; **author** roll/backstep as key poses | no mocap needed |
| 6 | Hit reactions all full clips | **Change**: additive flinch default, stagger on poise break, tiered hitstop | game feel |
| 7 | Look-at, acceleration lean missing | **Wire** `LookAt`; add accel/turn lean to `Lean` | alive; hides capsule turn |
| 8 | Motion matching | **Park**; spike after 1-2 | needs the 100STYLE cook |
| - | Mixamo, LaFAN1, Bandai, CMU | **BYO only** | licences |

### Candidate owner questions (player-facing; recommended answer first)
- **Q1 Roll input.** Dedicated roll key (keyboard Space, sprint on Shift hold); on gamepad keep Elden Ring tap-B roll, hold-B sprint?
  Yes: removes the tap/hold delay on keyboard, pad users keep the familiar scheme.
- **Q2 Stagger.** Light hits flinch without interrupting your swing; only heavy hits or a broken poise bar stagger you (enemies the same)?
  Yes (Elden Ring). Morrowind's every-hit interrupt stays a Classic seam.
- **Q3 NPC gaits.** Styled walks by age, mood or role (tired, old, proud; from 100STYLE) as character flavour?
 
- **Q4 Shipping clips.** Ship CC0 (UAL, KayKit) and CC BY (100STYLE) cooked clips in releases with an attribution file
  (open question 1 of `animation-sources.md`)? Yes; everything else BYO.

## 7. Unverified
- UESP 403 (group names from a search digest, memory); MCAR, Simply Walking, Brutal Morrowind not read; FromSoftware cancel internals and exact roll duration.
- `bevy_motion_matching` Bevy version; Mocap Online, Rokoko, Bandai `LICENSE` terms; 100STYLE raw take structure; KayKit quality and roll; Rain World, Kinematica.
