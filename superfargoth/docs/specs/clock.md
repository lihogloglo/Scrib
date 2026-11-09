# One authoritative game clock (`sf_time`)

Status: **spec written 2026-11-09, nothing built.** ROADMAP 1.17 (owner 2026-11-09: "an obvious fix,
needed for multiplayer"). Owner sign-off needed on "Open questions" before C1.

Agent summary: three unrelated clocks exist and nothing converts between them, so timers written in
one unit are never advanced by the other. This spec replaces them with one host-authoritative integer
instant advanced on the fixed tick in a new Bevy-free crate `sf_time`; every existing clock type
becomes a derived view. Rendering keeps a continuous f64 day fraction.

## Goal

One clock, owned by the host, advanced once per fixed tick, saved once, replicated once. Everything
that needs a calendar date, a minute count or a smooth day fraction derives it. No system owns time.

## What exists today (measured at 80c9a2c)

| Clock | Type | Unit | Advanced by | Saved |
|---|---|---|---|---|
| `sf_sky::SkyClock` | `{ days: f64, time_scale: f32, paused: bool }` | days, f64 | `sf_sky_bevy::advance_clock` from `Res<Time>::delta_secs_f64()` | `sf/clock` |
| `sf_rpg::GameTime` | `{ day: u32, minute: u16 }` | integer minutes | **nothing** | inside journal entries |
| `sf_effects::GameMinutes` | `u64` | integer minutes | **nothing** | not yet |

Consequences, all live:

1. **Only the sky clock moves.** `GameTime` and `GameMinutes` are written but never advanced, so AI
   directive expiries (`sf_ai::directive`), power recharges (`sf_effects::book`) and merchant purse
   restocks (`sf_rpg::service::purse`) cannot fire in game. The ROADMAP entry states this.
2. **Three independent minute conversions**, each hardcoding 1440 minutes per day:
   `sf_rpg::GameTime::total_minutes` (`quest.rs:66`), `sf_ai::directive::{minutes, game_time}`
   (`directive.rs:334-340`), and `sf_effects::GameMinutes::DAY` (`book.rs:21`). Meanwhile
   `sf_sky::Calendar::hours_per_day` is a free parameter, so a calendar that is not 24 hours silently
   desynchronises the three from the sky.
3. **The only moving clock is driven by frame delta**, against `net.md` rule 1 ("no frame `dt` in
   gameplay"): `advance_clock` reads `Res<Time>` (frame time), not the fixed tick. Two machines
   running the same session reach different game times.
4. The seam is already designed and only unimplemented: `sf_effects::book.rs:18` documents "the host
   converts from `sf_rpg::GameTime::total_minutes`; `sf_effects` does not depend on `sf_rpg`", and
   `sf_ai::save.rs:28` documents that `sf/ai` is "advanced with the host's logical clock, and
   `sf/clock` is snapshotted at the same tick". Nobody wrote the host side.

## Canonical technique

Shipped engines keep one authoritative scalar and derive every view of it:

- **Morrowind / OpenMW**: the `Timescale` global (30 by default) scales real seconds into game hours;
  `GameHour`/`Day`/`Month`/`Year` globals are derived, and `MWWorld::TimeStamp` (`{ hour: float, day:
  int }`) is the single stamp passed around. One source, many views.
- **Bethesda (Skyrim)**: the `GameHour` global is authoritative; quest and package timers read it.
- **Networked engines (Source, Overwatch)**: the authoritative quantity is the **integer tick**;
  wall-clock views are derived and never fed back. This is what makes a clock replicable.

So: an integer monotonic count advanced a fixed amount per tick, with every calendar and float view
derived. Integer because floats accumulate differently per machine and break both replication and
save-reload determinism; monotonic because expiry comparisons must never go backwards.

## Reuse (Bevy, then crates, then our own)

**Bevy already provides most of this, and `SkyClock` reimplements it.** `Time<Virtual>` carries
`relative_speed` and `pause()`/`unpause()`; `Time<Fixed>` is driven *by* virtual time, so a system in
`FixedUpdate` reading `Time<Fixed>::delta` already sees time that is scaled and pausable. The repo
already uses both (`sf_action_bevy/src/lib.rs:538`, `sf_camera_bevy`, `sf_physics_bevy/src/carry.rs`).

Therefore `SkyClock::time_scale` and `SkyClock::paused` duplicate `Time<Virtual>::relative_speed` and
its pause flag, and should be deleted rather than ported.

What Bevy does **not** provide, and what `sf_time` adds: a game-domain calendar, an integer instant
that survives a save, and a monotonic value safe to compare timers against. No crate was adopted:
`chrono` and `time` model the Gregorian calendar and leap seconds, which an invented world does not
have, and neither offers an integer game instant. This is a ~200-line core.

## Design

### `sf_time`, a new Bevy-free crate

Lowest level, no dependency but `serde` (hakari `final-excludes`, per `superfargoth/CLAUDE.md`).

```rust
/// Game time since the world's epoch, in game milliseconds. Monotonic, integer, replicated.
/// u64 ms spans about 584 million years.
pub struct GameInstant(pub u64);

/// How a world's days divide. Owned by content, not by the sky.
pub struct Calendar { pub hours_per_day: f32, pub days_per_year: f32 }

/// Game milliseconds per real second, as a rational so advancing is exact.
pub struct TimeScale { pub num: u32, pub den: u32 }   // Morrowind's 30 => 30/1
```

`GameInstant` derives every view, each taking the `Calendar` so nothing hardcodes 1440:

- `minutes(&Calendar) -> u64`, `hours(&Calendar) -> f64`
- `day_fraction(&Calendar) -> f64` — 0 = midnight, 0.5 = noon (what the sky wants)
- `days(&Calendar) -> f64` — the old `SkyClock::days`, for the sky and the save display field
- `game_time(&Calendar) -> GameTime` and `from_game_time` — the journal's stored form

### Advancing: exact integer accumulation on the fixed tick

One system in `FixedUpdate`, owned by the host:

```rust
fn advance(fixed: Res<Time<Fixed>>, scale: Res<TimeScale>, mut now: ResMut<GameClock>) {
    now.advance_exact(fixed.delta(), *scale);   // integer ms plus a carried remainder
}
```

`advance_exact` keeps a `remainder: u32` so a scale that does not divide evenly loses nothing, and two
machines ticking the same count reach the same instant. Pause and speed come from `Time<Virtual>`
(Bevy already gates `Time<Fixed>` on it), so `sf_time` holds no pause flag.

**Presentation may interpolate, never write back** (net rule 3): the sky reads the instant plus the
time elapsed since the last tick to place the sun smoothly, and that interpolated value never reaches
gameplay or the save.

### What each existing clock becomes

| Today | After |
|---|---|
| `SkyClock { days, time_scale, paused }` | deleted. `sf_sky` takes `day_fraction`/`days` as arguments; `time_scale`/`paused` become Bevy's `Time<Virtual>` |
| `sf_rpg::GameTime` | **kept as a value type** (journal entries serialise it, so the save format does not churn), constructed only via `GameInstant::game_time` |
| `sf_effects::GameMinutes` | the derived minute count, or deleted in favour of `GameInstant`; the host conversion its own doc already promises |
| `sf_ai::directive::{minutes, game_time}` | deleted, replaced by the `sf_time` conversions |
| `sf_action::ids::Tick` | kept (the action lane's tick), but `net.md`'s single host `Tick` counter lives beside `GameClock` |

## Saves

`sf/clock` becomes version 2: `{ ms: u64, scale: TimeScale }`. Migration from v1 (`days: f64`,
`time_scale`, `paused`): `ms = round(days * calendar.seconds_per_day() * 1000)`, `scale` from
`time_scale`, `paused` dropped (host state, not world state). **Old goldens must load** (`save.md`
"Versioning and migration"); the v1 to v2 migration gets an exact test. `sf_save::meta` keeps its
display-only days field, computed from `ms`.

Load order matters: `sf/clock` must apply **before** any section holding timers against it (`sf/ai`,
`sf/services`, effects state), so a timer is never compared to a stale instant.

## Co-op (net-ready rules)

Rule 1 (fixed tick): advancing moves to `FixedUpdate`, fixing the current frame-delta violation.
Rule 2 (commands): a client never sets the clock; `SetTime`, rest and wait are commands the host
applies. Rule 4 (host authority): `GameClock` is host-owned; clients receive it and never extrapolate
into gameplay. Rule 8 (data events): a `TimeJumped { from, to }` event carries both instants so
listeners can rebase their own timers after a rest, a travel service or a console `set`.

## Tests (exact)

- `advance_exact` over 60 ticks at scale 30/1 equals one advance of 60 ticks' worth — no drift; and a
  scale that does not divide evenly (7/3) loses nothing over 10,000 ticks.
- Round trip: `from_game_time(game_time(i)) == i` truncated to the minute, for a 24-hour and a
  non-24-hour calendar (the bug class the three hardcoded 1440s create).
- `day_fraction` is 0 at midnight and 0.5 at noon for both calendars.
- Monotonicity: no sequence of advances decreases the instant.
- Save v1 to v2 migration: a golden v1 `sf/clock` loads to the same wall time it displayed before.
- A directive whose expiry is set at instant X fires after advancing past X, and not before (the
  failure the ROADMAP reports).
- `#[ignore = "measurement: ..."]` for the cost of the advance system, per CONTRIBUTING.

## Industry comparison

| | Authority | Unit | Pause and scale | Replication |
|---|---|---|---|---|
| OpenMW | `TimeStamp` | f32 hour + int day | `Timescale` global | n/a |
| Skyrim | `GameHour` global | f32 hours | timescale global | n/a |
| Source / Overwatch | tick count | integer ticks | server-side | the tick is the message |
| **This spec** | `GameInstant` | integer game ms | Bevy `Time<Virtual>` | one u64 |

Gap we accept: OpenMW stores an f32 hour, which is simpler but drifts; we take the networked-engine
form because co-op is a stated constraint.

## Build plan (one deliverable per slice)

| Slice | Deliverable |
|---|---|
| **C1** | `sf_time` crate: `GameInstant`, `Calendar`, `TimeScale`, `advance_exact`, all conversions, the tests above. Bevy-free, hakari `final-excludes`. No consumer changes |
| **C2** | Host `GameClock` resource, the `FixedUpdate` advance system, `TimeJumped`; `sf/clock` v2 and the v1 migration |
| **C3** | `sf_sky` derives from `GameInstant`; `SkyClock` deleted; pause and scale move to `Time<Virtual>`; render-side interpolation |
| **C4** | `sf_rpg`, `sf_ai`, `sf_effects` take the instant; the three duplicate minute conversions deleted; AI directive expiry proven to fire |
| **C5** | rest, wait, travel and training time passing as host commands emitting `TimeJumped`; consumers rebase (what `sf_effects::book` cooldowns need) |

## Open questions (owner)

1. **Unit: game milliseconds (proposed) or game seconds?** Milliseconds cost nothing in a u64 and keep
   the sky smooth without interpolation; seconds are easier to read in a save dump. Default: ms.
2. **Does `Calendar` move from `sf_sky` to `sf_time`?** Proposed yes — it is a world property, not a
   rendering one, and it must be reachable by `sf_rpg` and `sf_effects` to kill the hardcoded 1440s.
3. **Does `sf_rpg::GameTime` survive as the journal's stored type?** Proposed yes, to avoid a save
   migration in the journal; the alternative is storing `GameInstant` everywhere and migrating.
4. **Default `TimeScale`:** Morrowind's 30 (content-set via the `Timescale` global), or our own
   default? Proposed: the adapter sets it from content, framework default 30.
