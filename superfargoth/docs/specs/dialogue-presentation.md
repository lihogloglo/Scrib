# Dialogue presentation: presenters, shots, pacing, gaze and talk, interrupts (DQ-26)

Status: **spec (DQ-26, 2026-11-26)**, nothing built. Roadmap: `docs/roadmaps/dialogue-quests.md` Stage 5 (DQ-27..DQ-32),
decisions 5, 6, 9, owner answers 2026-11-25. Related: [rpg.md](rpg.md) "Dialogue", [dialogue-rules.md](dialogue-rules.md)
"Barks", "Seams", [ui.md](ui.md) "Dialogue window view-model", [camera.md](camera.md) "Framed cameras",
[save.md](save.md). Game side: `bevyrrowind/docs/specs/rpg-adapter.md` "Presentation (DQ-26)", "Co-op".

> **Agent summary.** One host `DialogueSession` feeds one `DialogueView`; a **`DialoguePresenter`** shows it:
> `Classic` (today's window, camera on the player), `Cinematic` (auto camera, paged subtitles, topics as a panel beside
> the framed NPC, clickable keywords; Modern default), `Barks` (subtitles only: barks, co-op listeners). Presenters
> emit only `DialogueRequest`s; pacing, camera, gaze and talk never feed gameplay. The **shot solver** (Bevy-free,
> new `sf_dialogue_stage`) picks two-shot / over-shoulder / close on a fixed side of the line of action, cuts on
> speaker change or shot length, tests occlusion through a callback and falls back to Classic framing; it reaches the
> camera through a new **takeover** seam. Text lines get procedural talk and gaze; voiced lines a Rhubarb viseme
> track baked offline. Host `Interrupt { reason }` ends a session with a `talk.interrupted` bark; topics and history
> survive.

## Goal (what the player sees)

- **Cinematic, third person** (Modern default): the camera frames the NPC; lines are subtitles paged at reading speed
  while the NPC looks at you, talks and gestures; cuts follow the speaker, never crossing the line between you; topics,
  services and Goodbye in a panel on the free third; keywords clickable. Combat, a hit or a crime breaks it off with a
  line; talking again later keeps topics and history.
- **Cinematic, first person:** the view stays at your eyes, eased onto the speaker with a slight zoom (Cyberpunk 2077,
  Starfield); a setting gives the third-person shots. **Classic:** today's window, camera on the player (OpenMW).
  **Barks:** a named subtitle (edge arrow when off-screen), no input.

## Today (verified 2026-11-26)

- `sf_rpg/src/dialogue.rs`: `DialogueSession` `:182`, states `Talking`, `Choosing(Vec<Choice>)`, `Goodbye`, `Ended`
  (`:145-151`; rpg.md names a `Greeting` state the code lacks), outputs `:127-136`, `Conversations` claim table
  `:349-374`, which the game never uses (only a test, `mw_rpg_bevy/src/dialogue/services.rs:568`).
- `sf_rpg_ui/src/dialogue.rs`: `DialogueView` `:383` (`apply_outputs` `:562`, `nav` `:676`), `DialogueRequest`
  `:33`; screen `sf_ui_screens/src/dialogue.rs`, link span picking `:847`.
- `mw_rpg_bevy/src/dialogue.rs:1321` opens the window; controls pause (`:1590`), the world runs, the camera idles.
- Framed cameras: spec only (camera.md:75-81, :226-227); `ComfortSettings.auto_camera` (`sf_camera/src/comfort.rs:26-28`)
  is off by default and also gates them.
- `AnimDrive.look_target` (`sf_anim_bevy/src/lib.rs:171`) drives `LookAt`, unset (AN8b); additive layers exist
  (`sf_anim_graph/src/asset.rs:216-220`). Morrowind faces: `mw_actors_bevy/src/faces.rs` plays the head's `Talk`
  morph from a loudness (`Talking(f32)` `:92`; `Y` scaffold `:13-15`). Barks, `talk.interrupted`: spec only.

## Reuse check (rule 0, 2026-11-26)

- **Bevy 0.20:** cameras with vertical-FOV perspective, UI `Text`/`TextSpan` with span picking (the window uses it),
  `MorphWeights` (wiki bevy/skinning-and-animation §4); no conversation camera, subtitle pager, gaze or lip sync.
- **Crates:** `bevy_director` 0.4.0 (MIT/Apache, 2026-07, Bevy 0.19): authored shots and text overlays, owns the
  camera `Transform` (camera.md "Reuse" objection), one Bevy behind: not adopted; revisit for Stage 6 scenes.
  `bevy_talks` 0.6 (Bevy 0.19) and `bevy_yarnspinner` (DQ-33): runners with text boxes, no cameras. No maintained
  Rust lip-sync crate. **Rhubarb Lip Sync** 1.14.0 (MIT; WAV/Ogg in, optional dialog text, English PocketSphinx or
  phonetic recognizer): adopted as an **offline tool run as a process, never linked**. Excluded: OVR LipSync
  (proprietary), Audio2Face-3D (NVIDIA Open Model License, not OSI), FaceFX, JALI (commercial).
- **Own code:** the presenter seam (wraps our view-model), the solver (~400 lines on our `Look` and probe types),
  talk/gaze signals, the takeover (our rig).

## Industry comparison

**Canonical technique:** idiom-based automatic cinematography (He, Cohen, Salesin, "The Virtual Cinematographer",
SIGGRAPH 1996): per line an idiom (establishing two-shot, shot/reverse-shot over the shoulder, close-up) on one side of
the **line of action** (180-degree rule), cuts turning at least 30° (30-degree rule). Two-head framing in **toric
space** (Lino, Christie, SIGGRAPH 2015): the cameras putting two heads at given screen points form one surface.

| Them (source) | Us | Gap |
|---|---|---|
| Witcher 3: editor generates cameras, gestures per line, designers polish (Tomsinski, GDC 2016) | generated at run time | no polish pass (Stage 6 scenes) |
| Mass Effect: partly automated direction, then altered (Walters, Marino, GDC Austin 2008); DA Veilguard back to over-shoulder (Epler) | over-shoulder grammar | no hand-tuned shots |
| Cyberpunk 2077: first-person scenes, look around, timed choices, break on events (Kasprowicz, GDC 2024) | `Eyes`, interrupts, deadlines | no walk-and-talk |
| BG3: per-line cinematics; co-op initiator talks, others listen (Latino, GDC "Larian Cinematics") | listeners get subtitles | no shared cinematic |
| Skyrim none; Fallout 4 shot/reverse-shot with an off switch; Starfield first-person zoom; `.lip` baked from audio + text | Shots, `Eyes`, Off; Rhubarb bake | none |
| KCD2: first person cuts to third-person shots, timed answers (shipped) | optional shots from first person | none |
| Netflix: 5/6-7 s per event, 42 chars x 2 lines, English adults ≤ 20 cps; BBC 160-180 wpm; XAG 104: ≤ 40 chars, ≤ 2 lines, size to 200 %, background 0-100 %, names on speaker change | defaults below | long lines paged |

**Not built:** authored shots (Stage 6), walk-and-talk, facial emotion, runtime ML or TTS (decision 6), wheel
paraphrases, a shared co-op cinematic, moving NPCs to stage a shot.

## Design

**Crates.** `sf_rpg`: interrupts, resume, deadlines. `sf_rpg_ui` (Bevy-free): presenters, pager, panel model.
`sf_ui_screens`: widgets. New `sf_dialogue_stage` (Bevy-free, hakari `final-excludes`): solver, gaze, talk, visemes.
New thin `sf_dialogue_bevy`: participants, rays, takeover writes, `look_target`, talk rig. `sf_camera(_bevy)`: the
takeover seam. Tunable names below drop the `dialogue.` prefix.

### Presenter seam (`sf_rpg_ui::present`)

```rust
pub enum PresenterKind { Classic, Cinematic, Barks }
pub enum Role { Driver, Listener }
pub enum Speaker { Npc(ActorKey), Player(PlayerId) }
pub struct LineAudio { pub duration_s: f32, pub visemes: bool }
pub struct PresenterInput { pub advance: bool, pub held_s: f32, pub nav: Option<DialogueNav>,
                            pub click: Option<DialogueFocus>, pub leave: bool }
pub enum StageCue { Begin { speaker: ActorKey }, Speaking { who: Speaker, entry: EntryId, page_s: f32 }, Silent, End }
pub struct PresenterFrame { pub requests: Vec<DialogueRequest>, pub cues: Vec<StageCue> }
pub trait DialoguePresenter: Send + Sync {
    fn kind(&self) -> PresenterKind;
    fn on_entries(&mut self, view: &DialogueView, new: &[EntryId]);   // after apply_outputs/apply_events
    fn on_audio(&mut self, entry: EntryId, audio: LineAudio);         // from the voice player
    fn update(&mut self, view: &mut DialogueView, input: &PresenterInput, role: Role, dt: f32) -> PresenterFrame;
    fn screen(&self) -> PresenterScreen;                              // what sf_ui_screens draws
}
```

In: the view-model and `LineAudio`; out: `DialogueRequest` (the one wire format, as built) and cues. Advance and skip
are local; a `Listener` emits nothing. **Classic** = today's window (DJ-1 unchanged); **Cinematic** adds pager and
panel; **Barks** is a pager, also fed by `BarkLine`. `presenter` is read at session open.

### Pacing and subtitles

- **Pages** of at most `subtitle.max_lines` x `subtitle.max_line_chars`, broken at sentence ends, then clause
  punctuation, then spaces (XAG 104); link runs unsplit. **Time** = clamp(`read.base_s` + chars / `read.cps`,
  `read.min_page_s`, `read.max_page_s`) x the reading-speed setting. **Voiced:** audio length wins, shared by
  characters, plus `read.voice_tail_s`.
- **Input:** tap = next page (not within `read.skip_lockout_s` of one appearing; cuts voiced audio); hold
  `read.hold_skip_s` = last page. `auto_advance` off: each page waits.
- **Panel** from the last page until a pick (Outer Worlds "minimal": the line stays beside the options): actions,
  topics (new marked), Goodbye; `Choosing` replaces them modally. On the third opposite the speaker
  (`ShotDecision.subject_x`); scrolls and searches as the window.
- **Keywords:** link runs clickable (span picking, as built) and on the link cursor; activate = `Ask`. Speaker name on
  a new speaker's first page (XAG 104); a "log" action opens the Classic scrollback (BG3); the HUD hides (Mass Effect).

### Shot solver (`sf_dialogue_stage::shots`)

```rust
pub struct Participant { pub id: u32, pub head: Vec3, pub eyes: Vec3, pub height: f32, pub facing_yaw: f32 }
pub enum ShotKind { Two, OverShoulder { subject: u32 }, Close { subject: u32 }, Eyes }
pub enum Side { Left, Right }
pub struct ShotPose { pub eye: Vec3, pub rotation: Quat, pub fov_y_deg: f32 }
pub struct ShotState { pub kind: ShotKind, pub pose: ShotPose, pub side: Side, pub age_s: f32 }
pub struct ShotRequest<'a> { pub people: &'a [Participant], pub player: u32, pub speaker: u32, pub addressee: u32,
    pub prev: Option<&'a ShotState>, pub new_line: bool, pub view: ViewMode, pub gameplay: ShotPose }
pub trait Occlusion { fn clear(&self, a: Vec3, b: Vec3, r: f32) -> bool; fn free(&self, at: Vec3, r: f32) -> bool; }
pub struct ShotDecision { pub state: ShotState, pub cut: bool, pub subject_x: f32, pub fallback: Fallback }
pub fn solve(req: &ShotRequest, p: &ShotParams, occ: &dyn Occlusion) -> ShotDecision;
pub fn follow(state: &mut ShotState, people: &[Participant], p: &ShotParams, dt: f32);   // in-shot framing
```

Positions are `f32` relative to the speaker's feet (no `big_space` precision issue).

- **Line of action** = player head to NPC head; `Side` = the gameplay camera's side at entry, **kept all session**
  (180-degree rule), changed only through a re-establishing `Two`.
- **Grammar:** entry `Two` (heads on `shot.thirds_x`); speaker change: `OverShoulder` onto the speaker
  (shot/reverse-shot); same speaker past `shot.max_s`: alternate `OverShoulder`/`Close`; heads over
  `shot.ots_max_sep_m` apart: `Close`; the player's echoed choice: reverse shot on the player.
- **Placement:** `Two`/`OverShoulder` by toric angles, up to `shot.max_candidates` yaw/pitch variants on the kept
  side; `Close` on the addressee's eye ray at `shot.fov_close_deg`, never nearer than `shot.min_subject_dist_m`.
  **Valid:** `free(eye, shot.eye_radius_m)`, `clear` to each framed head, outside body capsules. **Score:** toric
  distance, continuity with `prev`, headroom.
- **Cuts** only when the view turns ≥ `shot.cut_min_angle_deg`, else ease (`shot.follow_halflife_s`); none before
  `shot.min_s` unless invalid. Re-check every `shot.recheck_s` (one ray); occluded past `shot.occluded_grace_s`:
  re-solve.
- **Fallbacks:** other candidates -> other kinds, same side -> `Close` on the addressee's eye ray (people who see each
  other keep it clear) -> `Eyes` (the player's view eased onto the speaker: Classic framing, always valid). Speaker
  beyond `shot.max_range_m`: `Eyes`.
- **First person,** `camera = Eyes` (default): only `Eyes`, look eased (`fp.look_halflife_s`) onto the speaker's eyes,
  FOV narrowed by `fp.zoom_deg`, look input within `fp.look_limit_deg`. `Shots`: third-person grammar (KCD2) with
  the world body (camera.md "Render bodies").

### Camera takeover (camera.md "Framed cameras"; DQ-28a)

```rust
pub struct CameraTakeover { pub pose: ShotPose, pub cut: bool, pub blend_in_s: f32, pub blend_out_s: f32,
                            pub hand_back_look: Option<Look> }   // sf_camera_bevy, on the PlayerCamera entity
```

The rig runs as usual, then blends to `pose` (smoothstep over `shot.enter_blend_s`; a cut when the blend midpoint
fails `free`); `cut` snaps. Removal blends back over `shot.exit_blend_s` and writes `hand_back_look` into the look
accumulator (as `recentre_behind` does). `Command.view` never reads it. Switch: `camera` (Off = Classic framing),
not `auto_camera`.

### Gaze and talk (`sf_dialogue_stage::talk`)

```rust
pub enum Gaze { At(u32), Avert { at: u32, yaw_deg: f32, pitch_deg: f32 } }
pub fn gaze(plan: &mut GazePlan, speaking: Option<u32>, me: u32, t: f32, p: &GazeParams) -> Gaze;
pub struct TalkSignal { pub open: f32, pub nod: f32, pub gesture: Option<GestureCue> }
pub enum TalkSource<'a> { Text { text: &'a str }, Visemes(&'a VisemeTrack) }
pub fn talk(src: &TalkSource, t: f32, p: &TalkParams) -> TalkSignal;      // pure, seeded by entry id
pub trait TalkRig { fn apply(&mut self, who: ActorKey, s: &TalkSignal); }   // adapter
```

- **Gaze** (to `AnimDrive.look_target`): the speaker at the addressee's eyes, listeners and the player's body at the
  speaker; the speaker averts at phrase starts (Cassell et al. 1994; Lee, Badler, Badler, "Eyes Alive", SIGGRAPH 2002)
  every `gaze.avert_every_s` for `gaze.avert_s`, up to `gaze.avert_deg`.
- **Text lines:** speech lasts chars / `talk.speech_cps` (capped by the page); `open` follows a syllable rhythm at
  `talk.syllable_hz` shaped by letters (vowels open, `m b p` close, punctuation pauses), peak `talk.jaw_open`; nods
  of `talk.nod_deg` on sentence starts; a gesture at most every `talk.gesture_min_gap_s`, chance
  `talk.gesture_chance`. Ends closed. **Voiced:** `open` from the adapter's viseme table, cross-faded over
  `viseme.blend_s`; no track: the adapter's path (Morrowind: loudness). No bone or morph names (`TalkRig`).
- **Needed from the anim lane** (their design): a per-character `look_target` writer with a weight (AN8b), an
  additive head/neck rotation input for nods, a one-shot upper-body slot for `talk.gesture.*` clips (none: no
  gestures). All presentation, skippable past LOD A0.

### Interrupts and resume (`sf_rpg`)

```rust
pub enum InterruptReason { Combat, Damage, Crime, Left }   // RpgCommand::Interrupt { speaker, reason }
pub struct ResumeRecord { pub speaker: ActorKey, pub player: PlayerId, pub topic: Option<RecordId>,
                          pub history: Vec<ShownLine>, pub ended_tick: u64 }
```

- **Triggers** (game glue, host): combat involving a participant or within `interrupt.combat_radius_m`; damage to
  either; an offence the speaker witnesses or suffers; `Left`: apart past `interrupt.leave_range_m`, dead, gone.
- **Effect** (one tick): session `Ended`, claim released, `RpgEvent::Interrupted`; except for `Left`, a `BarkCue` on
  `talk.interrupted` with `event.reason` (urgency `Reaction`). A fight started by the speaker's own last result
  script lets that line finish as a bark subtitle, no break-off.
- **Resume:** topics, heard lines, journal are world state. A `ResumeRecord` (`resume.history_lines`) lives
  `resume.window_s`: reopening shows it greyed, the greeting query gets `event.resumed = 1`. Not saved.
- **Timed choices:** `SessionState::Choosing` gains `deadline: Option<Deadline { at_tick: u64, default: i32 }>`
  (content, `offer_choices_timed`); on `at_tick` the host applies `Choose(default)`; clients draw the bar from the
  replicated host tick. Quick replies (DQ-39) reuse `Deadline`.

### Viseme bake

`VisemeTrack { cues: Vec<(u32 /* ms */, Viseme)> }`, `Viseme = A..H | X` (Rhubarb's shapes); one postcard bake file
keyed by voice path + content hash; `read_rhubarb_tsv(&str)`. The bake decodes each voiced line to WAV, runs Rhubarb
with the text as `--dialogFile` (PocketSphinx for English, else phonetic), in parallel, skipping unchanged hashes;
no Rhubarb installed: log and skip. The voice player looks the track up by path.

### Co-op, saves, accessibility

- **Co-op (decision 9):** the claimant (`Conversations::claim`, wired at open) drives: panel, requests on the host
  tick. Players within `listen_range_m` of the speaker get the same `Line`s in a `Barks` presenter as `Listener`,
  camera free. Shots, pages, gaze are per client; the host never waits for pages.
- **Saves:** manual and quick saves refused while a session is open, autosaves wait (BG3, Witcher 3 never save in
  dialogue): the save gate (save.md:125-128) asks `sf_rpg`. Nothing presentational is saved.
- **Accessibility (XAG 104):** subtitle scale to 200 %, background opacity 0-100 %, sans-serif mixed case, names,
  reading-speed factor, auto-advance, hold-to-skip, `camera = Off` (no motion), Classic as a setting.

## Tunables

Prefix `dialogue.`; defaults by base-mod record, per preset where marked. Net: **P** presentation (client only),
**G** gameplay (host).

| Name | Default | Range | Unit | Net | Source |
|---|---|---|---|---|---|
| `presenter` (preset, setting) | Cinematic | Classic/Cinematic | enum | P | Owner(2026-11-25, decision 5) |
| `camera` (setting) | Shots 3rd / Eyes 1st | Shots/Eyes/Off | enum | P | Reference(camera.md:79-80; Cyberpunk, KCD2) |
| `auto_advance` (setting) | true | bool | none | P | Owner(decision 5) |
| `read.cps` | 17 | 8-40 | chars/s | P | Reference(Netflix ≤ 20 English adults, 17 most languages) |
| `read.base_s` | 0.5 | 0-2 | s | P | Guess |
| `read.min_page_s` / `read.max_page_s` | 0.833 / 7 | 0.3-3 / 3-20 | s | P | Reference(Netflix 5/6 s, 7 s) |
| `read.voice_tail_s` / `skip_lockout_s` / `hold_skip_s` | 0.3 / 0.25 / 0.4 | 0-2 | s | P | Guess |
| `subtitle.max_line_chars` | 42 | 20-80 | chars | P | Reference(Netflix 42; XAG 104 ≤ 40) |
| `subtitle.max_lines` | 2 | 1-4 | count | P | Reference(XAG 104, Netflix) |
| `subtitle.scale` (setting) | 1 | 1-2 | factor | P | Reference(XAG 104 200 %) |
| `subtitle.bg_opacity` (setting) | 0.6 | 0-1 | fraction | P | Reference(XAG 104 range); default Guess |
| `shot.min_s` / `shot.max_s` | 1.5 / 6 | 0.5-5 / 2-20 | s | P | Guess (research note: 3-5 s shots) |
| `shot.cut_min_angle_deg` | 30 | 15-60 | deg | P | Reference(30-degree rule) |
| `shot.thirds_x` | 0.333 | 0.2-0.45 | screen fraction | P | Reference(rule of thirds) |
| `shot.fov_two_deg` / `_ots_` / `_close_` | 45 / 35 / 25 | 10-80 | deg, vertical | P | Guess |
| `shot.ots_max_sep_m` | 4 | 1-10 | m | P | Guess |
| `shot.min_subject_dist_m` | 0.6 | 0.3-2 | m | P | Guess |
| `shot.eye_radius_m` | 0.15 | 0.05-0.5 | m | P | Guess (as the camera arm sphere) |
| `shot.max_candidates` | 8 | 1-32 | count | P | Guess |
| `shot.recheck_s` / `occluded_grace_s` / `follow_halflife_s` | 0.25 / 0.5 / 0.25 | 0.02-3 | s | P | Guess |
| `shot.enter_blend_s` | 0.6 | 0-2 | s | P | Guess |
| `shot.exit_blend_s` | 1 | 0-3 | s | P | Reference(camera.md:79, ~1-2 s) |
| `shot.max_range_m` | 8 | 2-30 | m | P | Guess |
| `fp.zoom_deg` / `fp.look_limit_deg` | 8 / 25 | 0-20 / 0-90 | deg | P | Guess (Starfield-style zoom) |
| `fp.look_halflife_s` | 0.2 | 0.02-1 | s | P | Guess |
| `gaze.avert_every_s` / `gaze.avert_s` | 4 / 0.6 | 1-20 / 0.1-3 | s | P | Guess (Cassell 1994; Lee 2002) |
| `gaze.avert_deg` | 12 | 0-30 | deg | P | Guess |
| `talk.speech_cps` | 14 | 5-25 | chars/s | P | Reference(≈ 150 wpm conversational English) |
| `talk.syllable_hz` | 4.5 | 2-8 | Hz | P | Reference(English ≈ 4-5 syllables/s) |
| `talk.jaw_open` / `talk.gesture_chance` | 0.6 / 0.5 | 0-1 | fraction | P | Guess |
| `talk.nod_deg` | 4 | 0-15 | deg | P | Guess |
| `talk.gesture_min_gap_s` | 3 | 0.5-20 | s | P | Guess |
| `viseme.blend_s` | 0.06 | 0-0.2 | s | P | Guess |
| `interrupt.combat_radius_m` / `leave_range_m` | 15 / 6 | 0-60 / 2-30 | m | G | Guess |
| `resume.window_s` | 600 | 0-7200 | s, host time | G | Guess |
| `resume.history_lines` | 8 | 0-64 | count | G | Guess |
| `listen_range_m` | 12 | 2-40 | m | G | Guess |

`const`: shot kinds and fallback order, the viseme alphabet, page-break classes. `Deadline` values are content data.
Every look Guess is a live tunable judged in DJ-8/DJ-9, never a question.

## Presets and the classic seam

- **Classic** (Morrowind Classic): `Classic` presenter, camera untouched, talk from loudness (as built); interrupts
  still end sessions (the world runs during dialogue, as today), a break-off line only if the pack has one.
- **Modern** (framework, Modern Morrowind): `Cinematic`, shots, gaze, procedural talk, visemes, break-off barks.
- One session and one view-model under both; the presenter is the only switch.

## Performance

A cut: at most `max_candidates` x 3 = 24 rapier queries (~2-5 µs each, ~0.1 ms), a few times a minute; between cuts one
ray per `recheck_s`. Pager, gaze, talk: a few hundred float ops for 2-4 characters. Expected < 0.05 ms per frame,
0.15 ms on a cut: negligible against 8.3 ms. DQ-28c's test prints queries per cut.

## Tests

- **Exact:** page splits, page time, voiced length, tap/hold/lockout, auto-advance off, panel timing; Classic =
  today's requests for a scripted input; `Interrupt` ends, releases, cues `talk.interrupted` (none for a self-started
  fight or `Left`); resume window and cap; `Deadline` fires on `at_tick`, not before; `Listener` silent; save gate;
  Rhubarb TSV; talk deterministic, ends closed.
- **Sanity:** solver on synthetic people and random boxes: no NaN, eye never in a box or capsule, side kept over 50
  speaker changes, every cut turns ≥ `cut_min_angle_deg`, all-occluded ends valid; on `sf_level/tests/levels/camera.ron`
  the eye sphere is free every frame through entry, cuts and exit. No automated visual checks.

## Verification (owner visual checks)

- **DJ-8 cinematic conversation:** `view "Seyda Neen, Census and Excise Office"`: Sellus Gravius (small room), then
  Fargoth outside; third person, then first. (1) Speaker framed, topics beside, never crossing the line? (2) Cuts
  comfortable, never inside a wall or body? (3) Pages at a good pace, advance and hold-skip responsive? (4) Does a
  clicked subtitle keyword ask it? (5) Smooth hand-back on leaving?
- **DJ-9 gaze and talk:** same scene, a long text line. (1) Does the NPC look at you, glancing away now and then?
  (2) Does the mouth move while it speaks and close after? (3) Are nods and gestures natural?
- **DJ-10 visemes:** a voiced greeting with and without its track (live switch). (1) Do the lips close on m, b, p?
  (2) Is it better than the loudness mouth?

## Slices

| ID | Deliverable + test | Crates | Depends | Verify |
|---|---|---|---|---|
| DQ-27 | Presenter trait; Classic = today's window behind it. Unit: same requests | `sf_rpg_ui`, `mw_rpg_bevy` | DQ-26 | `cargo test -p sf_rpg_ui`; DJ-1 |
| DQ-28a | Camera takeover (blend, cut, look hand-back). Headless: no NaN, eye free | `sf_camera(_bevy)` | DQ-27 | `cargo test -p sf_camera_bevy` |
| DQ-28b | Shot solver, pure. Sanity suite | new `sf_dialogue_stage` | DQ-28a | `cargo test -p sf_dialogue_stage` |
| DQ-28c | Shell: participants, rapier `Occlusion`, writes the takeover. camera.ron | new `sf_dialogue_bevy` | DQ-28b | `cargo test -p sf_dialogue_bevy` |
| DQ-29a | Cinematic + Barks logic: pager, pacing, panel model. Exact | `sf_rpg_ui` | DQ-27 | `cargo test -p sf_rpg_ui` |
| DQ-29b | Subtitle, panel, deadline widgets; theme; Modern default | `sf_ui_screens`, `mw_ui`, `mw_rpg_bevy` | DQ-29a, DQ-28c | `cargo test -p sf_ui_screens`; DJ-8 |
| DQ-30a | Gaze plan + talk signal, pure. Exact | `sf_dialogue_stage` | DQ-28b | `cargo test -p sf_dialogue_stage` |
| DQ-30b | `look_target`, nods, gestures, `TalkRig` (Morrowind `Talking`; drop the `Y` scaffold) | `sf_dialogue_bevy`, `mw_actors_bevy` | DQ-30a, anim lane | `cargo test -p sf_dialogue_bevy`; DJ-9 |
| DQ-31a | `Interrupt`, resume records, `talk.interrupted` cue, save gate. Exact | `sf_rpg` | DQ-25a | `cargo test -p sf_rpg` |
| DQ-31b | `Deadline` on `Choosing`. Exact | `sf_rpg` | DQ-31a | `cargo test -p sf_rpg` |
| DQ-31c | Morrowind triggers + wire the claim. Headless | `mw_rpg_bevy` | DQ-31a, DQ-12 | `cargo test -p mw_rpg_bevy` |
| DQ-32a | `VisemeTrack`, Rhubarb TSV reader, talk from visemes. Exact | `sf_dialogue_stage` | DQ-30a | `cargo test -p sf_dialogue_stage` |
| DQ-32b | Bake: decode, Rhubarb, cache by hash; log counts and seconds | `mw_bake` | DQ-32a | `cargo test -p mw_bake`; DJ-10 |

## Open questions (what the player sees or can do)

None open. Answered (owner via meta-5, 2026-11-26), each as recommended:

1. **First person:** a first-person player stays in the eyes by default (Cyberpunk, Starfield); third-person shots
   (KCD2) are a setting (camera.md:79-80).
2. **Long Morrowind lines:** paged two-line subtitles (standard), not a taller box beside the NPC.
3. **Walking away** mid-conversation (Cyberpunk): not now, controls stay paused; revisit with walk-and-talk.
