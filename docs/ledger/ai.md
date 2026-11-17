# Ledger: NPC AI (directives, drives, HTN, tiers)

Format: `date | tried | result | verdict | commit or doc`. Newest first. Owner checks are AI- ids in `bevyrrowind/docs/visual-checks.md`; AI1-AI5 results are in `superfargoth/docs/specs/ai-as-built.md`.

2026-11-17 | AI7 drives: IAUS compensated product x weight, momentum as `1 + momentum` on the running drive, minimum commit in ticks (released at once when the running drive's raw score is 0), directive floor only while raw > 0, ties to the lowest record id | 14 exact tests (curves, product, switch ticks, floor, ties, NaN, load report, serde, trace); no cost measured (not in game) | built, headless | ai-as-built.md "AI7"
