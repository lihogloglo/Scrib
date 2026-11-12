//! `clip-preview --browse <dir>` (motion-generation.md MG10a): the stage's NPC in a normal,
//! real-time window. Left/Right step clips within the activity, Up/Down step activities (both
//! wrap, [`super::browse::BrowseList`]); the chosen clip is cooked on demand (the capture's
//! [`super::cook`] path, ~80 ms) and loops in the full-body slot. Each start puts the actor back
//! on the origin, so a clip with root travel replays from the mark instead of walking off. Mouse
//! drag orbits the camera, the wheel zooms; an overlay shows the clip and its `.prov.json`.
//! Y / N / M record keep / reject / maybe for the clip in `verdicts.toml` and advance
//! ([`BrowseList::rate`]), Backspace clears it, U jumps to the next unrated clip.

use std::sync::Arc;
use std::time::Instant;

use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollUnit};
use bevy::prelude::*;
use mw_bake::anim::HumanoidTarget;
use sf_anim_bevy::Animator;
use sf_anim_graph::LateClip;

use super::browse::{BrowseList, Prov, Tally, Verdict, Verdicts};
use super::cook::{self, ClipJob};
use super::{ActorQuery, Phase, Preview, play_full};

/// Inertialization into each browsed clip and each loop replay, seconds (UE5 Inertialization; Bollo, GDC 2018).
/// 0 cut hard from the idle: thighs jumped ~20 deg, knees ~28 (`mw_bake/tests/thigh_snap.rs`, 2026-11-12).
const BROWSE_BLEND_IN: f32 = 0.15;
/// The loop replays this long before the clip ends, so the montage never reaches its blend-out to the idle.
const BROWSE_REPLAY_LEAD: f32 = 0.05;

/// The browser's state, inside [`Preview`].
pub(super) struct Browse {
    list: BrowseList,
    /// The browsed folder's `verdicts.toml`, or why it can't be used (then keys don't rate).
    verdicts: Result<Verdicts, String>,
    /// `sources.ron` id the clips cook with (unit, up, rest frame, root rules).
    source: String,
    /// The dressed actor's skeleton, set once it has loaded.
    pub(super) target: Option<HumanoidTarget>,
    /// Cook and start the cursor's clip on the next update.
    pending: bool,
    /// The last cook's status line, and the last verdict key's message.
    status: String,
    message: String,
    /// The looping clip and when it last started (seconds of `Time`).
    playing: Option<(Arc<LateClip>, f32)>,
}

impl Browse {
    pub(super) fn new(list: BrowseList, source: String) -> Self {
        let verdicts = Verdicts::load(&list.root);
        if let Err(e) = &verdicts {
            eprintln!("error: verdicts off: {e}");
        }
        Self { list, verdicts, source, target: None, pending: true, status: String::new(), message: String::new(), playing: None }
    }

    /// Y / N / M / Backspace / U; `true` if the overlay needs a refresh.
    fn verdict_keys(&mut self, keys: &ButtonInput<KeyCode>) -> bool {
        let pressed = [(KeyCode::KeyY, Some(Verdict::Keep)), (KeyCode::KeyN, Some(Verdict::Reject)), (KeyCode::KeyM, Some(Verdict::Maybe)), (KeyCode::Backspace, None)]
            .into_iter()
            .find(|(k, _)| keys.just_pressed(*k));
        let unrated = keys.just_pressed(KeyCode::KeyU);
        if pressed.is_none() && !unrated {
            return false;
        }
        let verdicts = match self.verdicts.as_mut() {
            Ok(v) => v,
            Err(e) => {
                self.message = format!("verdicts off: {e}");
                return true;
            }
        };
        let id = self.list.current().id.clone();
        let result = match pressed {
            Some((_, Some(verdict))) => {
                self.pending = true;
                self.list.rate(verdicts, verdict).map(|()| format!("{id}: {}", verdict.as_str()))
            }
            Some((_, None)) => self.list.clear_rating(verdicts).map(|had| if had { format!("{id}: cleared") } else { format!("{id}: no verdict to clear") }),
            None => {
                self.pending = self.list.next_unrated(verdicts);
                Ok(if self.pending { "next unrated".to_owned() } else { "every clip is rated".to_owned() })
            }
        };
        self.message = result.unwrap_or_else(|e| format!("error: {e}"));
        println!("browse verdict: {}", self.message);
        true
    }

    fn overlay_text(&self) -> String {
        let clip = self.list.current();
        let prov = Prov::read(clip).unwrap_or_default();
        let activity = self.list.current_activity();
        let (verdict, tally, all) = match &self.verdicts {
            Ok(v) => (v.get(&clip.id).unwrap_or("unrated").to_owned(), tally_text(v.tally(&activity.clips)), tally_text(v.tally(self.list.activities.iter().flat_map(|a| &a.clips)))),
            Err(_) => ("(verdicts off)".to_owned(), String::new(), String::new()),
        };
        format!(
            "{} ({}/{})   clip {}/{} of {} in all\n{}   band: {}   {}\nprompt: {}\nverdict: {verdict}   activity: {tally}   all: {all}   {}\n\
             Y keep  N reject  M maybe  Bksp clear  U next unrated   Left/Right: clip   Up/Down: activity   drag: orbit   wheel: zoom",
            activity.name,
            self.list.activity + 1,
            self.list.activities.len(),
            self.list.clip + 1,
            activity.clips.len(),
            self.list.len(),
            clip.rel,
            prov.band.as_deref().unwrap_or("-"),
            self.status,
            prov.prompt.as_deref().unwrap_or("(no .prov.json prompt)"),
            self.message,
        )
    }
}

fn tally_text(t: Tally) -> String {
    let note = if t.note > 0 { format!(" {} note", t.note) } else { String::new() };
    format!("{} keep {} maybe {} reject{note} {} unrated", t.keep, t.maybe, t.reject, t.unrated)
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
            b.message.clear();
        }
    }
    let refresh = b.verdict_keys(&keys);
    if refresh && !b.pending {
        let text = b.overlay_text();
        for mut t in &mut overlay {
            t.0.clone_from(&text);
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
            .and_then(|late| play_full(&mut animator, &mut transform, root, Arc::clone(&late), BROWSE_BLEND_IN).map(|()| late));
        b.status = match cooked {
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
        println!("browse {}: {}", clip.rel, b.status);
        let text = b.overlay_text();
        for mut t in &mut overlay {
            t.0.clone_from(&text);
        }
        for mut w in &mut windows {
            w.title = format!("Bevyrrowind - clip preview - {}", clip.rel);
        }
        return;
    }
    // The loop: replay from the mark just before the clip ends, inertialized from its last pose (a raw take has no
    // loop range; replaying after the end would first blend out to the idle, then cut to frame 0).
    let Some((late, started)) = b.playing.as_mut() else { return };
    if now - *started >= (late.clip.duration - BROWSE_REPLAY_LEAD).max(0.1) {
        if let Err(e) = play_full(&mut animator, &mut transform, root, Arc::clone(late), BROWSE_BLEND_IN) {
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
