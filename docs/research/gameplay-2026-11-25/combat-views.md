# Combat in first vs third person: one mechanic or two? (research, 2026-11-26)

Tags: `[V: url]` = page fetched and read. `[S: url]` = only a web-search result summary of that page (page itself blocked or not opened); treat as weaker. `[M]` = memory, unverified. Fetch failures: Fandom wikis (HTTP 402), Torn Banner/UE interview (403), PC Gamer and GamesRadar Avowed pieces (truncated). No developer source was found for Chivalry 2, Mordhau or Bannerlord view rules; those rows rest on guides and player reports.

## 0. Verdict: holds, with two changes

One rules core plus per-view profiles is how every game that offers both views does it; no source found a game with separate melee rules per view.
- Avowed (the only dev-sourced both-views case): same rules, same swing/hit-reaction/dodge/parry timings, 1P timing is master and 3P is made to match; dev warns timings drifting apart is felt on switching [V: https://www.cgmagonline.com/interviews/avowed-first-third-person/]. Chivalry 2 and Mordhau: same combat, 3P is a camera option; the only lever is a server toggle that disables 3P [S: https://help.bisecthosting.com/hc/de/articles/40438583674395-How-to-Disable-Third-Person-on-a-Mordhau-Server].
- **Change 1 (contradicts the hypothesis):** the evidence supports *small* per-view profiles, not different dodge types. Avowed explicitly avoided separate 3P dodge/parry timing. Roll-with-i-frames in 3P vs backstep in 1P is our own taste call, not an industry pattern; the standard split is camera, aiming/targeting and awareness aids, not move rules.
- **Change 2:** feint/morph is not a 1P feature in the sources: Chivalry 2 and Mordhau have it in both views (3P is just a camera). Gate it by input scheme, not view.
- The real fairness problem is information: 3P gives wider awareness (Avowed dev, Chivalry/Mordhau players, below). Industry answers are server toggles (MP) and, in SP, off-screen attacker rules (tokens that favour in-view enemies, reduced out-of-view damage).

## 1. Per-game table

| Game | Views | Same rules? | Per-view differences | Off-screen fairness | Source |
|---|---|---|---|---|---|
| Avowed | 1P + 3P (3P planned from start, accessibility first) | Yes: "always make sure that it felt the same"; no separate 3P dodge/parry timing | 3P pivots on character, 1P spins body under camera; 3P footwork tuned "a little more fluid"; VFX, blood, slide effects, hit-stop weight differ; 3P "more than twice the work" in animation | Dev: 3P "harder for enemies to slip into your blind spots"; 1P compensates with design (title of a GamesRadar piece says full awareness is needed; text not read) | [V: cgmagonline above]; [V: https://gamedeveloper.com/art/what-obsidian-entertainment-learned-making-avowed-a-first-person-fantasy-game]; [S: https://www.purexbox.com/news/2024/06/xboxs-avowed-is-fully-playable-in-third-person-confirms-obsidian] |
| Chivalry 2 | 1P default + 3P option | Yes (3P is camera; no dev statement) | 3P: wider view, see own full swings, "rushing a 1P player from the blind spot feels like cheating"; some say 3P hurts drag control; no dev statement | Not all servers allow 3P (guide claim); no tokens, MP | [V: https://gamerant.com/chivalry-2-combat-tips-guide/]; [V: https://steamcommunity.com/app/1824220/discussions/0/4932019356829793376] (player claims only) |
| Mordhau | 1P default, 3P toggle (P key) | Yes; feint/morph/chamber/riposte exist for all | 3P wider peripheral view; pre-release report said 3P "balanced so no advantage" (secondhand); 3P controversial in early access | Server flag disables 3P to stop looking round corners | [S: https://twinfinite.net/guides/mordhau-play-in-third-person-how/]; [V: https://twinfinite.net/guides/mordhau-play-in-third-person-how/] fetch blocked, so only [S]; [V: https://www.gamepressure.com/mordhau/combat/zac3ca] (feint/morph/chamber) |
| Bannerlord | 1P + 3P (R key) | Yes; attack/block direction by mouse drag in both [S] | Players: 3P gives awareness; 1P preferred for lancing/archery | none official found; auto-block added by patch | [V: https://www.gamepressure.com/mount-blade-ii-bannerlord/how-to-fight-tips/zcd2a5] (mouse direction only); [S: https://steamcommunity.com/app/261550/discussions/0/2149847049319549753] |
| Skyrim | 1P + 3P, free toggle | Same rules (no source states otherwise) | Players: 3P floaty/choppy, ranged and magic better in 1P; mods auto-switch view for bow/combat | Enemies and hits unchanged; no view rule | [S: https://en.uesp.net/wiki/Skyrim:Combat] (page not opened; search summary only; claim of identical mechanics is [M]) |
| KCD2 | 1P only | n/a | n/a | No lock-on; players use free look to pick other targets [S] | [V: https://exputer.com/interviews/kingdom-come-deliverance-2-plans-after-dlc/]: "first-person mode conveys what we want to achieve most, maximum immersion"; [S: https://gameleap.com/articles/kingdom-come-deliverance-is-third-person-camera-possible] (KCD1, combat claim is the author's, not Warhorse's) |
| Dark Messiah | 1P only | n/a | n/a | Kick/block/parry/adrenaline vs groups [S] | [V: https://m.hexus.net/gaming/features/ds/6203-gameplay-impressions-controls-dark-messiah-might-magic-pc/?page=2] (stance/block/parry; kick not on that page) |
| Dying Light 1/2 | 1P (DL2 stated: studio chose first-person open world; parkour-combat merged) | n/a | PC Gamer: dodge feels unintuitive in 1P | not found | [S: https://gamerant.com/dying-light-2-stay-human-interview-tymon-smektala-parkour-combat-progression/]; DL1 view option unverified [M] |
| Exanima | Top-down | n/a | Character faces cursor; swing by mouse, physics-based | n/a | [S: https://cdn.akamai.steamstatic.com/steam/apps/362490/manuals/Exanima_Quickstart_Guide.pdf] |
| Elden Ring / Dark Souls | 3P | n/a | Lock-on focuses camera and actions on one target, priority by proximity, camera orientation, movement; big enemies' limbs can block view | Off-screen attackers are normal and unlimited without lock-on [M]; no source on FromSoft design intent (a Miyazaki quote circulating is an April Fools parody) | [S: https://uxdesign.cc/improving-elden-rings-lock-on-experience-3207035bea28] |
| Doom 2016 | 1P | n/a | n/a | Token pools per attack type (melee/ranged/charged), request/release, steal if better placed, counts per difficulty; **article says nothing on off-screen** | [V: https://www.gamedeveloper.com/design/cyber-demons-the-ai-of-doom-2016-] |
| Assassin's Creed / Arkham | 3P | n/a | n/a | "Battle circle": max simultaneous attackers cap and an attack rate; enemies wait their turn; no source on off-screen handling | [V: https://code.tutsplus.com/battle-circle-ai-let-your-player-feel-like-theyre-fighting-lots-of-enemies--gamedev-13535t] |

Cross-genre melee-AI source (Sulman, Game Developer): enemies "almost never attack you off screen" (he calls the rare case hard to read); slot system standard, attacks every ~2-3 s; enemies try to stay in the camera's view before attacking; Dead Island 2: "when you are hit outside your field of view you take less damage"; Arkham: an icon flashes above an attacker-to-be [V: https://gamedeveloper.com/design/enemy-design-and-enemy-ai-for-melee-combat-systems].

## 2. What differs per view, by topic (standard practice)

- **Dodge.** Avowed: one timing for both views, no 3P-only dodge [V: cgmagonline]. No source found for roll in 3P vs backstep in 1P in any game with both views. 3P-only games roll (Souls) [M]; 1P-only games use strafe/backstep/block (KCD2, Dark Messiah, Chivalry/Mordhau dodge perk [V: gamepressure Mordhau]). Practice: same i-frame/commit data per move; the *choice* of move follows input and camera, not an extra rule.
- **Targeting.** 3P: lock-on (Souls), with known flaws (switching, big enemies hide the view) [S: uxdesign]. 1P: no lock-on; crosshair/look direction plus free look (KCD2 players [S]); Bannerlord/Mordhau/Chivalry direct attacks by mouse look. Practice: lock-on is a 3P camera/steering aid; hit resolution never depends on it.
- **Telegraph readability.** 1P melee is "a lot more difficult to make readable" (Avowed art lead); fixes were 60 fps motion-trail source animation, real-time trails, higher-fidelity near-camera VFX, 1P-specific hit stop weight [V: gamedeveloper Avowed]. Practice: same timing, view-specific presentation (trails, FOV, hit stop).
- **Commitment/cancel windows.** No source differentiates them per view. Mordhau/Chivalry give feint/morph to every player regardless of camera [V: gamepressure Mordhau; gamerant Chivalry riposte]. Practice: one set of cancel windows.
- **Off-screen attackers.** Three industry tools: (a) tokens/slots cap simultaneous attackers (Doom, AC/Arkham, Sulman) [V]; (b) attackers prefer to stay in the camera's view and almost never strike off-screen [V Sulman]; (c) reduced damage from unseen hits (Dead Island 2) [V Sulman]. None of the fetched sources document a view-aware token variant by name; that is our extension. Souls-likes accept off-screen hits [M].
- **Audio/UI cues.** Arkham flashing icon above the attacker [V Sulman]; Avowed 1P uses screen-edge camera effects and reduces camera animation as an option [V cgmagonline]. Directional audio cues for off-screen attackers: no source fetched [M].
- **View switching.** Free toggle in Mordhau (P), Bannerlord (R), Skyrim, Avowed. Avowed dev: drifting timing between views is felt on switching [V cgmagonline]; so timing must not change on switch. Mid-fight switching restrictions: none found.
- **Mixed-view co-op/MP.** Avowed built animation slots so a 1P player's view and another's 3P view run the same animations; stayed foundational even after co-op was cut [V: gamedeveloper Avowed]. Mordhau/Chivalry: 3P toggle is server-controlled (Mordhau flag, hosts say fairness) [S bisect]; Chivalry players call 3P an edge (blind-spot rushes, mob awareness) while others dispute it [V steam thread, opinions only]. Practice: the host decides; the rules never differ between players.

## 3. Recommended design for us

**One core, rules identical for every view and every actor** (player, NPC, remote player): hit contacts, poise/stagger, parry window, damage, tokens, backstab eligibility, i-frame data per move. Matches decisions 1-3 and 15 in `docs/roadmaps/combat.md`. Timing stays 1P-authoritative like Avowed (our Morrowind clips are the shared source), so no per-view timing copy.

**Per-view profile (data, `sf_action` input layer, not `sf_combat`):**
1. Targeting: 3P lock-on + flick switching (C4c); 1P soft aim, no lock-on. Both feed the same `Command` direction.
2. Dodge *choice* mapping: which `MoveSpec` a dodge input picks (roll vs backstep vs sidestep). Same i-frame/commit rules per move; per-view default is the only difference. Option to offer all moves in both views (recommended, see Q1).
3. Feint/morph: sheet data available in both views (change from decision 7, which gates it to 1P); gated by input scheme.
4. Camera: FOV, shake, trails/hit-stop weight, motion-sickness options (Avowed).
5. Threat cues: attacker indicator, screen-edge cue, directional audio; heavier defaults in 1P.
6. Off-screen attacker rule (the one thing that must be view-aware): in the *token request*, an attacker outside the target's view cone gets a lower priority or longer telegraph floor; unseen hits optionally damage-reduced (Dead Island 2 precedent). This reads the *player's current view* (host-side `ViewState`) but stays a pure AI-side function; it changes nothing in `resolve_contact`. Tunables per decision 17.

**Must differ per view:** camera and presentation; targeting; awareness aids. **Must not differ:** damage, poise, i-frames, windows, parry timing.
MP/co-op: the host picks one of {allow 3P, force 1P} for PvP-style modes; PvE co-op allows mixed views since only information differs, not rules.

## 4. Candidate owner questions

1. **Dodge in 1P: backstep only, or the same roll as 3P?** Recommend both views get both (roll and backstep), default bindings differ (3P roll, 1P backstep/sidestep); no source supports a rule difference, and Avowed argues against timing drift.
2. **Should the player be able to switch view mid-fight?** Recommend yes, instantly, no penalty; the rules do not change on switch (Avowed, Mordhau, Bannerlord).
3. **How hard should off-screen attackers be limited in 1P?** Recommend a soft rule: out-of-view attackers wait longer (token priority) and take a longer telegraph; no hard "never attack from behind". Difficulty slider scales it.
4. **Does 3P gain an awareness advantage we accept in co-op?** Recommend accept in PvE; offer a host option "force 1P" for any future PvP mode.

## 5. Not verified / missing

- No dev statement on third-person advantage or server policy from Torn Banner (Chivalry 2), Triternion (Mordhau), TaleWorlds (Bannerlord); only player/guide claims and a host provider help page.
- Doom 2016 and AC: no source on off-screen enemies; the view-aware token rule is our design, not an industry fact. GDC "Bringing Hell to Life" and Loudy/Campbell talks not watched.
- Avowed dodge: PC Gamer and GamesRadar pieces not readable; whether Avowed tunes dodge identically beyond Paramo's statement is [V cgmagonline] only.
- Skyrim/Dying Light 1/Dark Messiah third-person details: mostly [S]; Dark Messiah kick not confirmed on the fetched page.
- KCD2: Warhorse states immersion only; the "combat would not work in 3P" line is the article author's.
- Elden Ring off-screen attackers and lock-on rationale: no developer source; [M].
- Exanima, Dying Light 2 lock-on absence: not confirmed.
