# Ledger: interaction (smart objects, claims, carry, destruction, doors, world rules)

Format: `date | tried | result | verdict | commit or doc`. Newest first. Owner checks are IN- ids in `bevyrrowind/docs/visual-checks.md`.

2026-11-25 | IN0a `sf_interaction` (Bevy-free, hakari final-excludes): def types + `Claims` (priority preempts Reserved only, queue tickets, lease/renew, exclusion groups as bit masks, one body hold + `hand_max` hand claims within `reach`, typed refusals, invalidation by scope, sorted `apply_batch`), `InteractParams` defaults | 26 exact tests pass; clippy -D warnings, deny, hakari, tunables census clean; deps glam/serde/smallvec/sf_ids/sf_records/sf_world, no Bevy | kept; IN0b next (index, verbs, entrances, cook) | scaffold in 24d230db, rest uncommitted; spec "As built (IN0a)"
2026-11-25 | IN27 spec update from the lane roadmap + gta6.md (hand claims for seated chains, stable focus, contact points); spec split into core + carry/destruction/vehicles files | doccheck in27b OK (534 lines kept) | kept | `superfargoth/docs/specs/interaction.md`
