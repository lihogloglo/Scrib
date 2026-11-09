# services-sv1 (sub-agent)
Scope: sf_rpg::service SV1 per superfargoth/docs/specs/services.md Build plan row SV1. Edits only superfargoth/crates/sf_rpg/, services.md, services-adapter.md.
Status: started 2026-11-08. No cargo on this machine.
Decisions so far: module superfargoth/crates/sf_rpg/src/service/{mod,trade,purse,price,host}.rs + src/tests_service.rs; no benches dir -> 1,000-stack quote is an #[ignore] test. No exhaustive RpgCommand/RpgEvent matches found outside sf_rpg (sf_ui_inventory view.rs has `_ =>`).
SV1: code written (service/{mod,trade,purse,price,host}.rs, MoveStack/SetStackAttr in state.rs, set_attr/can_add); next tests_service.rs then docs.
Status 2026-11-08: DONE (unbuilt). Tests in src/tests_service.rs; services.md "As built (SV1)" + owner defaults marked; services-adapter.md status + defaults. services.md now ~28 KB (over 25 KB budget; archive move needs a session allowed to create files). Next: a cargo-capable session runs `cargo test -p sf_rpg`, clippy, and the ignored measure_pane_quoting.
