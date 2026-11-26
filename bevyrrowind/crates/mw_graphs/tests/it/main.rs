//! All of `mw_graphs`'s integration tests as ONE test binary (ROADMAP 1.14): each file in this folder is a module,
//! so the crate links one test executable where it linked one per file. Run one file's tests with
//! `cargo test -p mw_graphs --test it <file>::`; the `#[ignore]` measurements with
//! `cargo test -p mw_graphs --test it <file>:: -- --ignored --nocapture`. A new test file needs its `mod` line here.

mod gait_set;
mod look_at;
mod real_install;
mod slope_walk;
mod torch_walk;
mod walk_ladder;
