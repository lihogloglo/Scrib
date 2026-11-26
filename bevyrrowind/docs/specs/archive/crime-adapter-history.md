# crime-adapter history

Text moved verbatim from [crime-adapter.md](../crime-adapter.md) (2026-11-26, LW0e): slice A6 was dropped
(crime roadmap decision 14) and `awarenessCheck` is ported by ai-adapter MA5 in `mw_ai`. OpenMW
`ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`, **MM** = `apps/openmw/mwmechanics/mechanicsmanagerimp.cpp`.

## Stealth and awareness (OpenMW reading, 2026-11-10)

`isSneaking` (MM 372-382): the Sneak stance and (the character controller reports sneaking, or the actor is airborne
and not swimming or flying). Ours: the `Sneak` mode flag (action.md) is the stance; the airborne clause is classic.
`awarenessCheck(ptr, observer)` (MM 1603-1674), integer rolls, **dead or disabled observers never notice**:

```
sneakTerm = 0; if sneaking: sneakTerm = fSneakSkillMult*Sneak + 0.2*Agility + 0.1*Luck + bootWeight*fSneakBootMult
            (bootWeight = boots slot weight, NPCs on the ground only)
distTerm  = fSneakDistanceBase + fSneakDistanceMultiplier * distance
x         = sneakTerm * distTerm * fatigueTerm + Chameleon;   if Invisibility > 0: x += 100
obsTerm   = obsSneak + 0.2*obsAgility + 0.1*obsLuck - Blind
y         = obsTerm * obsFatigueTerm * (angle(observer facing, to target) > 90 deg ? fSneakNoViewMult : fSneakViewMult)
noticed   = roll0to99 >= x - y          (cached roll variant for repeat checks)
```

(A character not sneaking has `x = Chameleon` only.) Classic (MA5): `concealment = x - y` against `sf_ai`'s one
seeded roll per sight check. Meter (MA5, ai-adapter "Perception profile"): `y` moves into the observer's
`PerceptionProfile`, `concealment = x`, facing is `f_cone`. The `useCache` variant (`awarenessRoll`) is the AI lane's.

GMSTs (expected Morrowind.esm values, U = from memory, unverified):

| GMST | Use | Expected |
|---|---|---|
| `fSneakSkillMult`, `fSneakBootMult`, `fSneakDistanceBase`, `fSneakDistanceMultiplier`, `fSneakViewMult`, `fSneakNoViewMult` | awareness | U (~0.75, ~0.5, ~0.5, ~0.002, ~1.5-3, ~0.5-1) |

Test GMSTs: `fSneakSkillMult` 0.75, `fSneakBootMult` 0.5, `fSneakDistanceBase` 0.5, `fSneakDistanceMultiplier` 0.002,
`fSneakViewMult` 1.5, `fSneakNoViewMult` 0.5, `fFatigueTerm` 1.

- **Awareness** (MA5's cases, run in `mw_ai`): sneaking, Sneak 40 Agi 50 Luck 50, no boots, distance 500: `x = 45 x 1.5 = 67.5`; observer
  Sneak 30 Agi 40 Luck 40 in front: `y = 63`, target 4.5, roll 4 safe, 5 seen; behind: `y = 21`, target 46.5, roll 46 safe,
  47 seen; not sneaking always seen; Invisibility: 104.5 never seen; dead observer never.

Moved verbatim from crime-adapter.md on 2026-11-26 (LW0f, size); the crime roadmap §2 keeps the current state.

## What exists in the repo (2026-11-10)

- `mw_stats` key `bounty`; S7 `GetPCCrimeLevel`/`SetPCCrimeLevel`/`ModPCCrimeLevel` (floor 0) are live but skip
  `recordCrimeId` at 0 (`statsextensions.cpp` 463-493 call it): A2 closes this.
- `functions.txt`: `PayFine`, `PayFineThief`, `GoToJail` `later`; `GetPCInJail` `stub`; `Lock`, `Unlock`,
  `GetLocked` `live` (check them against "Locks").
- `DerivedDisposition::input`: `crime_modifier` is a named TODO (A5); `fDispCrimeMod x bounty` is applied (MM 544).
