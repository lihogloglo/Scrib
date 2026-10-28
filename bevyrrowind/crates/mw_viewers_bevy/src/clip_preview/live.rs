//! `clip-preview --browse <dir>` (motion-generation.md MG10a): the stage's NPC in a normal,
//! real-time window. Left/Right step clips within the activity, Up/Down step activities (both
//! wrap, [`super::browse::BrowseList`]); the chosen clip is cooked on demand (the capture's
//! [`super::cook`] path, ~80 ms) and loops in the full-body slot. Each start puts the actor back
//! on the origin, so a clip with root travel replays from the mark instead of walking off. Mouse
//! drag orbits the camera, the wheel zooms; an overlay shows the clip and its `.prov.json`.

use std::sync::Arc;
use std::time::Instant;

use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollUnit};
use bevy::prelude::*;
use mw_bake::anim::HumanoidTarget;
use sf_anim_bevy::Animator;
use sf_anim_graph::LateClip;

use super::browse::{BrowseList, Prov};
use super::cook::{self, ClipJob};
use super::{ActorQuery, Phase, Preview, play_full};

/// The browser's state, inside [`Preview`].
pub(super) struct Browse {
    list: BrowseList,
    /// `sources.ron` id the clips cook with (unit, up, rest frame, root rules).
    source: String,
    /// The dressed actor's skeleton, set once it has loaded.
    pub(super) target: Option<HumanoidTarget>,
    /// Cook and start the cursor's clip on the next update.
    pending: bool,
    /// The looping clip and when it last started (seconds of `Time`).
    playing: Option<(Arc<LateClip>, f32)>,
}

impl Browse {
    pub(super) fn new(list: BrowseList, source: String) -> Self {
        Self { list, source, target: None, pending: true, playing: None }
    }
}

#[derive(Component)]
pub(super) struct Overlay;

pub(super) fn spawn_overlay(mut commands: Commands) {
    commands.spawn((
        Overlay,
        Text::new("loading the NPC..."),
        TextFont::from_font_size(15.0),
        TextColor(Color::srgb(0.95, 0.95, 0.85)),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.55)),
        Node { position_type: PositionType::Absolute, top: Val::Px(6.0), left: Val::Px(6.0), right: Val::Px(6.0), padding: UiRect::all(Val::Px(6.0)), ..Node::default() },
    ));
}

/// Keys, the on-demand cook, and the loop.
#[allow(clippy::needless_pass_by_value)]
pub(super) fn browse_step(
    mut preview: ResMut<Preview>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut actors: Query<ActorQuery>,
    mut overlay: Query<&mut Text, With<Overlay>>,
    mut windows: Query<&mut Window>,
) {
    if preview.phase != Phase::Browse {
        return;
    }
    let Some(b) = preview.browse.as_mut() else { return };
    for (key, step_activity, step) in [(KeyCode::ArrowRight, false, 1), (KeyCode::ArrowLeft, false, -1), (KeyCode::ArrowDown, true, 1), (KeyCode::ArrowUp, true, -1)] {
        if keys.just_pressed(key) {
            if step_activity {
                b.list.step_activity(step);
            } else {
                b.list.step_clip(step);
            }
            b.pending = true;
        }
    }
    let Some((mut animator, _, mut transform, root)) = actors.iter_mut().next() else { return };
    let now = time.elapsed_secs();
    if b.pending {
        b.pending = false;
        let Some(target) = b.target.as_ref() else { return };
        let clip = b.list.current().clone();
        let t = Instant::now();
        let job = ClipJob { file: clip.path.to_string_lossy().into_owned(), name: None, source: Some(b.source.clone()) };
        let asset = Arc::clone(&animator.asset);
        let cooked = cook::resolve(&job)
            .and_then(|r| cook::cook(&r, target))
            .and_then(|c| LateClip::new(&asset.graph, &asset.skeleton, Arc::new(c)).map_err(|e| format!("late clip: {e:?}")))
            .map(Arc::new)
            .and_then(|late| play_full(&mut animator, &mut transform, root, Arc::clone(&late)).map(|()| late));
        let status = match cooked {
            Ok(late) => {
                animator.time_scale = 1.0;
                let s = format!("{:.2} s, cooked + retargeted in {:.0} ms", late.clip.duration, t.elapsed().as_secs_f32() * 1000.0);
                b.playing = Some((late, now));
                s
            }
            Err(e) => {
                // Back to the graph's own idle, so a failed clip doesn't look like the last one.
                if let Some(slot) = asset.graph.slot(mw_graphs::build::slots::FULL) {
                    animator.instance.stop_slot(&asset.binding(), slot, 0.0);
                }
                b.playing = None;
                format!("error: {e}")
            }
        };
        println!("browse {}: {status}", clip.rel);
        let prov = Prov::read(&clip).unwrap_or_default();
        let activity = b.list.current_activity();
        let text = format!(
            "{} ({}/{})   clip {}/{} of {} in all\n{}   band: {}   {status}\nprompt: {}\nLeft/Right: clip   Up/Down: activity   drag: orbit   wheel: zoom",
            activity.name,
            b.list.activity + 1,
            b.list.activities.len(),
            b.list.clip + 1,
            activity.clips.len(),
            b.list.len(),
            clip.rel,
            prov.band.as_deref().unwrap_or("-"),
            prov.prompt.as_deref().unwrap_or("(no .prov.json prompt)"),
        );
        for mut t in &mut overlay {
            t.0.clone_from(&text);
        }
        for mut w in &mut windows {
            w.title = format!("Bevyrrowind - clip preview - {}", clip.rel);
        }
        return;
    }
    // The loop: replay from the mark once the clip has run its length.
    let Some((late, started)) = b.playing.as_mut() else { return };
    if now - *started >= late.clip.duration.max(0.1) {
        if let Err(e) = play_full(&mut animator, &mut transform, root, Arc::clone(late)) {
            eprintln!("error: {e}");
        }
        *started = now;
    }
}

/// Orbit camera around the actor: yaw and pitch from a mouse drag (any button), distance from the wheel.
#[derive(Resource)]
pub(super) struct OrbitCam {
    yaw: f32,
    pitch: f32,
    distance: f32,
}

impl Default for OrbitCam {
    /// The capture's `front` view: in front of the actor, slightly above its chest.
    fn default() -> Self {
        Self { yaw: 0.0, pitch: 0.1, distance: 3.6 }
    }
}

/// Radians per pixel dragged.
const ORBIT_SPEED: f32 = 0.006;
/// Distance factor per wheel line.
const ZOOM_STEP: f32 = 0.9;

#[allow(clippy::needless_pass_by_value)]
pub(super) fn orbit(
    mut cam: ResMut<OrbitCam>,
    buttons: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    scroll: Res<AccumulatedMouseScroll>,
    actors: Query<&Transform, With<Animator>>,
    mut cameras: Query<&mut Transform, (With<Camera3d>, Without<Animator>)>,
) {
    if buttons.any_pressed([MouseButton::Left, MouseButton::Right, MouseButton::Middle]) {
        cam.yaw -= motion.delta.x * ORBIT_SPEED;
        cam.pitch = (cam.pitch + motion.delta.y * ORBIT_SPEED).clamp(-1.2, 1.45);
    }
    let lines = match scroll.unit {
        MouseScrollUnit::Line => scroll.delta.y,
        MouseScrollUnit::Pixel => scroll.delta.y / 100.0,
    };
    if lines != 0.0 {
        cam.distance = (cam.distance * ZOOM_STEP.powf(lines)).clamp(0.5, 25.0);
    }
    let Some(at) = actors.iter().next().map(|t| t.translation) else { return };
    let target = Vec3::new(at.x, 0.95, at.z);
    let (sy, cy) = cam.yaw.sin_cos();
    let (sp, cp) = cam.pitch.sin_cos();
    let eye = target + cam.distance * Vec3::new(cp * sy, sp, cp * cy);
    for mut t in &mut cameras {
        *t = Transform::from_translation(eye).looking_at(target, Vec3::Y);
    }
}
