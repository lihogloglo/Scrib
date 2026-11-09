# dialogue-actions (sub-agent, 2026-11-08)

Scope: host-supplied action buttons + one-level submenu in `sf_rpg_ui::DialogueView`
(Bevy-free view-model): rows, focus/nav, `DialogueRequest` on activation.

Status: done, written 2026-11-08 without compiling: UNTESTED (owner test pass 2026-11-10).

## Files touched
- superfargoth/crates/sf_rpg_ui/src/dialogue.rs (new: `ActionId`, `ActionOption`, `DialogueAction`,
  `DialogueRequest::Action { action, option }` + `service_open(&[ServiceOffer])`,
  `DialogueFocus::{Action, SubAction}`, `DialogueView::{actions, set_actions, submenu, close_submenu}`;
  nav/focus/settle/fallback extended)
- superfargoth/crates/sf_rpg_ui/src/tests_dialogue.rs (3 new tests, 5 existing untouched)
- superfargoth/docs/specs/ui.md ("Dialogue window view-model": one "Action buttons" paragraph)

## BLOCKER for the workspace build (file not mine)
superfargoth/crates/sf_ui_screens/src/dialogue.rs ~line 131, `focus_entity` matches `DialogueFocus`
exhaustively. Add one arm (action buttons aren't drawn yet):
    DialogueFocus::Action(_) | DialogueFocus::SubAction(_) => None,
No other exhaustive match on `DialogueFocus`/`DialogueRequest` outside sf_rpg_ui (grepped).

## Re-exports for lib.rs (owner of lib.rs adds)
    pub use dialogue::{ActionId, ActionOption, DialogueAction, ...existing...};
Then the `use crate::dialogue::{ActionId, ActionOption, DialogueAction};` line at the top of
tests_dialogue.rs can go (harmless if left: explicit import shadows the glob).

## Unverified guesses / risk spots
- `ActorKey` is Copy (inferred from `ctx.provider` used by value twice in sf_rpg service/host.rs `open`);
  `service_open` writes `provider: offer.provider` without clone.
- `activate`: `match &target { Action(a) => { ...; self.move_focus(Some(target.clone()), false); return ..} }`
  relies on NLL ending the borrow (cloned to be safe).
- Without the lib.rs re-export, `DialogueAction::new`/`from_offer`/builders may draw dead_code warnings
  (I expect not: the type is reachable via `set_actions`), so add the re-export anyway.
- mw_rpg_bevy `Rpg::request` will apply `Action` as a silent no-op until a host slice routes it
  (service_open → ServiceRequest, Persuasion options → `persuade_and_reply`).

## Owner test commands
    cargo test -p sf_rpg_ui
    cargo clippy -p sf_rpg_ui -p sf_ui_screens --all-targets -- -D warnings
    cargo test -p sf_ui_screens
