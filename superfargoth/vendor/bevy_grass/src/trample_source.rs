// superfargoth patch (own: trample source): the whole file. Written to be
// sent upstream: it names no game.
//! An external trample texture: an app that already keeps where things pressed
//! the ground (footprints in snow, a deformation ring) hands it to the blade
//! generator, which then reads it instead of its own trample cells, and skips
//! its own trample passes (decay and stamping) while it is set.
//!
//! Two ways in, use one:
//!
//! - [`GrassTrampleTexture`], a main-world resource holding two images (this
//!   frame's and last frame's) and where each one sits;
//! - [`GrassTrampleSource`], a render-world resource holding two GPU texture
//!   views, for an app that writes its texture on the GPU. Insert or update it
//!   before [`RenderSystems::PrepareBindGroups`]
//!   (bevy::render::RenderSystems::PrepareBindGroups), remove it to go back to
//!   the built-in trampling.
//!
//! The texture is a square window of `size × size` texels over the ground, in
//! the space the fields' `GlobalTransform`s are in (Bevy's world space: with a
//! floating origin, the render space, before
//! [`GrassWorldOffset`](crate::GrassWorldOffset)). One channel holds the press,
//! 0 (untouched) to 1 (flattened). The window may be addressed as a torus, as
//! a ring that follows the player is: its first corner is held by texel
//! `first_texel`, and the texels wrap at `size`. A plain texture has its first
//! corner at texel 0.
//!
//! Each blade reads the texel its root is in (no filtering, as the built-in
//! cells), and where it is pressed the texels one step +X and +Z: the blade
//! lies down the press slope, away from where the ground is pressed most, as
//! far as the press. Where the slope is flat (the middle of a footprint), its
//! own facing, mixed in at [`GrassTrampleLookup::direction_floor`], gives the
//! direction, so it never jumps. Last frame's texture and placement give last
//! frame's bend, so motion vectors stay exact.

use bevy::asset::Handle;
use bevy::ecs::prelude::*;
use bevy::ecs::schedule::SystemSet;
use bevy::ecs::reflect::ReflectResource;
use bevy::image::Image;
use bevy::math::{IVec2, UVec2, Vec2};
use bevy::reflect::Reflect;

/// The blade generator's compute passes (render app, `RenderGraph` schedule,
/// before `camera_driver`). An app that writes its trample texture on the GPU
/// in the same schedule orders that pass before this set, so the blades read
/// this frame's texture.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GrassGenerateSystems;

/// Where a trample texture lies over the ground.
///
/// Placed when both [`size`](Self::size) and
/// [`texels_per_metre`](Self::texels_per_metre) are above zero
/// ([`is_placed`](Self::is_placed)); [`NONE`](Self::NONE) is not, and presses
/// nothing.
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub struct GrassTramplePlacement {
    /// The window's lowest corner (its minimum X and Z), in metres.
    pub corner: Vec2,
    /// Texels per metre, along both axes.
    pub texels_per_metre: f32,
    /// Texels along each side of the window.
    pub size: u32,
    /// The texel that holds the window's first corner; the texels wrap at
    /// `size` along each axis. Zero for a texture that does not wrap.
    pub first_texel: UVec2,
}

impl GrassTramplePlacement {
    /// No window: nothing is pressed.
    pub const NONE: Self = Self {
        corner: Vec2::ZERO,
        texels_per_metre: 0.0,
        size: 0,
        first_texel: UVec2::ZERO,
    };

    /// Whether the window covers any ground.
    pub fn is_placed(&self) -> bool {
        self.size > 0 && self.texels_per_metre > 0.0
    }

    /// The texel of the window holding `point` (XZ, metres), counted from the
    /// first corner; `None` outside the window or when not placed.
    pub fn window_texel(&self, point: Vec2) -> Option<IVec2> {
        if !self.is_placed() {
            return None;
        }
        let t = ((point - self.corner) * self.texels_per_metre).floor();
        let n = self.size as f32;
        (t.cmpge(Vec2::ZERO).all() && t.cmplt(Vec2::splat(n)).all()).then(|| t.as_ivec2())
    }

    /// The texture texel holding window texel `texel` (`first_texel` added,
    /// wrapped at `size`); `None` outside the window.
    pub fn texture_texel(&self, texel: IVec2) -> Option<UVec2> {
        let n = self.size as i32;
        if !self.is_placed() || texel.cmplt(IVec2::ZERO).any() || texel.cmpge(IVec2::splat(n)).any()
        {
            return None;
        }
        // `first_texel` is reduced once, so one subtraction wraps (the
        // shader's twin divides nothing).
        let first = (self.first_texel % self.size).as_ivec2();
        let wrapped = texel + first;
        let wrapped = IVec2::select(wrapped.cmpge(IVec2::splat(n)), wrapped - n, wrapped);
        Some(wrapped.as_uvec2())
    }

    /// Metres from `point` to the window's nearest edge (negative outside).
    pub fn edge_distance(&self, point: Vec2) -> f32 {
        let n = self.size as f32;
        let t = (point - self.corner) * self.texels_per_metre;
        t.x.min(n - t.x).min(t.y.min(n - t.y)) / self.texels_per_metre
    }
}

impl Default for GrassTramplePlacement {
    fn default() -> Self {
        Self::NONE
    }
}

/// How a trample texture is read; the same for both frames.
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub struct GrassTrampleLookup {
    /// The channel that holds the press: 0 red, 1 green, 2 blue, 3 alpha.
    pub channel: u32,
    /// How far a press of 1 flattens a blade: 1 lays it flat, as the
    /// built-in trampling's strongest push. The push is clamped to 1.
    pub strength: f32,
    /// Metres inside the window's edge where nothing is pressed yet.
    pub edge_margin: f32,
    /// Metres over which the press fades in after the margin; 0 for a hard
    /// edge.
    pub edge_fade: f32,
    /// How much of the blade's own facing is mixed into the press slope, so a
    /// blade where the press is flat still has a direction. A slope this
    /// small (press per texel) and a facing turn it about equally.
    pub direction_floor: f32,
}

impl Default for GrassTrampleLookup {
    fn default() -> Self {
        Self {
            channel: 0,
            strength: 1.0,
            edge_margin: 0.0,
            edge_fade: 1.0,
            // One step of an 8-bit texture.
            direction_floor: 1.0 / 256.0,
        }
    }
}

impl GrassTrampleLookup {
    /// The press fade at `edge` metres inside the window's edge: 0 within the
    /// margin, 1 past the fade.
    pub fn fade(&self, edge: f32) -> f32 {
        ((edge - self.edge_margin) / self.edge_fade.max(1e-6)).clamp(0.0, 1.0)
    }
}

/// An external trample texture, as images: this frame's press and last
/// frame's, and where each lies. While this resource exists the grass reads
/// it instead of trampling itself; see the [module docs](self).
///
/// The images must be readable as `texture_2d<f32>` (any float or unorm
/// format) and at least `size` texels on each side.
#[derive(Resource, Debug, Clone, PartialEq, Reflect)]
#[reflect(Resource)]
pub struct GrassTrampleTexture {
    /// This frame's press.
    pub current: Handle<Image>,
    /// Last frame's press, for the motion vectors.
    pub previous: Handle<Image>,
    /// Where `current` lies.
    pub current_placement: GrassTramplePlacement,
    /// Where `previous` lay.
    pub previous_placement: GrassTramplePlacement,
    /// How both are read.
    pub lookup: GrassTrampleLookup,
}

/// The CPU twin of the generator's press lookup (`generate.wesl`
/// `texture_push`): the push (direction the blade lies toward, times how
/// flat, 0 to 1) at the blade root `root` (XZ) of a blade facing `facing`
/// (turns), from a texture of `width` texels per row whose texels are
/// `texels` (RGBA).
///
/// Zero outside the window, where it is not placed, or where the press is 0.
pub fn trample_texture_push(
    texels: &[[f32; 4]],
    width: u32,
    placement: &GrassTramplePlacement,
    lookup: &GrassTrampleLookup,
    root: Vec2,
    facing: f32,
) -> Vec2 {
    let Some(texel) = placement.window_texel(root) else {
        return Vec2::ZERO;
    };
    let fade = lookup.fade(placement.edge_distance(root));
    if fade <= 0.0 {
        return Vec2::ZERO;
    }
    let channel = lookup.channel.min(3) as usize;
    let press = |t: IVec2| -> f32 {
        placement.texture_texel(t).map_or(0.0, |t| {
            texels
                .get((t.y * width + t.x) as usize)
                .map_or(0.0, |v| v[channel])
        }) * fade
    };
    let p = press(texel);
    if p <= 0.0 {
        return Vec2::ZERO;
    }
    let slope = Vec2::new(press(texel + IVec2::X) - p, press(texel + IVec2::Y) - p);
    let turn = facing * core::f32::consts::TAU;
    let own = Vec2::new(turn.sin(), turn.cos());
    let away = -slope + own * lookup.direction_floor;
    away / away.length().max(1e-6) * (p * lookup.strength).min(1.0)
}

/// Which of the built-in trample passes run this frame: (decay, stamp). An
/// external source replaces both; without one, decay runs every frame and
/// stamping when interactors were gathered.
pub fn built_in_trample_passes(external_source: bool, interactor_count: u32) -> (bool, bool) {
    (!external_source, !external_source && interactor_count > 0)
}
