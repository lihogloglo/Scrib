# Animation: motion style (gait classes and modifiers)

Status: spec (AN12, 2026-11-26); nothing built. Crates: `sf_anim_graph` (Bevy-free: profile, table, chooser, class
remap, `Posture` and `Limp` ops), `sf_anim_bevy` (thin glue); game side `mw_graphs` (data), `mw_walkers_bevy`
(adapter, walkers). Parent spec: [animation.md](animation.md) ("Motion style" points here).

<!-- agent-summary -->
> **Agent summary (2026-11-26).** Why an old, heavy, drunk, wounded or tired character moves differently.
> A per-individual `MotionProfile` (host gameplay data, like `PerceptionProfile`) picks a **gait class** (a clip
> set: data rows `loco.<class>.<gait>.<part>`, compiled into a per-graph clip remap, no code per class) by a
> data chooser with hysteresis, and drives **continuous modifiers** (`Posture`: hunch, slump, load, sway,
> breathing; `Limp`: hip drop, trunk lean, short stance), exactly identity at weight 0. Missing class clips fall
> back to the identity class, then the base set. LOD: A0 all, A1 class + bone ops, A2 class + speed match, A3 none;
> style share ≤ 3 µs of the 25 µs A0 budget. Classic: no style, pinned digest unchanged. Slices AN14a-AN23 at the end.
<!-- /agent-summary -->

## Goal

In a Balmora street no two people walk alike: an old matriarch walks slower and stooped, a heavy guard rolls
his weight, a drunk sways and drifts, a wounded NPC or companion limps on one leg and recovers as health returns,
a tired or overloaded character (the player too) slumps and slows. Identity reads at 50 m (the class clip),
detail up close (modifiers). Classes keep coming in data batches (owner: "a LOT"); none needs code.

## Today (verified 2026-11-26)

- Style seam: `SourcesManifest::gait_slot` looks up `loco.<style>.<gait>.<part>`, then `loco.<gait>.<part>`
  (`bevyrrowind/crates/mw_graphs/src/sources.rs:316`; the wiki's `:294` drifted), called with `gaits::STYLE =
  None` (`mw_graphs/src/build.rs:177`, `:824`): base set only. Base slots: `sources.ron:156-183`.
- Graph clips are slot names, compiled into one clip list shared by a graph's characters (`AnimAsset`,
  `sf_anim_bevy/src/lib.rs:123`; `Binding::new`, `sf_anim_graph/src/instance.rs:71`). Params are `Float`, `Bool`,
  `Vec2`, `Trigger` (`asset.rs:76`): a class cannot be a graph param.
- Ops (`OpSpec`, `asset.rs:793`) have `max_lod` and `gate`; `Lean` (`:917`) is the pattern; joints by humanoid bone
  (`JointRef::Rig`, `:188`). `inertialize_from` (`instance.rs:625`); sync-group join (`:1489-1503`); `Variety`
  seed (`variety.rs:1-20`); cook mirror `#mirror` (`mw_graphs/src/sources.rs:148-169`); `sf_anim::rig::Side`.
- Classic: the asset's Debug-text digest is pinned (`mw_graphs/src/tests.rs:256-284`); `GRAPH_FORMAT` 16
  (`compiled.rs:626`). Ledger: nothing on runtime style yet; source posture is fixed at the retarget
  (`PoseOffset`, 2026-11-12); AN13: MotionBricks style presets are key poses.

## Reuse check (rule 0)

1. **Bevy 0.20:** `AnimationGraph` `Clip`/`Blend`/`Add` nodes, bit masks, crossfades
   (`docs/wiki/bevy/skinning-and-animation.md`); no per-character clip override, inertialization or op hook. Our
   runtime replaced it for characters (animation.md "Why not Bevy's animation system", settled 2026-10-11).
2. **Crates** (2026-11-26): `bevy_animation_graph` (MIT/Apache, no style layer, lags Bevy), `bevy_dynanim` (one
   0.1 release, "not production ready"). None has a class chooser, clip-set remap or gait modifiers.
3. **Our own:** small pieces on structures we own (a compile-time table, one indirection per clip, two ops).
   Clips are made offline (scrib-motiongen), never at runtime.

## Industry comparison

**Canonical technique:** a handful of authored locomotion clip sets per gait class, swapped per character by a
data table (clipset / chooser), plus cheap additive or procedural modifiers driven by continuous scalars; physics
only for rare strong events.

| Them | Us | Gap |
|---|---|---|
| GTA V: `SET_PED_MOVEMENT_CLIPSET(ped, "move_m@drunk@verydrunk", blend)`; scripts set `move_m@injured` under a health threshold, reset on recovery; ped personalities assign sets (LCPDFR native DB; cfx.re limp-walk thread). RDR2 alike [I] | class remap, chooser rows, fallback to identity then base | none |
| TLOU Part II: injured sets by health band, partial-body additive layers [I] | `limp_*` classes by health band + ops | injured starts/stops wait for batches |
| UE5: Chooser Table picks assets from variable columns (Epic "Dynamic asset selection"); Lyra swaps locomotion sets by Linked Anim Layers | `rules` = chooser rows; remap = layer swap, no graph per class | no per-class graph logic (deliberate) |
| AC Unity: crowd archetypes, 40 AIs, 120 high-res, ~10,000 crowd (Cournoyer, GDC 2015) | identity class (`style_id`) reads at A2 | VAT crowds later |
| Cyberpunk 2077: mood walk sets (calm, worried, sad, angry, confident; a mod restores unused ones); crowd instancing, no update when occluded past 5 m (GDC 2024 "Anatomy of a frame") | `mood` field; mood classes in batches; A3 frozen | class streaming later |
| Uncharted 4 / TLOU: additive breathing by exertion [I] | `Posture` breathing, procedural | none |
| Limp: antalgic gait (short stance, lean over the painful leg), Trendelenburg pelvic drop (Perry, *Gait Analysis*); semi-procedural locomotion (Johansen 2009; Rosen, GDC 2014) | `Limp` op; feet held by foot IK | none |
| Euphoria (GTA IV, RDR2): stagger, drunk stumbles; authored baseline gait | physics tint later (AN39) | by design |

**Not built:** per-class graphs or blueprints, motion matching per style (parked, decision 11), learned style
models, runtime generation, datasets or packs (decision 1), per-style interaction copies (modifiers instead).

## Design

### Profile (per individual, host gameplay state)

Mirrors `PerceptionProfile` (ai.md "Perception profile"): the framework knows fields, never a stat or a race.

```rust
// sf_anim_graph::style
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MotionProfile {
    pub style_id: ClassName,                 // identity class; saved; default "neutral"
    #[serde(default)] pub age: f32,          // 0 young adult .. 1 very old; saved; not smoothed
    #[serde(default)] pub injury: f32,       // 0..1; this and below smoothed, derived
    #[serde(default)] pub injury_side: Side, // sf_anim::rig::Side; Center = none
    #[serde(default)] pub fatigue: f32,      // 0..1
    #[serde(default)] pub load: f32,         // 0..1 (1 = at capacity)
    #[serde(default)] pub intoxication: f32, // 0..1
    #[serde(default)] pub mood: f32,         // -1 dejected/afraid .. +1 proud
}
pub struct MotionTargets { pub injury: f32, pub injury_side: Side, pub fatigue: f32, pub load: f32,
                           pub intoxication: f32, pub mood: f32 }   // the adapter writes these
pub struct ClassName(pub Arc<str>);   // stable text id (saves, mods)
pub struct ClassId(pub u16);          // index into the loaded table and the compiled remap
impl MotionProfile {
    /// One host tick: x += (t - x)(1 - e^(-dt/tau)), tau = the field's rise or fall time. Tick-based.
    pub fn step(&mut self, t: &MotionTargets, dt: f32, rates: &SmoothRates);
    pub fn snap(&mut self, t: &MotionTargets);   // spawn and load: no ramp
}
```

Values are clamped at load and write; no profile = neutral at zeros (today's look).

### Class table (data, `animation/styles.ron`, through `sf_vfs`/`sf_mods`)

A mod adds a class with one row here plus its slot rows in the sources manifest (rows layer by class name). A
row declares slot prefix, fallback, gait speeds, numeric bands (decision 6; read by `motiongen grade`, AN17, not
the runtime) and modifier scales (0-2, default 1).

```ron
(
  base_prefix: "loco.",            // the base set's slot prefix; a class prefix replaces it
  default_class: "neutral",
  classes: {
    "neutral": (prefix: None),
    "old":   (prefix: Some("loco.old."), fallback: Some("neutral"), recipe: Some("old"),
              speeds: { "walk": 1.15, "jog": 2.6 },          // no sprint: the mover caps at jog
              bands: (speed_mps: { "walk": (0.9, 1.3) }, stride_m: (0.45, 0.65), spine_pitch_deg: (8, 22),
                      asymmetry: (0.0, 0.06)),
              modifiers: (hunch: 0.3)),                      // the clips already stoop
    "heavy": (prefix: Some("loco.heavy."), fallback: Some("neutral"), recipe: Some("heavy"),
              speeds: { "walk": 1.4, "jog": 3.8, "sprint": 5.8 }),
    "drunk": (prefix: Some("loco.drunk."), fallback: Identity, recipe: Some("drunk"),
              speeds: { "walk": 1.2, "jog": 3.2 }, modifiers: (sway: 0.3)),
    "limp_l": (prefix: Some("loco.limp_l."), fallback: Identity, recipe: Some("wounded"), side: Left,
              speeds: { "walk": 1.0, "jog": 2.2 }, modifiers: (limp: 0.4)),
    "limp_r": (prefix: Some("loco.limp_r."), fallback: Identity, side: Right,   // rows = mirrored candidates
              speeds: { "walk": 1.0, "jog": 2.2 }, modifiers: (limp: 0.4)),
  },
  rules: [                          // the chooser, first match wins (UE Chooser rows)
    (class: "drunk",  when: [(field: Intoxication, enter: 0.6, exit: 0.45)]),
    (class: "limp_l", when: [(field: Injury, enter: 0.5, exit: 0.35)], side: Some(Left)),
    (class: "limp_r", when: [(field: Injury, enter: 0.5, exit: 0.35)], side: Some(Right)),
    (class: "old",    when: [(field: Age, enter: 0.85, exit: 0.8)]),
  ],                                // no row matches: the profile's style_id, else default_class
)
```

```rust
pub struct StyleTable { pub base_prefix: String, pub default_class: ClassName,
                        pub classes: Vec<ClassRow>, pub rules: Vec<ChooserRule>, pub digest: u64 }
pub enum Fallback { None, Class(ClassName), Identity }   // Identity: the character's style_id class
pub struct ClassRow { pub name: ClassName, pub prefix: Option<String>, pub fallback: Fallback,
    pub recipe: Option<String>, pub side: Side, pub speeds: Vec<(String, f32)>,   // gait key -> m/s
    pub bands: Bands, pub modifiers: ModifierScales }
pub struct ChooserRule { pub class: ClassName, pub when: Vec<Cond>, pub side: Option<Side> }
pub struct Cond { pub field: Field, pub enter: f32, pub exit: Option<f32> } // default enter - hysteresis
impl StyleTable {
    pub fn parse(ron: &str) -> Result<Self, StyleError>;  // refuses unknown class, fallback cycle, enter < exit
    pub fn class(&self, name: &ClassName) -> Option<ClassId>;
    /// Class slot for base slot `loco.walk.cycle`: prefix, then the declared fallback chain; `None` = base.
    /// `has` asks the manifest (gait_slot's rule, generalized; mw_graphs::gait_slot then delegates).
    pub fn resolve(&self, class: ClassId, slot: &str, has: impl Fn(&str) -> bool) -> Option<String>;
    /// Mover speed (gameplay) for `gait`: the class's speed or `base_mps`, times speed_scale.
    pub fn gait_speed(&self, class: ClassId, gait: &str, base_mps: f32, p: &MotionProfile) -> Option<f32>;
}
```

`gait_speed` is `None` for a gait missing from a class's `speeds` (the mover caps at its fastest listed gait);
a class with no `speeds` keeps base speeds. A class with only walk cycles still works: the rest falls back.
`speed_scale = Π (1 - <field>_slow * smoothstep(start, 1, field))` over fatigue, load, injury; a game whose
movement rules already scale speed by a field sets that `*_slow` to 0 (never twice).

### Chooser (host fixed tick, gameplay)

```rust
pub struct StyleState { pub active: ClassId, pub identity: ClassId, pub since_tick: u64 }  // replicated
pub fn choose(table: &StyleTable, p: &MotionProfile, prev: &StyleState, tick: u64, hold_ticks: u64) -> StyleState;
```

Rows in order; a row holds while it is the active class and every condition stays at or above `exit`; another row
takes over only when all its conditions reach `enter` and the active class has been held `hold_s`. Side rows match
`injury_side`. Pure and deterministic: equal inputs and tick give equal output on every peer.

### Class remap (compile) and switch (runtime)

`animgraph.compile` takes the graph asset and, for the Modern preset, the `StyleTable`. For each class and each
graph clip whose reference starts with `base_prefix`, `resolve` gives a class slot; the compiler appends that clip
to the graph's clip list and records it. The remap lives only in the compiled graph, never in `GraphAsset`, so
the Classic asset (and its pinned digest) cannot change.

```rust
// compiled.rs (GRAPH_FORMAT 17)
pub struct CompiledStyles { pub table_digest: u64, pub classes: Vec<CompiledClass> }
pub struct CompiledClass { pub name: ClassName, pub remap: Box<[u16]> /* base clip -> clip, u16::MAX none */,
                           pub at: Box<[(u32 /*node*/, u16 /*sample*/, f32 /*m/s*/)]> }  // BlendSpace1D speeds
// CompiledGraph gains `#[serde(default, skip_serializing_if = "CompiledStyles::is_empty")] styles`
// instance.rs
pub struct StyleSel { pub active: u16, pub identity: u16 }        // compiled class indices
impl GraphInstance {
    /// Next update samples clip i as active.remap[i], else identity.remap[i], else i; if any playing clip
    /// changes, the update inertializes over `blend_s` from the last pose (as a slot switch does).
    pub fn set_style(&mut self, b: &Binding<'_>, sel: StyleSel, blend_s: f32);
}
```

- **Blend:** inertialization (Bollo, GDC 2016; UE5), not a cross-fade: one pose sampled during the switch. With
  `switch.at_marker` it waits up to `marker_wait_s` for the next `down.*` marker (UE sync-marker transitions).
- **Sync:** state and blend space stay, so players keep their marker phase (`join`), else normalized time. Class
  lanes sit at the class `at` speeds, so `SpeedMatch` stays inside its 0.8-1.25 clamp.
- **Stale bake:** table digest ≠ `table_digest`: one warning, base set plays (the chooser still runs).
- **Capacity:** all class clips load with the graph; per-class streaming is a later slice if memory demands.

### Modifiers (graph ops, Modern humanoid only)

Weights come from one pure function on the presentation side: `weights(p: &MotionProfile, row: &ClassRow,
t: &StyleTunables) -> StyleWeights { hunch, slump, chest_up, load, sway, breathing, limp, side }`, each
`smoothstep(start, full, field) * row.modifiers.<channel>`; `slump = max(fatigue part, mood_gain * -mood)`,
`chest_up = max(0, mood)`. The glue writes them as Float params (`style.hunch` ...) every frame; ops read
`ValueSpec` params, so a game wires names in its graph.

```rust
OpSpec::Posture {                       // after FootPlacement, before Lean (Lean's counter keeps it)
    spine: Vec<(JointRef, f32)>,        // Rig spine, chest, upperChest shares of the pitch
    neck_head: Vec<(JointRef, f32)>,    // counter: head back toward level by hunch_head_counter
    clavicles: [JointRef; 2], upper_arms: [JointRef; 2],
    hunch: ValueSpec, slump: ValueSpec, chest_up: ValueSpec, load: ValueSpec,
    sway: ValueSpec, breathing: ValueSpec,
    #[serde(default)] halflife: f32,    // channel springs
    #[serde(default = "lod1")] max_lod: u8,    // breathing channel runs at A0 only
}
OpSpec::Limp {                          // after StrideWarp, before FootPlacement (feet re-solve to plants)
    pelvis: JointRef, trunk: Vec<(JointRef, f32)>, feet: [String; 2] /* leg names */,
    weight: ValueSpec, side: ValueSpec, // side param: -1 left, +1 right
    #[serde(default = "lod1")] max_lod: u8,
}
```

- **Hunch** (age): spine pitch up to `hunch.max_deg` by shares, neck and head counter-pitch (gaze forward).
  **Slump** (fatigue, low mood): clavicles down and forward, chest pitch; **chest_up** the reverse. **Load**:
  trunk pitch, upper-arm swing damped toward rest (`HandIk` after it still wins). **Sway** (drunk): trunk roll
  and yaw at `sway.hz`, `Variety` seed phase. **Breathing**: chest and clavicle sine, rate and depth by fatigue.
- **Limp** (injured leg = `side`): during its stance (contact curve > 0.5) the pelvis drops on the swing side
  (Trendelenburg) and the trunk leans over the injured leg (antalgic). **Short stance, asymmetric timing:** the op
  hands its sync group a phase-rate multiplier for the next advance: `1 + k` in injured stance, `1 - k'` elsewhere,
  mean exactly 1 per cycle, so cadence and capsule speed are unchanged; `StrideWarp` reads the instantaneous rate
  and `FootPlacement` holds the plant, so no added foot slide (tested).
- **Identity at weight 0:** each op returns the pose bit-equal when its weights and springs are 0; `Limp` skips
  the rate multiplier. **Never fixes a source:** source posture stays in the retarget (`PoseOffset`).
- **Interactions** keep shared clips (decision 7): `Posture` runs over them, `speed_scale` reaches their rate.

### LOD and budget (decision 5)

| Tier | Class remap | `Posture` | `Limp` pose + timing | Breathing | Style cost (Guess, AN23 measures) |
|---|---|---|---|---|---|
| A0 | yes | all channels | yes | yes | ≤ 3 µs of the 25 µs character |
| A1 | yes | hunch, slump, load, sway | yes | no | ≤ 2.5 µs |
| A2 | yes | no | no (no stride warp at A2: timing would slide) | no | ≤ 0.2 µs (one indirection per clip) |
| A3 | state kept, pose frozen | no | no | no | 0 (chooser on the host tick only) |

Ops past `max_lod` fade their weight to 0 over `lod_fade_s`, then stop (as the foot ops do); coming back into
range they fade in. The chooser costs ≤ 0.2 µs per actor per host tick, outside the frame's animation stage.

### Physics tint (attach point only; AN39 specifies it)

A `PhysicsTint` (stagger, stumble, collapse; Delirium fills it, decision 10) is a host event triggered from the
profile (intoxication, injury, fatigue at 0) and medium hits; it attaches as the existing physics share
(`GraphInstance::set_physics`), after `Posture`/`Limp`, A0/A1 only, capped, additive fallback over the cap and
in Classic. The baseline limp is never physics.

### Net, saves, split

- **Net:** profile and `StyleState` are host gameplay state on the fixed tick, replicated on change (`u16` ids,
  `u8` scalars); `gait_speed` feeds the mover from them, never from the pose. Weights, springs and pose are
  presentation, per peer. A remote player is another profile.
- **Saves:** `style_id`, `age`. Derived on load: targets (adapter), smoothed fields (`snap`), `StyleState`.
- **Split:** `superfargoth/` names no race, stat or record: the game gives targets (adapter, AN21), class rows
  and slot rows (manifest). Logic in `sf_anim_graph`; `sf_anim_bevy` adds components, weight params, `set_style`.

## Tunables (`anim.style.*`; framework, net `Presentation` unless noted)

Unit = name suffix (`_s`, `_deg`, `_hz`, `_bpm` per minute); others ratio 0-1 unless a range says otherwise.
Source `Guess` unless noted. `[G]` = net `Gameplay` (host).

| Names | Defaults | Ranges |
|---|---|---|
| `smooth.injury_rise_s` / `injury_fall_s` (recovery) | 0.5 / 20 | 0.05-10 / 1-600 |
| `smooth.fatigue_s` / `load_s` / `mood_s` / `intox_rise_s` / `intox_fall_s` | 3 / 1 / 2 / 5 / 60 | 0.1-600 |
| `choose.hold_s` [G] / `choose.hysteresis` [G] | 2 / 0.1 | 0-30 / 0-0.5 |
| `switch.blend_s` (Industry: UE5 inertialization 0.2-0.5) | 0.35 | 0.05-1.5 |
| `switch.at_marker` / `switch.marker_wait_s` (Industry: UE sync markers) | true / 0.5 | bool / 0-2 |
| `lod.fade_s` / `posture.halflife_s` | 0.25 / 0.3 | 0-2 |
| `hunch.start` / `hunch.full` (age) / `hunch.max_deg` / `hunch.head_counter` | 0.6 / 1.0 / 25 / 0.8 | 0-1, 0-60 |
| `slump.start` (fatigue) / `slump.max_deg` / `slump.clavicle_deg` | 0.4 / 12 / 8 | 0-1, 0-40 |
| `mood.gain` / `chest_up.max_deg` | 0.5 / 6 | 0-2, 0-20 |
| `load.start` / `load.pitch_deg` / `load.arm_swing_damp` | 0.5 / 10 / 0.5 | 0-1, 0-30 |
| `sway.start` / `sway.amp_deg` / `sway.hz` | 0.3 / 6 / 0.35 | 0-1, 0-20, 0.05-2 |
| `breath.rest_bpm` (Reference: adult resting 12-20) / `tired_bpm` / `rest_deg` / `tired_deg` | 14 / 32 / 1.5 / 4 | 4-60, 0-10 |
| `limp.start` (injury) / `limp.hip_drop_deg` / `limp.lean_deg` / `limp.stance_short` | 0.25 / 6 / 8 / 0.25 | 0-1, 0-20, 0-0.5 |
| `speed.fatigue_slow` [G] / `load_slow` [G] / `injury_slow` [G] | 0.25 / 0.3 / 0.35 | 0-0.8 |

40 values (36 `Guess`); game values (`mw.style.*`) in animation-sources.md "Gait classes". Class rows' `speeds`, `bands` and `modifiers` are data, not tunables. A "how much" of a
look is one of these, live, plus a yes/no line in the owner check.

## Presets and the classic seam

- **Modern:** all of it; player per owner Q3: fatigue, load and drunk sway on, limp an option, off by default
  (`mw.style.player_limp`, animation-sources.md "Gait classes").
- **Classic:** no style: no profile, no style ops, no classes compiled; the remap is outside `GraphAsset`, so the
  pinned Classic digest (`mw_graphs/src/tests.rs:284`) must not change (AN14b asserts it).

## Performance

The whole A0 character stays ≤ 25 µs (animation.md "LOD and budget"). Remap: one array read per sampled clip;
`Posture` ~10 joint rotations on the model-space pose already held; `Limp` 3 joints and a rate. AN23 measures µs
per tier (High, 4060 laptop) into `docs/ledger/perf.md`.

## Tests

- **Exact:** parse refusals; `resolve` chain (class, fallback, identity, base); a walk-only class falls back for
  the rest; `gait_speed` caps; chooser thresholds, hysteresis, `hold_s`, side rows, same result at any tick rate;
  `step`/`snap`; remap and `at`; `set_style` swaps only changed clips; Classic digest; `Limp` side and phase (rate
  > 1 exactly in the injured contact window, mean 1). **Identity:** both ops bit-equal at weight 0.
- **Sanity:** at weight 1 no NaN, no joint past its limit, feet not through the floor; contact foot slide with
  `Limp` ≤ without + 0.5 cm/s. Tuned numbers only in `#[ignore = "measurement: ..."]`. No automated visual checks.

## Verification (owner visual checks)

- **ANIM-7 (class clips; AN20):** `level:` greybox scene `style-lineup` (AN20 registers it): one walker per
  class on a loop at walk and jog beside a neutral one, day, camera at 5 m then 30 m. Yes/no: (1) does each class
  read as what it is (old, heavy, drunk, limp left/right)? (2) are feet as planted as the neutral's? (3) does
  anything look cartoony? (4) is the mirrored limp the right leg?
- **ANIM-8 (styled walkers; AN22):** `view "Balmora"` 10:00, dev console sets a few NPCs wounded, drunk, tired.
  Yes/no: (1) varied walkers, nobody out of place? (2) a wounded NPC starts and stops limping without a pop?
  (3) at 30-90 m classes still read and nothing flickers at tier changes? (4) tired or loaded player: slump and
  slowdown acceptable? (5) drunk sway on the player acceptable?

## Slices (each: one deliverable + its test; verify `bash bevyrrowind/tools/cargo_turn.sh test -p <crate>`)

| ID | Deliverable | Crate | Depends | Test |
|---|---|---|---|---|
| AN14a | `style` module: `MotionProfile`, targets, `step`/`snap`, `StyleTable` parse + `resolve` + `choose` | `sf_anim_graph` | AN12 | exact lookup, fallback, thresholds |
| AN14b | Compiled class remap + `at` speeds + `GraphInstance::set_style` (inertialized, marker wait) | `sf_anim_graph` | AN14a | exact remap; walk-only class; Classic digest |
| AN15a | `Posture` op (hunch, slump, chest up, load, sway, breathing) + `weights()` | `sf_anim_graph` | AN14a | bit-equal at 0; sanity at 1 |
| AN15b | `gait_speed` / speed scale (class speeds, slow factors, caps) | `sf_anim_graph` | AN14a | exact |
| AN16 | `Limp` op: hip drop, lean, stance timing, side | `sf_anim_graph` | AN14a | exact side/phase; slide ≤ +0.5 cm/s |
| AN20 | `styles.ron` launch rows + picks into `loco.<class>.*` slots | `mw_graphs` data | AN19, MG9 | counts per class; owner ANIM-7 |
| AN21 | Adapter: `MotionTargets` + `style_id`/`age` from records; Classic none | `mw_walkers_bevy`, `mw_graphs` | AN14a | exact per record |
| AN22a | Glue: components, host-tick step + choose, weight params by tier, `set_style` | `sf_anim_bevy` | AN14b, AN15a, AN16 | exact tiers; counts |
| AN22b | Walkers and player carry the profile; mover reads `gait_speed` | `mw_walkers_bevy` | AN20, AN21, AN22a | counts; owner ANIM-8 |
| AN23 | Measurement: style µs per tier, High, 4060 laptop | tests | AN22b | `#[ignore = "measurement"]`; perf ledger |

## As built

- **AN14a (2026-11-26), `sf_anim_graph::style`:** as designed above, with these differences. `StyleTable::parse` is
  behind the crate's `bake` feature (`ron` is optional there); every type is postcard-serialisable, so the runtime
  either loads a cooked table or the glue turns the feature on (AN22a decides). `choose` takes
  `ChooseParams { hold_ticks, hysteresis }` in place of the bare `hold_ticks` (a condition without `exit` uses
  `enter - hysteresis`); every change of the active class waits for the hold, the identity updates at once.
  `StyleTable::initial(p, tick)` gives the state at spawn and load (first entering row, else the identity, no hold).
  `resolve` stops at a `Fallback::Identity` or at the end of the `Class` chain and returns `None`: the identity
  class is resolved by the caller (AN14b's two remaps). The `smooth.*` tunables are `Net::Gameplay`, not
  `Presentation`: the profile they smooth is host state. `injury_side` keeps its last side while the injury decays;
  `snap` with no injury sets `Center`. `gait_speed` is AN15b; `mw_graphs::gait_slot` does not delegate yet (AN14b).

## Open questions (taste and scope; each with a recommendation)

1. **Ambient drunks** (answered, owner via meta-5, 2026-11-26): not at launch. The first game's NPCs never drink, so
   drunks come only from the player or a script; tavern patrons drunk in the evening are a later data rule after AN22.
2. **Female neutral** (answered, owner via meta-5, 2026-11-26): yes, the AN18 batch is every woman's identity class
   once it passes clip review; old, heavy and state classes stay shared until female variants pass.
3. (Decided, owner 2026-11-25: nobody is "very old" except chosen NPCs by override; how age shows is low priority.)
   So only overridden NPCs reach age ≥ 0.85 (`old`); the rest stay below the hunch start at launch.
