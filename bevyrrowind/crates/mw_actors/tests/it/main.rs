//! All of `mw_actors`'s integration tests as ONE test binary (ROADMAP 1.14): each file in this folder is a module,
//! so the crate links one test executable where it linked one per file. Run one file's tests with
//! `cargo test -p mw_actors --test it <file>::`; the `#[ignore]` measurements with
//! `cargo test -p mw_actors --test it <file>:: -- --ignored --nocapture`. A new test file needs its `mod` line here.

mod equipment_audit;
mod held_install;
mod outfit_memory;
mod real_install;
