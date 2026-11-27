# IN3 research: beds, lean spots, rest, hand IK (2026-11-27)

Input for IN3-S (spec section "Beds, lean spots and rest (IN3)" in `superfargoth/docs/specs/interaction.md`). Tags:
**[S]** read at the source (code or page text), **[W]** search-result summary only (page not read in full), **[M]** from
memory or general knowledge, unverified here, **U** could not read (blocked or not found).

## 1. What exists in the repo (checked in code, not docs)

- `mw_interaction/src/activation.rs:266-270`: a bed offers `lie` ("Lie down"), `sit` ("Sit") and `sleep` ("Sleep",
  action `A::Rest`, Classic `RestMenu`), the last still `.not_yet("IN3")`. Bed defs: 46 records / 138 slots (lie +
  two edge sits; `interaction-adapter.md` "Beds").
- No rest or wait UI exists: `mw_script/src/functions.txt:107` lists `ShowRestMenu` as `later`. `ai.md` has
  `fast_forward(actor, hours)` for rest and wait (tier 0), not built for the player.
- Crime: `sf_crime` has only the stealth seam (`lib.rs` doc: "Ownership, offences ... follow in later slices");
  `crime-adapter.md` "Sleeping in a bed (A3)" already specs the OpenMW gate (below). Nothing in code yet.
- Lie and lean clips: anim AN30a generated only (bed lying loops); AN30b-e (hub, bake slots, lean wall, counters) not
  done (`docs/roadmaps/anim.md:154`). IK: `sf_anim` has `LimbIk`, `ChainIk`, `FullBodyIk`
  (`docs/wiki/engine/animation.md:25`); the grip table MG7b is not started. Slot defs already carry `ik: IkTargets`
  (`interaction.md` "Definitions").

## 2. Morrowind / OpenMW (the content rule we port) [S]

OpenMW `ca5e2576`:
- A bed is an activator whose script calls `ShowRestMenu` (vanilla scripts `BedStandard`, `CharGenBed`; UESP
  "Morrowind Mod:ShowRestMenu" [S]). `mwscript/guiextensions.cpp:50-59` `OpShowRestMenu`: if
  `sleepInBed(player, bed)` returns false, push `GM_Rest` with the bed.
- `mwmechanics/mechanicsmanagerimp.cpp:927-952` `sleepInBed`, in order: werewolf -> `sWerewolfRefusal`, refuse;
  enemies nearby -> `sNotifyMessage2`, refuse; `isAllowedToUse` (owned / faction-owned bed: no; an activator counts as
  a bed when its script name starts with "Bed", line 902) -> allowed: proceed; not allowed: `commitCrime(
  OT_SleepingInOwnedBed)`; **seen -> `sNotifyMessage64`, refuse; unseen -> proceed** (the crime still runs its
  witness check; no refusal if nobody saw).
- `mwgui/waitdialog.cpp:88-117` `setPtr`: underwater refuses always; in the air refuses **only without a bed**;
  enemies nearby refuse; `canSleep` = a bed was passed **or** the cell allows sleep and the player is no werewolf
  (`mwworld/worldimp.cpp:2281-2310` `canRest`). So a bed lets you sleep in a town or a no-sleep cell, where the
  plain rest key only offers Wait (UESP "Morrowind:Time" Resting [S]: "Activating a bed ... will force you to rest
  and not wait").
- Sleep interruption (exterior only, region sleep list, `fSleepRandMod` / `fSleepRestMod`, waitdialog 197-217) and
  level-up on sleep (279) belong to the rest UI, not the bed.
- Vanilla NPCs never use beds (they stand) [M].

## 3. Industry

| Game | Bed | Owned bed | Lean | Hand contact |
|---|---|---|---|---|
| Skyrim | activating a bed plays get-in, opens Sleep/Wait; Rested 5 %, own bed Well Rested 10 %, Lover's Comfort 15 %; sleep autosaves; no sleep with enemies near or while trespassing (UESP "Skyrim:Sleeping" [S]) | **cannot sleep** in another's bed unless the owner is befriended [S] | furniture markers: each marker is sit, sleep or lean, one actor each, entry points per marker, marker keywords filter users (ck.uesp "Furniture" U: Cloudflare; [W] snippets); wall-lean markers placed by hand [W] | none: clips authored per furniture [M] |
| KCD2 | sleeping in an owned or rented bed is the free permanent save; inn rooms rented (e.g. 120 g permanent room) [W] | another's bed = trespassing; inn rooms count as private areas [W] | NPC-only [M] | U |
| RDR2 | camp bed, hotel room, wilderness camp: pick a duration, restores cores, saves [W] | hotel rooms rented [W] | player prompt "lean on the bar" in Valentine saloon (mission "A Quiet Time") [W]; NPC scenarios incl. "Lean on bar stand", "Lean with hand on post" [W] | U |
| GTA V | safehouse bed = save + time skip [M] | n/a | scenario points with task, time window, probability, model filter (`Scenarios.dat`, grandtheftwiki [W]); community list names `WORLD_HUMAN_LEANING`, `WORLD_HUMAN_SEAT_WALL`, `PROP_HUMAN_SEAT_*` [W] | U |
| Cyberpunk 2077 | V's apartment bed: sleep / skip time [M] | n/a | `.workspot` files: a spot where an NPC plays a set of animations, placed by AISpot nodes; ready templates include "Lean (rail)", sit, sleep, drink (redmodding wiki [W]); rig mismatch clips [W] | U |
| UE5 Smart Objects | slots with behavior definitions (slot overrides the object default); claim -> `Claimed`, use -> `Occupied`, release; claim priority, lower priority taken unless in use (Epic docs [W]) | n/a | as slots | Motion Warping / Control Rig [M] |
| Co-op | Valheim: all players must be in bed to skip the night; Minecraft: a configurable share of players [M] | | | |

Bevy 0.20: nothing for slots, beds, rest or lean (spec "Reuse check" stands). crates.io 2026-11-27 searches
("bevy smart object", "bevy interaction", "bevy furniture"): no maintained smart-object or furniture crate [S: API
listing, low-download hobby crates only]. IK: ours (`sf_anim`), no reason to add a crate.

## 4. Findings for the spec (recommendations, not decisions)

1. **The framework sends a request, the host gates and opens the UI.** Every engine above separates the bed (a
   slot + verb) from the rest UI. `sleep` stays an ordinary verb through the activation pipeline: the game's gates
   (werewolf, enemies near, crime's owned-bed rule) refuse in the gate stage with a typed reason, exactly like other
   uses; the default handler sends `RestRequested { tick, user, placement, slot }` once per approved use. **Drop the
   planned `owned` field**: ownership is crime's (game) data, read by the host from the placement; carrying it in a
   framework message duplicates crime state.
2. **Owned beds follow OpenMW in both modes** (crime is a content mechanic, CONTRIBUTING "Not a 1:1 port"): seen ->
   refused with `sNotifyMessage64`, unseen -> sleep. Skyrim's flat refusal and KCD2's trespass are modern
   alternatives; not taken (would change Morrowind's crime rules). The prompt can mark an owned bed like any owned
   object (crime spec's job, not IN3).
3. **A bed overrides the rest gates it should** (OpenMW `setPtr`): in-air and the cell's no-sleep flag do not refuse
   with a bed; underwater and enemies do. These live in the rest UI / adapter, written in the adapter's Beds row.
4. **Lying before the menu (Skyrim, KCD2, RDR2)**: modern = lie down, then the rest UI; Classic = menu at once.
   Keep the hook invariant simple: `RestRequested` fires at the approved use; the host's rest UI waits for its lie
   use's `Entered` when a lie clip exists (IN3b), else opens at once. One message, no second hook later.
5. **Co-op (net-ready)**: the request carries `user`; the host decides the skip (Valheim / Minecraft vote pattern).
   Single player: the request is the skip. Write this as the rule, build nothing now.
6. **NPC sleep** is an AI activity on the `lie` slot (AI14 / IN8), never a `RestRequested` (players only).
7. **Lean = stations, wall leans later.** Counter and table leans are already cooked as `counter` stations
   (`interaction-adapter.md`); they use the seat use path with `counter.*` clips (IN3c, needs AN30d/e). Wall leans
   need spots on walls: Bethesda and GTA author them by hand, Cyberpunk places AISpots; we have no authored data, so a
   cook pass that finds vertical faces beside the navmesh would be new work: its own row after IN3c, not in IN3.
8. **Hand IK = cooked contact points + `LimbIk`**: armrest tops (seat collision above the seat band, per side) and
   the table/counter edge at `reach`, stored in the slot's `IkTargets`; weight ramps in over the enter clip's last
   part and out over the exit's first part. Two-bone `LimbIk` per arm, not `FullBodyIk` (cheaper; the body pose is the
   clip's). Depends on MG7b (grip table) only for held props, not for armrests; still its own slice (IN3d).

## 5. Not read / open

- ck.uesp.net "Furniture" (Cloudflare challenge): marker entry-point semantics are from search snippets only.
- KCD2, RDR2 facts are guide/forum summaries [W]; GTA V, Cyberpunk beds and co-op sleep rules from memory [M].
- No source read on how GTA / RDR2 / Cyberpunk place hands on furniture (IK vs authored); recommendation 8 rests on
  our own `sf_anim` and UE practice [M].

## Sources

- OpenMW `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`: `apps/openmw/mwmechanics/mechanicsmanagerimp.cpp`,
  `apps/openmw/mwscript/guiextensions.cpp`, `apps/openmw/mwgui/waitdialog.cpp`, `apps/openmw/mwworld/worldimp.cpp`
  (raw.githubusercontent.com, read 2026-11-27).
- UESP: [Skyrim:Sleeping](https://en.uesp.net/wiki/Skyrim:Sleeping),
  [Morrowind:Time](https://en.uesp.net/wiki/Morrowind:Time),
  [Morrowind Mod:ShowRestMenu](https://en.uesp.net/wiki/Morrowind_Mod:ShowRestMenu),
  [CK Furniture](https://ck.uesp.net/wiki/Furniture) (U).
- KCD2: [gamerguides beds and sleeping](https://www.gamerguides.com/kingdom-come-deliverance-ii/guide/getting-started/gameplay),
  [Steam discussion (trespass in rented rooms)](https://steamcommunity.com/app/1771300/discussions/0/669452846144481132).
- RDR2: [powerpyx where to sleep](https://www.powerpyx.com/red-dead-redemption-2-where-to-sleep-advance-time),
  [powerpyx A Quiet Time](https://www.powerpyx.com/red-dead-redemption-2-a-quiet-time-walkthrough/),
  [Nexus scenario mod changelog](https://www.nexusmods.com/reddeadredemption2/mods/1889/?tab=logs).
- GTA V: [Scenarios.dat](https://grandtheftwiki.com/Scenarios.dat), [scenario list](https://pastebin.com/wSXf3B6b).
- Cyberpunk: [redmodding scripted interactions](https://wiki.redmodding.org/cyberpunk-2077-modding/modding-guides/world-editing/archived-guides/scripted-interactions),
  [placing AISpot nodes](https://wiki.redmodding.org/cyberpunk-2077-modding/modding-guides/world-editing/ai-and-npcs/placing-aispot-nodes).
- UE: [Smart Objects overview](https://dev.epicgames.com/documentation/unreal-engine/smart-objects-in-unreal-engine---overview).
