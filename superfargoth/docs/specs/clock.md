# One authoritative game clock

Status: **C1 built 2026-11-11** (the Bevy-free core); **C2 built 2026-11-11** (the resource, its readers, `sf/clock` v2); C3 (calendar) specified below, not built. ROADMAP 1.17 (owner 2026-11-09: "an
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
purses stamp from it. C3: see "Calendar (C3)" below (slices C3a-c).

**C2 as built.** `GameClock` is a `Resource` behind `sf_ids`'s `bevy` feature, re-exported (with `GameInstant`,
`sf_ids`) by `sf_sky_bevy`, whose `SkyPlugin` inserts `SkyPlugin::clock` when absent and advances it in
`FixedUpdate` (`SkySystems::Clock`); the sky builds `SkyClock { days }` per frame (`SkyConfig::sky_clock`).
Weather and surface step on `GameInstant::hours()`; the save menu, held lights, devtools and Lua read views.
`mw_rpg_bevy`: `services::sync_clock` copies the instant into `Rpg` before the dialogue host, so
`Rpg::game_time()` (purses, journal stamps) is the clock (the `MWScript` globals only without one);
`effects::feed_clock` reads the clock after it advanced; services' passed hours `skip`. A load keeps the
session's `paused`. Goldens: `clock-v2.sfsave` is current, the five older goldens load through the v1 step.
No AI tick reads game time in the game yet (sf_ai_bevy has none), so AI timers have no live consumer.

## Calendar (C3)

Owner 2026-11-17: a **generic game calendar** in the framework (a game may have a weird one; Morrowind's must fit).
Today `sf_sky::Calendar` has only `hours_per_day` + `days_per_year` (the game never sets the year: the sky uses
365.2422), the month table and OpenMW's roll sit in `mw_rpg::training` (`days_per_month`, `advance_calendar`), there
are no weekdays and the epoch is implicit. NS-3 (moons) needs OpenMW's "days since epoch".

**Reuse check.** Bevy 0.20: nothing (no calendar; `Time` only). Crates (searched 2026-11-17): none for fantasy
calendars; crates.io has CLI/iCal/business-day crates (`eventix`, `icalendar`, `calends`, `findates`); `chrono`,
`time`, `jiff` are Gregorian + zones (fixed 12 months, 7-day week, 24 h), so no. Own code, ~300 lines: the need is
small and the inputs are data. OpenMW is verified from `datetimemanager.cpp`; the other rows are from memory (UESP
was unreachable), unverified.

**Industry comparison.**

| | Model | Weekday | Leap | Epoch |
|---|---|---|---|---|
| OpenMW | globals `GameHour`, `DaysPassed`, `Day`, `Month` (0-11), `Year`; `getDaysPerMonth` hardcoded 31,28,31,30,31,30,31,31,30,31,30,31; names from l10n `month1..12`; `advanceTime` rolls day/month/year; `setHour` clamps < 0 to 0 and rolls days | none | none (Feb 28) | whatever the globals hold (new game Day 16, Month 7, Year 427) |
| Skyrim | `GameHour`, `GameDaysPassed`, `GameDay/Month/Year` globals; 4E, same 12 months | names in UI (unverified) | none | 17 Last Seed 4E 201 (unverified) |
| Daggerfall Unity | `DaggerfallDateTime`: 12 x 30-day months, 7-day week, seasons (unverified) | yes | none | fixed start |
| Fantasy-Calendar (web tool) | months, intercalary days, leap rules (every N except M), weekdays, eras, moons with cycles (unverified) | yes, any length | rule-based | any date |
| Ours | one record: hours/day, months, optional leap rule, weekdays, epoch, era; the instant stays the only state | yes | optional | the record |

**Design.**
- **Record** `sf.calendar` (`register_serde`, the `sf_ai::htn::register_kinds` pattern; RON in a mod via `sf_mods`;
  id `main`, a later definition replaces it, so a mod can change the calendar):
  ```
  Calendar { hours_per_day: f32 (24), months: [Month { name, days: u32, leap_days: u32 (0) }],
    leap: Option<Leap { every, skip_every, restore_every, offset }>,   // Gregorian generalised
    weekdays: [String] (empty = none), era: Option<String>,
    epoch: Epoch { year: i64, month, day (1-based), weekday: u32, start_hour: f32 } }
  ```
  `epoch` is the date of instant 0 at 00:00; `start_hour` is where a new game's clock starts (content, not calendar
  state). A leap year (`(year-offset) % every == 0 && ((year-offset) % skip_every != 0 || % restore_every == 0)`)
  adds `leap_days` to each month that has them. An intercalary day is a 1-day month and counts in the week
  (non-goal: weekday-less festival days).
- **Where:** new Bevy-free crate `sf_calendar` (serde, `sf_ids`, `sf_records` for the kind; cold in seconds, in
  hakari `final-excludes`), not `sf_ids`: it holds strings and records, `sf_ids` stays ids + the integer clock. The
  sky and the game depend on it (wide, shallow). `CalendarView` is a `Resource` behind a `bevy` feature like
  `GameClock`, inserted by `sf_sky_bevy`'s plugin; no `_bevy` crate.
- **Functions** (`CalendarView::new(&Calendar)` validates: >= 1 month, days >= 1, hours > 0): `date(instant) ->
  Date { year, month, day, weekday, hour, minute }`, `instant_at(Date)`, `days_since_epoch(instant)` (OpenMW's count,
  for moons), `days_in_year(year)`, `mean_year_days()` and `hours_per_day()` (the sky's `Calendar` is built from
  them: its two fields go, one source), `month_name`, `weekday_name`. Year lookup: estimate by the mean year, correct
  by +-1 with the closed-form leap count. `format(&Date, pattern)` with `{d} {month} {year} {era} {weekday} {hh}:{mm}`;
  the game picks the pattern (UI, save menu), names come from the record (l10n keys later).
- **Morrowind** (`mw_rpg::calendar` + content record `main`): Morning Star 31, Sun's Dawn 28, First Seed 31, Rain's
  Hand 30, Second Seed 31, Midyear 30, Sun's Height 31, Last Seed 31, Heartfire 30, Frostfall 31, Sun's Dusk 30,
  Evening Star 31 (OpenMW's lengths); weekdays Sundas, Morndas, Tirdas, Middas, Turdas, Fredas, Loredas; era `3E`;
  epoch 16 Last Seed 427 (Month 7, Year 427 as the vanilla start globals), `start_hour` 9. **The epoch's weekday is
  unverified** (OpenMW has none): cosmetic, to be filled from a lore source, not guessed in code. The adapter keeps
  OpenMW's semantics: `GameHour` = hour of day, `Day/Month/Year` = the view's date (equal to `advance_calendar`'s roll),
  `DaysPassed` = start global + `days_since_epoch`. The month table and `advance_calendar` leave `training.rs`; the
  save menu's copy goes.
- **Saves:** only the instant (`sf/clock`), unchanged, no migration. The calendar is content, identical on host and
  clients through the load order; changing it between saves re-dates the same instant (documented, no check).

**Tests (exact).** Tamrielic: instant <-> date round trip at every month edge over 5 years, incl. Evening Star 31 ->
Morning Star 1 next year. Agreement with `mw_rpg::advance_calendar` over 1000 seeded random hour advances while both
exist (day, month, year, days_passed equal; hour within 1e-3). Weird calendar (13 months, 30-hour days, a leap rule
with skip/restore, a 1-day intercalary month): round trip over 400 years, `days_in_year` sums match. Epoch weekday:
day 0 = `epoch.weekday`, +1 day increments and wraps. Sky `hours_per_day`/`days_per_year` equal the view's.

**Slices** (one deliverable each).
- **C3a** framework: `sf_calendar` (record, view, framework tests), `sf_sky::Calendar` built from the view
  (Morrowind's year becomes 365, seasons shift 0.07%: ledger line), `sf_sky_bevy` resource + `register_kinds`.
  Files: `superfargoth/crates/sf_calendar/*`, `sf_sky/src/clock.rs`, `sf_sky_bevy` plugin, root `Cargo.toml`, hakari.
- **C3b** Morrowind: `bevyrrowind/crates/mw_rpg/src/calendar.rs` + the `main` record; globals written from the view in
  `mw_rpg_bevy/src/dialogue/services.rs` (`sync_clock`); the table leaves `training.rs` once the agreement test
  passes; NS-3 moons read `days_since_epoch`.
- **C3c** two-way: `set GameHour` = clock jump (OpenMW `setHour`: clamp < 0, may roll days), script writes to
  `Day/Month/Year` shift the date, dev `day +n` console/BRP command (`bevyrrowind/set`, keeps the hour), save-menu
  date from `CalendarView::format`. Files: `mw_rpg_bevy`, `mw_devtools_bevy/src/remote.rs`, `mw_exterior_bevy/src/pause_menu.rs`.

**Owner decisions (2026-11-17).** Weekday display is a mod, not the game: the record carries weekday names (and the
epoch weekday, unset for Morrowind until a lore source fixes it); no vanilla screen shows them, a UI mod may. Seasons:
the calendar exposes month and day of year to the weather as a generic input, so a data-driven seasonal modulation of
weather chances (per region, per month) can come later as data or a mod; vanilla stays neutral (owner: "in theory;
in practice it could get boring fast", so subtle and optional, not built now).
