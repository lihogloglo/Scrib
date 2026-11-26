//! All of `mw_bake`'s integration tests as ONE test binary (ROADMAP 1.14): each file in this folder is a module,
//! so the crate links one test executable where it linked one per file. Run one file's tests with
//! `cargo test -p mw_bake --test it <file>::`; the `#[ignore]` measurements with
//! `cargo test -p mw_bake --test it <file>:: -- --ignored --nocapture`. A new test file needs its `mod` line here.

mod actor_fx_real_install;
mod anim_cook_survey;
mod anim_real_install;
mod anim_sources_real_install;
mod attack_clips_survey;
mod census_real_install;
mod collision_real_install;
mod creature_parts_real_install;
mod creature_rigs_real_install;
mod creature_root_motion_real_install;
mod gait_diagnosis;
mod gait_heading;
mod generated_arms;
mod generated_pose;
mod generated_real_install;
mod interact_real_install;
mod lafan1_retarget_real_install;
mod leg_snap;
mod loop_cut;
mod mantle_grip_real_install;
mod particles_real_install;
mod rewrap_real_install;
mod roll_travel_real_install;
mod scene_cells_real_install;
mod seat_fit_survey;
mod skins_real_install;
mod terrain_layers_real_install;
mod texture_sources;
mod thigh_snap;
