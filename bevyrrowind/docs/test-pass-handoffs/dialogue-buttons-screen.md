# dialogue-buttons-screen (sub-agent, 2026-11-08)

Scope: draw the dialogue action buttons + their one-level submenu in the `sf_ui_screens` dialogue
screen, over the view-model of `dialogue-actions.md`.

Status: done, written 2026-11-08 without compiling: UNTESTED (owner test pass 2026-11-10).

## Files touched
- superfargoth/crates/sf_ui_screens/src/dialogue.rs
  - `sf-dialogue-actions` column spawned in `sf-dialogue-side` before the topics (so above them).
  - `DialogueParts`: `actions: KeyedChildren<ActionId>`, `action_shown` (label, live, has-submenu),
    `submenu: Option<SubmenuParts>`; accessors `action`, `actions`, `submenu` (backdrop), `submenu_option`.
  - `focus_entity`: `Action(a)` → its button, `SubAction(o)` → its option row (placeholder arm gone).
  - `sync_dialogue` (inside the version block, before topics): keyed `sf-dialogue-action` buttons
    (`DialogueTarget(Action)`), classes `disabled` (= `!is_live()`) and `submenu`, `InteractionDisabled`,
    muted label when not live; the submenu is redrawn whole when `view.submenu()` != what was drawn:
    `sf-dialogue-submenu` backdrop (absolute 100% of the window, black 50% alpha, `ZIndex(10)`, blocks the
    pointer) centring `sf-dialogue-submenu-panel` (opaque panel colour) with a title label and
    `sf-dialogue-submenu-option` buttons (`DialogueTarget(SubAction)`, `disabled` class + muted when off).
  - Input: unchanged path. Click/confirm → `Activate` → `on_dialogue_button` → `view.activate`; hover →
    `set_focus`; Back → `view.nav(Back)` closes the submenu. Modality is the view's (`focus_valid`) plus
    the backdrop catching the pointer.
- superfargoth/crates/sf_ui_screens/src/tests.rs (appended at the end): `DialogueRequests` collector +
  `dialogue_action_buttons_and_submenu` (spawn, order, classes, InteractionDisabled, focus → entity,
  pointer opens submenu with focus on first enabled option, topic behind is inert, Back closes and
  despawns the backdrop with focus back on the button, pad confirm on an option sends
  `Action { persuade, Some(admire) }`, plain button sends `Action { barter, None }`, re-sync keeps entities).
- superfargoth/docs/specs/ui.md ("Dialogue window view-model", Action buttons paragraph): one "Screen" line.

## Unverified API guesses / risk spots
- `bevy_color::Alpha::with_alpha` on `Color` and `Color::BLACK` (grepped, bevy_color 0.20 color.rs 612/626).
- Absolute backdrop `left/top 0, 100%` covers the window's padding box (taffy, CSS semantics): visual only.
- `World::get_entity(..).is_err()` in the test after despawn (Result API since 0.15).
- `let id = ActionId::new;` in the test (fn item with `&str` arg, late-bound lifetime).
- The `submenu` class string is a plain word; a theme rule `.sf-dialogue-action.submenu` may want "...".
- `.sf-button` theme colour may override the muted label colour of disabled actions (same as topics today);
  the Morrowind theme has no `.sf-dialogue-action`/`.sf-dialogue-submenu*` rules yet (game-side CSS, not mine).
- No click-outside-to-close on the backdrop (Back/Esc closes): would need an entity observer; follow-up.

## Owner test commands
    cargo test -p sf_ui_screens dialogue
    cargo clippy -p sf_ui_screens --all-targets -- -D warnings
    cargo dev view "Seyda Neen"   (DJ-2 below; needs the host to call `refresh_actions`)

## For the orchestrator
CHANGELOG line:
- 2026-11-08 sf_ui_screens: dialogue action buttons above the topic list and their modal submenu panel (keyed, disabled look, focus, pointer + pad), headless test; UNTESTED (owner pass 2026-11-10).

Visual-check entry for bevyrrowind/docs/visual-checks/player-ui.md (after DJ-1):

### DJ-2: Dialogue action buttons and the Persuasion submenu (`cargo dev view "Seyda Neen"`) (awaiting owner)
- Talk to Fargoth (E): a "Persuasion" button sits above the topic list, same look as the topics; Goodbye stays under it.
- Click it (or pad: up from the first topic, A): the window dims and a centred panel lists Admire, Intimidate,
  Taunt and the three Bribes; bribes you can't afford are greyed and skipped by the pad/arrows.
- With the panel open, topics, links and Goodbye behind it don't react to hover or clicks.
- Esc / B closes the panel, focus back on Persuasion. Picking Admire closes it too (the reply/disposition
  change needs the host routing slice; until then nothing else happens, no crash).
- Focused rows show the hover colour with arrows/pad, no bright outline with the mouse (as uipolish).

Index line for visual-checks.md:
DJ-2 | awaiting owner | [player-ui](visual-checks/player-ui.md) | Dialogue action buttons, Persuasion submenu modal
