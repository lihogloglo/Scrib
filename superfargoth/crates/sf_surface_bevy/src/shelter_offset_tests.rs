//! shelter-offset (owner 2026-11-17, Skaal Village, snow: "some trees have an offset with the
//! area they protect ... sometimes present, sometimes not ... can update suddenly"): an
//! end-to-end CPU twin of one tree's shelter through the real per-level frame
//! (`shelter::level_frame`: triggers, world-snapped centres, `ExposureClock`, the uniform's clip
//! switch, floating-origin shifts), the real view matrices (`ShelterLevel::view`, the clip
//! handed to `sun_view`), a rasteriser at wgpu texel centres, the pack pass (`exposure.wgsl`
//! `main` / `outer_at` through `exposure_texel_in` and `outer_uv_map`) and the layer's read
//! (`layer.wesl` `surface_level_exposure` / `surface_exposure`: uv, row flip, heights over the
//! centre, the lag and its `below_top` gate, the near-to-far hand-over). The ground under the
//! crown must read sheltered and the ground just outside it open, on every frame of a walk from
//! 300 m out to past the tree, through both levels' recentres, vertical recentres and a
//! floating-origin shift.

#![allow(clippy::float_cmp, clippy::cast_precision_loss, clippy::cast_possible_truncation)]

use std::collections::HashMap;

use bevy_math::{IVec2, IVec3, Mat4, Vec2, Vec3, Vec4};
use sf_coords::WorldPos;
use sf_surface::view::{
    ExposureMode, ExposurePass, NEAR_LEVEL, SHELTER_SIZE, ShelterLevel, exposure_texel_in, level_fade, outer_uv_map,
};

use crate::ShelterFarSettings;
use crate::shelter::{LevelState, LevelUniform, level_frame};

const N: i32 = SHELTER_SIZE.cast_signed();
/// `layer.wesl` / `exposure.wgsl` constants.
const NEAR_M: f32 = 0.12;
const FULL_M: f32 = 0.35;
const LAG_TOP_M: f32 = 1.0;
const FADE_START: f32 = 0.875;

fn smoothstep(e0: f32, e1: f32, x: f32) -> f32 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// A tree: a solid crown disc `radius` m wide, `crown` m over flat ground at world y 0.
struct Tree {
    world: Vec2,
    radius: f32,
    crown: f32,
}

#[derive(Clone, Copy)]
struct Texel {
    /// R: the caster's height over the map centre (m).
    r: f32,
    lag: f32,
    fresh: f32,
}

/// One level's render side: the depth map last drawn (sparse: texels around the tree, the rest
/// cleared to 0) and the pack (sparse the same way; a missing texel is an empty, steady one).
#[derive(Default)]
struct Gpu {
    map: HashMap<IVec2, f32>,
    pack: HashMap<IVec2, Texel>,
    valid: bool,
}

struct Level {
    level: ShelterLevel,
    test: f32,
    state: LevelState,
    clip: Mat4,
    shelter: Vec4,
    canopy: Vec4,
    gpu: Gpu,
}

impl Level {
    fn new(level: ShelterLevel, test: f32) -> Self {
        Self { level, test, state: LevelState::default(), clip: Mat4::ZERO, shelter: Vec4::ZERO, canopy: Vec4::ZERO, gpu: Gpu::default() }
    }

    fn empty(&self) -> Texel {
        Texel { r: -self.level.down_m, lag: 0.0, fresh: 1.0 }
    }

    fn texel(&self, a: IVec2) -> Texel {
        self.gpu.pack.get(&a).copied().unwrap_or_else(|| self.empty())
    }

    /// Texels within `reach` m of render point `p` in the uniform's map.
    fn window(&self, p: Vec3, reach: f32) -> Vec<IVec2> {
        let c = self.clip.project_point3(p);
        let t = Vec2::new(c.x * 0.5 + 0.5, -c.y * 0.5 + 0.5) * N as f32;
        let k = (reach / self.level.texel_m()).ceil() as i32 + 2;
        let base = t.floor().as_ivec2();
        let mut out = Vec::new();
        for y in -k..=k {
            for x in -k..=k {
                let a = base + IVec2::new(x, y);
                if a.x >= 0 && a.y >= 0 && a.x < N && a.y < N {
                    out.push(a);
                }
            }
        }
        out
    }

    /// Bevy's shadow pass drawing the request's clip: depth at each covered texel centre (wgpu:
    /// row 0 at NDC y +1), reverse Z, depth clamp.
    fn draw(&mut self, clip: Mat4, tree: &Tree, origin: WorldPos) {
        self.gpu.map.clear();
        let o = origin.to_f64();
        let at = Vec3::new((f64::from(tree.world.x) - o.x) as f32, -o.y as f32, (f64::from(tree.world.y) - o.z) as f32);
        let crown = clip.project_point3(at + Vec3::Y * tree.crown);
        let inv = clip.inverse();
        let t = Vec2::new(crown.x * 0.5 + 0.5, -crown.y * 0.5 + 0.5) * N as f32;
        let k = (tree.radius / self.level.texel_m()).ceil() as i32 + 2;
        for y in -k..=k {
            for x in -k..=k {
                let a = t.floor().as_ivec2() + IVec2::new(x, y);
                if a.x < 0 || a.y < 0 || a.x >= N || a.y >= N {
                    continue;
                }
                let ndc = Vec3::new((a.x as f32 + 0.5) / N as f32 * 2.0 - 1.0, 1.0 - (a.y as f32 + 0.5) / N as f32 * 2.0, crown.z);
                let w = inv.project_point3(ndc);
                if Vec2::new(w.x - at.x, w.z - at.z).length() < tree.radius {
                    self.gpu.map.insert(a, crown.z.clamp(0.0, 1.0));
                }
            }
        }
    }

    /// `exposure.wgsl` `fade_of`.
    fn fade_of(a: IVec2) -> f32 {
        let ndc = ((a.as_vec2() + 0.5) / N as f32 * 2.0 - 1.0).abs();
        level_fade(ndc.x.max(ndc.y))
    }

    /// `exposure.wgsl` `ground` for the far bilinear test (the texel alone), used for both
    /// levels: the near tent is a symmetric blur of the same texels.
    fn ground(&self, a: IVec2) -> f32 {
        let over = self.gpu.map.get(&a).copied().unwrap_or(0.0) * self.level.depth_m();
        1.0 - smoothstep(NEAR_M, FULL_M, over)
    }

    /// `exposure.wgsl` `main` over the texels around the tree; `outer` is the next level out's
    /// shown value at a texel (open sky past the last).
    fn pass(&mut self, pass: ExposurePass, around: Vec3, outer: &dyn Fn(IVec2) -> f32) {
        let mut pass = pass;
        if !self.gpu.valid && pass.mode != ExposureMode::Ease {
            pass.mode = ExposureMode::Fresh;
        }
        if pass.mode == ExposureMode::Ease {
            for t in self.gpu.pack.values_mut() {
                t.lag *= pass.decay;
            }
            return;
        }
        let shift = pass.mode.shift();
        let mut next = HashMap::new();
        for a in self.window(around, 12.0) {
            let r = self.gpu.map.get(&a).copied().unwrap_or(0.0) * self.level.depth_m() - self.level.down_m;
            let fresh = self.ground(a);
            let src = a + shift;
            let held = (src.x >= 0 && src.y >= 0 && src.x < N && src.y < N).then(|| {
                let v = self.texel(src);
                (v.lag, v.fresh, Self::fade_of(src))
            });
            let (lag, fresh) = exposure_texel_in(pass, held, fresh, Self::fade_of(a), outer(a));
            next.insert(a, Texel { r, lag, fresh });
        }
        self.gpu.pack = next;
        self.gpu.valid = true;
    }

    /// `exposure.wgsl` `outer_at` reading this (outer) level's pack at its uv.
    fn shown_at_uv(&self, uv: Vec2) -> f32 {
        if uv.x <= 0.0 || uv.y <= 0.0 || uv.x >= 1.0 || uv.y >= 1.0 {
            return 1.0;
        }
        let t = uv * N as f32 - 0.5;
        let base = t.floor().as_ivec2();
        let b = t - t.floor();
        let shown = |o: IVec2| {
            let c = (base + o).clamp(IVec2::ZERO, IVec2::splat(N - 1));
            let v = self.texel(c);
            v.fresh + (1.0 - v.fresh) * Self::fade_of(c) + v.lag
        };
        let lo = shown(IVec2::new(0, 0)) + (shown(IVec2::new(1, 0)) - shown(IVec2::new(0, 0))) * b.x;
        let hi = shown(IVec2::new(0, 1)) + (shown(IVec2::new(1, 1)) - shown(IVec2::new(0, 1))) * b.x;
        lo + (hi - lo) * b.y
    }

    /// `layer.wesl` `surface_level_exposure` on flat ground (normal up: no offset, no rise),
    /// bilinear: (now, lag past the top gate, edge).
    fn read(&self, p: Vec3) -> Vec3 {
        if self.shelter.x == 0.0 || !self.gpu.valid {
            return Vec3::new(1.0, 0.0, 2.0);
        }
        let c = self.clip * p.extend(1.0);
        let edge = c.x.abs().max(c.y.abs());
        if edge >= 1.0 {
            return Vec3::new(1.0, 0.0, edge);
        }
        let uv = Vec2::new(c.x * 0.5 + 0.5, -c.y * 0.5 + 0.5);
        let t = uv * N as f32 - 0.5;
        let base = t.floor().as_ivec2();
        let b = t - t.floor();
        let height = c.z * self.shelter.y - self.canopy.z;
        let bil = |f: &dyn Fn(Texel) -> f32| {
            let v = |o: IVec2| f(self.texel(base + o));
            let lo = v(IVec2::new(0, 0)) + (v(IVec2::new(1, 0)) - v(IVec2::new(0, 0))) * b.x;
            let hi = v(IVec2::new(0, 1)) + (v(IVec2::new(1, 1)) - v(IVec2::new(0, 1))) * b.x;
            lo + (hi - lo) * b.y
        };
        let now = bil(&|v| 1.0 - smoothstep(NEAR_M, FULL_M, v.r - height));
        let lag = bil(&|v| v.lag);
        let top = self.texel((uv * N as f32).floor().as_ivec2()).r;
        let below_top = smoothstep(FULL_M, LAG_TOP_M, (top - height).abs());
        Vec3::new(now, lag * below_top, edge)
    }
}

/// `layer.wesl` `surface_exposure`.
fn shown(near: &Level, far: &Level, p: Vec3) -> f32 {
    let n = near.read(p);
    let near_fade = smoothstep(FADE_START, 1.0, n.z);
    let mut outer = 1.0;
    if near_fade > 0.0 && far.shelter.x != 0.0 {
        let f = far.read(p);
        outer = (f.x + (1.0 - f.x) * smoothstep(FADE_START, 1.0, f.z) + f.y).clamp(0.0, 1.0);
    }
    (n.x + (outer - n.x) * near_fade + n.y).clamp(0.0, 1.0)
}

/// One main-world frame of `update_shelter` for both levels, then the render: the passes (far
/// first), then this frame's redraws.
fn frame(near: &mut Level, far: &mut Level, tree: &Tree, origin: WorldPos, cam: Vec3, dt: f32) {
    let o = origin.to_f64();
    let around = Vec3::new((f64::from(tree.world.x) - o.x) as f32, -o.y as f32, (f64::from(tree.world.y) - o.z) as f32);
    let run = |l: &mut Level| {
        let u = LevelUniform { clip: &mut l.clip, shelter: &mut l.shelter, canopy: &mut l.canopy };
        level_frame(&mut l.state, &l.level, l.test, origin, Some(cam), true, dt, false, u)
    };
    let (far_pass, far_redraw) = run(far);
    let (near_pass, near_redraw) = run(near);
    if let Some(p) = far_pass {
        far.pass(p, around, &|_| 1.0);
    }
    if let Some(p) = near_pass {
        let (scale, offset) = outer_uv_map(near.clip, near.level.half_m, far.clip, far.level.half_m);
        let far_ref = &*far;
        let far_on = far.shelter.x > 0.0 && far.gpu.valid;
        near.pass(p, around, &|a| {
            if !far_on {
                return 1.0;
            }
            far_ref.shown_at_uv((a.as_vec2() + 0.5) / N as f32 * scale + offset)
        });
    }
    if let Some((_, clip)) = far_redraw {
        far.draw(clip, tree, origin);
    }
    if let Some((_, clip)) = near_redraw {
        near.draw(clip, tree, origin);
    }
}

/// The ground under a tree reads sheltered and the ground just outside its crown open on every
/// frame of a walk from 300 m out to past it (5, 60, 200 m and everything between), through the
/// near level's 16 m and the far level's 64 m recentres, vertical recentres and a floating-origin
/// shift: the levels' carries, clocks, uniforms and reads agree on where the world point is.
#[test]
fn tree_shelter_stays_under_the_tree_through_recentres() {
    let tree = Tree { world: Vec2::new(1500.3, -2490.7), radius: 2.5, crown: 6.0 };
    let mut near = Level::new(NEAR_LEVEL, 0.0);
    let mut far = Level::new(ShelterFarSettings::default().level(), 1.0);
    let mut origin = WorldPos::new(IVec3::new(5, 0, -10), Vec3::new(0.03, 0.0, -0.07));
    let start = Vec3::new(300.0, 1.7, 40.0);
    let end = Vec3::new(-60.0, 1.7, -10.0);
    let steps = 720;
    let dt = 0.1;
    let mut failures = Vec::new();
    let mut checked = 0;
    let mut shifted = false;
    for i in 0..=steps {
        let s = i as f32 / steps as f32;
        // The camera relative to the tree (world), bobbing ±12 m vertically for the Y recentres.
        let rel = start.lerp(end, s) + Vec3::Y * 12.0 * (s * 40.0).sin();
        if !shifted && i == steps / 2 {
            // One cell east (the render space moves 256 m under the maps).
            origin = WorldPos::from_f64(origin.to_f64() + bevy_math::DVec3::new(256.0, 0.0, 0.0));
            shifted = true;
        }
        let o = origin.to_f64();
        let render = |w: Vec2, y: f32| Vec3::new((f64::from(w.x) - o.x) as f32, y - o.y as f32, (f64::from(w.y) - o.z) as f32);
        let cam = render(tree.world + Vec2::new(rel.x, rel.z), rel.y);
        frame(&mut near, &mut far, &tree, origin, cam, dt);
        if i < 2 {
            continue;
        }
        let d = Vec2::new(rel.x, rel.z).length();
        let under = [Vec2::new(0.3, -0.4), Vec2::new(-1.0, 0.8), Vec2::new(0.9, 0.9)];
        let beside = [Vec2::new(4.5, 0.0), Vec2::new(0.0, -4.5), Vec2::new(-3.3, 3.3), Vec2::new(-4.5, 0.0), Vec2::new(0.0, 4.5)];
        for u in under {
            let v = shown(&near, &far, render(tree.world + u, 0.0));
            checked += 1;
            if v > 0.02 {
                failures.push(format!("frame {i} camera {d:.1} m: under {u} shows {v:.3}"));
            }
        }
        for b in beside {
            let v = shown(&near, &far, render(tree.world + b, 0.0));
            checked += 1;
            if v < 0.98 {
                failures.push(format!("frame {i} camera {d:.1} m: beside {b} shows {v:.3}"));
            }
        }
    }
    assert!(shifted && checked > 5000);
    assert!(failures.is_empty(), "{} of {checked} reads wrong, first: {:#?}", failures.len(), &failures[..failures.len().min(12)]);
}
