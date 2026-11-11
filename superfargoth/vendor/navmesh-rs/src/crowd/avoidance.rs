//! Sampled velocity avoidance, translated from DetourObstacleAvoidance.cpp; see NOTICE.
use super::*;
use crate::common::{cosf, sinf};
/// Weights and sampling pattern for obstacle avoidance.
/// Construct with [`Self::default`] or [`Self::quality`] and assign fields.
///
/// ```compile_fail,E0639
/// use navmesh_rs::crowd::ObstacleAvoidanceParams;
/// let params = ObstacleAvoidanceParams { grid_size: 17, ..Default::default() };
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct ObstacleAvoidanceParams {
    /// Bias toward desired velocity, in [0, 1].
    pub vel_bias: f32,
    /// Desired velocity penalty weight.
    pub weight_des_vel: f32,
    /// Current velocity penalty weight.
    pub weight_cur_vel: f32,
    /// Side preference weight.
    pub weight_side: f32,
    /// Time of impact weight.
    pub weight_toi: f32,
    /// Collision horizon in seconds.
    pub horiz_time: f32,
    /// Grid resolution (at least 2).
    pub grid_size: u8,
    /// Adaptive angular divisions (1..=32). Odd counts preserve the defined C++
    /// pattern; even counts use initialized Rust storage where C++ reads an
    /// unwritten sample, and carry no C++ parity claim.
    /// [`UpstreamFixes::ADAPTIVE_PATTERN`] writes that sample instead.
    pub adaptive_divs: u8,
    /// Adaptive ring count (1..=4).
    pub adaptive_rings: u8,
    /// Adaptive refinement depth (1..=32).
    pub adaptive_depth: u8,
}
impl Default for ObstacleAvoidanceParams {
    fn default() -> Self {
        Self {
            vel_bias: 0.4,
            weight_des_vel: 2.,
            weight_cur_vel: 0.75,
            weight_side: 0.75,
            weight_toi: 2.5,
            horiz_time: 2.5,
            grid_size: 33,
            adaptive_divs: 7,
            adaptive_rings: 2,
            adaptive_depth: 5,
        }
    }
}
impl ObstacleAvoidanceParams {
    /// Demo low, medium, good and high quality presets.
    pub fn quality(level: usize) -> Result<Self, Error> {
        if level > 3 {
            return Err(Error::InvalidParameter("avoidance quality"));
        }
        Ok(Self {
            vel_bias: 0.5,
            adaptive_divs: [5, 5, 7, 7][level],
            adaptive_rings: [2, 2, 2, 3][level],
            adaptive_depth: [1, 2, 3, 3][level],
            ..Self::default()
        })
    }
    pub(crate) fn validate(&self) -> Result<(), Error> {
        if !self.vel_bias.is_finite()
            || !(0.0..=1.0).contains(&self.vel_bias)
            || self.grid_size < 2
            || !(1..=32).contains(&self.adaptive_divs)
            || !(1..=4).contains(&self.adaptive_rings)
            || !(1..=32).contains(&self.adaptive_depth)
        {
            return Err(Error::InvalidParameter("avoidance parameters"));
        }
        for v in [
            self.weight_des_vel,
            self.weight_cur_vel,
            self.weight_side,
            self.weight_toi,
        ] {
            nonnegative(v)?;
        }
        positive(self.horiz_time)
    }
}
#[derive(Clone, Copy)]
struct Circle {
    p: V3,
    rad: f32,
    vel: V3,
    dvel: V3,
    dp: V3,
    np: V3,
}
#[derive(Clone, Copy)]
struct Segment {
    p: V3,
    q: V3,
    touch: bool,
}
/// Bounded circles and wall segments for grid or adaptive velocity sampling.
pub struct ObstacleAvoidanceQuery {
    circles: Vec<Circle>,
    segments: Vec<Segment>,
    max_circles: usize,
    max_segments: usize,
    fixes: UpstreamFixes,
}
impl ObstacleAvoidanceQuery {
    /// Selects the upstream corrections sampling applies
    /// ([`UpstreamFixes::ADAPTIVE_PATTERN`]); the default is [`UpstreamFixes::NONE`].
    pub fn set_fixes(&mut self, fixes: UpstreamFixes) {
        self.fixes = fixes;
    }
    /// Allocates obstacle storage.
    pub fn new(ws: &mut Workspace, max_circles: usize, max_segments: usize) -> Result<Self, Error> {
        ws.begin();
        Self::new_in(ws, max_circles, max_segments)
    }
    pub(crate) fn new_in(
        ws: &mut Workspace,
        max_circles: usize,
        max_segments: usize,
    ) -> Result<Self, Error> {
        Ok(Self {
            circles: ws.vec_with_capacity(max_circles)?,
            segments: ws.vec_with_capacity(max_segments)?,
            max_circles,
            max_segments,
            fixes: UpstreamFixes::NONE,
        })
    }
    /// Clears obstacles without reallocating.
    pub fn reset(&mut self) {
        self.circles.clear();
        self.segments.clear();
    }
    /// Adds a moving circle, or returns [`Error::BufferTooSmall`] if full.
    pub fn add_circle(&mut self, p: V3, rad: f32, vel: V3, dvel: V3) -> Result<(), Error> {
        finite(p)?;
        finite(vel)?;
        finite(dvel)?;
        nonnegative(rad)?;
        if self.circles.len() == self.max_circles {
            return Err(Error::BufferTooSmall);
        }
        self.circles.push(Circle {
            p,
            rad,
            vel,
            dvel,
            dp: [0.; 3],
            np: [0.; 3],
        });
        Ok(())
    }
    /// Adds a wall, or returns [`Error::BufferTooSmall`] if full.
    pub fn add_segment(&mut self, p: V3, q: V3) -> Result<(), Error> {
        finite(p)?;
        finite(q)?;
        if self.segments.len() == self.max_segments {
            return Err(Error::BufferTooSmall);
        }
        self.segments.push(Segment { p, q, touch: false });
        Ok(())
    }
    fn prepare(&mut self, pos: V3, dvel: V3) {
        for c in &mut self.circles {
            c.dp = vsub(&c.p, &pos);
            vnormalize(&mut c.dp);
            let dv = vsub(&c.dvel, &dvel);
            c.np = if tri_area_2d(&[0.; 3], &c.dp, &dv) < 0.01 {
                [-c.dp[2], 0., c.dp[0]]
            } else {
                [c.dp[2], 0., -c.dp[0]]
            };
        }
        for s in &mut self.segments {
            s.touch = distance_pt_seg_sqr_2d(&pos, &s.p, &s.q).0 < sqr(0.01);
        }
    }
    #[allow(clippy::too_many_arguments)]
    fn penalty(
        &self,
        cand: V3,
        pos: V3,
        rad: f32,
        vmax: f32,
        vel: V3,
        dvel: V3,
        p: &ObstacleAvoidanceParams,
        min_penalty: f32,
    ) -> f32 {
        let inv = if vmax > 0. { 1. / vmax } else { f32::MAX };
        let vpen = p.weight_des_vel * (vdist_2d(&cand, &dvel) * inv);
        let vcpen = p.weight_cur_vel * (vdist_2d(&cand, &vel) * inv);
        let min_pen = min_penalty - vpen - vcpen;
        let threshold = (p.weight_toi / min_pen - 0.1) * p.horiz_time;
        if threshold - p.horiz_time > -f32::EPSILON {
            return min_penalty;
        }
        let mut tmin = p.horiz_time;
        let mut side = 0.;
        let mut nside = 0;
        for c in &self.circles {
            let vab = vsub(&vsub(&vscale(&cand, 2.), &vel), &c.vel);
            side += fclamp(
                fmin(vdot_2d(&c.dp, &vab) * 0.5 + 0.5, vdot_2d(&c.np, &vab) * 2.),
                0.,
                1.,
            );
            nside += 1;
            let s = vsub(&c.p, &pos);
            let r = rad + c.rad;
            let cc = vdot_2d(&s, &s) - r * r;
            let a = vdot_2d(&vab, &vab);
            if a < 0.0001 {
                continue;
            }
            let b = vdot_2d(&vab, &s);
            let d = b * b - a * cc;
            if d < 0. {
                continue;
            }
            let a = 1. / a;
            let rd = sqrtf(d);
            let mut lo = (b - rd) * a;
            let hi = (b + rd) * a;
            if lo < 0. && hi > 0. {
                lo = -lo * 0.5;
            }
            if lo >= 0. && lo < tmin {
                tmin = lo;
                if tmin < threshold {
                    return min_penalty;
                }
            }
        }
        for s in &self.segments {
            let mut hit = 0.;
            if s.touch {
                let d = vsub(&s.q, &s.p);
                let n = [-d[2], 0., d[0]];
                if vdot_2d(&n, &cand) < 0. {
                    continue;
                }
            } else {
                let v = vsub(&s.q, &s.p);
                let w = vsub(&pos, &s.p);
                let mut d = vperp_2d(&cand, &v);
                if d.abs() < 1e-6 {
                    continue;
                }
                d = 1. / d;
                hit = vperp_2d(&v, &w) * d;
                if !(0.0..=1.0).contains(&hit) {
                    continue;
                }
                let t = vperp_2d(&cand, &w) * d;
                if !(0.0..=1.0).contains(&t) {
                    continue;
                }
            }
            hit *= 2.;
            if hit < tmin {
                tmin = hit;
                if tmin < threshold {
                    return min_penalty;
                }
            }
        }
        if nside > 0 {
            side /= nside as f32;
        }
        let spen = p.weight_side * side;
        let tpen = p.weight_toi * (1. / (0.1 + tmin * (1. / p.horiz_time)));
        vpen + vcpen + spen + tpen
    }
    #[allow(clippy::too_many_arguments)]
    fn validate_sample(
        &mut self,
        ws: &mut Workspace,
        pos: V3,
        rad: f32,
        vmax: f32,
        vel: V3,
        dvel: V3,
        p: &ObstacleAvoidanceParams,
        work: u64,
    ) -> Result<(), Error> {
        finite(pos)?;
        finite(vel)?;
        finite(dvel)?;
        nonnegative(rad)?;
        nonnegative(vmax)?;
        p.validate()?;
        ws.work(work.saturating_mul((self.circles.len() + self.segments.len() + 1) as u64))?;
        self.prepare(pos, dvel);
        Ok(())
    }
    /// Samples a square velocity grid; returns chosen velocity and sample count.
    #[allow(clippy::too_many_arguments)]
    pub fn sample_velocity_grid(
        &mut self,
        ws: &mut Workspace,
        pos: V3,
        rad: f32,
        vmax: f32,
        vel: V3,
        dvel: V3,
        p: &ObstacleAvoidanceParams,
    ) -> Result<(V3, usize), Error> {
        ws.begin();
        self.sample_velocity_grid_in(ws, pos, rad, vmax, vel, dvel, p)
    }
    pub(crate) fn sample_velocity_grid_in(
        &mut self,
        ws: &mut Workspace,
        pos: V3,
        rad: f32,
        vmax: f32,
        vel: V3,
        dvel: V3,
        p: &ObstacleAvoidanceParams,
    ) -> Result<(V3, usize), Error> {
        self.validate_sample(
            ws,
            pos,
            rad,
            vmax,
            vel,
            dvel,
            p,
            u64::from(p.grid_size).pow(2),
        )?;
        let cvx = dvel[0] * p.vel_bias;
        let cvz = dvel[2] * p.vel_bias;
        let cs = vmax * 2. * (1. - p.vel_bias) / (p.grid_size - 1) as f32;
        let half = (p.grid_size - 1) as f32 * cs * 0.5;
        let mut best = [0.; 3];
        let mut penalty = f32::MAX;
        let mut ns = 0;
        for y in 0..p.grid_size {
            for x in 0..p.grid_size {
                let c = [cvx + x as f32 * cs - half, 0., cvz + y as f32 * cs - half];
                if sqr(c[0]) + sqr(c[2]) > sqr(vmax + cs / 2.) {
                    continue;
                }
                let v = self.penalty(c, pos, rad, vmax, vel, dvel, p, penalty);
                ns += 1;
                if v < penalty {
                    penalty = v;
                    best = c;
                }
            }
        }
        Ok((best, ns))
    }
    /// Samples and refines an adaptive pattern; returns velocity and sample count.
    #[allow(clippy::too_many_arguments)]
    pub fn sample_velocity_adaptive(
        &mut self,
        ws: &mut Workspace,
        pos: V3,
        rad: f32,
        vmax: f32,
        vel: V3,
        dvel: V3,
        p: &ObstacleAvoidanceParams,
    ) -> Result<(V3, usize), Error> {
        ws.begin();
        self.sample_velocity_adaptive_in(ws, pos, rad, vmax, vel, dvel, p)
    }
    pub(crate) fn sample_velocity_adaptive_in(
        &mut self,
        ws: &mut Workspace,
        pos: V3,
        rad: f32,
        vmax: f32,
        vel: V3,
        dvel: V3,
        p: &ObstacleAvoidanceParams,
    ) -> Result<(V3, usize), Error> {
        let nd = p.adaptive_divs as usize;
        let nr = p.adaptive_rings as usize;
        self.validate_sample(
            ws,
            pos,
            rad,
            vmax,
            vel,
            dvel,
            p,
            ((nd * nr + 1) * p.adaptive_depth as usize) as u64,
        )?;
        let da = (1. / nd as f32) * core::f32::consts::PI * 2.;
        let ca = cosf(da);
        let sa = sinf(da);
        let mut dir = dvel;
        let d = sqrtf(sqr(dir[0]) + sqr(dir[2]));
        if d != 0. {
            let inv = 1. / d;
            dir[0] *= inv;
            dir[2] *= inv;
        }
        let c = cosf(da * 0.5);
        let s = sinf(da * 0.5);
        let dirs = [
            dir,
            [dir[0] * c - dir[2] * s, dir[1], dir[0] * s + dir[2] * c],
        ];
        // One extra sample also contains upstream's final even-division write
        // when nd=32 and nr=4; C++ only allocates 129 samples.
        let mut pat = [[0.; 2]; 130];
        let mut n = 1;
        for j in 0..nr {
            let r = (nr - j) as f32 / nr as f32;
            pat[n] = [dirs[j % 2][0] * r, dirs[j % 2][2] * r];
            let mut last1 = n;
            let mut last2 = n;
            n += 1;
            for _ in (1..nd.saturating_sub(1)).step_by(2) {
                let a = pat[last1];
                let b = pat[last2];
                pat[n] = [a[0] * ca + a[1] * sa, -a[0] * sa + a[1] * ca];
                pat[n + 1] = [b[0] * ca - b[1] * sa, b[0] * sa + b[1] * ca];
                last1 = n;
                last2 = n + 1;
                n += 2;
            }
            // Upstream writes this slot one position too far for even divisions and
            // samples the unwritten slot before it. Without the fix the port keeps
            // that shape (the unwritten slot reads as zero); with it the slot is
            // written in place.
            if nd & 1 == 0 {
                let b = pat[last2];
                let at = if self.fixes.contains(UpstreamFixes::ADAPTIVE_PATTERN) {
                    n
                } else {
                    n + 1
                };
                pat[at] = [b[0] * ca - b[1] * sa, b[0] * sa + b[1] * ca];
                n += 1;
            }
        }
        let mut cr = vmax * (1. - p.vel_bias);
        let mut res = [dvel[0] * p.vel_bias, 0., dvel[2] * p.vel_bias];
        let mut ns = 0;
        for _ in 0..p.adaptive_depth {
            let mut penalty = f32::MAX;
            let mut best = [0.; 3];
            for v in &pat[..n] {
                let cand = [res[0] + v[0] * cr, 0., res[2] + v[1] * cr];
                if sqr(cand[0]) + sqr(cand[2]) > sqr(vmax + 0.001) {
                    continue;
                }
                let pval = self.penalty(cand, pos, rad, vmax, vel, dvel, p, penalty);
                ns += 1;
                if pval < penalty {
                    penalty = pval;
                    best = cand;
                }
            }
            res = best;
            cr *= 0.5;
        }
        Ok((res, ns))
    }
}
