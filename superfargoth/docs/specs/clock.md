# One authoritative game clock

Status: **C1 built 2026-11-11** (the Bevy-free core); **C2 built 2026-11-11** (the resource, its readers, `sf/clock` v2); C3 next. ROADMAP 1.17 (owner 2026-11-09: "an
obvious fix, needed for multiplayer"). The earlier long draft (5 slices, owner questions) is on
`origin/backup/mechanics-wip-2026-11-09`; this page supersedes it and answers its questions.

**Problem.** Three unconverted clocks: `sf_sky::SkyClock` (f64 days, saved `sf/clock`, advanced from frame
time), `sf_rpg::GameTime` (minutes; AI, journal, purses) and `sf_effects::GameMinutes`; plus the game's
`MWScript` globals (`GameHour`, `DaysPassed`, `Day`, `Month`, `Year`), which only rest/train moves. AI timers,
power recharges and purse restocks never advance in play; the dialogue host stamps `GameTime::default()`.

**Industry.** OpenMW: `DateTimeManager` (`apps/openmw/mwworld/datetimemanager.cpp`) holds hour + days passed +
day/month/year + `Timescale`, mirrors them into the globals, `getTimeStamp()` = (hour, days passed), Lua
`core.getGameTime()` = game seconds. Skyrim: the `GameHour`/`GameDaysPassed` globals. Networked engines
(Source, Overwatch): the integer tick is authoritative, wall-clock views derive. Bevy has `Time<Virtual>`
(pause, speed) and `Time<Fixed>` but no game calendar or saveable integer instant; `chrono`/`time` model
Earth. So: one integer instant, every view derived (`sf_ids`, ~150 lines, no new crate).

**Design.**
- **Value:** `sf_ids::GameInstant(u64)`, game **milliseconds** since the epoch (day 0, 00:00): ms keeps the
  sun smooth at any scale with no interpolation. `sf_ids::GameClock { now, time_scale, paused }`: serde
  carries `now` + `time_scale` (saved, replicated); `paused` (dev freeze: screenshot, bench, live A/B) and a
  sub-ms carry are host session state. Host-owned; clients receive it, never set it (rest/wait are commands).
- **Advance:** `GameClock::advance(Time<Fixed>::delta())` once per fixed tick (net rule 1), exact integer ms +
  ns carry (64 ticks of 1/64 s at x30 = 30 000 ms exactly). Menus pause Bevy's virtual time, which stops the
  fixed tick, so the clock stops too. `skip(ms)`: rest, wait, train, travel (works while paused).
  `set_hour`: dev only (may go back; timers set later stall).
- **Scale:** game seconds per real second, `f32`, framework default 30; the adapter sets it from content
  (Morrowind's `Timescale` global).
- **Views (all round down):** sky `days(day_ms(calendar.hours_per_day))` (f64, exact to the ms; `SkyClock`
  becomes this view, its `time_scale`/`paused`/`advance` go); `sf_rpg::GameTime::from(instant)` (1,440-minute
  bookkeeping days; AI `now`, journal, purses); `sf_effects::GameMinutes::from(instant)`; `EffectClock` fed
  from it; Lua `core.game_time()` = hours. A reading never runs ahead of the clock, so a load can't freeze
  stack timers (ai-as-built "AI5 Clock").
- **Game calendar (adapter):** the `MWScript` globals are written from the instant every tick:
  `GameHour` = hour of day, `DaysPassed`/`Day`/`Month`/`Year` = `mw_rpg::advance_calendar` (OpenMW's month
  roll) from the content's start globals over the hours since the start instant. A script `set GameHour`
  becomes a clock jump (OpenMW `setHour`); until C3 it is logged and overwritten. One month table only
  (`mw_rpg::days_per_month`; the save menu's copy goes).
- **Save:** `sf/clock` v2 = `{ clock: GameClock, sim_tick }`. v1 (`{ days: f64, time_scale, paused }`)
  migrates `now = GameInstant::from_days(days, 86 400 000)` (24 h days: the only calendar with old saves),
  `paused` dropped. Order 200 stays before `sf/ai`, `sf/services` and effects, which hold timers against it.

**Slices.** C1 (done): `sf_ids::clock` + `GameTime`/`GameMinutes` from the instant, AI's minute conversion
deduplicated, exact tests (conversion, rounding, round trip, fixed-tick carry, an AI timer across a load).
C2: the Bevy resource (`sf_ids` `bevy` feature, the `sf_vfs` precedent), `FixedUpdate` advance, sky/weather/
held lights/scripts read it, `sf/clock` v2 + v1 migration test (old goldens load), dialogue/journal/effects/
purses stamp from it. C3: calendar globals derived both ways (+ `set GameHour`), save-menu calendar dedupe.

**C2 as built.** `GameClock` is a `Resource` behind `sf_ids`'s `bevy` feature, re-exported (with `GameInstant`,
`sf_ids`) by `sf_sky_bevy`, whose `SkyPlugin` inserts `SkyPlugin::clock` when absent and advances it in
`FixedUpdate` (`SkySystems::Clock`); the sky builds `SkyClock { days }` per frame (`SkyConfig::sky_clock`).
Weather and surface step on `GameInstant::hours()`; the save menu, held lights, devtools and Lua read views.
`mw_rpg_bevy`: `services::sync_clock` copies the instant into `Rpg` before the dialogue host, so
`Rpg::game_time()` (purses, journal stamps) is the clock (the `MWScript` globals only without one);
`effects::feed_clock` reads the clock after it advanced; services' passed hours `skip`. A load keeps the
session's `paused`. Goldens: `clock-v2.sfsave` is current, the five older goldens load through the v1 step.
No AI tick reads game time in the game yet (sf_ai_bevy has none), so AI timers have no live consumer.
