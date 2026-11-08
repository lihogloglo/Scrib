# effects-e1 (sub-agent, 2026-11-08)

Scope: sf_effects slice E1 (core) per superfargoth/docs/specs/effects.md Build plan row E1.
Status: DONE (written, NOT compiled: this machine cannot run cargo). Nothing committed.

Files: superfargoth/crates/sf_effects/{Cargo.toml, src/lib.rs, def.rs, active.rs, op.rs, rules.rs, step.rs, tests.rs};
root Cargo.toml (+sf_effects dep line); .config/hakari.toml (+"sf_effects" in final-excludes);
effects.md (status agreed, table trimmed, "As built (E1)", questions marked; 26 099 B);
effects-adapter.md (status, questions marked).

Handoff: owner runs `cargo hakari generate && cargo hakari manage-deps`, then
`cargo test -p sf_effects`, clippy pedantic, `cargo fmt` (some lines > 120 cols).
Decisions listed in effects.md "As built (E1)".
