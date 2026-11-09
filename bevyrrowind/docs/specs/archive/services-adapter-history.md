# services-adapter history

Dated "As built" blocks moved verbatim from [services-adapter.md](../services-adapter.md) (2026-11-09, doc budget).

## As built (A1)

`mw_rpg::services` (`crates/mw_rpg/src/services.rs`, tests `tests_services.rs`, 25 tests): `PartyTerms`, `BarterTerms`,
`barter_offer`, `effective_value`, and the six rules as pure `fn(&In) -> Out` (`barter_price`, `barter_haggle`,
`training_price`, `travel_price`, `spell_price`, `repair_price`; ids in `RULE_IDS`); each `In` holds its GMSTs and has
`In::new(.., &MwGmst)`. The 12 GMSTs above are in `mw_stats::MwGmst` (the fatigue ones already were). Changes from the
design: travel takes `player_pos`/`dest_pos` (fare distance in `f64` as `std::pow` promotes, hours from the `f32` 2D
length); the training skill and attribute are `f32` (OpenMW compares floats); `HaggleOut::disposition_delta` is 0 for
creatures. Not wired (A2). Real-install GMST check: `mw_stats` test `real_install` (ignored) prints every GMST against
`vanilla()`.

**As built (A2a)**, written 2026-11-08 without compiling: UNTESTED (owner test pass 2026-11-10). `mw_rpg::barter`
(`barter.rs`, tests `tests_barter.rs`, 12 tests): `MwServiceRules` over a host `MwTradeHost` (terms, owned stock, the
"Service Refusal" filter); `AllItems` excludes `MagicItems` (no Barter button for it); bound items from `sMagicBound*`.

