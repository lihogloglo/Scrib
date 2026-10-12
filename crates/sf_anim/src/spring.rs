//! Critically damped springs and dampers for smoothing IK targets, weights
//! and look-at directions (Daniel Holden, *Spring-It-On: The Game
//! Developer's Spring-Roll-Call*, 2021). All updates are the exact
//! closed-form solutions, so they are stable at any `dt` and frame-rate
//! independent. Parameterized by half-life: the time to cover half the
//! remaining distance (for the damper; the spring takes a little longer
//! because it starts at rest).

use std::ops::{Add, Mul, Sub};

use glam::{Quat, Vec2, Vec3};

const LN2: f32 = std::f32::consts::LN_2;

/// Damping coefficient of a critically damped spring with this half-life.
pub fn halflife_to_damping(halflife: f32) -> f32 {
    (4.0 * LN2) / (halflife + 1e-5)
}

/// A value a spring can move: a vector space over f32.
pub trait SpringValue: Copy + Add<Output = Self> + Sub<Output = Self> + Mul<f32, Output = Self> {}
impl SpringValue for f32 {}
impl SpringValue for Vec2 {}
impl SpringValue for Vec3 {}

/// Exponential damper: moves `x` toward `goal`, halving the gap every `halflife`.
pub fn damper<T: SpringValue>(x: T, goal: T, halflife: f32, dt: f32) -> T {
    let t = 1.0 - (-(LN2 * dt) / (halflife + 1e-5)).exp();
    x + (goal - x) * t
}

/// Critically damped spring toward a still `goal` (velocity `v` in units/s).
pub fn critical_spring<T: SpringValue>(x: &mut T, v: &mut T, goal: T, halflife: f32, dt: f32) {
    let y = halflife_to_damping(halflife) / 2.0;
    let j0 = *x - goal;
    let j1 = *v + j0 * y;
    let e = (-y * dt).exp();
    *x = (j0 + j1 * dt) * e + goal;
    *v = (*v - j1 * (y * dt)) * e;
}

/// Critically damped spring toward a goal moving at `goal_velocity`; the
/// value converges to the goal's motion without the lag of
/// [`critical_spring`] (only the half step the goal makes inside one
/// update, as the goal is held still for the update).
pub fn critical_spring_moving<T: SpringValue>(x: &mut T, v: &mut T, goal: T, goal_velocity: T, halflife: f32, dt: f32) {
    let d = halflife_to_damping(halflife);
    let y = d / 2.0;
    // Stiffness of the critical spring is d²/4.
    let c = goal + goal_velocity * (d / (d * d / 4.0));
    let j0 = *x - c;
    let j1 = *v + j0 * y;
    let e = (-y * dt).exp();
    *x = (j0 + j1 * dt) * e + c;
    *v = (*v - j1 * (y * dt)) * e;
}

/// Critically damped rotation spring; `v` is angular velocity (rad/s,
/// scaled axis). Always takes the short way round.
pub fn critical_spring_quat(x: &mut Quat, v: &mut Vec3, goal: Quat, halflife: f32, dt: f32) {
    let y = halflife_to_damping(halflife) / 2.0;
    let mut diff = *x * goal.inverse();
    if diff.w < 0.0 {
        diff = -diff;
    }
    let j0 = diff.to_scaled_axis();
    let j1 = *v + j0 * y;
    let e = (-y * dt).exp();
    *x = (Quat::from_scaled_axis((j0 + j1 * dt) * e) * goal).normalize();
    *v = (*v - j1 * (y * dt)) * e;
}

/// A spring with its state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Spring<T> {
    pub value: T,
    pub velocity: T,
    pub halflife: f32,
}

impl<T: SpringValue> Spring<T> {
    pub fn new(value: T, zero: T, halflife: f32) -> Self {
        Self { value, velocity: zero, halflife }
    }

    pub fn update(&mut self, goal: T, dt: f32) -> T {
        critical_spring(&mut self.value, &mut self.velocity, goal, self.halflife, dt);
        self.value
    }
}

/// A rotation spring with its state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuatSpring {
    pub value: Quat,
    pub velocity: Vec3,
    pub halflife: f32,
}

impl QuatSpring {
    pub fn new(value: Quat, halflife: f32) -> Self {
        Self { value, velocity: Vec3::ZERO, halflife }
    }

    pub fn update(&mut self, goal: Quat, dt: f32) -> Quat {
        critical_spring_quat(&mut self.value, &mut self.velocity, goal, self.halflife, dt);
        self.value
    }
}
