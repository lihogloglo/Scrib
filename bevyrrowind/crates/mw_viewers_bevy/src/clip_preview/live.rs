//! `clip-preview --browse <dir>` (motion-generation.md MG10a): the stage's NPC in a normal,
//! real-time window. Left/Right step clips within the activity, Up/Down step activities (both
//! wrap, [`super::browse::BrowseList`]); the chosen clip is cooked on demand (the capture's
//! [`super::cook`] path, ~80 ms) and loops in the full-body slot. Each start puts the actor back
//! on the origin, so a clip with root travel replays from the mark instead of walking off. Mouse
//! drag orbits the camera, the wheel zooms; an overlay shows the clip and its `.prov.json`.
//! Y / N / M record keep / reject / maybe for the clip in `verdicts.toml` and advance
//! ([`BrowseList::rate`]), Backspace clears it, U jumps to the next unrated clip, Z undoes the
//! session's last verdict action, newest first, in either review ([`BrowseList::undo`]).
//! Cycle takes (`*.cycle.*`) cook through the loop cutter ([`cook::cook_report`]) and loop in the
//! slot with no re-blend, so the seam shows as the game would play it; the overlay gives the cut's
//! window and seam errors, and L switches between the cut loop and the raw take (A/B).
//! With the grader's `grades.json` in the folder ([`super::browse::Grades`]) each activity lists
//! ungated clips first, the overlay shows the clip's gates, metrics, mirror flag and prompt group,
//! and C toggles "candidates only" (gated clips hidden); V cycles the view all / unrated / maybe /
//! keep. Arrows, rating's advance and U stay inside the filters.
//! G (AN10) toggles group review: the cursor's prompt group plays side by side, one actor per take
//! ([`Seat`]), on one [`GroupClock`]; 1-4 or Left/Right select a take, N rejects it (it leaves, the
//! row closes up), Y / M give keep / maybe once two are left, Up/Down step groups; an emptied stage
//! opens the next group ([`BrowseList::rate_take`]), G goes back to single clips.
//! The overlay is three lines ([`BrowseList::overlay_lines`]: where, the verdict and the
//! saved count, the keys that work now); Tab shows the details page. Each verdict action shows a
//! large SAVED / NOT SAVED / UNDONE notice for [`NOTICE_S`], and a key that does nothing says so.

use std::sync::Arc;
use std::time::Instant;

use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollUnit};
use bevy::prelude::*;
use mw_bake::anim::HumanoidTarget;
use sf_anim_bevy::{AnimAsset, AnimTier, Animator, SkeletonBinding};
use sf_anim_graph::LateClip;
use sf_anim_graph::slot::LOOP_FOREVER;
use sf_character::Readiness;
use sf_character_bevy::{Character, CharacterReadiness};

use super::browse::{BrowseClip, BrowseList, Details, GROUP_MAX, GroupClock, GroupRated, Grades, Notice, NoticeKind, Prov, Transport, Verdict, Verdicts, row_x};
use super::cook::{self, ClipJob};
use super::{ActorQuery, Phase, Preview, play_full};

/// Free space between two actors of the group row, metres, on top of the actor's measured width
/// (unit m, range 0..3; source Guess: clear of a neighbour's arm swing, four still in the default view).
const GROUP_GAP_M: f32 = 0.6;
/// A take's label floats this high over its actor's feet, metres (unit m, range 1.5..3; source Guess: above the head).
const LABEL_HEIGHT_M: f32 = 2.1;

/// The browser's actor seat: 0 is the stage's own NPC (single-clip review), 1.. the copies group
/// review adds; a seat plays the stage take that sits in it ([`super::browse::StageTake::seat`]).
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Seat(pub(super) usize);

/// The animated bodies (children of their [`Seat`] roots).
pub(super) type Bodies<'w, 's> = Query<'w, 's, (&'static Transform, &'static ChildOf), With<Animator>>;
/// The seat roots and whether each is shown.
pub(super) type SeatRoots<'w, 's> = Query<'w, 's, (&'static Transform, &'static Visibility), (With<Seat>, Without<Animator>)>;

/// Where the camera and floor look: the mean of the shown actors (root plus the body's root travel).
pub(super) fn focus(actors: &Bodies<'_, '_>, roots: &SeatRoots<'_, '_>) -> Option<Vec3> {
    let (sum, n) = actors.iter().filter_map(|(body, parent)| roots.get(parent.parent()).ok().filter(|(_, v)| **v != Visibility::Hidden).map(|(root, _)| root.translation + body.translation)).fold((Vec3::ZERO, 0.0_f32), |(s, n), p| (s + p, n + 1.0));
    (n > 0.0).then(|| sum / n)
}

/// What the group row does next.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum RowStep {
    /// The clock runs.
    #[default]
    Run,
    /// Cook every take on stage.
    Cook,
    /// Start the row together once every seat on stage has its body.
    Start,
}

/// Group review's actors and clock, inside [`Browse`].
#[derive(Default)]
struct GroupPlay {
    /// The row is on screen (leaving parks the copies and puts seat 0 back on the mark).
    active: bool,
    step: RowStep,
    /// The copies (seats 1..) were spawned.
    spawned: bool,
    /// Each seat's cooked take (`None`: failed, or nothing cooked yet).
    clips: [Option<Arc<LateClip>>; GROUP_MAX],
    clock: Option<GroupClock>,
    /// The row's spacing, metres (the actor's measured width plus [`GROUP_GAP_M`]).
    spacing: f32,
}

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
    /// The playing clip and its clip seconds played since it last started (the transport's scaled
    /// time); `None` once it loops in the slot (a cut loop, never replayed).
    playing: Option<(Arc<LateClip>, Option<f32>)>,
    /// Cut cycle takes into loops (L toggles; the raw take otherwise).
    loop_cut: bool,
    /// The last cook's loop line ([`cook::cut_text`]); empty for a clip that isn't a cycle.
    cut: String,
    /// Group review (AN10).
    group: GroupPlay,
    /// Pause / frame step / speed, both review modes.
    transport: Transport,
    /// The overlay shows the details page ([`super::browse::HELP_KEY`] toggles; off at start).
    details: bool,
    /// A notice to show on the next update ([`Browse::show_notice`]).
    notice: Option<Notice>,
    /// When the notice on screen appeared (real time).
    notice_since: Option<Instant>,
}

impl Browse {
    pub(super) fn new(mut list: BrowseList, source: String) -> Self {
        list.set_grades(Grades::load(&list.root));
        match list.grades.counts() {
            (0, _) => println!("browse grades: no grades.json"),
            (graded, gated) => println!("browse grades: {graded} clips graded, {gated} gated"),
        }
        let verdicts = Verdicts::load(&list.root);
        if let Err(e) = &verdicts {
            eprintln!("error: verdicts off: {e}");
        }
        Self {
            list,
            verdicts,
            source,
            target: None,
            pending: true,
            status: String::new(),
            message: String::new(),
            playing: None,
            loop_cut: true,
            cut: String::new(),
            group: GroupPlay::default(),
            transport: Transport::default(),
            details: false,
            notice: None,
            notice_since: None,
        }
    }

    /// Space (pause / resume), Period (one frame, pausing first), Minus / Equal (slower / faster);
    /// `true` if the overlay needs a refresh.
    fn transport_keys(&mut self, keys: &ButtonInput<KeyCode>) -> bool {
        let mut changed = false;
        for (key, act) in [
            (KeyCode::Space, Transport::toggle_pause as fn(&mut Transport)),
            (KeyCode::Period, Transport::step_frame),
            (KeyCode::Minus, Transport::slower),
            (KeyCode::Equal, Transport::faster),
        ] {
            if keys.just_pressed(key) {
                act(&mut self.transport);
                changed = true;
            }
        }
        changed
    }

    /// Y / N / M / Backspace / U / Z; `true` if the overlay needs a refresh.
    fn verdict_keys(&mut self, keys: &ButtonInput<KeyCode>) -> bool {
        let pressed = [(KeyCode::KeyY, Some(Verdict::Keep)), (KeyCode::KeyN, Some(Verdict::Reject)), (KeyCode::KeyM, Some(Verdict::Maybe)), (KeyCode::Backspace, None)]
            .into_iter()
            .find(|(k, _)| keys.just_pressed(*k));
        let unrated = keys.just_pressed(KeyCode::KeyU);
        let undo = keys.just_pressed(KeyCode::KeyZ);
        if pressed.is_none() && !unrated && !undo {
            return false;
        }
        let verdicts = match self.verdicts.as_mut() {
            Ok(v) => v,
            Err(e) => {
                self.message = format!("verdicts off: {e}");
                self.notice = Some(Notice::not_saved(&self.message));
                return true;
            }
        };
        let id = self.list.current().id.clone();
        let tail = self.list.notice_tail(&id);
        let result = match pressed {
            Some((_, Some(verdict))) => {
                self.pending = true;
                self.list.rate(verdicts, verdict).map(|()| (format!("{id}: {}", verdict.as_str()), Some(Notice::rated(&tail, verdict))))
            }
            Some((_, None)) => self.list.clear_rating(verdicts).map(|had| (if had { format!("{id}: cleared") } else { format!("{id}: no verdict to clear") }, Some(Notice::cleared(&tail, had)))),
            // Z: the undone clip comes back under the cursor and replays.
            None if undo => self.list.undo(verdicts).map(|undone| {
                self.pending |= undone.is_some();
                let notice = Notice::undo(undone.as_deref().map(|u| self.list.notice_tail(u)).as_deref());
                (undo_text(undone), Some(notice))
            }),
            None => {
                self.pending = self.list.next_unrated(verdicts);
                Ok((if self.pending { "next unrated".to_owned() } else { "every clip is rated".to_owned() }, None))
            }
        };
        self.take_result(result);
        println!("browse verdict: {}", self.message);
        true
    }

    /// A verdict action's message and notice; an error is shown as not saved.
    fn take_result(&mut self, result: Result<(String, Option<Notice>), String>) {
        match result {
            Ok((message, notice)) => {
                self.message = message;
                if notice.is_some() {
                    self.notice = notice;
                }
            }
            Err(e) => {
                self.notice = Some(Notice::not_saved(&e));
                self.message = format!("error: {e}");
            }
        }
    }

    /// C (candidates only) / V (view); `true` if the overlay needs a refresh.
    fn filter_keys(&mut self, keys: &ButtonInput<KeyCode>) -> bool {
        let candidates = keys.just_pressed(KeyCode::KeyC);
        let view = keys.just_pressed(KeyCode::KeyV);
        if !candidates && !view {
            return false;
        }
        if candidates {
            self.list.candidates_only = !self.list.candidates_only;
        }
        if view {
            self.list.view = self.list.view.next();
        }
        let verdicts = self.verdicts.as_ref().ok();
        let before = self.list.current().id.clone();
        self.list.settle(verdicts);
        self.pending |= self.list.current().id != before;
        let shown = self.list.shown_count(verdicts);
        let empty = if self.list.shown(self.list.current(), verdicts) { "" } else { " (nothing to show: cursor stays)" };
        self.message = format!("{}: {shown} of {} shown{empty}", self.filter_text(), self.list.len());
        println!("browse filter: {}", self.message);
        true
    }

    /// The active filters, for the overlay and messages.
    fn filter_text(&self) -> String {
        self.list.filter_text(self.verdicts.is_ok())
    }

    /// The overlay: three lines ([`BrowseList::overlay_lines`]), or the details page. The first
    /// three-line text is logged once (`browse overlay: ...`, the proof line of an owner check).
    fn overlay_text(&self) -> String {
        static LOGGED: std::sync::Once = std::sync::Once::new();
        let verdicts = self.verdicts.as_ref().map_err(String::as_str);
        let transport = self.transport.label();
        if !self.details {
            let text = self.list.overlay_lines(verdicts, &transport);
            LOGGED.call_once(|| println!("browse overlay: {}", text.replace('\n', " | ")));
            return text;
        }
        let prov = Prov::read(self.list.current()).unwrap_or_default();
        let details = Details { prov: &prov, status: &self.status, cut: &self.cut, message: &self.message, transport: &transport, loop_cut: self.loop_cut };
        self.list.details_text(verdicts, &details)
    }

    /// Put the pending notice on screen, or take the shown one down after [`NOTICE_S`].
    fn show_notice(&mut self, notices: &mut Notices<'_, '_>) {
        if let Some(notice) = self.notice.take() {
            let color = match notice.kind {
                NoticeKind::Saved | NoticeKind::Undone => Color::srgb(0.45, 1.0, 0.45),
                NoticeKind::NotSaved => Color::srgb(1.0, 0.4, 0.35),
            };
            for (mut text, mut text_color, mut node) in &mut *notices {
                text.0.clone_from(&notice.text);
                text_color.0 = color;
                node.display = Display::Flex;
            }
            self.notice_since = Some(Instant::now());
        } else if self.notice_since.is_some_and(|t| t.elapsed().as_secs_f32() >= NOTICE_S) {
            self.notice_since = None;
            for (.., mut node) in &mut *notices {
                node.display = Display::None;
            }
        }
    }
}

/// How long a notice stays on screen, seconds of real time (a new one replaces it).
// tunables: const (dev-tool UI, a toast's on-screen time)
const NOTICE_S: f32 = 2.0;

#[derive(Component)]
pub(super) struct Overlay;

/// The large notice line under the overlay ([`Notice`]).
#[derive(Component)]
pub(super) struct NoticeLine;

/// The notice line's text, colour and node (shown / hidden).
type Notices<'w, 's> = Query<'w, 's, (&'static mut Text, &'static mut TextColor, &'static mut Node), (With<NoticeLine>, Without<Overlay>)>;

/// Whether the browser (or content it runs, or the window's own handling) uses `key`; digits 1-4
/// only in group review. Modifiers, Escape and F1-F12 never count as unbound.
fn bound(key: KeyCode, group: bool) -> bool {
    use KeyCode as K;
    let browser = matches!(
        key,
        K::KeyY | K::KeyN | K::KeyM | K::KeyU | K::KeyZ | K::KeyC | K::KeyV | K::KeyG | K::KeyL | K::Tab | K::Backspace | K::Space | K::Period | K::Minus | K::Equal | K::ArrowLeft | K::ArrowRight | K::ArrowUp | K::ArrowDown
    );
    // B / H: the actors' bodies and cloth A/B toggles (`mw_actors_bevy::actors`), live in the browser too.
    let content = matches!(key, K::KeyB | K::KeyH);
    let skipped = matches!(
        key,
        K::ShiftLeft | K::ShiftRight | K::ControlLeft | K::ControlRight | K::AltLeft | K::AltRight | K::SuperLeft | K::SuperRight | K::Escape | K::F1 | K::F2 | K::F3 | K::F4 | K::F5 | K::F6 | K::F7 | K::F8 | K::F9 | K::F10 | K::F11 | K::F12
    );
    let take = group && matches!(key, K::Digit1 | K::Digit2 | K::Digit3 | K::Digit4);
    browser || content || skipped || take
}

/// The label over a group seat's actor ([`seat_labels`]).
#[derive(Component)]
pub(super) struct SeatLabel(usize);

/// A label's left edge sits this far left of its anchor, pixels (layout: roughly centres a short label).
const LABEL_OFFSET_PX: f32 = 60.0;

pub(super) fn spawn_overlay(mut commands: Commands) {
    for seat in 0..GROUP_MAX {
        commands.spawn((
            SeatLabel(seat),
            Text::new(""),
            TextFont::from_font_size(16.0),
            TextColor(Color::srgb(0.95, 0.95, 0.85)),
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.55)),
            Node { position_type: PositionType::Absolute, display: Display::None, padding: UiRect::all(Val::Px(3.0)), ..Node::default() },
        ));
    }
    // The overlay across the top, the notice centred right under it (whatever the overlay's height).
    commands.spawn(Node { position_type: PositionType::Absolute, top: Val::Px(6.0), left: Val::Px(6.0), right: Val::Px(6.0), flex_direction: FlexDirection::Column, row_gap: Val::Px(10.0), ..Node::default() }).with_children(|top| {
        top.spawn((
            Overlay,
            Text::new("loading the NPC..."),
            TextFont::from_font_size(18.0),
            TextColor(Color::srgb(0.95, 0.95, 0.85)),
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.55)),
            Node { padding: UiRect::all(Val::Px(6.0)), ..Node::default() },
        ));
        top.spawn((
            NoticeLine,
            Text::new(""),
            TextFont::from_font_size(40.0),
            TextColor(Color::WHITE),
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            Node { display: Display::None, align_self: AlignSelf::Center, padding: UiRect::axes(Val::Px(16.0), Val::Px(6.0)), ..Node::default() },
        ));
    });
}

/// The browser's actors: (the [`ActorQuery`] of a body, its seat root).
type Actors<'w, 's> = Query<'w, 's, (ActorQuery<'static>, &'static ChildOf)>;
/// The seat roots (group review places and hides them; seat 0 also lends its NPC to the copies).
type Roots<'w, 's> = Query<'w, 's, (&'static Seat, &'static mut Transform, &'static mut Visibility, &'static Character), Without<Animator>>;
/// The bodies' rigs and where each body is, for the row's width.
/// The orbit camera, apart from the actors and their seat roots.
type OrbitCamera = (With<Camera3d>, Without<Animator>, Without<Seat>);

type Rigs<'w, 's> = Query<'w, 's, (&'static SkeletonBinding, &'static GlobalTransform, &'static ChildOf), With<Animator>>;

/// The seat of the root `parent`.
fn seat_of(roots: &Roots<'_, '_>, parent: Entity) -> Option<usize> {
    roots.get(parent).ok().map(|(s, ..)| s.0)
}

/// One take cooked onto the actor's skeleton and bound to its graph (the capture's cook path), and
/// the cut's loop line for a cycle take (empty otherwise).
fn cook_take(clip: &BrowseClip, source: &str, target: &HumanoidTarget, asset: &AnimAsset, loop_cut: bool) -> (Result<Arc<LateClip>, String>, String) {
    let job = ClipJob { file: clip.path.to_string_lossy().into_owned(), name: None, source: Some(source.to_owned()) };
    let cycle = cook::is_cycle(&clip.rel);
    let mut report = None;
    let late = cook::resolve(&job)
        .and_then(|r| cook::cook_report(&r, target, loop_cut && cycle))
        .and_then(|(c, r)| {
            report = r;
            LateClip::new(&asset.graph, &asset.skeleton, Arc::new(c)).map_err(|e| format!("late clip: {e:?}"))
        })
        .map(Arc::new);
    (late, if cycle { cook::cut_text(report.as_ref()) } else { String::new() })
}

/// Z's message: the undone clip, or that the session's log is empty.
fn undo_text(undone: Option<String>) -> String {
    undone.map_or_else(|| "nothing to undo".to_owned(), |id| format!("{id}: undone"))
}

/// A take's clip time the row clock runs to: its end less the replay lead (never into the blend-out).
fn take_end(late: &LateClip) -> f32 {
    (late.clip.duration - BROWSE_REPLAY_LEAD).max(0.0)
}

impl Browse {
    /// G in single-clip review: the cursor's group on stage, the copies spawned (once), the row's
    /// spacing measured from the actor (once).
    fn enter_group(&mut self, commands: &mut Commands, roots: &Roots<'_, '_>, rigs: &Rigs<'_, '_>, globals: &Query<&GlobalTransform>) {
        if !self.list.open_group(self.verdicts.as_ref().ok()) {
            "group: the cursor's clip isn't shown".clone_into(&mut self.message);
            return;
        }
        let npc = if self.group.spawned { None } else { roots.iter().find(|(s, ..)| s.0 == 0).map(|(.., c)| c.clone()) };
        if let Some(npc) = npc {
            for seat in 1..GROUP_MAX {
                commands.spawn((npc.clone(), Transform::IDENTITY, Visibility::Hidden, Name::new(format!("seat {seat}")), CharacterReadiness(Readiness::Sheathed), Seat(seat)));
            }
            self.group.spawned = true;
        }
        if self.group.spacing <= 0.0 {
            // The actor's width: twice its farthest joint from the body's vertical axis, in its current pose.
            let rig = rigs.iter().find(|(.., p)| seat_of(roots, p.parent()) == Some(0));
            let width = rig.map_or(0.0, |(binding, body, _)| {
                let at = body.translation();
                binding.joints.iter().filter_map(|j| globals.get(*j).ok()).fold(0.0_f32, |w, g| w.max(2.0 * (g.translation() - at).xz().length()))
            });
            self.group.spacing = width + GROUP_GAP_M;
            println!("clip-preview: group row spacing {:.2} m (actor width {width:.2} m + gap {GROUP_GAP_M} m)", self.group.spacing);
        }
        self.group.active = true;
        self.group.step = RowStep::Cook;
        self.transport.resume();
        self.message.clear();
    }

    /// Group review's keys; `true` if the overlay needs a refresh.
    fn group_keys(&mut self, keys: &ButtonInput<KeyCode>) -> bool {
        let before = self.list.group.clone();
        let mut changed = false;
        if keys.just_pressed(KeyCode::KeyG) {
            self.list.leave_group(self.verdicts.as_ref().ok());
            self.transport.resume();
            "single clips".clone_into(&mut self.message);
            return true;
        }
        for (rank, key) in [KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3, KeyCode::Digit4].into_iter().enumerate() {
            if keys.just_pressed(key) {
                self.message = if self.list.select_take(rank) { String::new() } else { format!("no take {}", rank + 1) };
                changed = true;
            }
        }
        for (key, step) in [(KeyCode::ArrowRight, 1), (KeyCode::ArrowLeft, -1)] {
            if keys.just_pressed(key) {
                self.list.step_take(step);
                self.message.clear();
                changed = true;
            }
        }
        for (key, step) in [(KeyCode::ArrowDown, 1), (KeyCode::ArrowUp, -1)] {
            if keys.just_pressed(key) {
                self.message = if self.list.step_group(step, self.verdicts.as_ref().ok()) { String::new() } else { "no other group is shown".to_owned() };
                changed = true;
            }
        }
        if keys.just_pressed(KeyCode::KeyL) {
            self.loop_cut = !self.loop_cut;
            self.group.step = RowStep::Cook;
            self.message = format!("loop cut {}", if self.loop_cut { "on" } else { "off: raw takes" });
            changed = true;
        }
        if self.filter_keys(keys) {
            // The filters changed what is shown: the cursor's group again, as the filters now show it.
            if !self.list.open_group(self.verdicts.as_ref().ok()) {
                self.message.push_str("; group: nothing shown");
            }
            changed = true;
        }
        let rated = self.group_verdict_keys(keys);
        changed |= rated;
        // A take new on stage (another group, or the same group's next takes) needs its cook.
        let fresh = match (&before, &self.list.group) {
            (_, None) => false,
            (None, Some(_)) => true,
            (Some(b), Some(a)) => a.takes.iter().any(|t| !b.takes.contains(t)),
        };
        if fresh {
            self.group.step = RowStep::Cook;
        }
        // New takes, a re-cook (L, filters) or a verdict action: play resumes, the speed stays.
        // Selecting a take keeps a pause: the row on stage is the same.
        if rated || self.group.step == RowStep::Cook {
            self.transport.resume();
        }
        changed
    }

    /// Y / N / M on the selected take, Backspace clears its verdict, U the next unrated clip's group,
    /// Z undoes the last verdict action (its take back on stage; [`Self::group_keys`] cooks it).
    fn group_verdict_keys(&mut self, keys: &ButtonInput<KeyCode>) -> bool {
        let rate = [(KeyCode::KeyY, Verdict::Keep), (KeyCode::KeyN, Verdict::Reject), (KeyCode::KeyM, Verdict::Maybe)].into_iter().find(|(k, _)| keys.just_pressed(*k)).map(|(_, v)| v);
        let clear = keys.just_pressed(KeyCode::Backspace);
        let unrated = keys.just_pressed(KeyCode::KeyU);
        let undo = keys.just_pressed(KeyCode::KeyZ);
        if rate.is_none() && !clear && !unrated && !undo {
            return false;
        }
        let verdicts = match self.verdicts.as_mut() {
            Ok(v) => v,
            Err(e) => {
                self.message = format!("verdicts off: {e}");
                self.notice = Some(Notice::not_saved(&self.message));
                return true;
            }
        };
        let id = self.list.current().id.clone();
        let tail = self.list.notice_tail(&id);
        let result = if let Some(verdict) = rate {
            self.list.rate_take(verdicts, verdict).map(|out| {
                let message = match out {
                    GroupRated::Refused => format!("{id}: keep / maybe once 2 takes are left (N rejects)"),
                    GroupRated::Removed => format!("{id}: {}", verdict.as_str()),
                    GroupRated::Opened => format!("{id}: {}; next takes", verdict.as_str()),
                    GroupRated::Finished => format!("{id}: {}; no other group is shown", verdict.as_str()),
                };
                (message, Some(Notice::group_rated(&tail, verdict, out)))
            })
        } else if clear {
            self.list.clear_rating(verdicts).map(|had| (if had { format!("{id}: cleared") } else { format!("{id}: no verdict to clear") }, Some(Notice::cleared(&tail, had))))
        } else if undo {
            self.list.undo(verdicts).map(|undone| {
                let notice = Notice::undo(undone.as_deref().map(|u| self.list.notice_tail(u)).as_deref());
                (undo_text(undone), Some(notice))
            })
        } else if self.list.next_unrated(verdicts) {
            self.list.open_group(Some(verdicts));
            Ok(("next unrated".to_owned(), None))
        } else {
            Ok(("every clip is rated".to_owned(), None))
        };
        self.take_result(result);
        println!("browse verdict: {}", self.message);
        true
    }

    /// Group review's actors: park them once it ended; else place the row, cook new takes, start
    /// them together once every seat has its body, and run the shared clock.
    #[allow(clippy::too_many_lines)]
    /// `scale`: the transport's time scale for this frame ([`Transport::scale`]), the same for every seat.
    fn group_play(&mut self, dt: f32, scale: f32, actors: &mut Actors<'_, '_>, roots: &mut Roots<'_, '_>, windows: &mut Query<&mut Window>) {
        let Some(stage) = self.list.group.clone() else {
            // Back to single clips: the copies hidden and stopped, seat 0 on the mark, the cursor's clip cooked.
            for ((mut animator, ..), parent) in &mut *actors {
                animator.time_scale = scale;
                if seat_of(roots, parent.parent()).is_some_and(|s| s > 0) {
                    let asset = Arc::clone(&animator.asset);
                    if let Some(slot) = asset.graph.slot(mw_graphs::build::slots::FULL) {
                        animator.instance.stop_slot(&asset.binding(), slot, 0.0);
                    }
                }
            }
            for (seat, mut transform, mut visibility, _) in &mut *roots {
                let want = if seat.0 == 0 { Visibility::Inherited } else { Visibility::Hidden };
                visibility.set_if_neq(want);
                if seat.0 == 0 && transform.translation != Vec3::ZERO {
                    transform.translation = Vec3::ZERO;
                }
            }
            self.group = GroupPlay { spawned: self.group.spawned, spacing: self.group.spacing, ..GroupPlay::default() };
            self.pending = true;
            return;
        };
        let n = stage.takes.len();
        for (seat, mut transform, mut visibility, _) in &mut *roots {
            let place = stage.in_seat(seat.0).map(|(rank, _)| Vec3::new(row_x(rank, n, self.group.spacing), 0.0, 0.0));
            visibility.set_if_neq(if place.is_some() { Visibility::Inherited } else { Visibility::Hidden });
            if let Some(at) = place.filter(|at| transform.translation != *at) {
                transform.translation = at;
            }
        }
        if self.group.step == RowStep::Cook {
            self.group.step = RowStep::Run;
            let (Some(target), Some(asset)) = (self.target.as_ref(), actors.iter().next().map(|((a, ..), _)| Arc::clone(&a.asset))) else { return };
            let t = Instant::now();
            let mut errors = Vec::new();
            self.group.clips = Default::default();
            for take in &stage.takes {
                let (late, cut) = cook_take(&take.clip, &self.source, target, &asset, self.loop_cut);
                if !cut.is_empty() {
                    println!("browse {}: {cut}", take.clip.rel);
                }
                match late {
                    Ok(late) => self.group.clips[take.seat] = Some(late),
                    Err(e) => errors.push(format!("take {}: {e}", super::browse::take_number(&take.clip))),
                }
            }
            self.cut.clear();
            self.status = format!("{n} takes cooked + retargeted in {:.0} ms{}", t.elapsed().as_secs_f32() * 1000.0, if errors.is_empty() { String::new() } else { format!("   error: {}", errors.join("; ")) });
            println!("clip-preview: group {} {n} takes", stage.name);
            println!("browse group {}: {}", stage.name, self.status);
            for mut w in &mut *windows {
                w.title = format!("Bevyrrowind - clip preview - group {}", stage.name);
            }
            self.group.step = RowStep::Start;
            self.group.clock = None;
        }
        let on_stage = |seat: Option<usize>| seat.and_then(|s| stage.in_seat(s).map(|_| s));
        let ends: Vec<f32> = stage.takes.iter().filter_map(|t| self.group.clips[t.seat].as_deref().map(take_end)).collect();
        let restart = if self.group.step == RowStep::Start {
            let bodies: Vec<usize> = actors.iter().filter_map(|(_, p)| seat_of(roots, p.parent())).collect();
            let waiting = stage.takes.iter().filter(|t| !bodies.contains(&t.seat)).count();
            if waiting > 0 {
                self.status = format!("waiting for {waiting} actor(s) to load");
                return;
            }
            self.group.step = RowStep::Run;
            self.group.clock = Some(GroupClock::new(ends.iter().copied()));
            true
        } else {
            let Some(clock) = self.group.clock.as_mut() else { return };
            clock.set_length(ends.iter().copied());
            // The row clock runs on the transport's time, so a slowed row plays its takes out.
            clock.rate = scale;
            clock.tick(dt)
        };
        let Some(clock) = self.group.clock else { return };
        for ((mut animator, mut lod, mut transform, root), parent) in &mut *actors {
            let Some(seat) = on_stage(seat_of(roots, parent.parent())) else { continue };
            let Some(late) = self.group.clips[seat].clone() else { continue };
            if restart {
                lod.forced = Some(AnimTier::A0);
                animator.time_scale = scale;
                if let Err(e) = play_full(&mut animator, &mut transform, root, late, BROWSE_BLEND_IN, 0) {
                    eprintln!("error: seat {seat}: {e}");
                }
            } else if clock.holds(take_end(&late)) {
                // A shorter take holds its last frame until the row restarts.
                animator.time_scale = 0.0;
            } else {
                animator.time_scale = scale;
            }
        }
    }

    /// The label over seat `seat`'s actor (row place, take, grade, verdict) and whether it is selected.
    pub(super) fn seat_label(&self, seat: usize) -> Option<(String, bool)> {
        let stage = self.list.group.as_ref()?;
        let (rank, take) = stage.in_seat(seat)?;
        Some((format!("{}  {}", rank + 1, self.list.take_label(&take.clip, self.verdicts.as_ref().ok())), rank == stage.selected))
    }
}

/// Keys, the on-demand cook, and the loop; group review (G) through [`Browse::group_play`].
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments, clippy::too_many_lines)]
pub(super) fn browse_step(
    mut commands: Commands,
    mut preview: ResMut<Preview>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut actors: Actors,
    mut roots: Roots,
    rigs: Rigs,
    globals: Query<&GlobalTransform>,
    mut overlay: Query<&mut Text, With<Overlay>>,
    mut notices: Notices,
    mut windows: Query<&mut Window>,
) {
    if preview.phase != Phase::Browse {
        return;
    }
    let Some(b) = preview.browse.as_mut() else { return };
    // Last update's notice (a verdict action), or the one on screen timing out.
    b.show_notice(&mut notices);
    if let Some(key) = keys.get_just_pressed().copied().find(|k| !bound(*k, b.list.group.is_some())) {
        println!("browse key: {key:?} ignored");
        b.notice = Some(Notice::unbound(&format!("{key:?}")));
    }
    if keys.just_pressed(KeyCode::Tab) {
        b.details = !b.details;
        let text = b.overlay_text();
        for mut t in &mut overlay {
            t.0.clone_from(&text);
        }
    }
    let dt = time.delta_secs();
    let transport_refresh = b.transport_keys(&keys);
    let group_refresh = if b.list.group.is_none() {
        let entering = keys.just_pressed(KeyCode::KeyG);
        if entering {
            b.enter_group(&mut commands, &roots, &rigs, &globals);
        }
        entering
    } else {
        b.group_keys(&keys)
    } || transport_refresh;
    if b.group.active {
        let status = b.status.clone();
        // Once per frame, the same scale for every seat (a row stays in step).
        let scale = b.transport.scale(dt);
        b.group_play(dt, scale, &mut actors, &mut roots, &mut windows);
        // Parking hands the actor back to single clips: their cook refreshes the overlay.
        if b.group.active && (group_refresh || b.status != status) {
            let text = b.overlay_text();
            for mut t in &mut overlay {
                t.0.clone_from(&text);
            }
        }
        return;
    }
    if group_refresh {
        let text = b.overlay_text();
        for mut t in &mut overlay {
            t.0.clone_from(&text);
        }
    }
    for (key, step_activity, step) in [(KeyCode::ArrowRight, false, 1), (KeyCode::ArrowLeft, false, -1), (KeyCode::ArrowDown, true, 1), (KeyCode::ArrowUp, true, -1)] {
        if keys.just_pressed(key) {
            let verdicts = b.verdicts.as_ref().ok();
            if step_activity {
                b.list.step_activity_in(step, verdicts);
            } else {
                b.list.step_clip_in(step, verdicts);
            }
            b.pending = true;
            b.message.clear();
        }
    }
    if keys.just_pressed(KeyCode::KeyL) {
        b.loop_cut = !b.loop_cut;
        b.pending = true;
        b.message = format!("loop cut {}", if b.loop_cut { "on" } else { "off: raw takes" });
    }
    let refresh = b.filter_keys(&keys) | b.verdict_keys(&keys);
    // A clip change (navigation, L, a rating, undo, back from group review): play resumes, the speed stays.
    if b.pending {
        b.transport.resume();
    }
    if refresh && !b.pending {
        let text = b.overlay_text();
        for mut t in &mut overlay {
            t.0.clone_from(&text);
        }
    }
    // Single-clip review plays on seat 0, the stage's own NPC.
    let Some(((mut animator, _, mut transform, root), _)) = actors.iter_mut().find(|(_, p)| seat_of(&roots, p.parent()) == Some(0)) else { return };
    let scale = b.transport.scale(dt);
    animator.time_scale = scale;
    if b.pending {
        b.pending = false;
        let Some(target) = b.target.as_ref() else { return };
        let clip = b.list.current().clone();
        let t = Instant::now();
        let asset = Arc::clone(&animator.asset);
        let (late, cut) = cook_take(&clip, &b.source, target, &asset, b.loop_cut);
        let cooked = late.and_then(|late| {
            // A cut loop has a loop range: the slot loops it (the seam as the game plays it); a raw take replays below.
            let loops = if late.clip.loop_range.is_some() { LOOP_FOREVER } else { 0 };
            play_full(&mut animator, &mut transform, root, Arc::clone(&late), BROWSE_BLEND_IN, loops).map(|()| (late, loops > 0))
        });
        b.cut = cut;
        if !b.cut.is_empty() {
            println!("browse {}: {}", clip.rel, b.cut);
        }
        b.status = match cooked {
            Ok((late, looped)) => {
                let s = format!("{:.2} s, cooked + retargeted in {:.0} ms", late.clip.duration, t.elapsed().as_secs_f32() * 1000.0);
                b.playing = Some((late, (!looped).then_some(0.0)));
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
    let Some((late, played)) = b.playing.as_mut() else { return };
    // A cut loop walks on in the slot (the floor and camera follow it); only a raw take replays.
    let Some(played) = played else { return };
    // Clip time as the transport scales it: a slowed take plays out before it replays.
    *played += dt * scale;
    if *played >= (late.clip.duration - BROWSE_REPLAY_LEAD).max(0.1) {
        if let Err(e) = play_full(&mut animator, &mut transform, root, Arc::clone(late), BROWSE_BLEND_IN, 0) {
            eprintln!("error: {e}");
        }
        *played = 0.0;
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
    actors: Bodies,
    roots: SeatRoots,
    mut cameras: Query<&mut Transform, OrbitCamera>,
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
    let Some(at) = focus(&actors, &roots) else { return };
    let target = Vec3::new(at.x, 0.95, at.z);
    let (sy, cy) = cam.yaw.sin_cos();
    let (sp, cp) = cam.pitch.sin_cos();
    let eye = target + cam.distance * Vec3::new(cp * sy, sp, cp * cy);
    for mut t in &mut cameras {
        *t = Transform::from_translation(eye).looking_at(target, Vec3::Y);
    }
}

/// Each group seat's label over its actor (row place, take, grade, verdict; the selected one
/// highlighted), after transforms propagate; hidden outside group review.
#[allow(clippy::needless_pass_by_value)]
pub(super) fn seat_labels(
    preview: Res<Preview>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    bodies: Query<(&GlobalTransform, &ChildOf), With<Animator>>,
    seats: Query<&Seat>,
    mut labels: Query<(&SeatLabel, &mut Text, &mut Node, &mut TextColor)>,
) {
    let camera = cameras.iter().next();
    for (label, mut text, mut node, mut color) in &mut labels {
        let at = bodies.iter().find(|(_, p)| seats.get(p.parent()).is_ok_and(|s| s.0 == label.0)).map(|(g, _)| g.translation());
        let shown = preview.browse.as_ref().and_then(|b| b.seat_label(label.0)).zip(at).zip(camera).and_then(|(((line, selected), at), (camera, eye))| {
            camera.world_to_viewport(eye, at + Vec3::Y * LABEL_HEIGHT_M).ok().map(|px| (line, selected, px))
        });
        let Some((line, selected, px)) = shown else {
            if node.display != Display::None {
                node.display = Display::None;
            }
            continue;
        };
        if text.0 != line {
            text.0 = line;
        }
        let tint = if selected { Color::srgb(1.0, 0.85, 0.2) } else { Color::srgb(0.95, 0.95, 0.85) };
        if color.0 != tint {
            color.0 = tint;
        }
        let (left, top) = (Val::Px(px.x - LABEL_OFFSET_PX), Val::Px(px.y));
        if node.display != Display::Flex || node.left != left || node.top != top {
            node.display = Display::Flex;
            node.left = left;
            node.top = top;
        }
    }
}
