//! Which AI actors a player can see, for the tier allocator (`MA20a`; `ai.md` "Tiers and budget"):
//! the host writes `sf_ai_bevy`'s [`AiInView`] on every awake actor inside any player viewer's view
//! frustum, widened by [`MARGIN_M`] and cut at [`MAX_M`] (UE `MassLOD` / Significance Manager
//! viewers).
//!
//! **A viewer is gameplay data, not render culling.** Each player carries an [`AiViewer`] (eye,
//! rotation, vertical FOV, aspect: the shape of `MassLOD`'s viewer info). Bevy's `ViewVisibility`
//! is never read: it is presentation, and it would not exist for a remote player. The host's local
//! player gets its viewer from its camera ([`write_camera_viewers`]: `PlayerCamera::pose`'s eye and
//! rotation, the camera's `Projection`), on the camera's followed body. A remote player's viewer
//! will be written on its character the same way from its replicated look (no code yet). No viewer
//! at all (free camera, no player): nobody is in view.
//!
//! **The test** ([`sees`], Bevy-free): an actor's bounding sphere ([`actor_sphere`]) against each
//! frustum plane pushed out by the margin (near plane at the eye, four sides), and its nearest
//! point within the max distance. No second hysteresis here: the allocator already holds in-view
//! actors in their band and re-allocates on its own clock.
//!
//! **Schedule** ([`add`]): `FixedUpdate`, [`InViewSystems::Viewers`] then
//! [`InViewSystems::Mark`], both before `AiAllocSystems::Allocate`, so this tick's tiers read
//! this tick's marks. The marker is inserted or removed only on change.
//!
//! **Pins** (`MA20b`): the dialogue partner's `AiPin(Pin::Dialogue)` is written by `mw_rpg_bevy`
//! when a talk session opens (and removed when it closes), in the same sets' place: before
//! `AiAllocSystems::Allocate`. Combat and scripted pins follow when those run in game.

use bevy_app::{App, FixedUpdate};
use bevy_camera::{PerspectiveProjection, Projection};
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use bevy_math::{Quat, Vec3};
use sf_ai_bevy::{AiActor, AiAllocSystems, AiInView};
use sf_camera_bevy::PlayerCamera;
use sf_controller_bevy::{Dormant, Mover};
use sf_coords::WorldPos;
use sf_tunables_bevy::TunablesAppExt;
use sf_tunables_bevy::sf_tunables::{AnyDef, Def, Lifetime, Net, Scope, Source, Unit, def};

// --- Tunables ---------------------------------------------------------------------------------

/// `mw.ai.view.margin_m`: [`ViewRules::margin_m`].
pub const MARGIN_M: Def<f32> = def!(float "mw.ai.view.margin_m" = 2.0)
    .range(0.0, 20.0)
    .unit(Unit::Meters)
    .help("How far outside the view frustum an actor still counts as in view (the camera turns onto it soon)")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// `mw.ai.view.max_m`: [`ViewRules::max_m`]. 60 m: past sight's lose range (36 m), so every actor
/// that can still see the player ranks as in view, and about where a dense town's Full cap (48 at
/// the Floor tier) runs out; the allocator's tiers are cap-ranked, with no band distance to match.
pub const MAX_M: Def<f32> = def!(float "mw.ai.view.max_m" = 60.0)
    .range(5.0, 500.0)
    .unit(Unit::Meters)
    .help("Farthest an actor counts as in view (an in-view actor ranks before every unseen one)")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// The in-view tunables (`mw.ai.view.*`).
pub const TUNABLES: &[&dyn AnyDef] = &[&MARGIN_M, &MAX_M];

// --- Test (Bevy-free) -------------------------------------------------------------------------

/// The test's numbers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewRules {
    /// [`MARGIN_M`]: every frustum plane is pushed out by this, meters.
    pub margin_m: f32,
    /// [`MAX_M`]: the sphere's nearest point must be within this of the eye, meters.
    pub max_m: f32,
}

impl Default for ViewRules {
    fn default() -> Self {
        Self { margin_m: MARGIN_M.default_value(), max_m: MAX_M.default_value() }
    }
}

/// One player's view: where it looks from and its frustum's shape.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct View {
    pub eye: WorldPos,
    /// The view's rotation; forward is `rotation * -Z`, up `rotation * Y` (Bevy's camera axes).
    pub rotation: Quat,
    /// Vertical field of view, radians, in (0, π).
    pub fov_y: f32,
    /// Width over height.
    pub aspect: f32,
}

impl View {
    pub fn forward(&self) -> Vec3 {
        self.rotation * Vec3::NEG_Z
    }
}

/// Whether a sphere of `radius` at `centre` is inside `view`'s frustum widened by
/// `rules.margin_m` and within `rules.max_m`. Public: later slices (`AI12` doors' fade-only unseen
/// rule, follower catch-up) use it as "unseen by this player".
pub fn sees(view: &View, centre: WorldPos, radius: f32, rules: ViewRules) -> bool {
    let rel = view.eye.delta_f32(centre);
    if rel.length() - radius > rules.max_m {
        return false;
    }
    let local = view.rotation.inverse() * rel;
    let reach = radius + rules.margin_m;
    // Near plane through the eye (the view looks down -Z): behind it by more than the reach.
    if local.z > reach {
        return false;
    }
    let depth = -local.z;
    let tan_y = (view.fov_y * 0.5).tan();
    let tan_x = tan_y * view.aspect;
    // A side plane's outward normal is (1, 0, tan) / |..| in its own axis: signed distance.
    let inside = |off: f32, tan: f32| (off - tan * depth) / tan.mul_add(tan, 1.0).sqrt() <= reach;
    inside(local.x, tan_x) && inside(-local.x, tan_x) && inside(local.y, tan_y) && inside(-local.y, tan_y)
}

/// Whether any of `views` [`sees`] the sphere (false with no view). Public: "unseen by every
/// player" for later slices (`AI12`).
pub fn seen_by_any<'a>(views: impl IntoIterator<Item = &'a View>, centre: WorldPos, radius: f32, rules: ViewRules) -> bool {
    views.into_iter().any(|v| sees(v, centre, radius, rules))
}

/// A mover's bounding sphere: centre half its height above its feet, radius its half height (or
/// its capsule radius when wider).
pub fn actor_sphere(mover: &Mover) -> (WorldPos, f32) {
    let half = mover.params.height * 0.5;
    (mover.loco.position.offset(Vec3::Y * half), half.max(mover.params.radius))
}

// --- Bevy -------------------------------------------------------------------------------------

/// A player's view, on its character (the host's local body from its camera; a remote player's
/// from its replicated look, later). Gameplay data: the in-view test reads only these.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct AiViewer(pub View);

/// The live [`ViewRules`], bound to [`TUNABLES`].
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct InViewSettings {
    pub rules: ViewRules,
}

/// The viewer set: every [`AiViewer`] and the live rules. Public: "unseen by every player" for
/// later slices (`AI12` doors, follower catch-up); needs [`add`].
#[derive(SystemParam)]
pub struct AiViewers<'w, 's> {
    viewers: Query<'w, 's, &'static AiViewer>,
    settings: Res<'w, InViewSettings>,
}

impl AiViewers<'_, '_> {
    /// Whether any viewer sees the sphere ([`seen_by_any`]).
    pub fn sees(&self, centre: WorldPos, radius: f32) -> bool {
        seen_by_any(self.viewers.iter().map(|v| &v.0), centre, radius, self.settings.rules)
    }

    /// Whether any viewer sees the mover ([`actor_sphere`]).
    pub fn sees_mover(&self, mover: &Mover) -> bool {
        let (centre, radius) = actor_sphere(mover);
        self.sees(centre, radius)
    }
}

/// The in-view sets on `FixedUpdate`, in this order, before `AiAllocSystems::Allocate`.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InViewSystems {
    /// [`write_camera_viewers`] (a remote player's viewer writer goes here too).
    Viewers,
    /// `mark_in_view`: inserts or removes [`AiInView`].
    Mark,
}

/// Adds the settings, [`TUNABLES`] and the two systems (called by `walkers::add`).
pub fn add(app: &mut App) {
    app.init_resource::<InViewSettings>();
    let _margin = app.add_tunable(&MARGIN_M).bind_resource(|s: &mut InViewSettings, v| s.rules.margin_m = v);
    let _max = app.add_tunable(&MAX_M).bind_resource(|s: &mut InViewSettings, v| s.rules.max_m = v);
    app.configure_sets(
        FixedUpdate,
        (InViewSystems::Viewers, InViewSystems::Mark).chain().before(AiAllocSystems::Allocate),
    )
    .add_systems(
        FixedUpdate,
        (write_camera_viewers.in_set(InViewSystems::Viewers), mark_in_view.in_set(InViewSystems::Mark)),
    );
}

/// The host's local player: its camera's last pose (eye, rotation) and projection as the
/// [`AiViewer`] of the body it follows. A camera without a pose yet writes nothing; a projection
/// that is not perspective counts as Bevy's default perspective.
#[allow(clippy::needless_pass_by_value)] // system parameters
pub fn write_camera_viewers(
    cams: Query<(&PlayerCamera, Option<&Projection>)>,
    mut viewers: Query<&mut AiViewer>,
    mut commands: Commands,
) {
    for (cam, projection) in &cams {
        let Some(pose) = cam.pose() else { continue };
        let (fov_y, aspect) = if let Some(Projection::Perspective(p)) = projection {
            (p.fov, p.aspect_ratio)
        } else {
            let p = PerspectiveProjection::default();
            (p.fov, p.aspect_ratio)
        };
        let viewer = AiViewer(View { eye: pose.eye, rotation: pose.rotation, fov_y, aspect });
        if let Ok(mut current) = viewers.get_mut(cam.follow) {
            current.set_if_neq(viewer);
        } else if let Ok(mut body) = commands.get_entity(cam.follow) {
            body.try_insert(viewer);
        }
    }
}

/// [`AiInView`] on every awake actor any viewer sees, off the others (only on change).
#[allow(clippy::needless_pass_by_value, clippy::type_complexity)] // system parameters
fn mark_in_view(
    viewers: AiViewers,
    actors: Query<(Entity, &Mover, Has<AiInView>), (With<AiActor>, Without<Dormant>)>,
    mut commands: Commands,
) {
    for (entity, mover, marked) in &actors {
        match (viewers.sees_mover(mover), marked) {
            (true, false) => {
                commands.entity(entity).insert(AiInView);
            }
            (false, true) => {
                commands.entity(entity).remove::<AiInView>();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
#[path = "in_view_tests.rs"]
mod tests;
