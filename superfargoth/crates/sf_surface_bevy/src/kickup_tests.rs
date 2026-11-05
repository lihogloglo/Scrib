//! Kick-up (W7): the shader links and validates per preset, its constants and uniform layout match
//! the CPU twin, the burst mesh, the packing, and the headless plugin turning stamps into bursts.
#![allow(clippy::float_cmp, clippy::cast_precision_loss, clippy::cast_sign_loss, clippy::cast_possible_truncation, clippy::cast_possible_wrap, reason = "exact layouts and small test indices")]

use std::time::Duration;

use bevy_camera::Camera;
use bevy_math::{DVec2, DVec3, Vec2, Vec3, Vec4};
use bevy_mesh::VertexAttributeValues;
use bevy_render::render_resource::ShaderType;
use bevy_time::Time;
use bevy_transform::components::GlobalTransform;
use sf_surface::Channel;
use sf_surface::cell_of;
use sf_surface::kickup::{BURSTS, Burst, BurstRing, FLOOR_BELOW_M, KickKind, KickLook, RANGE_M, SEED_STEP, SPRITES};
use sf_surface::trail::FOOT_HALF_M;
use sf_weather_bevy::WorldSurface;

use crate::kickup::*;
use crate::shader_tests::{HIGH, LOW, compile, with};
use crate::tests::{app, id};
use crate::{SurfaceOrigin, SurfaceSettings, SurfaceUniform, SurfaceWeatherCamera, TrailStamp};

const MODULE: &str = "sf_surface_bevy::kickup";

#[test]
fn shader_compiles_per_preset() {
    let mut failures = Vec::new();
    let jobs = [
        ("low", with(LOW, &[])),
        ("high", with(HIGH, &[])),
        ("high, probes in an array", with(HIGH, &["MULTIPLE_LIGHT_PROBES_IN_ARRAY"])),
        ("ultra", with(HIGH, &["GI_PROBES", "MULTIPLE_LIGHT_PROBES_IN_ARRAY"])),
    ];
    for (preset, defs) in jobs {
        match compile(MODULE, &defs) {
            // The sky term and the soft fade are linked in exactly when their defs are on.
            Ok(wgsl) if wgsl.contains("diffuse_environment_map") != defs.contains(&"ENVIRONMENT_MAP") => {
                failures.push(format!("[{preset}] environment map linked = {}", !defs.contains(&"ENVIRONMENT_MAP")));
            }
            Ok(wgsl) if wgsl.contains("depth_prepass_texture") != defs.contains(&"DEPTH_PREPASS") => {
                failures.push(format!("[{preset}] soft fade linked = {}", !defs.contains(&"DEPTH_PREPASS")));
            }
            Ok(wgsl) if !wgsl.contains("textureSampleCompareLevel") => failures.push(format!("[{preset}] no shadow tap")),
            Ok(_) => {}
            Err(e) => failures.push(format!("[{preset}] {e}")),
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

#[test]
fn shader_mirrors_the_cpu_twin() {
    let src = include_str!("kickup.wesl");
    let squash = |s: &str| s.split_whitespace().collect::<String>();
    let src = squash(src);
    assert!(src.contains(&format!("bursts:array<KickBurst,{BURSTS}>")), "BURSTS");
    assert!(src.contains(&squash(&format!("const FLOOR_BELOW_M: f32 = {FLOOR_BELOW_M};"))), "FLOOR_BELOW_M");
    assert!(src.contains(&squash(&format!("const RANGE_M: f32 = {RANGE_M:.1};"))), "RANGE_M");
    let step = format!("vec3<f32>({},{},{})", SEED_STEP.x, SEED_STEP.y, SEED_STEP.z);
    assert!(src.contains(&format!("constSEED_STEP:vec3<f32>={step};")), "SEED_STEP {step}");
    // The uniform: two kinds of four vec4, the wind, BURSTS bursts of three vec4.
    assert_eq!(KickupParams::min_size().get(), (2 * 4 * 16 + 16 + BURSTS * 3 * 16) as u64);
}

#[test]
fn burst_mesh_covers_every_slot_and_sprite() {
    let mesh = burst_mesh(BURSTS as u32, SPRITES);
    let quads = BURSTS * SPRITES as usize;
    assert_eq!(mesh.count_vertices(), quads * 4);
    assert_eq!(mesh.indices().unwrap().len(), quads * 6);
    let Some(VertexAttributeValues::Float32x4(kick)) = mesh.attribute(ATTRIBUTE_KICK) else { panic!("kick attribute") };
    assert_eq!(kick[0], [0.0, 0.0, 0.0, 0.0]);
    assert_eq!(kick.last().unwrap()[..2], [(BURSTS - 1) as f32, (SPRITES - 1) as f32]);
    let Some(VertexAttributeValues::Float32x3(seeds)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) else { panic!("seeds") };
    assert!(seeds.iter().flatten().all(|s| (0.0..1.0).contains(s)));
    // A burst's sprites spread over the azimuth: one per 1/SPRITES sector.
    for (s, v) in seeds.iter().step_by(4).take(SPRITES as usize).enumerate() {
        let sector = (v[0] * SPRITES as f32).floor() as usize;
        assert_eq!(sector, s);
    }
}

#[test]
fn packing_writes_live_slots_and_marks_the_rest_empty() {
    let mut ring = BurstRing::default();
    let b = Burst { at: DVec3::new(105.0, 2.0, -7.0), forward: Vec2::Y, kind: KickKind::Ash, strength: 0.5, born_s: 10.0, seed: 0.25 };
    ring.spawn(b);
    ring.spawn(Burst { born_s: 10.5, ..b });
    let packed = pack_bursts(&ring, DVec3::new(100.0, 0.0, 0.0), 11.0);
    assert_eq!(packed[0], KickBurstGpu { a: Vec4::new(5.0, 2.0, -7.0, 1.0), b: Vec4::new(0.0, 1.0, 0.5, 1.0), c: Vec4::new(0.25, 0.0, 0.0, 0.0) });
    assert_eq!(packed[1].a.w, 0.5);
    assert!(packed[2..].iter().all(|p| *p == EMPTY_BURST));
    let kinds = KickupParams::default().kinds;
    assert_eq!(kinds[1].motion.x, KickLook::ASH.life_s);
    assert_eq!(kinds[0].color.w, KickLook::SNOW.opacity);
}

#[test]
fn stamps_on_snow_start_bursts_that_age_out() {
    let mut app = app();
    app.init_resource::<Time>();
    app.world_mut().spawn((Camera::default(), SurfaceWeatherCamera, GlobalTransform::from_translation(Vec3::new(10.0, 0.0, -3.0))));
    app.update();
    let origin = app.world().resource::<SurfaceOrigin>().0;
    let at = |x: f32, z: f32| origin.offset(Vec3::new(x, 0.0, z));
    let stamp = |p| TrailStamp { at: p, forward: Vec2::new(0.0, 2.0), half_m: FOOT_HALF_M, depth: 1.0 };
    let near = at(12.0, -3.0).to_f64();
    {
        let mut surface = app.world_mut().resource_mut::<WorldSurface>();
        surface.state.set_cell(&id("ws"), cell_of(DVec2::new(near.x, near.z)), Channel::Snow, 1.0);
        surface.touch();
    }
    let stats = |app: &App| *app.world().resource::<KickupStats>();
    // Snow in the cell but no snow look drawn: nothing thrown.
    app.world_mut().write_message(stamp(at(12.0, -3.0)));
    app.update();
    assert_eq!(stats(&app).spawned, 0);
    // With the look: the near stamp throws, the far one (50 m) doesn't, nor bare ground.
    app.world_mut().resource_mut::<SurfaceUniform>().cover[0] = Vec4::new(0.25, 0.15, 1.0, 1.0);
    app.world_mut().write_message(stamp(at(12.0, -3.0)));
    app.world_mut().write_message(stamp(at(60.0, -3.0)));
    app.world_mut().write_message(stamp(at(10.0, 200.0)));
    app.update();
    assert_eq!((stats(&app).spawned, stats(&app).live), (1, 1));
    let burst = *app.world().resource::<KickupState>().ring.get(0).unwrap();
    assert_eq!(burst.kind, KickKind::Snow);
    assert_eq!(burst.forward, Vec2::Y);
    assert!(burst.strength > 0.0 && burst.strength <= 1.0);
    // Aged past the snow look's longest life: gone.
    app.world_mut().resource_mut::<Time>().advance_by(Duration::from_secs_f32(KickLook::SNOW.max_age_s() + 0.05));
    app.update();
    assert_eq!(stats(&app).live, 0);
    // `--off kickup`: nothing thrown.
    app.world_mut().resource_mut::<KickupSettings>().on = false;
    app.world_mut().write_message(stamp(at(12.0, -3.0)));
    app.update();
    assert_eq!((stats(&app).spawned, stats(&app).live), (1, 0));
    // Cover off (`--off cover`) also stops it.
    app.world_mut().resource_mut::<KickupSettings>().on = true;
    app.world_mut().resource_mut::<SurfaceSettings>().cover = false;
    app.world_mut().write_message(stamp(at(12.0, -3.0)));
    app.update();
    assert_eq!(stats(&app).spawned, 1);
}
