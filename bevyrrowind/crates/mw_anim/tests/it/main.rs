//! All of `mw_anim`'s integration tests as ONE test binary (ROADMAP 1.14): each file in this folder is a module,
//! so the crate links one test executable where it linked one per file. Run one file's tests with
//! `cargo test -p mw_anim --test it <file>::`; the `#[ignore]` measurements with
//! `cargo test -p mw_anim --test it <file>:: -- --ignored --nocapture`. A new test file needs its `mod` line here.

mod kf_curve_conventions;
mod kf_snaps;
mod morph_real_install;
mod real_install;
mod rig_real_install;
