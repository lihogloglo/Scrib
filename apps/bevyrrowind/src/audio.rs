//! Morrowind audio glue (superfargoth audio.md, ROADMAP 2.6): `mw_audio`'s bank, cues and
//! playlists into `sf_audio_bevy`, plus the few systems that turn game facts into sounds.
//!
//! - Listener: `SpatialListener` on the door-using camera (first and third person alike).
//! - Doors: a load door the player opens plays its `DOOR` open sound beside the camera, as a
//!   child of it, so the sound travels with the player through the load.
//! - Loop emitters: lights with a sound and vanilla sound activators (`mw_audio::loop_script`)
//!   get a `LoopEmitter` child when their cell's near level spawns (a placement-spawner hook,
//!   like `scripts.rs`'s); they unload with the cell. `emitter_census` logs how many are loaded
//!   and audible.
//! - Music: explore by default; battle while [`CombatMusic`] says a hostile fights the player,
//!   held 5 s after (`mw_audio::MusicMood`). Nothing sets the flag before Phase 3 combat.
//! - Footsteps: animated movers near the listener step on their clips' `SoundGen` text keys (the
//!   graph's `footstep` events); movers without an animator fall back to a `StrideTimer`. The
//!   sound follows the vanilla boots weight rule (`Footwear`).
//! - Interior beds: vanilla interiors get their ambience from sound activators (positional, the
//!   emitters above); the `AmbienceLayers` "cell" bed table for the modern preset is a follow-up.

use std::collections::HashMap;
use std::sync::Arc;

use bevy::audio::SpatialListener;
use bevy::prelude::*;
use mw_audio::{Footwear, MusicMood};
use sf_audio::{FootstepMap, Foot, StrideTimer};
use sf_audio_bevy::{AudioBank, AudioCues, LoopEmitter, Music, PlaySound, SfAudioPlugin, SoundAt, SpatialVoice, Voice};
use sf_anim_bevy::AnimEvent;
use sf_controller_bevy::Mover;
use sf_records::{RecordDb, RecordId};
use sf_vfs::Vfs;
use sf_world::{DoorLink, IndexedPlacement, Level, PlacementClass, PlacementId};
use sf_world_bevy::{ActivateCx, DoorLinks, LoadedModel, PlacementSpawner, PlacementSpawners, SpawnCx};

/// Footsteps of other movers play within this distance of the listener.
const STEP_RANGE_M: f32 = 15.0;
/// Per-frame travel above this is a teleport or an origin shift, not a step.
const STEP_JUMP_M: f32 = 3.0;

/// The hook for the battle music rule: combat sets it while any hostile is in combat with the
/// player (Phase 3). Until then it stays false and the music explores.
#[derive(Resource, Default, Clone, Copy, Debug)]
pub struct CombatMusic {
    pub hostile_in_combat: bool,
}

/// What the glue systems need from `mw_audio`.
#[derive(Resource)]
struct MwAudio {
    doors: HashMap<RecordId, mw_audio::DoorSounds>,
    footsteps: FootstepMap,
}

#[derive(Resource, Default)]
struct Mood(MusicMood);

/// A mover's distance-driven footsteps.
#[derive(Component, Default)]
pub struct Footsteps {
    timer: StrideTimer,
    last: Option<Vec3>,
    pub footwear: Footwear,
}

/// Adds the audio plugin with the content's sounds and music. `db` is the loaded content
/// (`None`: synthetic runs, silent apart from what other plugins add to the bank).
pub fn add(app: &mut App, db: Option<Arc<RecordDb>>, vfs: &Vfs) {
    let sounds = db.as_deref().map(|db| mw_audio::vfs_sounds(db, vfs)).unwrap_or_default();
    info!(
        "audio: {} sounds, {} cues, {} door sounds, {} loop emitter bases",
        sounds.bank.0.len(),
        sounds.cues.groups.len(),
        sounds.doors.len(),
        sounds.emitters.len()
    );
    let mut music = Music::default();
    for (tag, tracks) in mw_audio::vfs_playlists(vfs) {
        info!("audio: playlist {tag}: {} tracks", tracks.len());
        music.0.set_playlist(tag, tracks);
    }
    let emitters = Arc::new(sounds.emitters);
    app.add_plugins(SfAudioPlugin { bank: sounds.bank, playback: true })
        .insert_resource(AudioCues(sounds.cues))
        .insert_resource(music)
        .insert_resource(MwAudio { doors: sounds.doors, footsteps: sounds.footsteps })
        .add_message::<sf_script_bevy::ActivationOutcome>()
        .init_resource::<CombatMusic>()
        .init_resource::<Mood>()
        .add_systems(PostStartup, move |spawners: Option<ResMut<PlacementSpawners>>| hook_emitters(spawners, &emitters))
        .add_systems(
            Update,
            (
                attach_listener,
                music_mood,
                door_sounds.after(sf_script_bevy::ScriptSlot::Update),
                add_footsteps,
                anim_footsteps,
                footsteps,
                emitter_census,
            )
                .before(sf_audio_bevy::AudioSystems),
        );
}

/// Adds sounds another builder resolved (the weather's) that the bank lacks.
pub fn merge_bank(app: &mut App, bank: &sf_audio::SoundBank) {
    let mut ours = app.world_mut().resource_mut::<AudioBank>();
    for (id, file) in &bank.0 {
        ours.0.0.entry(id.clone()).or_insert_with(|| file.clone());
    }
}

fn attach_listener(mut commands: Commands, cams: Query<Entity, (With<crate::doors::DoorUser>, Without<SpatialListener>)>) {
    for cam in &cams {
        commands.entity(cam).insert(SpatialListener::new(0.2));
    }
}

fn music_mood(time: Res<Time<Real>>, combat: Res<CombatMusic>, mut mood: ResMut<Mood>, mut music: ResMut<Music>) {
    let tag = mood.0.update(time.delta_secs(), combat.hostile_in_combat);
    if music.0.playing() != Some(tag) {
        music.0.request(Some(tag));
    }
}

/// The open sound of a load door the player activated (non-load doors don't swing yet).
fn door_sounds(
    mut outcomes: MessageReader<sf_script_bevy::ActivationOutcome>,
    placements: Option<Res<sf_script_bevy::ScriptPlacements>>,
    links: Option<Res<DoorLinks>>,
    audio: Res<MwAudio>,
    cams: Query<(Entity, &GlobalTransform), With<crate::doors::DoorUser>>,
    mut play: MessageWriter<PlaySound>,
) {
    let (Some(placements), Some(links)) = (placements, links) else { return };
    for o in outcomes.read().filter(|o| !o.handled && o.activation.actor == sf_script_bevy::PLAYER) {
        let Some(door) = placements.get(o.activation.target) else { continue };
        let Some(open) = audio.doors.get(&RecordId::new(&door.record)).and_then(|s| s.open.clone()) else { continue };
        let id = PlacementId(o.activation.target.0);
        if !links.0.iter().flat_map(DoorLink::exits).any(|(from, _)| from.door == Some(id)) {
            continue;
        }
        let (Ok((cam, global)), Some(player)) = (cams.single(), placements.player.as_ref()) else { continue };
        #[allow(clippy::cast_possible_truncation)]
        let world = Vec3::from_array(std::array::from_fn(|i| (door.position[i] - player.position[i]) as f32));
        let (_, rotation, _) = global.to_scale_rotation_translation();
        play.write(PlaySound { cue: open, at: SoundAt::Offset(cam, rotation.inverse() * world) });
    }
}

fn add_footsteps(mut commands: Commands, movers: Query<Entity, Added<Mover>>) {
    for e in &movers {
        commands.entity(e).insert(Footsteps::default());
    }
}

/// Animated movers step on their clips' `SoundGen: Left/Right` text keys
/// (`mw_anim::events::FOOTSTEP`, payload the foot), as the graph's leading clip
/// fires them at its playback rate: the sound lands on the plant (AN-3). Only
/// a dominant clip's keys count (weight ≥ 0.5), so a crossfade doesn't step twice.
#[allow(clippy::needless_pass_by_value)]
fn anim_footsteps(
    audio: Res<MwAudio>,
    mut events: MessageReader<AnimEvent>,
    listener: Query<&GlobalTransform, With<SpatialListener>>,
    movers: Query<(Entity, &Mover, &GlobalTransform, &Footsteps)>,
    mut play: MessageWriter<PlaySound>,
) {
    let ear = listener.iter().next().map(GlobalTransform::translation);
    let mut by_animator: Option<HashMap<Entity, Entity>> = None;
    for e in events.read() {
        if &*e.name != mw_anim::events::FOOTSTEP || e.weight < 0.5 {
            continue;
        }
        let (Some(ear), Some(foot)) = (ear, e.payload.as_deref()) else { continue };
        let map = by_animator.get_or_insert_with(|| movers.iter().filter_map(|(m, mover, ..)| Some((mover.animator?, m))).collect());
        let Some((entity, mover, global, steps)) = map.get(&e.entity).and_then(|&m| movers.get(m).ok()) else { continue };
        if mover.loco.grounded.is_none() || global.translation().distance(ear) > STEP_RANGE_M {
            continue;
        }
        if let Some(cue) = audio.footsteps.resolve(foot, Some(steps.footwear.surface())) {
            play.write(PlaySound { cue: cue.clone(), at: SoundAt::Entity(entity) });
        }
    }
}

/// Movers without an animator: one step per stride of grounded travel
/// (`mw_audio::stride_m` from the speed).
fn footsteps(
    time: Res<Time>,
    audio: Res<MwAudio>,
    listener: Query<&GlobalTransform, With<SpatialListener>>,
    mut movers: Query<(Entity, &Mover, &GlobalTransform, &mut Footsteps)>,
    mut play: MessageWriter<PlaySound>,
) {
    let dt = time.delta_secs();
    let Some(ear) = listener.iter().next().map(GlobalTransform::translation) else { return };
    for (e, mover, global, mut steps) in &mut movers {
        if mover.animator.is_some() {
            continue;
        }
        let pos = global.translation();
        let Some(last) = steps.last.replace(pos) else { continue };
        let travel = (pos - last).with_y(0.0).length();
        if mover.loco.grounded.is_none() || travel < 1e-4 || travel > STEP_JUMP_M || dt <= 0.0 {
            steps.timer.reset();
            continue;
        }
        let Some(foot) = steps.timer.advance(travel, mw_audio::stride_m(travel / dt)) else { continue };
        if pos.distance(ear) > STEP_RANGE_M {
            continue;
        }
        let event = if foot == Foot::Left { "left" } else { "right" };
        if let Some(cue) = audio.footsteps.resolve(event, Some(steps.footwear.surface())) {
            play.write(PlaySound { cue: cue.clone(), at: SoundAt::Entity(e) });
        }
    }
}

/// Logs the loaded loop emitters and how many are inside their audible range (on change, at
/// most every 5 s): the Balmora / Vivec voice count that decides virtual-voice culling.
fn emitter_census(
    time: Res<Time<Real>>,
    emitters: Query<(), With<LoopEmitter>>,
    voices: Query<(&Voice, &SpatialVoice, &GlobalTransform)>,
    listener: Query<&GlobalTransform, With<SpatialListener>>,
    mut next: Local<f32>,
    mut last: Local<(usize, usize)>,
) {
    *next -= time.delta_secs();
    if *next > 0.0 {
        return;
    }
    *next = 5.0;
    let Some(ear) = listener.iter().next().map(GlobalTransform::translation) else { return };
    let loaded = emitters.iter().count();
    let audible = voices.iter().filter(|(v, s, g)| v.looping && s.0.gain(g.translation().distance(ear)) > 0.0).count();
    if *last != (loaded, audible) {
        *last = (loaded, audible);
        info!("audio: {loaded} loop emitters loaded, {audible} audible");
    }
}

/// Wraps the light and activator spawners so a near-level spawn of an emitter base adds its
/// `LoopEmitter` (a child of the level group at the placement, unloaded with it).
fn hook_emitters(spawners: Option<ResMut<PlacementSpawners>>, emitters: &Arc<HashMap<RecordId, RecordId>>) {
    let Some(mut spawners) = spawners else { return };
    for class in [PlacementClass::Light, mw_world::source::ACTIVATOR, PlacementClass::Static, PlacementClass::Dynamic] {
        if let Some(inner) = spawners.for_class(class).cloned() {
            spawners.set(class, Arc::new(EmitterHook { inner, emitters: emitters.clone() }));
        }
    }
}

struct EmitterHook {
    inner: Arc<dyn PlacementSpawner>,
    emitters: Arc<HashMap<RecordId, RecordId>>,
}

impl PlacementSpawner for EmitterHook {
    fn spawn(&self, world: &mut World, cx: &SpawnCx<'_>, placement: &IndexedPlacement) -> u32 {
        let spawned = self.inner.spawn(world, cx, placement);
        let Some(cue) = (cx.level == Level::Near).then(|| self.emitters.get(&placement.placement.base.id)).flatten() else {
            return spawned;
        };
        world.spawn((Name::new("loop emitter"), LoopEmitter { cue: cue.clone() }, cx.local, ChildOf(cx.parent)));
        spawned + 1
    }
    fn needs_model(&self, placement: &IndexedPlacement) -> bool {
        self.inner.needs_model(placement)
    }
    fn prepare(&self, assets: &AssetServer, placement: &IndexedPlacement, model: Option<&LoadedModel>) -> Option<UntypedHandle> {
        self.inner.prepare(assets, placement, model)
    }
    fn activate(&self, world: &mut World, cx: &ActivateCx<'_>, placement: &IndexedPlacement) {
        self.inner.activate(world, cx, placement);
    }
    fn deactivate(&self, world: &mut World, cx: &ActivateCx<'_>, placement: &IndexedPlacement) {
        self.inner.deactivate(world, cx, placement);
    }
}
