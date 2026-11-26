# Followers and companions (slice AI0d)

> Framework spec: `sf_ai` (Bevy-free core), `sf_ai_bevy`, `sf_rpg_ui` (wheel). AI lane decision 9, Stage 7 of
> [docs/roadmaps/ai.md](../../../docs/roadmaps/ai.md). Parent: [ai.md](ai.md). Morrowind mapping:
> [ai-adapter.md](../../../bevyrrowind/docs/specs/ai-adapter.md) AI_E/AI_F rows.

## Goal

A companion walks beside you, never in your way, follows you through doors, waits or holds a spot on command, sneaks
when you sneak, holds fire while you are hidden, fights your target, and is knocked down, not killed. An escort leads
at the pace of the one it escorts. All of it is one directive (`Follow` / `Escort`) on the actor's stack, host-only,
so a co-op guest's companion behaves the same. Not here: party inventory, recruitment dialogue, mounts.

## Reuse check (CONTRIBUTING rule 0/2)

- **Bevy 0.20:** no AI, steering, formation or follower layer. Movement uses our nav (corridor, ORCA, stuck monitor
  NV14) and `Traverse` (AI12).
- **Crates (crates.io, 2026-11-26):** `bevy-steering` 0.5 (Bevy-coupled, ~17 recent downloads), `cohort_steering`
  0.1, `bevy_follow_path` 0.3 (2022). None is Bevy-free, serde, deterministic or has slots; `dodgy_2d` stays the ORCA
  reference (orca.md). Slots are a few hundred lines of geometry: ours.
- **Industry:** a leader-follower slot model with unseen catch-up and a small command set (table below). UE5 and
  Unity ship no follower component: it is game code over their path following. OpenMW's packages are ported in `mw_ai`.

## Data model

```rust
// sf_ai (Bevy-free). Illustrative. Replaces today's `DirectiveKind::Follow(ActorKey)` (AI38).
pub enum DirectiveKind { /* ... */
    Follow { leader: ActorKey, slot: SlotPref, rules: FollowRulesId },
    Escort { who: ActorKey, to: Location },   // unchanged: the escort leads, `who` follows
}
pub enum SlotPref { Any, Left, Right, Rear }
pub struct FollowRules {                       // record `sf.ai.follow_rules`, mod-patchable
    pub formation: FormationId,                // record `sf.ai.formation`
    pub engage: Engage,                        // default combat stance
    pub downed_not_killed: bool,               // default true: sets `Fighter.always_recover` while following
    pub obeys_stealth: bool,                   // default true
    pub commandable: bool,                     // false: scripted followers ignore commands
    pub notice_reactions: bool,                // default false: no Investigate/Report/Tidy while following
    pub remarks: Option<BarkSetId>,            // AI42
}
pub enum Engage { Assist, Defensive, Passive }
pub enum FollowMode { Follow, Wait { at: Location }, Hold { at: Location } }
pub struct FollowParams { mode: FollowMode, engage: Engage, hidden: bool, focus: Option<ActorKey> } // saved
pub struct FormationRow { pub slots: Vec<SlotOffset> }   // row n = n followers
pub struct SlotOffset { pub right_m: f32, pub back_m: f32, pub side: SlotPref }
pub fn slot_targets(leader: &LeaderFrame, row: &FormationRow, narrow: bool, trail: &Trail) -> SmallVec<[Location; 4]>;
pub fn assign(poses: &[FollowerPose], targets: &[Location], prev: &[u8], prefs: &[SlotPref]) -> SmallVec<[u8; 4]>;
```

`FollowerState` (host, derived, not saved): slot index, `Trail` (the leader's breadcrumbs every `.trail_step_m` plus
the links it crossed), yield flag, last-seen tick per player, catch-up timer.

## Slots in the leader's frame

- **Frame:** the leader's position and a smoothed heading (velocity direction above `.heading_min_speed`, else
  facing), so slots don't spin when the player turns the camera.
- **By count:** the formation row for the follower count. Shipped default (Guess, after Dragon Age and RDR2): 1 =
  right-rear (1.2 m right, 1.8 m back); 2 = both rear corners; 3 = corners + one 3 m behind; 4 = corners + two
  staggered behind. Past 4 (Classic has no cap) extra followers queue on the trail.
- **Side preference:** `assign` tries all permutations (4! = 24), minimising path length plus a side penalty; it keeps
  the previous assignment unless the new one saves `.reassign_gain_m` (no swapping).
- **Single file:** indoors (`Space::Interior`) or where nav clearance at the leader is under `.single_file_width_m`,
  slots become trail points `k × .file_spacing_m` behind the leader, so nobody cuts through walls. The formation
  returns after `.file_exit_s` in open space.
- **Projection:** a slot snaps to the navmesh within `.slot_snap_m`; off-mesh or unreachable, it falls back to the
  trail point at the same distance.

## Moving: personal space, never blocking the leader

- **Speed matching:** within `.slot_tolerance_m` the follower matches the leader's gait; behind, it runs past `.run_m`
  and sprints past `.sprint_m`; ahead, it slows. At a still leader it settles after `.settle_s`, faces out, and may
  take a free-time pick within `.idle_radius_m` (Skyrim followers sit); the leader moving breaks it off.
- **Personal space:** no slot or settle point within `.personal_m` of the leader.
- **Yield on the leader's path:** the leader's predicted capsule (velocity × `.yield_lookahead_s`, width
  `.yield_width_m`) is a no-go zone; a follower inside steps out to the nearer side the same tick. It is a steering
  term that runs whatever the actor does. ORCA's `AvoidRole::NonReactive` (orca.md) covers players and, toward its
  own followers, a leader NPC.
- **Doors and narrow links:** when the leader's path enters a door, ladder or corridor link, followers hold
  `.door_hold_m` back until it is through. If the leader still walks into a follower, the follower takes the push
  (`sf_controller` yield-to-push, as Skyrim and Fallout 4); the leader is never stopped.
- **Through doors (AI12):** the follower walks the `Trail` and runs `Traverse(link)` on each link the leader crossed.
  Fade links (load doors) cross only unseen (AI12): the follower appears `.door_delay_s` after the leader, behind it,
  out of its view cone.

## Catch-up when unseen (Bethesda)

- **Trigger:** farther than `.teleport_m` from its slot, in another space than the leader, or at the stuck monitor's
  last escalation (NV14), for `.catch_up_s`.
- **Rule:** it teleports only when **no player** sees it (AI12's unseen test) and the arrival point is unseen too: a
  navmesh point `.teleport_behind_m` behind the leader on the `Trail`. While seen it keeps running; seen and stuck
  for `.catch_up_max_s` in co-op, it takes the trail point farthest from every player's view.
- **Travel:** travel.md step 6 brings followers through `followers_of(leader)` (mode `Follow` only).
- **Tiers:** a follower in mode `Follow` is pinned to tier 2 while its leader is simulated (like combat pins, bounded
  by the per-leader cap). A waiting follower takes tiers normally; at tier 0 `Wait` is a still `LogicalState`.

## Escort

`Escort { who, to }`: the escort leads to `to`; `who` follows it (a `Follow` directive or a player).
- **Speed clamp:** never faster than `who` (the slowest of `who`'s party) × `.escort_speed_ratio`.
- **Wait:** `who` beyond `.escort_wait_m` stops the escort, which looks back; it resumes within `.escort_resume_m`
  (hysteresis). It waits at corners and links until `who` has line of sight to it.
- Done at `to` within tolerance or by its `Limit`, under the adapter's completion rules.

## Commands (host commands; the wheel is presentation)

```rust
pub enum FollowerCommand { Follow, Wait, HoldPosition { at: Location }, AttackTarget(ActorKey),
    Passive(bool), StayOutOfSight(bool) }
pub struct CompanionCommand { pub from: PlayerKey, pub follower: Option<ActorKey> /* None = all mine */, pub cmd: FollowerCommand }
```

- A client command (net rule 2). The host checks `from` is the leader, `rules.commandable` and `.command_range_m`;
  rejects are logged, never applied.
- **Mapping (AI39, exact):** `Follow` -> mode `Follow`; `Wait` -> `Wait { at: here }`; `HoldPosition` -> `Hold { at }`
  (go, stand, engage only within `.hold_engage_m`); `AttackTarget(t)` -> `focus = t` until `t` is Downed, dead or
  beyond `.focus_lost_m`; `Passive(b)` -> `engage = Passive`, or back to the record's default; `StayOutOfSight(b)` ->
  `hidden = b` (always sneak, slots at `.hidden_spacing_m`, slot cost adds light exposure).
- Commands only change the served `Follow` directive's params and push no directive, so stack ranks are untouched.
  `Wait` has no timeout; a pack may add one (Skyrim's return home is content).
- **Wheel (AI40, `sf_rpg_ui`):** a radial on a focused companion (`sf_interaction` focus) or on all; it only sends
  `CompanionCommand`s and reads `FollowerView { leader, mode, engage, hidden }`, replicated to the leader only.

## Where it ranks (ai.md "Interrupt contract")

- `Follow` and `Escort` are **row 5 stack directives at their source's rank**: recruited by dialogue = `Script` or
  `Quest`; Morrowind AI_F = `Record`. Player commands are params of that directive, not a layer of the table.
- **Row 1** (Combat, Flee) pre-empts it, shaped by `engage`, `focus` and a **leash** consideration: the Combat score
  falls to 0 beyond `.leash_m` from the leader, so `Follow` wins and the follower disengages. **Row 2** (Pursue) is
  never scored for followers. **Row 3** (Converse) pre-empts as for anyone. **Row 4** (Notice) runs, but with
  `notice_reactions: false` it pushes no `Reaction` directive; a hostile still raises Combat.
- Rows 6-7 run inside `Follow` only when settled. Yield and catch-up run in every row. After a pre-emption, `Follow`
  replans from the root and reassigns its slot.

## Stealth rules (`obeys_stealth`)

- **Sneak with you:** stance mirrors the leader's after `.stance_delay_s`; `hidden` forces sneak.
- **Hold fire while you are hidden:** while the leader sneaks with `PlayerDetection.band` below Alerted, the Combat
  drive scores no *initiate* option for the follower. It fights only if attacked, if the leader attacks, or on
  `AttackTarget`.
- **Never break your stealth:** while the leader is hidden, a sight check on its follower adds no sight gain. It emits
  `NoiseEvent { kind: companion_glimpse, at: follower, loudness_m: .glimpse_noise_m, source: None, reveals_source:
  false }`: the observer may grow Curious and look at the spot, never sights the leader, never starts combat from it.
  Footsteps are noise as usual. A follower that fights is a normal target.
- Followers never report their leader (crime.md `can_report`) and don't count toward `PlayerDetection` (ai.md).

## Combat rules

- **Assist** (default): followers join when the leader attacks or is attacked; target choice adds a bonus for the
  leader's target and for whoever last hit the leader. **Defensive**: only when attacked themselves. **Passive**: no
  attack options; evade and stay near the leader. `focus` overrides target choice. Followers take attack tokens
  (combat-npc.md C6a) like any attacker.
- **Friendly fire:** the leader's hits are tolerated up to `.friendly_hits` within `.friendly_window_s`; past that,
  `Follow` ends and normal hostility runs (Skyrim). Followers' area hits don't damage their leader in Modern.
- **Downed, not killed:** while following with `downed_not_killed`, `Fighter.always_recover` is set: health 0 gives
  `Downed(Knockout)` (combat.md "Non-lethal and mercy"), hits while Downed restart it, no Finish, enemies drop it
  from targeting. Content may clear the flag per companion (a story death).
- **Wake:** after `combat.downed.knockout_s`; or `.wake_after_combat_s` once no hostile is within
  `combat.downed.leave_m`; or at once through the leader's **Help up** interaction (focus prompt, hold `.help_up_s`).
  Health on wake: `combat.downed.wake_health`.

## Remarks and opinions (AI42)

- **Remarks:** `BarkCue`s (AI30) in a `companion` category: on facts (a crime, a quest stage, a place entered, a
  fight won, weather), with a per-companion cooldown `.remark_cooldown_s` and a party cooldown `.party_remark_s`;
  none while hidden or fighting, except warnings. **Paired-slot chatter:** two companions, or a companion and an NPC,
  at a paired slot (AI14) play a cue pair.
- **Opinions and leaving: pending owner Q5 (parked "much later").** Recommendation designed in: an opinion is the
  companion's relation to the leader in AI memory (AI13 facts), read by remark conditions; a companion leaves only
  where its pack's content says so (a content condition ends `Follow`); no framework approval meter.

## Co-op

- Each follower has **one leader**: an `ActorKey`, resolved to a `PlayerKey` for a player. Only that player's
  commands apply; a leader change is content (dialogue), never a command.
- A leader who disconnects leaves its followers in `Wait` where they stand; they resume `Follow` on rejoin.
- Slots, catch-up and commands are host state; unseen means unseen by every player.

## Saves

In `sf/ai` (ai.md "Saves"): the directive with `FollowParams` (mode, wait or hold spot, engage, hidden, focus). Not
saved: slots, `Trail`, yield, timers, `FollowerView`. On load, followers in mode `Follow` within
`travel.follow.radius_m` of their leader are placed on the navmesh behind it by the promotion resolve.

## Tunables (`ai.follow.*`, net `Gameplay`, scope `Dev`; all `Guess`; name (unit, range, default))

Slots: `trail_step_m` (m, 0.25-2, 0.5), `heading_min_speed` (m/s, 0.1-2, 0.5), `reassign_gain_m` (m, 0-5, 1.5),
`single_file_width_m` (m, 1-4, 2), `file_spacing_m` (m, 0.8-3, 1.4), `file_exit_s` (s, 0-5, 1.5), `slot_snap_m`
(m, 0.5-4, 2), `slot_tolerance_m` (m, 0.2-2, 0.6). Moving: `run_m` (m, 2-10, 4), `sprint_m` (m, 5-30, 10),
`settle_s` (s, 1-30, 5), `idle_radius_m` (m, 1-10, 4), `personal_m` (m, 0.5-3, 1), `yield_lookahead_s` (s, 0.3-3,
1.2), `yield_width_m` (m, 0.6-2, 1), `door_hold_m` (m, 1-5, 2), `door_delay_s` (s, 0-3, 0.8). Catch-up:
`teleport_m` (m, 15-100, 30), `teleport_behind_m` (m, 2-10, 4), `catch_up_s` (s, 0.5-10, 2), `catch_up_max_s` (s,
5-60, 15). Escort: `escort_speed_ratio` (x, 0.5-1, 0.95), `escort_wait_m` (m, 3-30, 10), `escort_resume_m` (m,
2-20, 6). Commands and combat: `command_range_m` (m, 5-100, 30), `hold_engage_m` (m, 5-50, 15), `focus_lost_m` (m,
10-100, 40), `leash_m` (m, 15-100, 35), `hidden_spacing_m` (m, 2-10, 4), `friendly_hits` (count, 1-10, 3),
`friendly_window_s` (s, 2-60, 10), `wake_after_combat_s` (s, 0-60, 8), `help_up_s` (s, 0.3-5, 1.5). Stealth:
`stance_delay_s` (s, 0-2, 0.4), `glimpse_noise_m` (m, 0-15, 4). Remarks: `remark_cooldown_s` (s, 10-600, 120),
`party_remark_s` (s, 5-120, 30).

Data, not tunables: formation offsets (`sf.ai.formation`), `FollowRules` per companion. The per-leader cap is an
adapter rule (`FollowerRules::max_followers`: Modern 4, Classic none).

## Tests (exact, headless; `sf_ai` unless marked)

1. `slot_targets` for 1-4 followers at headings 0°, 90°, 180°; heading frozen under `heading_min_speed`.
2. Interior or narrow: trail points `k × file_spacing_m`; formation back after `file_exit_s`.
3. `assign` keeps side preferences and never swaps under `reassign_gain_m` (100 leader steps).
4. Yield: inside the capsule, a sideways goal the same tick; outside, none.
5. Catch-up (fake visibility): never while the follower or the arrival point is seen; after `catch_up_s` when not.
6. Escort: speed clamp; wait and resume thresholds without flicker.
7. Commands: exact `FollowParams` per command; wrong player, non-commandable, out of range: rejected and logged.
8. Ranking: Combat beats `Follow` within `leash_m` only; `notice_reactions: false` pushes no `Reaction`.
9. Stealth: no initiate option while the leader is hidden; a sight hit gives one `companion_glimpse`, no gain.
10. Downed: health 0 gives `Downed(Knockout)`, no Finish, hits restart it; Help up wakes at `wake_health`.
11. Saves: `FollowParams` round trip. 12. `sf_ai_bevy`: door link crossed after the leader; fade link only unseen.

## Industry comparison

| Game | Technique | Ours |
|---|---|---|
| Skyrim / Fallout 4 | follow package, unseen teleport, wait/commands, protected followers, push past | **match** |
| RDR2 | posse slots in the leader's frame, speed matching | **match**; mounts later |
| Mass Effect | squad orders: move to, attack target, hold, follow | **match** (`HoldPosition`, `AttackTarget`, wheel) |
| TLOU2 | companions can't be spotted while the player is hidden | **adapted**: a glimpse is only noise |
| Dragon Age / Baldur's Gate 3 | party formations, single file in tight space, downed and revive | **match** (rows, single file, Help up) |
| OpenMW `AiFollow` / `AiEscort` | follow at distance and through doors; escort waits for the escorted | **match** in Classic via `mw_ai` |

## Build plan (rows in [docs/roadmaps/ai.md](../../../docs/roadmaps/ai.md) Stage 7)

- **AI38** `Follow`/`Escort`: slots, trail, personal space, yield, unseen catch-up, escort clamp.
- **AI39** `CompanionCommand` validation and mapping.
- **AI40** command wheel in `sf_rpg_ui` (owner check AI-9).
- **AI41** stealth and combat rules, `always_recover` while following, wake and Help up.
- **AI42** companion remarks and paired-slot chatter cues.
- **MA9** Morrowind follow/escort/pursue through doors and fallback (`mw_ai`; AI_F -> `Follow` with Classic rules).
