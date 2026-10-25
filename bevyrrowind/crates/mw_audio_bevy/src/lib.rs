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
//!
//! Dev builds only (`dev` feature) until the owner hears them (visual-checks.md AU-3..AU-5):
//! - NPC voices ([`voices`]): Hello on approach, idle chatter, [`voices::SayVoice`] for the
//!   Phase 3 combat and crime barks, one line per NPC, on the Voice bus (speech ducks music),
//!   and the [`voices::LipSync`] loudness on the speaker.
//! - Boots: an NPC's footsteps follow the boots it wears (`mw_actors` auto-equip, weight class).
//! - Swinging doors: activating a non-load door plays its open, then its close sound, at the door.

use std::collections::HashMap;
use std::sync::Arc;

use bevy_app::{App, PostStartup, Update};
use bevy_asset::{AssetServer, UntypedHandle};
use bevy_audio::SpatialListener;
use bevy_ecs::prelude::*;
use bevy_log::info;
use bevy_math::Vec3;
use bevy_time::{Real, Time};
use bevy_transform::components::GlobalTransform;
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
    #[cfg(feature = "dev")]
    if let Some(db) = db {
        app.insert_resource(Boots { rules: mw_actors::equip::EquipRules::from_db(&db), db: Arc::clone(&db) })
            .init_resource::<SwingDoors>()
            .add_plugins(voices::VoicePlugin { db })
            .add_systems(
                Update,
                (
                    boots_footwear.after(add_footsteps).before(anim_footsteps).before(footsteps),
                    swing_door_sounds.after(sf_script_bevy::ScriptSlot::Update),
                )
                    .before(sf_audio_bevy::AudioSystems),
            );
    }
}

/// Adds sounds another builder resolved (the weather's) that the bank lacks.
pub fn merge_bank(app: &mut App, bank: &sf_audio::SoundBank) {
    let mut ours = app.world_mut().resource_mut::<AudioBank>();
    for (id, file) in &bank.0 {
        ours.0.0.entry(id.clone()).or_insert_with(|| file.clone());
    }
}

fn attach_listener(mut commands: Commands, cams: Query<Entity, (With<mw_doors_bevy::DoorUser>, Without<SpatialListener>)>) {
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

/// The open sound of a load door the player activated (swinging doors: [`swing_door_sounds`]).
fn door_sounds(
    mut outcomes: MessageReader<sf_script_bevy::ActivationOutcome>,
    placements: Option<Res<sf_script_bevy::ScriptPlacements>>,
    links: Option<Res<DoorLinks>>,
    audio: Res<MwAudio>,
    cams: Query<(Entity, &GlobalTransform), With<mw_doors_bevy::DoorUser>>,
    mut play: MessageWriter<PlaySound>,
) {
    let (Some(placements), Some(links)) = (placements, links) else { return };
    for o in outcomes.read().filter(|o| !o.handled && o.activation.actor == sf_script_bevy::PLAYER) {
        let Some(door) = placements.get(o.activation.target) else { continue };
        let Some(open) = audio.doors.get(&RecordId::new(&door.record)).and_then(|s| s.open.clone()) else { continue };
        if !is_load_door(&links, o.activation.target) {
            continue;
        }
        let (Ok((cam, global)), Some(player)) = (cams.single(), placements.player.as_ref()) else { continue };
        #[allow(clippy::cast_possible_truncation)]
        let world = Vec3::from_array(std::array::from_fn(|i| (door.position[i] - player.position[i]) as f32));
        let (_, rotation, _) = global.to_scale_rotation_translation();
        play.write(PlaySound { cue: open, at: SoundAt::Offset(cam, rotation.inverse() * world) });
    }
}

fn is_load_door(links: &DoorLinks, door: sf_script::Handle) -> bool {
    let id = PlacementId(door.0);
    links.0.iter().flat_map(DoorLink::exits).any(|(from, _)| from.door == Some(id))
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

/// Boots weight classes for footsteps (vanilla: the boots slot's armour class, shoes and bare
/// feet alike `Bare`).
#[cfg(feature = "dev")]
#[derive(Resource)]
struct Boots {
    db: Arc<RecordDb>,
    rules: mw_actors::equip::EquipRules,
}

#[cfg(feature = "dev")]
impl Boots {
    /// What an NPC record wears on its feet: the same auto-equip pick that dresses it
    /// (`MwActors::default_equipment`: leveled items resolved, beast races without boots).
    fn of(&self, id: &RecordId) -> Footwear {
        use mw_actors::equip::{EquipSlot, Item, WeightClass, auto_equip, npc_skills};
        let Some(npc) = self.db.get::<tes3::esp::Npc>(id) else { return Footwear::Bare };
        let beast = self
            .db
            .get::<tes3::esp::Race>(&RecordId::new(&npc.race))
            .is_some_and(|r| r.data.flags.contains(tes3::esp::RaceFlags::BEAST_RACE));
        let items = npc
            .inventory
            .iter()
            .filter(|(count, _)| *count != 0)
            .filter_map(|(_, item)| mw_actors::held::leveled_item_pick(&self.db, item.as_str()));
        let worn = auto_equip(&self.db, &self.rules, items, &npc_skills(&self.db, npc), beast);
        match worn.iter().find(|(slot, _)| *slot == EquipSlot::Boots) {
            Some((_, Item::Armor(boots))) => match self.rules.weight_class(boots) {
                WeightClass::Light => Footwear::Light,
                WeightClass::Medium => Footwear::Medium,
                WeightClass::Heavy => Footwear::Heavy,
            },
            _ => Footwear::Bare,
        }
    }
}

/// New movers that are NPC records step in the boots they wear.
#[cfg(feature = "dev")]
#[allow(clippy::needless_pass_by_value)]
fn boots_footwear(boots: Res<Boots>, mut steps: Query<(&mut Footsteps, &sf_character_bevy::Character), Added<Footsteps>>) {
    for (mut steps, character) in &mut steps {
        if let sf_character_bevy::Character::Record { id, .. } = character {
            steps.footwear = boots.of(id);
        }
    }
}

/// Named scaffold `SwingDoors`: which swinging (non-load) doors are open, flipped on each
/// activation, so the sounds alternate open / close. Doors don't move yet; removal path: the
/// door swing (doors.rs) owns the open state and its open / close starts play the sounds.
#[cfg(feature = "dev")]
#[derive(Resource, Default)]
struct SwingDoors(std::collections::HashSet<sf_script::Handle>);

/// The open or close sound of a swinging door the player activated, at the door.
#[cfg(feature = "dev")]
#[allow(clippy::needless_pass_by_value)]
fn swing_door_sounds(
    mut outcomes: MessageReader<sf_script_bevy::ActivationOutcome>,
    placements: Option<Res<sf_script_bevy::ScriptPlacements>>,
    links: Option<Res<DoorLinks>>,
    audio: Res<MwAudio>,
    mut open: ResMut<SwingDoors>,
    cams: Query<&GlobalTransform, With<mw_doors_bevy::DoorUser>>,
    mut play: MessageWriter<PlaySound>,
) {
    let (Some(placements), Some(links)) = (placements, links) else { return };
    for o in outcomes.read().filter(|o| !o.handled && o.activation.actor == sf_script_bevy::PLAYER) {
        let target = o.activation.target;
        let Some(door) = placements.get(target) else { continue };
        let Some(sounds) = audio.doors.get(&RecordId::new(&door.record)) else { continue };
        if is_load_door(&links, target) {
            continue;
        }
        let opening = open.0.insert(target);
        if !opening {
            open.0.remove(&target);
        }
        let Some(cue) = (if opening { &sounds.open } else { &sounds.close }).clone() else { continue };
        let (Ok(cam), Some(player)) = (cams.single(), placements.player.as_ref()) else { continue };
        #[allow(clippy::cast_possible_truncation)]
        let offset = Vec3::from_array(std::array::from_fn(|i| (door.position[i] - player.position[i]) as f32));
        play.write(PlaySound { cue, at: SoundAt::Point(cam.translation() + offset) });
    }
}

/// NPC voices (`mw_audio::voice`): who speaks when, the line's voice on the speaker, its
/// loudness for lip-sync.
#[cfg(feature = "dev")]
pub mod voices {
    use std::collections::HashMap;
    use std::sync::Arc;

    use bevy_app::{App, Plugin, Update};
    use bevy_asset::{AssetServer, Assets, Handle};
    use bevy_audio::{AudioSink, AudioSinkPlayback, AudioSource, Decodable, Source, SpatialAudioSink, SpatialListener};
    use bevy_ecs::prelude::*;
    use bevy_log::{debug, info, warn};
    use bevy_math::Vec3;
    use bevy_tasks::{AsyncComputeTaskPool, Task, block_on, poll_once};
    use bevy_time::Time;
    use bevy_transform::components::GlobalTransform;
    use mw_audio::voice::{BarkView, ENVELOPE_HZ, Envelope, VoiceDirector, VoiceRules, VoiceSense, VoiceTopic, ducking, select};
    use mw_rpg::{ActorFacts, GlobalNumber, MwDialogue};
    use sf_audio_bevy::{AudioBank, AudioDucking, AudioSystems, CueInstance, PlaySound, SoundAt};
    use sf_character_bevy::Character;
    use sf_records::{RecordDb, RecordId};
    use sf_rpg::{ActorKey, ConversationCtx, PlayerId};

    /// A line whose voice never started (beyond its range, over a cap) ends after this.
    const START_GRACE_S: f32 = 0.5;
    /// Slower than this, an NPC stands idle (idle chatter).
    const IDLE_SPEED_MPS: f32 = 0.2;
    /// Speakers' origins are at their feet, the listener is an eye: distances are eye to eye.
    const EYE_M: f32 = 1.6;
    /// Decoded envelopes kept (a few KB each); the cache restarts past it.
    const ENVELOPE_CACHE: usize = 256;

    /// Ask an NPC to bark a voice topic (the Phase 3 combat and crime hooks: Attack, Hit,
    /// Flee, Thief, Intruder, Alarm). Dropped while the NPC is already saying something.
    #[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
    pub struct SayVoice {
        pub actor: Entity,
        pub topic: VoiceTopic,
    }

    /// An NPC that can speak: its record's facts for the INFO filters.
    #[derive(Component, Debug)]
    pub struct Speaker {
        facts: ActorFacts,
        hello: i16,
        disposition: i32,
        last: Option<Vec3>,
        speed_mps: f32,
    }

    /// The line an NPC is saying (removed when its voice ends).
    #[derive(Component, Debug)]
    pub struct Speaking {
        pub cue: RecordId,
        path: String,
        elapsed: f32,
        heard: bool,
    }

    /// Lip-sync hook: the playing line's loudness now (0..1), for the face animation; on the
    /// speaker while [`Speaking`].
    #[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
    pub struct LipSync {
        pub level: f32,
    }

    #[derive(Resource)]
    struct VoiceContent {
        db: Arc<RecordDb>,
        dialogue: MwDialogue,
        globals: HashMap<RecordId, GlobalNumber>,
        /// The player's facts (no player character yet: none).
        player: ActorFacts,
    }

    #[derive(Resource)]
    pub struct Director(pub VoiceDirector);

    #[derive(Resource, Default)]
    struct Envelopes {
        ready: HashMap<String, Arc<Envelope>>,
        loading: HashMap<String, Handle<AudioSource>>,
        decoding: Vec<(String, Task<Option<Envelope>>)>,
    }

    /// Needs `SfAudioPlugin` (the bank with `mw_audio`'s voice cues, the ducker) first.
    pub struct VoicePlugin {
        pub db: Arc<RecordDb>,
    }

    impl Plugin for VoicePlugin {
        fn build(&self, app: &mut App) {
            let dialogue = match MwDialogue::from_records(&self.db) {
                Ok(d) => d,
                Err(e) => {
                    warn!("audio: NPC voices off: {e}");
                    return;
                }
            };
            let rules = VoiceRules::from_db(&self.db);
            info!("audio: NPC voices on ({rules:?})");
            if let Some(mut ducker) = app.world_mut().get_resource_mut::<AudioDucking>() {
                for rule in ducking() {
                    ducker.0.add_rule(rule);
                }
            }
            app.insert_resource(VoiceContent {
                globals: mw_audio::voice::start_globals(&self.db),
                db: Arc::clone(&self.db),
                dialogue,
                player: ActorFacts { npc: true, ..ActorFacts::default() },
            })
            .insert_resource(Director(VoiceDirector::new(rules, 0x5eed)))
            .init_resource::<Envelopes>()
            .add_message::<SayVoice>()
            .add_systems(Update, (add_speakers, npc_voices, track_lines, end_lines, envelopes).chain().before(AudioSystems));
        }
    }

    fn key(e: Entity) -> ActorKey {
        ActorKey(u128::from(e.to_bits()))
    }

    #[allow(clippy::needless_pass_by_value)]
    fn add_speakers(mut commands: Commands, content: Res<VoiceContent>, fresh: Query<(Entity, &Character), Added<Character>>) {
        for (e, character) in &fresh {
            let Character::Record { id, .. } = character else { continue };
            let Some(npc) = content.db.get::<tes3::esp::Npc>(id) else { continue };
            commands.entity(e).try_insert(Speaker {
                facts: ActorFacts::from_npc(&content.db, npc, ""),
                hello: npc.ai_data.hello,
                disposition: i32::from(npc.data.disposition),
                last: None,
                speed_mps: 0.0,
            });
        }
    }

    /// Greetings and idle chatter from the director, plus [`SayVoice`] requests, become lines.
    #[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
    fn npc_voices(
        time: Res<Time>,
        content: Res<VoiceContent>,
        bank: Res<AudioBank>,
        mut director: ResMut<Director>,
        mut says: MessageReader<SayVoice>,
        listener: Query<&GlobalTransform, With<SpatialListener>>,
        mut speakers: Query<(Entity, &mut Speaker, &GlobalTransform)>,
        mut play: MessageWriter<PlaySound>,
        mut commands: Commands,
    ) {
        let dt = time.delta_secs();
        let Some(ear) = listener.iter().next().map(GlobalTransform::translation) else { return };
        let mut senses = Vec::new();
        for (e, mut s, global) in &mut speakers {
            let pos = global.translation();
            if let Some(last) = s.last.replace(pos)
                && dt > 0.0
            {
                s.speed_mps = (pos - last).length() / dt;
            }
            // Named gaps: the player is the listener (the door-using camera) until the player
            // character exists (ROADMAP 2.1 A2), every NPC sees the player (no line-of-sight or
            // sneak test yet) and is free to talk (no combat or conversation state yet).
            senses.push(VoiceSense {
                actor: key(e),
                distance_m: (pos + Vec3::Y * EYE_M).distance(ear),
                hello: s.hello,
                aware: true,
                free: true,
                idle: s.speed_mps < IDLE_SPEED_MPS,
            });
        }
        let by_key: HashMap<ActorKey, Entity> = speakers.iter().map(|(e, ..)| (key(e), e)).collect();
        let wanted = director.0.update(dt, &senses).into_iter().filter_map(|r| by_key.get(&r.actor).map(|e| (*e, r.topic)));
        let asked: Vec<(Entity, VoiceTopic)> = says.read().map(|s| (s.actor, s.topic)).collect();
        for (e, topic) in wanted.chain(asked) {
            let Ok((_, speaker, _)) = speakers.get(e) else { continue };
            let view = BarkView {
                records: &content.db,
                speaker: speaker.facts.clone(),
                player: &content.player,
                globals: &content.globals,
                disposition: speaker.disposition,
            };
            let ctx = ConversationCtx { player: PlayerId::HOST, speaker: key(e), speaker_base: speaker.facts.id.clone(), choice: None };
            let Some(line) = select(&content.dialogue, &view, &ctx, topic) else { continue };
            let Some(file) = bank.0.get(&line.cue) else { continue };
            if !director.0.begin(key(e)) {
                continue;
            }
            // Subtitles wait for the message box (ROADMAP 2.2 U4); the log shows them.
            debug!("audio: {} says {:?}: {}", speaker.facts.id.as_str(), topic, line.subtitle);
            play.write(PlaySound { cue: line.cue.clone(), at: SoundAt::Entity(e) });
            commands.entity(e).try_insert((
                Speaking { cue: line.cue, path: file.path.clone(), elapsed: 0.0, heard: false },
                LipSync::default(),
            ));
        }
    }

    /// Follows each line's voice (a child of the speaker): its playback time drives the
    /// lip-sync level; once it is gone (or never started) the line ends.
    #[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
    fn track_lines(
        time: Res<Time>,
        envelopes: Res<Envelopes>,
        mut lines: Query<(Entity, &mut Speaking, &mut LipSync, Option<&Children>)>,
        voices: Query<(&CueInstance, Option<&AudioSink>, Option<&SpatialAudioSink>)>,
        mut commands: Commands,
    ) {
        for (e, mut line, mut lip, children) in &mut lines {
            line.elapsed += time.delta_secs();
            let voice = children.into_iter().flatten().filter_map(|c| voices.get(*c).ok()).find(|(cue, ..)| cue.0 == line.cue);
            let Some((_, sink, spatial)) = voice else {
                if line.heard || line.elapsed > START_GRACE_S {
                    commands.entity(e).try_remove::<(Speaking, LipSync)>();
                }
                continue;
            };
            line.heard = true;
            let t = sink.map(AudioSink::position).or_else(|| spatial.map(SpatialAudioSink::position)).map_or(line.elapsed, |p| p.as_secs_f32());
            lip.level = envelopes.ready.get(&line.path).map_or(0.0, |env| env.at(t));
        }
    }

    /// A finished line (or a despawned speaker) frees the NPC for the next one.
    fn end_lines(mut ended: RemovedComponents<Speaking>, mut director: ResMut<Director>) {
        for e in ended.read() {
            director.0.end(key(e));
        }
    }

    /// Decodes each new line's file off the main thread into its loudness envelope (cached).
    #[allow(clippy::needless_pass_by_value)]
    fn envelopes(
        server: Option<Res<AssetServer>>,
        sources: Option<Res<Assets<AudioSource>>>,
        mut env: ResMut<Envelopes>,
        new: Query<&Speaking, Added<Speaking>>,
    ) {
        let (Some(server), Some(sources)) = (server, sources) else { return };
        for line in &new {
            let pending = env.ready.contains_key(&line.path)
                || env.loading.contains_key(&line.path)
                || env.decoding.iter().any(|(p, _)| *p == line.path);
            if !pending {
                env.loading.insert(line.path.clone(), server.load(line.path.clone()));
            }
        }
        let loaded: Vec<(String, AudioSource)> = env
            .loading
            .iter()
            .filter_map(|(path, handle)| sources.get(handle).map(|s| (path.clone(), s.clone())))
            .collect();
        for (path, source) in loaded {
            env.loading.remove(&path);
            env.decoding.push((path, AsyncComputeTaskPool::get().spawn(async move { decode(&source) })));
        }
        env.loading.retain(|_, h| !server.load_state(h.id()).is_failed());
        let mut done = Vec::new();
        env.decoding.retain_mut(|(path, task)| match block_on(poll_once(task)) {
            Some(result) => {
                done.push((std::mem::take(path), result));
                false
            }
            None => true,
        });
        for (path, result) in done {
            if env.ready.len() >= ENVELOPE_CACHE {
                env.ready.clear();
            }
            if let Some(envelope) = result {
                env.ready.insert(path, Arc::new(envelope));
            }
        }
    }

    /// `None` for a file rodio cannot decode (its decoder panics on an unknown format).
    fn decode(source: &AudioSource) -> Option<Envelope> {
        let decoder = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| source.decoder())).ok()?;
        let (channels, rate) = (decoder.channels().get(), decoder.sample_rate().get());
        Some(Envelope::from_samples(decoder, channels, rate, ENVELOPE_HZ))
    }
}

#[cfg(all(test, feature = "dev"))]
mod tests {
    use std::time::Duration;

    use bevy_app::TaskPoolPlugin;
    use bevy_time::{TimePlugin, TimeUpdateStrategy};
    use bevy_transform::components::Transform;
    use mw_audio::voice::VoiceTopic;
    use sf_audio::Bus;
    use sf_audio_bevy::{AudioEvent, AudioMixer, CueInstance};
    use sf_character::CharacterState;
    use sf_character_bevy::Character;
    use tes3::esp::{
        Armor, ArmorData, ArmorType, Dialogue, DialogueInfo, DialogueType2, FixedString, Npc, NpcData, NpcStats,
    };

    use super::voices::{Director, LipSync, SayVoice, Speaking, VoicePlugin};
    use super::*;

    const HELLO: &str = "voice/vo/d/m/hlo_test.mp3";

    fn id(s: &str) -> RecordId {
        RecordId::new(s)
    }

    fn info(topic: &str, info: &str, race: &str, sound: &str) -> (RecordId, DialogueInfo) {
        let mut i = DialogueInfo { id: info.into(), speaker_race: race.into(), sound_path: sound.into(), text: "Hm?".into(), ..DialogueInfo::default() };
        i.data.speaker_rank = -1;
        i.data.player_rank = -1;
        (mw_rpg::response_id(topic, info), i)
    }

    /// A Dunmer with a Hello and an Attack line, wearing light boots; a heavy-booted Nord.
    fn db() -> Arc<RecordDb> {
        let mut db = RecordDb::new();
        db.register::<Dialogue>("DIAL").unwrap();
        db.register::<DialogueInfo>("INFO").unwrap();
        db.register::<Npc>("NPC_").unwrap();
        db.register::<Armor>("ARMO").unwrap();
        let esm = db.add_source("Test.esm", &[]);
        for (topic, kind) in [("hello", DialogueType2::Voice), ("attack", DialogueType2::Voice)] {
            db.define(esm, id(topic), Dialogue { id: topic.into(), dialogue_type: kind, ..Dialogue::default() }).unwrap();
        }
        for (key, i) in [info("hello", "h1", "dark elf", "Vo\\d\\m\\Hlo_test.mp3"), info("attack", "a1", "", "Vo\\d\\m\\Atk_test.mp3")] {
            db.define(esm, key, i).unwrap();
        }
        for (boots, weight) in [("light_boots", 5.0), ("heavy_boots", 20.0)] {
            let data = ArmorData { armor_type: ArmorType::Boots, weight, armor_rating: 10, ..ArmorData::default() };
            db.define(esm, id(boots), Armor { id: boots.into(), data, ..Armor::default() }).unwrap();
        }
        for (npc, race, boots) in [("dunmer", "Dark Elf", "light_boots"), ("nord", "Nord", "heavy_boots")] {
            let mut n = Npc {
                id: npc.into(),
                race: race.into(),
                inventory: vec![(1, FixedString(boots.to_owned()))],
                data: NpcData { stats: Some(NpcStats { skills: [30; 27], ..NpcStats::default() }), ..NpcData::default() },
                ..Npc::default()
            };
            n.ai_data.hello = 30;
            db.define(esm, id(npc), n).unwrap();
        }
        Arc::new(db)
    }

    fn app(db: &Arc<RecordDb>) -> App {
        let sounds = mw_audio::build(db, |raw| Some(raw.to_ascii_lowercase()));
        let mut app = App::new();
        // `MinimalPlugins` minus the frame counter and runner (`app.update()` drives it).
        app.add_plugins((TaskPoolPlugin::default(), TimePlugin))
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(100)))
            .add_plugins(SfAudioPlugin { bank: sounds.bank, playback: false })
            .insert_resource(AudioCues(sounds.cues))
            .add_plugins(VoicePlugin { db: Arc::clone(db) });
        // An eye 1.6 m above the floor the NPCs stand on.
        let eye = Transform::from_xyz(0.0, 1.6, 0.0);
        app.world_mut().spawn((SpatialListener::new(0.2), eye, GlobalTransform::from(eye)));
        app
    }

    fn npc(app: &mut App, record: &str, at: Vec3) -> Entity {
        let character = Character::Record { kind: "npc".into(), id: id(record), state: CharacterState::default() };
        app.world_mut().spawn((character, Transform::from_translation(at), GlobalTransform::from_translation(at))).id()
    }

    fn voice_cues(app: &mut App) -> Vec<(RecordId, Bus)> {
        let events: Vec<AudioEvent> = app.world_mut().resource_mut::<Messages<AudioEvent>>().drain().collect();
        events.into_iter().filter_map(|e| if let AudioEvent::Cue { cue, bus, .. } = e { Some((cue, bus)) } else { None }).collect()
    }

    fn speaking(app: &App, e: Entity) -> bool {
        app.world().resource::<Director>().0.speaking(ActorKey(u128::from(e.to_bits())))
    }

    use sf_rpg::ActorKey;

    #[test]
    fn hello_on_approach_plays_on_the_voice_bus_one_line_at_a_time() {
        let db = db();
        let mut app = app(&db);
        let far = npc(&mut app, "dunmer", Vec3::new(10.0, 0.0, 0.0));
        let near = npc(&mut app, "dunmer", Vec3::new(2.0, 0.0, 0.0));
        app.update();
        app.update();
        assert_eq!(voice_cues(&mut app), vec![(id(HELLO), Bus::Voice)]);
        assert!(app.world().get::<Speaking>(near).is_some() && app.world().get::<LipSync>(near).is_some());
        assert!(app.world().get::<Speaking>(far).is_none());
        // The line's voice is the speaker's child; speech ducks the music.
        let voice = app.world_mut().query::<(Entity, &CueInstance, &ChildOf)>().iter(app.world()).find(|(_, c, p)| c.0 == id(HELLO) && p.parent() == near).map(|(e, ..)| e).expect("a voice under the speaker");
        for _ in 0..3 {
            app.update();
        }
        assert!(app.world().resource::<AudioMixer>().0.bus_gain(Bus::Music) < 0.5);
        // A combat bark while the greeting plays is dropped.
        app.world_mut().write_message(SayVoice { actor: near, topic: VoiceTopic::Attack });
        app.update();
        assert!(voice_cues(&mut app).is_empty());
        // The voice ends: the line ends, the next bark plays.
        app.world_mut().despawn(voice);
        app.update();
        assert!(app.world().get::<Speaking>(near).is_none() && !speaking(&app, near));
        app.world_mut().write_message(SayVoice { actor: near, topic: VoiceTopic::Attack });
        app.update();
        assert_eq!(voice_cues(&mut app), vec![(id("voice/vo/d/m/atk_test.mp3"), Bus::Voice)]);
        // No Hello line for a Nord: the greeting passes in silence.
        let nord = npc(&mut app, "nord", Vec3::new(0.0, 0.0, 2.0));
        app.update();
        app.update();
        assert!(voice_cues(&mut app).is_empty() && app.world().get::<Speaking>(nord).is_none());
    }

    #[test]
    fn a_despawned_speaker_frees_its_voice_slot() {
        let db = db();
        let mut app = app(&db);
        let near = npc(&mut app, "dunmer", Vec3::new(2.0, 0.0, 0.0));
        app.update();
        app.update();
        assert!(speaking(&app, near));
        app.world_mut().entity_mut(near).despawn();
        app.update();
        assert!(!speaking(&app, near));
    }

    #[test]
    fn boots_set_the_footstep_class() {
        let db = db();
        let boots = Boots { rules: mw_actors::equip::EquipRules::from_db(&db), db: Arc::clone(&db) };
        assert_eq!(boots.of(&id("dunmer")), Footwear::Light);
        assert_eq!(boots.of(&id("nord")), Footwear::Heavy);
        assert_eq!(boots.of(&id("nobody")), Footwear::Bare);
    }
}
