# a5b-player-cast: HANDOFF (stopped at research, no code written, nothing committed)
Status: stopped by coordinator (usage). Next agent: build from this design.

## Findings (verified in code at ed9144c)
- mw_player_bevy can NOT depend on mw_rpg_bevy (mw_rpg_bevy -> mw_scripts_bevy -> mw_player_bevy). CastRequest lives in
  mw_rpg_bevy/src/cast.rs. Plan: move `CastRequest` into mw_player_bevy (new `magic.rs`), `pub use` it from
  mw_rpg_bevy::cast (add `mw_player_bevy.workspace = true` to mw_rpg_bevy/Cargo.toml: no new lock packages; the other
  agent has a 1-line edit there -> hunk commit). mw_player_bevy needs sf_effects_bevy (+ sf_rpg for PlayerId).
- Integration tests (cast heals/costs magicka) need mw_rpg_bevy's `crate::stats::add` + `effects::test_records::spell_db`
  (see mw_rpg_bevy/src/cast/tests.rs); put them in mw_rpg_bevy (dev-dep path) or mw_saves_bevy, not mw_player_bevy.
- No cast class: sf_action::ActionClass has none. Plan: append `ActionClass::Cast` (bit 13; saves store ClassSet bits
  only, Command.hold_ticks [u16; CLASSES] is not saved), `PlayerCast => Cast` in sf_action_bevy/src/input.rs macro,
  action.md class list. In magic stance a FixedUpdate system in `ActionSystems::Command` (before CombatSystems::Gate)
  remaps Light -> Cast in the command (Morrowind: attack button casts with magic readied; OpenMW treats spell as a
  weapon type). Withheld: Sheathed {Light,Heavy,Guard,Cast}, Drawn {Cast}, Magic {Light,Heavy,Guard}.
- Moveset switch is a trap: rules gate on mode.moveset and the roll is in sword_1h; keep moveset sword_1h, add move
  `cast` (category Attack -> faces view first person, soft aim) to mw_combat/data/movesets/sword_1h.ron, slots
  ["SpellCast@touch"] (graph slot clip exists, mw_humanoid.animgraph.ron:119-123; Morrowind base_anim, no UAL cast clip).
  Release time from the baked clip's `mw_anim::events::CAST_RELEASE` event (like combat.rs StanceClips/ReadyClip).
  OpenMW picks self/touch/target clip by the first effect's range (character.cpp ~1615 at ca5e2576): needs a rule
  predicate, log as gap. Add the slot to player_slots() via sheet_slots (automatic if in the sheet).
- Release detection: FixedUpdate after ActionSystems::Step: state.mov == sheet.rule_key("cast") and
  output.clock (t0,t1] contains release -> CastRequest{HOST, book.selected, target}.
- Cancel on refusal: `state.forced = Some(Forced{rule: RuleKey::of("loco"), side:0})` (base_rules.ron id "loco").
  Cancel only for refusals (NoMana/Recharging/Silenced/NotKnown/...), not Fizzle (cost spent, anim plays as OpenMW).
- OpenMW startSpellCast (worldimp.cpp 2833-2888) spends magicka at anim START and refuses NoMana/PowerUsed there
  (no-mana anim still plays); our spec spends at release (effects.md "Casting"): keep, note the difference.
- Messages: Morrowind button-less messageBox = `sf_ui_bevy::Toasts` push; GMST strings via tes3 GameSettingValue::String
  (write gmst_string beside mw_ai::gmst_float). Fizzle sMagicSkillFail, NoMana sMagicInsufficientSP, Recharging sPowerAlreadyUsed.
- Touch target (OpenMW World::castSpell): focus object (crate::Focus, ScriptHandle -> actor with CharacterPlacement +
  Effects) else getHitContact (fCombatDistance, fCombatAngleXY; combat.cpp ~680) -> Effects.owner.
- Selection: EffectOp::Select via EffectCommand; cycle book.known() filtered by SourceTableRes is_castable, powers then
  spells, sorted by name (db SPEL name) then id.
- Bindings: R is HeavyAttack since C2b. Free keys: C, Z, X, T(dev live_ab uses T/C with conditions), digits.
  Proposal: ready_magic = C (Oblivion's cast key) / pad DPadDown; next/prev spell = X/Z / pad DPadRight/DPadLeft
  (Elden Ring cycles spells on the d-pad; DPadUp is toggle view). Owner question.
- Stance: add `Stance::Magic` in combat.rs (instant; SpellCast@equip/unequip clips are a follow-up); R from Drawn =
  sheathe then magic (queued); show_stance Magic -> Readiness::Sheathed.
- HUD: sf_ui_screens/src/hud.rs HudView (line 64) add `spell: Option<String>` and `guard: GuardCue {None, Up, Broken}`.
