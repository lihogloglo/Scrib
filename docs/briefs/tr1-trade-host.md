# Brief: slice TR1, barter in game (host side, no screens yet)

Written 2026-11-09 by the gameplay coordinator session for the next agent working on NPC services. Paste everything
below the line into a fresh session. It must compile: run every cargo command through
`bash bevyrrowind/tools/cargo_turn.sh`. The owner launches the game, never the agent.

---

You are implementing slice **TR1** in Scrib, a Rust/Bevy open-world RPG engine. The repo root is your working
directory. The framework is `superfargoth/`, game-agnostic `sf_*` crates; the Morrowind pack is `bevyrrowind/`,
`mw_*` crates. One deliverable plus its test. Do not start TR2.

## Where things stand

The parts of barter exist but are not connected:
- Framework trade logic (`sf_rpg::service`, SV1).
- View-models (`sf_rpg_ui` `TradeView`, SV2).
- Bevy screens (`sf_ui_screens` `TradeScreen`, SV3a).
- Morrowind price formulas (`mw_rpg::services`, A1) and merchant rules (`mw_rpg::barter::MwServiceRules`, A2a).
- Dialogue action buttons, with the Persuasion menu routed (`mw_rpg_bevy` `dialogue/persuasion.rs`).

All of it compiles and passes its unit tests (2026-11-09). None of it has been tried in game; the owner's pass is
`bevyrrowind/docs/test-pass-2026-11-10.md`. If that file lists a failure in these crates, fix it first.

## Deliverable (TR1)

In `bevyrrowind/crates/mw_rpg_bevy`:
1. Implement `mw_rpg::MwTradeHost` (`bevyrrowind/crates/mw_rpg/src/barter.rs` ~line 379) over the host's state:
   - `barter_terms`: from the sheets and the derived disposition. See how `Rpg::persuade` builds its inputs in
     `dialogue/persuasion.rs`.
   - `owned_stock`: may stay the default (empty) for now; say so.
   - `service_refusal`: the "Service Refusal" topic filter, with the choice rules in `barter.rs` docs (OpenMW
     rejects every `Choice` condition when the choice is `None`; inverted disposition check).
2. When a dialogue opens with a merchant, add one action button per offer (`MwServiceRules::offers`, then
   `sf_rpg_ui::DialogueAction::from_offer`). Services come before Persuasion, in OpenMW order (`barter.rs`
   `MwService` order). `Rpg::refresh_actions` has a `TODO(SV3)` for this.
3. Route a pressed service button: `Rpg::route_action` has a `TODO(SV3)`.
   - Use `DialogueView::service_open` to get the request, then `sf_rpg::service::host::open` (refusal first, then
     restock the purse).
   - Keep the open `TradeSession` in host state, keyed by player.
   - A refusal is a normal dialogue line.
4. Apply `ServiceRequest`s (Stage, Unstage, SetBalance, Accept, Cancel) to that session, and the `Settlement`'s
   commands and effects (disposition, later skill use) to the world.
5. Headless tests in the crate's existing style:
   - a merchant NPC shows a Barter button;
   - a non-merchant shows none;
   - pressing it opens a session;
   - staging and accepting moves an item and gold;
   - a plain-offer deal raises disposition by 1.

Not in TR1: opening `TradeScreen` and feeding it prices and events (that is **TR2**, with the visual check entry
the SV3a handoff drafted), Mercantile skill use (A2b), stolen goods (A7), list services (A3-A6).

## Read (grep first; read only the named sections)

1. `CONTRIBUTING.md` (short), `bevyrrowind/CLAUDE.md` "Parallel sessions" and "Hard rules".
2. `superfargoth/docs/specs/services.md`: "Trade session (host-owned)", "Dialogue integration", "Commands and
   events", "Co-op". `bevyrrowind/docs/specs/services-adapter.md`: "Offers", "Stock and what a merchant buys",
   "Merchant gold".
3. Handoffs: `bevyrrowind/docs/test-pass-handoffs/` `barter-a2.md`, `services-sv1.md`, `persuasion-menu.md`,
   `sv3-screens.md` (for TR2's needs).
4. Code: `superfargoth/crates/sf_rpg/src/service/{mod,trade,host}.rs`, `bevyrrowind/crates/mw_rpg/src/barter.rs`,
   `bevyrrowind/crates/mw_rpg_bevy/src/dialogue/persuasion.rs` (the pattern to follow), and `dialogue.rs` by range
   (it is large).

## Rules

- **Net-ready (services.md "Co-op"):** requests in, events out, state owned by the host. The view never mutates game
  state.
- **Port Morrowind behaviour from OpenMW** commit `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f` (cite file + commit).
  Add new OpenMW sources to the "OpenMW (barter and service prices)" entry in `superfargoth/THIRD_PARTY_NOTICES.md`.
- **Before you finish:**
  - `cargo test -p mw_rpg_bevy -p mw_rpg -p sf_rpg`;
  - pedantic clippy with `-D warnings` on the crates you touched;
  - `cargo fmt` on your files only.
- **End of task:**
  - ROADMAP 2.7: edit your part of the line only.
  - One CHANGELOG line.
  - A short "As built (TR1)" in `services-adapter.md`, keeping the spec ≤ ~25 KB (history goes to
    `specs/archive/services-adapter-history.md`).
  - Commit with explicit paths (`git commit -- <paths>`); other sessions share the tree.

## After TR1 (queue, one slice each)

- **TR2:** open `TradeScreen` over the dialogue on `ServiceOpened`. Send `pane_prices` on open, after `TradeDone`
  and when stacks change. Route `ServiceScreenEvent` and `ServiceEvent`. Register screen ids. Add visual check
  PUI-SV3 (text in `sv3-screens.md`).
- **A2b:** Mercantile skill use from `HaggleOut::mercantile_scale`.
- **E4:** `sf_effects_bevy` + saves (`superfargoth/docs/specs/effects.md` Build plan).
- **Training / travel / spells / repair:** A3-A6 (services-adapter.md Build plan).
- **MessageBox:** `^PCName` expansion and `Message::report` (MessageBox lowering handoff).
