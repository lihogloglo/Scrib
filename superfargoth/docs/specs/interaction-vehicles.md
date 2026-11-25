# Interaction (c): mounts and vehicles (`sf_interaction`)

Status: design 2026-11-11 (unchanged by IN27; moved out of [interaction.md](interaction.md) for size on 2026-11-25).
Slice TV0b rewrites it from [travel](../../../docs/roadmaps/travel.md) decisions 3-6. The shared core, "Budgets" and
"Build slices" stay in interaction.md.

## (c) Mounts and vehicles (seats, attach, control hand-off)

**Seat links.** A seat is a slot with `occupancy: Attach` on a moving owner. `SeatLink { seat: SlotRef, rider:
CharacterId }` (stable ids) follows Veloren's link: `create` refuses a taken seat, a rider already seated or dead;
`persist` re-validates each tick (owner and rider alive, seat still registered, same worldspace); `delete` places the
rider at a free dismount spot (exit side by `EndPoseFree`, else any side, else the nearest navmesh point). The rider
is **attached** (root = seat frame each tick, never simulated against the owner), in a `Mounted` owner (Docked-like):
mounted clip set (idle, gait-matched loops, enter/exit by side), leg IK to stirrups or footrests, hands to reins.

**Control hand-off.** A `Driver` seat routes the rider's `Command` to the owner: a mount's own action and locomotion
rules (a quadruped gait set; the mount's AI keeps only reactions such as panic and flee), or a vehicle controller.
`Passenger` seats only look and (later) fight. In co-op the driver's client predicts the mount as it predicts its own
character (net.md "Prediction" extended to "own mount"); the host stays authoritative.

**Kinds, cheapest first.**
1. **Rail vehicles** (travel boats, giant creatures on routes): the vehicle follows an authored route spline at a
   speed; the player sits in a seat and may skip (fade) at any time. "Go inside a vehicle" with no physics risk.
2. **Creature mounts**: the mount is a character (own controller, capsule, gaits, stats); call/summon, follow when
   unridden (`sf_ai` `Follow`), mount by side, dismount, mounted locomotion; mounted combat later (Elden Ring-style
   moveset rules, combat.md).
3. **Physics vehicles** (rowboats, carts): one body, buoyancy pontoons (ROADMAP 3.7) or Rapier's ray-cast vehicle
   controller; seats as above.
4. **Walk-on vehicles** (ship decks, a strider's howdah): needs **moving-platform support** in `sf_controller` (the
   ground body's point velocity added to the move, the mover solved in the platform's frame, as in moving-platform
   controllers) and nav in the platform's local frame; hard, last.

**Dependencies**: `sf_controller` (Mounted mode; moving platforms for kind 4), a quadruped action/locomotion set
(creature locomotion is not built), rider clips (no free mounted pack is known; Kimodo can make a seated-astride
rider, the creature's gaits come from the game's own clips), `sf_ai` Follow (3.10), physics buoyancy (3.7) for boats,
streaming (a ridden mount changes cells like an actor). **Cost**: a ridden mount is one more tier-2 character (pinned
while ridden); a rail vehicle is one transform; a physics boat one body plus pontoons. **Net**: seat links are host
state, replicated as events (`Mounted`/`Dismounted` with tick and ids) plus the link in the character's replicated
state. **Save**: the rider's `CharacterSave` / `sf/ai` activity holds its seat (`SlotRef`); the mount's own state is
its actor save; vehicles' positions are overlay moved or spawned placements; a rail vehicle saves its route progress
in the `interaction` blob. **Risk**: medium (mounts: clips and quadruped locomotion), high (walk-on vehicles).


