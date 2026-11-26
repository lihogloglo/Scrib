# Non-lethal outcome and mercy: research (C-R3, 2026-11-26)

Owner direction (fixed): fists damage health (Cyberpunk close combat); non-lethal damage taking health to 0 = knockdown/unconscious, not death; a mercy system like Cyberpunk 2077 (spare or finish a defeated or yielding enemy). Morrowind's fatigue-first fists are Classic-only.
Sources are community guides and wikis (official pages were unreachable or paywalled); see section 6.

## 0. Verdict

- Shipped games converge on: **the damage type or weapon decides lethal vs non-lethal, decided at the hit that empties health, and the result is a downed state, not a death.**
- A downed foe stays **vulnerable**: more damage kills (CP2077), or a deliberate finishing action does (KCD, Dishonored, Skyrim bleedout). Mercy is the *absence* of the finishing action, so it costs no UI.
- Human enemies **yield only in games with a morale layer** (Skyrim, KCD, RDR2, Mount & Blade). CP2077 enemies do not surrender; you drain health with non-lethal damage.
- Downed bodies are permanent-but-fragile (CP2077) or recover after a timer/area reload (Skyrim yield, M&B wounded).
- Finishing a helpless or yielded person is a social/crime event wherever a crime system exists (KCD, RDR2, Skyrim).

## 1. Cyberpunk 2077

- **What is non-lethal:** blunt melee (bats, hammers; "Blunt" trait), fists, Gorilla Arms, and a weapon/optics mod that makes a weapon's damage non-lethal (guides; Nexus mod 9360 is the mod version of a global toggle). A stealth grab offers knock out (non-lethal) or kill. Stun/sleep gadgets and some cyberware mods also exist (gamepressure guide).
- **Decision point:** per weapon trait, evaluated when the hit takes health to 0: lethal weapon = death, non-lethal = unconscious. Takedowns are a per-action prompt. Mixed play works: shoot health down, finish with blunt (Twinfinite).
- **Unconscious = state at health 0:** any further damage kills (second swing, bullet, thrown into a container). Damage-over-time quickhacks (Overheat, Contagion) are reported to keep ticking and kill (forum disagreement). Cyberpsychos die from one more hit after reaching zero (Game8). No wake-up reported.
- **Surrender/yield:** none found; community: "they don't give up, you remove the whole bar with non lethal stuff". Spare vs kill is therefore just not hitting again, plus a finisher prompt at a low-health trigger on the melee key (Nexus mod 10017). No separate spare prompt found.
- **Quests:** whether a quest treats unconscious as dead differs per quest (Steam reports). Story kill/spare choices are dialogue choices, not combat. CDPR line (GamesRadar headline): you can finish the game without killing anyone.
- **Police:** unverified (section 6).

## 2. Others

| Game | Non-lethal trigger | Downed state | Spare/finish | Consequences |
|---|---|---|---|---|
| RDR2 | Unarmed or pistol-butt knockout; lasso + hogtie | Knocked out, wakes; hogtied until released | Bounties alive pay more | Witnesses report crimes, bounty (GamesRadar, Shacknews); honor effects unverified |
| KCD / KCD2 | Fists, blunt weapons; yield | Knocked out; NPCs can be KO'd (535-KO pacifist run, PC Gamer) | **Yield**: either side presses surrender within a few metres; victor lets go, refuses, or takes payment; not vs bandits/Cumans/mortal enemies | Reputation/crime; some quests need kills so players mercy-kill the unconscious (mod page); NPCs sometimes refuse surrender (Steam) |
| Dishonored | Per-action takedown (choke/dart), no weapon flag | Sleeping body, must be hidden | Lethal vs non-lethal per action | Chaos: discovered bodies raise it, non-lethal targets lower it (Dishonored wiki); wake-up unverified |
| Skyrim | Essential/protected flags; brawl; yield | Essential: bleedout then recover; hostile yield near ~10% health in AI mods | Yielded hostile: kill or leave; returns hostile later (mod 79059) | Assault/murder bounty; essential NPCs cannot die (UESP) |
| Mount & Blade | Blunt weapons (fan claim: never kill) | Wounded/captured after battle | Prisoners: recruit, ransom, release, execute | Execution relation penalty unverified |
| Elden Ring / Sekiro | None (contrast) | Posture break only | Deathblow kills | None |
| GTA | Melee knockdown | Ragdoll, recovers | n/a | Wanted level |

## 3. Recurring patterns

- **Where the choice lives:** (a) per-weapon trait (CP2077 blunt, M&B), (b) global/ammo toggle (mods), (c) per-action takedown (CP2077 grab, Dishonored). Most games mix (a) and (c).
- **Downed handling:** permanent-but-fragile (CP2077) vs timed recovery (Skyrim) vs until map reload (Dishonored). Timed recovery needs a rule so wakers do not resurrect quest targets.
- **Yield:** keyed to morale and a health threshold (~10% in Skyrim mods), humans only, and not for factions that refuse (KCD bandits).
- **Crime:** finishing a helpless foe is where games add a consequence (witnessed kill = bounty); non-lethal violence is usually assault, cheaper than murder (Skyrim; others unverified).
- **Co-op:** no shipped source found; host-authoritative state is our design call.

## 4. Recommended shape for us

- **`CombatRules` seam:** `outcome_at_zero(hit) -> Death | Down(kind)`; modern rules read a `lethality: Lethal | NonLethal` flag on the hit. Classic later swaps in fatigue-first.
- **Source of lethality:** attacker-side, resolved per hit from a weapon tag (blunt, fist, gauntlet) plus an optional per-attack override (stealth grab, finisher choice). No global toggle in the core; a "non-lethal weapon mod" is just a tag.
- **Down state:** `Downed { kind: Knockout | Yielded, until }`, health 0. Hits while downed are lethal unless NonLethal (then the timer refreshes, no death). Knockdown via `Owner::Physics` (ragdoll/knockdown clip), stand-up clip on recovery.
- **Mercy:** a Downed or Yielded NPC offers a "Finish" interaction in range (warped finisher, no paired clips). Ignoring it = spare. No explicit spare button.
- **Yield/flee:** AI37 morale emits `Yield` (humans only, health under a tunable threshold, never for fanatic templates); the NPC drops its weapon and cowers, recovers to hostile after a time or when attacked. Optional begging barks.
- **Crime lane:** non-lethal hit on a witnessed non-hostile = assault offence; finishing a Downed/Yielded non-hostile = murder offence; witnessing by proximity/hearing (no view-aware AI). Finishing already-hostile bandits = no offence.
- **Co-op:** Downed state and timer live on the host with character state; finishing is a command validated by the host.

## 5. Open design questions

1. **Per-weapon flag vs global toggle?** Per-weapon tag plus per-attack override: matches CP2077/M&B, keeps the seam simple.
2. **Do NPCs yield/beg?** Yes, humans via AI37 morale; creatures and undead never. The mercy system needs something to spare.
3. **Do downed NPCs wake?** Knockouts wake after a tunable (minutes) unless tied or killed; Yielded recover hostile; essential NPCs always recover. Avoids CP2077 permanent-body clutter and quest softlocks.
4. **Can the player yield?** Yes later, short-range input against yield-capable humans only (KCD); seam now, implementation deferred.
5. **Is finishing a downed foe a crime?** Yes for non-hostile or yielded people with witnesses; no for foes the faction already treats as hostile.
6. **Do DoT effects kill the downed?** No: effects inherit the lethality of the hit that applied them (avoids CP2077's reported DoT deaths).
7. **Quests treating unconscious as dead?** Quest hooks read `Downed` as defeated by default (avoids CP2077's per-quest inconsistency).

## 6. Not verified

- Official CD Projekt docs/patch notes for 2.x non-lethal (only guides and Steam threads found; the fandom wiki returned 402).
- CP2077: whether unconscious enemies ever wake; NCPD heat for non-lethal vs lethal; finisher kill/spare behaviour on downed enemies; Gorilla Arms specifics.
- KCD2 knockout wake time and crime penalties; RDR2 honor for sparing/pleading; Dishonored wake-up; Mount & Blade official capture formula.
- Skyrim yield threshold comes from a mod description, not vanilla data.
- Sources: twinfinite.net/guides/cyberpunk-2077-neutralize-enemies-non-lethal-takedowns-how, game8.co/games/Cyberpunk-2077/archives/428868, gamepressure.com/cyberpunk-2077/can-i-finish-the-game-without-killing/z4c7b8, nexusmods.com/cyberpunk2077/mods/9360 and /10017, kingdomcomedeliverance.wiki.gg/wiki/Basic_Combat, pcgamer.com KCD2 pacifist run, dishonored.fandom.com/wiki/Chaos, uesp.net/wiki/Skyrim:Protected_NPCs, nexusmods.com/skyrim/mods/79059, gamesradar.com RDR2 bounty guide, mountandblade.fandom.com.
