//! Paper-derived reciprocal half-planes and incremental linear programs.
//!
//! Source: van den Berg et al., “Reciprocal n-Body Collision Avoidance” (2011),
//! sections 4.2, 5.1, 5.3 and 5.4, doi:10.1007/978-3-642-19457-3_1.
//! No reference-library implementation is used here.
use super::*;
type V2 = [f32; 2];

/// Local velocity planner. The default preserves the existing sampled planner.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[non_exhaustive]
pub enum AvoidanceMode {
    /// Use [`AgentParams::obstacle_avoidance_type`] to select the sampled preset.
    #[default]
    Sampled,
    /// Use reciprocal velocity half-planes.
    Orca(OrcaParams),
}

/// ORCA horizons and neighbour cap. Neither horizon may be zero or non-finite.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct OrcaParams {
    /// Prediction horizon for other agents, in seconds.
    pub time_horizon: f32,
    /// Prediction horizon for two-sided static segments, in seconds.
    pub time_horizon_obstacles: f32,
    /// Nearest agents retained, in ascending (distance squared, input index) order.
    pub max_neighbours: u16,
}
impl Default for OrcaParams {
    fn default() -> Self {
        Self {
            time_horizon: 2.,
            time_horizon_obstacles: 1.,
            max_neighbours: 16,
        }
    }
}
impl OrcaParams {
    pub(super) fn validate(&self) -> Result<(), Error> {
        positive(self.time_horizon)?;
        positive(self.time_horizon_obstacles)?;
        if self.max_neighbours == 0 {
            return Err(Error::InvalidParameter("ORCA neighbour cap"));
        }
        Ok(())
    }
}

/// Observable planar agent state and its preferred velocity.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OrcaAgent {
    /// Planar position (x, z).
    pub position: V2,
    /// Current velocity (x, z).
    pub velocity: V2,
    /// Preferred velocity (x, z), clipped to `max_speed` by the solver.
    pub preferred_velocity: V2,
    /// Positive disc radius.
    pub radius: f32,
    /// Positive speed limit.
    pub max_speed: f32,
}
impl OrcaAgent {
    fn validate(&self) -> Result<(), Error> {
        positive(self.radius)?;
        positive(self.max_speed)?;
        if self
            .position
            .iter()
            .chain(&self.velocity)
            .chain(&self.preferred_velocity)
            .any(|v| !v.is_finite() || v.abs() > 1.0e6)
            || self.radius > 1.0e6
            || self.max_speed > 1.0e6
        {
            return Err(Error::InvalidParameter("ORCA finite bounded state"));
        }
        Ok(())
    }
    pub(super) fn crowd(a: &CrowdAgent) -> Self {
        Self {
            position: [a.npos[0], a.npos[2]],
            velocity: [a.vel[0], a.vel[2]],
            preferred_velocity: [a.dvel[0], a.dvel[2]],
            radius: a.params.radius,
            max_speed: a.params.max_speed,
        }
    }
}

// Feasible side: n.v >= c; n is a unit outward normal.
#[derive(Clone, Copy, Debug)]
struct Plane {
    n: [f64; 2],
    c: f64,
}
/// Reusable solver storage. Capacity growth is fallible and charged to the workspace.
#[derive(Default)]
pub struct OrcaScratch {
    planes: Vec<Plane>,
    projected: Vec<Plane>,
    order: Vec<(usize, f32)>,
}
fn add(a: V2, b: V2) -> V2 {
    [a[0] + b[0], a[1] + b[1]]
}
fn sub(a: V2, b: V2) -> V2 {
    [a[0] - b[0], a[1] - b[1]]
}
fn mul(a: V2, s: f32) -> V2 {
    [a[0] * s, a[1] * s]
}
fn dot(a: V2, b: V2) -> f32 {
    a[0] * b[0] + a[1] * b[1]
}
fn det(a: V2, b: V2) -> f32 {
    a[0] * b[1] - a[1] * b[0]
}
fn norm(a: V2) -> f32 {
    sqrtf(dot(a, a))
}
fn unit(a: V2, fallback: V2) -> V2 {
    let l = norm(a);
    if l > 0. { mul(a, 1. / l) } else { fallback }
}
pub(super) fn clip(v: V2, speed: f32) -> V2 {
    let v = if dot(v, v) > speed * speed {
        mul(unit(v, [1., 0.]), speed)
    } else {
        v
    };
    // Rounding a point on the disc to f32 can put it just outside. Measure the
    // rounded coordinates in f64 and leave a radial rounding margin only when
    // needed; subsequent component rounding cannot consume it all.
    // Each f32 coordinate product is exact in f64. Recover the addition's
    // rounding residual too: a small orthogonal component can put the exact
    // vector outside while the rounded squared length equals the boundary.
    let [x, y] = v.map(f64::from);
    let (xx, yy) = (x * x, y * y);
    let squared = xx + yy;
    let residual = if xx >= yy {
        yy - (squared - xx)
    } else {
        xx - (squared - yy)
    };
    let limit = f64::from(speed) * f64::from(speed);
    if squared > limit || (squared == limit && residual > 0.) {
        let length = root64(squared);
        let scale = if speed > 0. && speed < f32::MIN_POSITIVE {
            // Subnormal coordinates have an absolute rounding step. One step
            // of radial margin exceeds their combined component-rounding error.
            f64::from(f32::from_bits(speed.to_bits() - 1)) / length
        } else {
            f64::from(speed) / length * (1. - f64::from(f32::EPSILON))
        };
        v.map(|x| (f64::from(x) * scale) as f32)
    } else {
        v
    }
}

// Eq. (5): nearest point on a disc's truncated velocity cone. In overlap,
// use the next integration step rather than the prediction horizon.
fn agent_plane(a: &OrcaAgent, b: &OrcaAgent, horizon: f32, dt: f32) -> Plane {
    let p = sub(b.position, a.position);
    let v = sub(a.velocity, b.velocity);
    let r = a.radius + b.radius;
    let d2 = dot(p, p);
    let (n, u) = if d2 > r * r {
        let w = sub(v, mul(p, 1. / horizon));
        let dp = dot(w, p);
        if dp < 0. && dp * dp > r * r * dot(w, w) {
            let n = unit(w, [1., 0.]);
            (n, mul(n, r / horizon - norm(w)))
        } else {
            let leg = sqrtf((d2 - r * r).max(0.));
            let t = if det(p, w) > 0. {
                [(p[0] * leg - p[1] * r) / d2, (p[0] * r + p[1] * leg) / d2]
            } else {
                [-(p[0] * leg + p[1] * r) / d2, (p[0] * r - p[1] * leg) / d2]
            };
            let n = [-t[1], t[0]];
            (n, sub(mul(t, dot(v, t)), v))
        }
    } else {
        let w = sub(v, mul(p, 1. / dt));
        // Coincident stationary centres have no observable separating direction.
        // The deterministic axis is a recovery policy, not a collision guarantee.
        let n = unit(w, unit(mul(p, -1.), [1., 0.]));
        (n, mul(n, r / dt - norm(w)))
    };
    plane(n, add(a.velocity, mul(u, 0.5)))
}

// Project the current velocity onto the boundary of the finite segment's
// truncated velocity obstacle (eq. 11). Its boundary consists of the visible
// cutoff capsule and two tangent rays. Endpoint arcs must be outside both the
// capsule's straight part and its cone; otherwise they are interior features.
fn wall_plane(a: &OrcaAgent, wall: [f32; 4], horizon: f32, dt: f32) -> Plane {
    let start = [wall[0], wall[1]];
    let edge = sub([wall[2], wall[3]], start);
    let d2 = dot(edge, edge);
    let t = if d2 > 0. {
        (dot(sub(a.position, start), edge) / d2).clamp(0., 1.)
    } else {
        0.
    };
    let away = sub(a.position, add(start, mul(edge, t)));
    let d = norm(away);
    let mut fallback = plane(unit(away, [1., 0.]), [0.; 2]);
    fallback.c = f64::from((a.radius - d) / if d < a.radius { dt } else { horizon });
    if d <= a.radius {
        return fallback;
    }
    let h = f64::from(horizon);
    let r = f64::from(a.radius) / h;
    let centers = [
        [
            (f64::from(wall[0]) - f64::from(a.position[0])) / h,
            (f64::from(wall[1]) - f64::from(a.position[1])) / h,
        ],
        [
            (f64::from(wall[2]) - f64::from(a.position[0])) / h,
            (f64::from(wall[3]) - f64::from(a.position[1])) / h,
        ],
    ];
    let v = a.velocity.map(f64::from);
    let mut best = f64::INFINITY;
    let mut selected = fallback;
    let mut consider = |n: [f64; 2], point: [f64; 2]| {
        let delta = [v[0] - point[0], v[1] - point[1]];
        let distance = dot64(delta, delta);
        if distance < best {
            best = distance;
            // Non-overlapping static obstacles always admit zero velocity.
            selected = Plane {
                n,
                c: dot64(n, point).min(0.),
            };
        }
    };
    let e = [centers[1][0] - centers[0][0], centers[1][1] - centers[0][1]];
    let ee = dot64(e, e);
    if ee > 0. {
        let l = root64(ee);
        let mut n = [-e[1] / l, e[0] / l];
        if dot64(n, centers[0]) > 0. {
            n = n.map(|x| -x);
        }
        let base = [centers[0][0] + n[0] * r, centers[0][1] + n[1] * r];
        let fraction = dot64([v[0] - base[0], v[1] - base[1]], e) / ee;
        if (0. ..=1.).contains(&fraction) && dot64(n, centers[0]) <= -r {
            consider(n, [base[0] + e[0] * fraction, base[1] + e[1] * fraction]);
        }
    }
    for i in 0..2 {
        let c = centers[i];
        let other = centers[1 - i];
        let delta = [v[0] - c[0], v[1] - c[1]];
        let length = root64(dot64(delta, delta));
        if length > 0. {
            let n = delta.map(|x| x / length);
            if dot64(n, c) <= -r && dot64(n, [other[0] - c[0], other[1] - c[1]]) <= 0. {
                consider(n, [c[0] + n[0] * r, c[1] + n[1] * r]);
            }
        }
        let cc = dot64(c, c);
        let leg = root64((cc - r * r).max(0.));
        for sign in [-1., 1.] {
            let tangent = [
                (c[0] * leg - sign * c[1] * r) / cc,
                (c[1] * leg + sign * c[0] * r) / cc,
            ];
            let n = [-sign * tangent[1], sign * tangent[0]];
            // The entire cutoff segment must lie inside this supporting line.
            if dot64(n, other) > -r + 1.0e-12 {
                continue;
            }
            let along = dot64(v, tangent).max(leg);
            consider(n, tangent.map(|x| x * along));
        }
    }
    selected
}

// Keep the normalized distance equations in double precision: rounding a unit
// normal in f32 invalidates the disc intersection and amplifies cancellation in
// nearly parallel projected constraints.
fn plane(n: V2, point: V2) -> Plane {
    let n = n.map(f64::from);
    let length = root64(dot64(n, n));
    let n = n.map(|x| x / length);
    Plane {
        n,
        c: dot64(n, point.map(f64::from)),
    }
}
fn dot64(a: [f64; 2], b: [f64; 2]) -> f64 {
    a[0] * b[0] + a[1] * b[1]
}
fn root64(x: f64) -> f64 {
    #[cfg(feature = "libm")]
    {
        libm::sqrt(x)
    }
    #[cfg(not(feature = "libm"))]
    {
        x.sqrt()
    }
}
// Incremental 2D LP. Intersect the actual (not assumed unit) boundary with the
// speed disc, then restrict its tangent interval by all earlier constraints.
fn solve(
    ws: &mut Workspace,
    planes: &[Plane],
    speed: f64,
    objective: [f64; 2],
    directional: bool,
) -> Result<([f64; 2], usize), Error> {
    let length = root64(dot64(objective, objective));
    let mut v = if directional || length > speed {
        objective.map(|x| x * speed / length)
    } else {
        objective
    };
    for (i, p) in planes.iter().enumerate() {
        ws.work(1)?;
        if dot64(p.n, v) >= p.c {
            continue;
        }
        ws.work(1)?;
        let nn = dot64(p.n, p.n);
        let disc = speed * speed - p.c * p.c / nn;
        if disc < 0. {
            return Ok((v, i));
        }
        let base = p.n.map(|x| x * p.c / nn);
        let tangent = [-p.n[1] / root64(nn), p.n[0] / root64(nn)];
        let mut lo = -root64(disc);
        let mut hi = -lo;
        for q in &planes[..i] {
            ws.work(1)?;
            let slope = dot64(q.n, tangent);
            let need = q.c - dot64(q.n, base);
            if slope.abs() < 1.0e-12 {
                if need > 0. {
                    return Ok((v, i));
                }
            } else if slope > 0. {
                lo = lo.max(need / slope);
            } else {
                hi = hi.min(need / slope);
            }
            if lo > hi {
                return Ok((v, i));
            }
        }
        let t = if directional {
            // The tangent points opposite to the input line direction.
            // On an exact directional tie, select its upper endpoint.
            if dot64(objective, tangent) >= 0. {
                hi
            } else {
                lo
            }
        } else {
            (dot64(objective, tangent) - dot64(base, tangent)).clamp(lo, hi)
        };
        v = [base[0] + tangent[0] * t, base[1] + tangent[1] * t];
    }
    Ok((v, planes.len()))
}

/// Selects a planar velocity using the paper's reciprocal half-planes.
///
/// `dt` must be positive; it controls overlap recovery. Walls are two-sided
/// `[start_x, start_z, end_x, end_z]` segments, in caller order. Neighbours are
/// sorted by (distance squared, slice index) and capped by `max_neighbours`.
/// Non-finite inputs, magnitudes above `1e6`, horizons outside `1e-5..=1e5`,
/// and steps outside that interval are rejected to avoid arithmetic overflow.
///
/// Returns a speed-bounded velocity, including when constraints are infeasible.
/// In that case the 3D epigraph LP minimizes the worst agent-plane violation;
/// static planes retain priority. Acceleration limits, missing neighbours and
/// infeasible constraints prevent an unconditional nonpenetration guarantee.
/// The workspace bounds storage, candidate visits, constraint additions and LP
/// pivots. On an error no velocity is returned; scratch may contain partial work.
pub fn orca_velocity(
    ws: &mut Workspace,
    agent: &OrcaAgent,
    neighbours: &[OrcaAgent],
    walls: &[[f32; 4]],
    params: &OrcaParams,
    dt: f32,
    scratch: &mut OrcaScratch,
) -> Result<V2, Error> {
    ws.begin();
    velocity_in(ws, agent, neighbours, walls, params, dt, scratch)
}
pub(super) fn velocity_in(
    ws: &mut Workspace,
    a: &OrcaAgent,
    neighbours: &[OrcaAgent],
    walls: &[[f32; 4]],
    params: &OrcaParams,
    dt: f32,
    s: &mut OrcaScratch,
) -> Result<V2, Error> {
    a.validate()?;
    params.validate()?;
    for t in [dt, params.time_horizon, params.time_horizon_obstacles] {
        if !(1.0e-5..=1.0e5).contains(&t) {
            return Err(Error::InvalidParameter("ORCA time range"));
        }
    }
    s.planes.clear();
    s.projected.clear();
    s.order.clear();
    let cap = usize::from(params.max_neighbours).min(neighbours.len());
    ws.reserve(&mut s.order, cap)?;
    for (i, b) in neighbours.iter().enumerate() {
        ws.work(1)?;
        b.validate()?;
        let distance = dot(sub(a.position, b.position), sub(a.position, b.position));
        let at = s
            .order
            .partition_point(|&(j, d)| d < distance || (d == distance && j < i));
        if at < cap {
            if s.order.len() == cap {
                s.order.pop();
            }
            s.order.insert(at, (i, distance));
        }
    }
    let count = walls.len().checked_add(cap).ok_or(Error::SizeOverflow)?;
    ws.reserve(&mut s.planes, count)?;
    ws.reserve(&mut s.projected, count)?;
    for &wall in walls {
        ws.work(1)?;
        if wall.iter().any(|v| !v.is_finite() || v.abs() > 1.0e6) {
            return Err(Error::InvalidParameter("ORCA wall coordinates"));
        }
        s.planes
            .push(wall_plane(a, wall, params.time_horizon_obstacles, dt));
    }
    let static_count = s.planes.len();
    for &(i, _) in &s.order {
        ws.work(1)?;
        s.planes
            .push(agent_plane(a, &neighbours[i], params.time_horizon, dt));
    }
    let (mut v, failed) = solve(
        ws,
        &s.planes,
        f64::from(a.max_speed),
        a.preferred_velocity.map(f64::from),
        false,
    )?;
    let mut violation = 0.;
    // Section 5.3: on plane i of the (vx,vz,slack) epigraph, substituting
    // slack=c_i-n_i.v projects earlier planes to (n_j-n_i).v >= c_j-c_i.
    for i in failed..s.planes.len() {
        ws.work(1)?;
        let p = s.planes[i];
        if p.c - dot64(p.n, v) <= violation {
            continue;
        }
        ws.work(1)?;
        s.projected.clear();
        s.projected
            .extend_from_slice(&s.planes[..static_count.min(i)]);
        for q in &s.planes[static_count.min(i)..i] {
            ws.work(1)?;
            let diff = [q.n[0] - p.n[0], q.n[1] - p.n[1]];
            let length = root64(dot64(diff, diff));
            if length > 1.0e-12 {
                s.projected.push(Plane {
                    n: diff.map(|x| x / length),
                    c: (q.c - p.c) / length,
                });
            }
        }
        let (candidate, end) = solve(ws, &s.projected, f64::from(a.max_speed), p.n, true)?;
        if end == s.projected.len() {
            v = candidate;
        }
        violation = p.c - dot64(p.n, v);
    }
    let v = clip(v.map(|x| x as f32), a.max_speed);
    if v.iter().any(|x| !x.is_finite()) {
        return Err(Error::InvalidInput("ORCA arithmetic range"));
    }
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rounded_output_remains_inside_the_exact_speed_disc() {
        let input = [0.6, 0.8];
        assert!(dot64(input.map(f64::from), input.map(f64::from)) > 1.);
        let v = clip(input, 1.);
        assert!(dot64(v.map(f64::from), v.map(f64::from)) <= 1.);
        assert!(norm(sub(v, input)) < 1e-6);
    }

    #[test]
    fn tiny_orthogonal_component_cannot_hide_outside_the_speed_disc() {
        let tiny = 1.4901161e-8_f32;
        assert_eq!(4. + f64::from(tiny) * f64::from(tiny), 4.);
        let v = clip([2., tiny], 2.);
        assert!(v[0] < 2.);
        assert_eq!(clip([2., 0.], 2.), [2., 0.]);
    }

    #[test]
    fn subnormal_speed_limit_retains_a_representable_rounding_margin() {
        for bits in [1, 2, 0x007f_ffff] {
            let speed = f32::from_bits(bits);
            let v = clip([speed, speed], speed);
            assert!(f64::from(v[0]).hypot(f64::from(v[1])) <= f64::from(speed));
        }
    }

    #[test]
    fn directional_tie_selects_the_upper_tangent_endpoint() -> Result<(), Error> {
        let p = Plane { n: [1., 0.], c: 1. };
        let (v, end) = solve(&mut Workspace::new(), &[p], 2., [-1., 0.], true)?;
        assert_eq!(end, 1);
        assert_eq!(v[0], 1.);
        assert!((v[1] - root64(3.)).abs() < 1e-14);
        Ok(())
    }

    #[test]
    fn rounded_normal_intersection_stays_on_the_speed_disc() -> Result<(), Error> {
        let p = Plane {
            n: [1.0000001, 0.],
            c: 1.,
        };
        let (v, end) = solve(&mut Workspace::new(), &[p], 1., [0., 1.], false)?;
        assert_eq!(end, 1);
        assert!((dot64(p.n, v) - p.c).abs() < 1e-14);
        assert!((dot64(v, v) - 1.).abs() < 1e-14);
        assert!(v[1] > 0.0004);
        Ok(())
    }
}
