# Ledger: build throughput (queue, CI, compile time)

Format: `date | what was tried | measured result | verdict | commit or doc`. Newest first. Machine: the owner's laptop
(Ryzen 7 7435HS, 16 threads, 32 GB), unless stated. Workday numbers come from `bash bevyrrowind/tools/cargo_turn.sh
--stats` (`.sessions/build-log.tsv`). How it works now: `superfargoth/docs/build-perf.md`.

2026-11-26 | baseline before BS2/BS3, 7 days of the build log | 2242 commands; cargo 1022 min, waiting 578 min (495 behind other tickets); test 1277 runs / 731 min, `test --workspace` 290 s with nothing compiled (112-1581 s); 317/733 clippy and 389/1277 tests failed; 295 commands compiled Bevy crates | baseline | build-perf.md "Queue, CI and worktrees"
2026-11-26 | BS2 two-phase `cargo test` (build under the ticket, binaries outside it) | scratch crate, 20 s test: a queued build ran in 3 s during the test (before: it would wait 20 s); log splits `test` / `test-run` | kept, default; whole-workday effect: re-read `--stats` after a week | cargo_turn.sh
2026-11-26 | shared target for git worktrees | not built: cargo hashes path crates relative to the workspace root, freshness by mtime, so two checkouts would reuse each other's builds | rejected; own target per worktree (worktree.sh) | build-perf.md
2026-11-26 | BS3 GitHub Actions gate (Windows runners), first run (cold cache) | policy 39 s, clippy (both passes) 11 min 39 s green; test still compiling at 50 min when a docs push to main cancelled it | fixed: no cancelling on main, docs-only pushes skip CI; test time pending | .github/workflows/ci.yml
