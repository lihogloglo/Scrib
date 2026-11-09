# disposition-wiring session (sub-agent)
Scope: bevyrrowind/crates/mw_rpg_bevy/src/ (dialogue.rs, maybe persuasion.rs + lib.rs), small additions to mw_rpg/src/{disposition.rs,persuasion.rs,lib.rs}, docs/specs/stats-adapter.md (open Q2 + As built), docs/specs/rpg-adapter.md (As built persuasion).
Status: started 2026-11-08, reading sources.
- 2026-11-08: read sources + OpenMW dialoguemanagerimp.cpp @ca5e2576 (persuade, goodbyeSelected, updateOriginalDisposition: the session tracks OpenMW's *base* disposition = our record base + NpcStates modifier). Plan: PersuasionSession + persuasion_roll (splitmix64) + speechcraft_use in mw_rpg persuasion.rs; DispositionCtx/DerivedDisposition in dialogue.rs; Rpg::persuade in new child module mw_rpg_bevy/src/dialogue/persuasion.rs (child so it sees Rpg's private fields).
- mw_rpg persuasion.rs: PersuasionSession (refresh/attempt/goodbye), splitmix64 + persuasion_roll, speechcraft_use, inline session_tests (roll values from a PowerShell BigInteger splitmix64; splitmix64(0) checked = 0xE220A8397B1DCDAF). lib.rs re-exports. Next: dialogue.rs DispositionCtx/DerivedDisposition, then dialogue/persuasion.rs.
- Specs updated: stats-adapter.md (Q2 owner note + As built bullet), rpg-adapter.md (new "As built (persuasion in the host)").

## Handoff (status: code complete, NOT compiled)
Owner to run: cargo test -p mw_rpg persuasion; cargo test -p mw_rpg_bevy dialogue (install-gated tests need Morrowind); clippy pedantic on both.
Watch: fargoths_ring_through_the_window / census / sellus tests now see the derived disposition (raceless player, new-game Personality) instead of record+modifier; a filter outcome may shift.
Files: mw_rpg/src/{persuasion.rs, lib.rs}; mw_rpg_bevy/src/dialogue.rs; mw_rpg_bevy/src/dialogue/persuasion.rs (new child module); docs/specs/{stats-adapter.md, rpg-adapter.md}.
