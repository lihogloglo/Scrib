# Interaction (a): moving and carrying (`sf_interaction`)

Status: design 2026-11-11, updated 2026-11-25 (IN27; moved out of [interaction.md](interaction.md) for size). Part
(a) of the interaction spec: its shared core (defs, claims, hand claims, events, tick and net rules, the
`interact.carry.*` tunables), "Budgets" and "Build slices" (IN35, IN6, IN9, IN16, IN36) stay there. Lane roadmap:
[interaction](../../../docs/roadmaps/interaction.md) decisions 4 and 9. Research:
[gameplay-2026-11-25/interaction](../../../docs/research/gameplay-2026-11-25/interaction.md) §2, §6.

## (a) Moving and carrying (player and NPC)

**One link, two presentations** (decision 4; RDR2/TLOU attach + clip, Skyrim/HL2 spring; research §2). This
**departs** from the 2026-11-11 spec, where the player used the physics spring only and `HeldProp` was NPC-only: a
spring-held object floating beside a visible third-person body looks wrong, and one link type keeps saves, net and AI
reactions single.

```rust
// Illustrative, not compiled.
pub struct HeldProp { pub holder: CharacterId, pub prop: PlacementId, pub grips: [Option<GripRef>; 2],
                      pub class: RecordId /* CarryClass */ }
pub struct CarryDef { pub grips: SmallVec<[Grip; 2]>, pub class: RecordId, pub relocatable: bool,
                      pub pairs_with: TagFilter /* e.g. a table edge, for Relocate */ }
pub struct CarryClass { pub id: RecordId, pub clips: ClipFamily /* carry.one_hand, carry.two_hand, carry.shoulder, drag.chair */,
                        pub hands: u8, pub drag: bool, pub speed_mul: f32 /* ratio 0.2-1.0 */,
                        pub can_sprint: bool, pub can_roll: bool }   // `sf.interact.carry_class` records, mod data
```

- **One thing held** per holder (owner 2026-11-25): a carry verb while holding is not offered; the hold list shows
  drop and throw. Small items still go to the inventory as today.
- **Third person** (player and every NPC): the prop becomes a kinematic body attached to the holder's hand socket(s)
  (the `Weapon Bone`-style attach of first-person.md "Held" and MG7a), hands on the def's grip points by IK, collision
  against the holder filtered (contact-pair filter, physics.md), the class's carry clip set. Throw: the class's throw
  clip releases the prop at its contact event with the built throw impulse (physics.md "Carry and throw").
- **First person** (player): the built spring stays (`sf_physics` `carry.rs`: soft 6-DOF joint from a kinematic
  hand, force cap, drag when too heavy, throw impulse). Switching camera keeps the link; the prop blends to the socket
  or the spring target over `interact.carry.present_blend_s`.
- **Mass and class gates**: `HolderLimits` (from strength, the adapter's) decide carry vs drag vs refuse for player
  and NPC alike; a dragged prop stays a dynamic body on the built drag joint in both views, the third-person view adds
  the drag clip and hand IK. While holding, the shell feeds the class's `speed_mul`, `can_sprint` and `can_roll` into
  the holder's locomotion and action gates (sprint and roll moves fail validation).
- **Furniture is grabbable** when its def has `carry`: a static in the static scene is **promoted** first (static-scene
  §1: one flag write, an entity spawned with the same placement id), then gets a dynamic body. Heavy pieces drag
  (mass > `carry_mass`), light ones carry; its slots follow the body and refresh on sleep.
- **Moved furniture persists** through `sf/overlay` moved placements (save.md; the physics blob already stores item
  poses). Ownership: moving is not theft (physics-adapter.md), taking is.

**NPC.** A simulated grab (joint + motors) for every NPC is costly and unpredictable; the industry attaches, so NPCs
use the third-person `HeldProp` above (mass gate: the NPC's `HolderLimits`).
- **Chair pull-out** (folded into IN9, owner 2026-11-11): a slot entrance may carry `pre_move: Slide { axis,
  distance }`; the enter clip's first phase slides the chair kinematically along its axis (0.3-0.5 m, synced to the
  clip's hand contact; the distance is def data), the exit pushes it back. The chair body is kinematic during the
  slide, then rests; the new pose is saved only if it stays.
- **Relocate** (the owner's "NPC moves the chair to sit"): an HTN fragment `Relocate(obj, spot)` = `MoveTo(obj)` →
  `Pick` → `MoveTo(spot)` → `Place` (navmesh-validated floor spot, capsule-free, facing from the def's `pairs_with`
  tags, e.g. a table edge) → `UseSlot`. The AI picks it when no free slot scores above a threshold but a movable object
  with that advert is near (Sims-style). Placement drops the object dynamic onto the floor, it sleeps, the index updates.

**Persistence** (decision 9, IN15; detail and classes in [world-reactions.md](world-reactions.md) "Reset and
persistence"): a dynamic body is **saved at rest** (a body still moving at save time saves its last sleeping pose; one
that never slept since it spawned is placed on the floor below it), so no save holds an item mid-air; on stream-out
the **holder link wins** (a held prop travels with its holder across cells; an orphaned prop in flight is placed at
its holder's last position); charred props take the **`burnt`** persistence class.

Dependencies: static-scene promotion (built B5a, B5a-2 running); AI wired into the game (ROADMAP 3.10) and nav (NV
slices) for NPCs; carry clips (3.8 clip factory, MG7b grip table); hand IK (animation.md, the mantle lip IK path).
**Cost**: a held prop is one kinematic transform per tick; a moved furniture body sleeps within ~1 s (physics budget
unchanged). **Nav**: moved furniture is not carved by default (nav.md: ORCA and the mover avoid it); a record may opt
into carving (`carves_nav`, NV10). **Static scene**: a moved or destroyed scene placement must stay an entity on the
next cell load (the overlay must make it ineligible): to confirm with scene-coord (adapter eligibility hook, §1).
**Net**: the host simulates; a held prop replicates as its link (holder, grips, class), not a transform stream, for
both presentations; dynamic furniture replicates like any body. **Save**: moved placements in `sf/overlay`; a held
prop saves as placed at its current pose, the link is not saved (on load it rests where it was; an NPC's plan is
rebuilt). **Risk**:
medium (hand IK quality, carry clips); relocation is ahead of the industry (no verified shipped example), so it comes
last (IN9).

