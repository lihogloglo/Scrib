# R13: how shipped games handle the owner-pass animation issues

Research only (2026-10-22). Nothing was built or run. Sources are public docs, talks and tutorials; no GPL code
was read. "est." marks my own physics derivations. **unverified** marks anything I could not confirm.

## 1. Planted foot slides in one phase of the cycle

**Technique.** A planted foot has zero ground speed, so root speed must equal the speed the clip's feet imply.
Either root motion drives the capsule, or the clip is in-place and warped to capsule speed. Unreal's
[Stride Warping](https://dev.epicgames.com/documentation/en-us/unreal-engine/pose-warping-in-unreal-engine)
scales foot-IK goals by `StrideScale = LocomotionSpeed / RootMotionSpeed`, with a minimum-speed bypass.
Playback-rate scaling changes cadence, not stride: keep it within about +-10-15 % (est.), else blend by speed
or warp. Foot lock (IK pin during the contact window) removes the residue.

**Pitfalls (slides in one phase only).** Baked root speed that varies over the cycle (hip sway) while the
capsule moves at the mean. Blend-space samples with different stride lengths. Blending clips out of foot phase.
**Check:** planted-foot world speed under about 0.05 m/s (est.).

## 2. Stops are a long smooth glide

**Technique (Lyra/Paragon).** Start, stop and pivot clips carry a distance curve (from root motion) built by
an animation modifier ([Epic notes](https://www.unrealengine.com/tech-blog/adapting-lyra-animation-to-your-ue5-game)).
[Distance Match to Target](https://dev.epicgames.com/documentation/unreal-engine/API/Plugins/AnimationLocomotionLibraryRuntim-/UAnimDistanceMatchingLibrary/DistanceMatchToT-)
sets clip time each frame so remaining curve distance equals predicted distance to stop
([overview](https://dev.epicgames.com/documentation/unreal-engine/distance-matching-in-unreal-engine)). It
ignores the phase of the preceding jog, so select stop-L or stop-R by the planted foot. Motion matching
([Clavet, GDC 2016](https://gdcvault.com/play/1022985/Motion-Matching-and-The-Road)) needs no authored stops:
desired velocity goes to zero and the search finds the stop pose. Its stop parameters are **unverified**.

**Who drives whom.** Third-person games are usually capsule-driven: a fixed braking rate on the capsule, the
stop clip distance-matched to it, and no clip root motion applied. Root-motion stops are more exact but delay
input response. Pick one source of truth.

**Numbers (est., unverified against a source).** Walk 1.5 m/s: stop in about 0.4-0.7 m, 1-2 steps, a about 2-3
m/s^2. Run 4-5 m/s: about 2-3.5 m, 3-4 steps, a about 3-5 m/s^2 (sprint stops up to 6-8). Unreal's default
walking braking is around 2000 cm/s^2 (from memory, **unverified**).

**Pitfalls.** Exponential damping on the capsule gives an asymptotic glide: use constant deceleration with a
cutoff at about 0.1 m/s. Locomotion clip played at a rate near zero. Wrong planted-foot stop clip.

## 3. Footstep sounds out of sync

**Technique.** Unreal [sync groups](https://dev.epicgames.com/documentation/unreal-engine/animation-sync-groups-in-unreal-engine):
one leader (highest blend weight by default) provides the playhead and fires notifies. Followers' notifies are
suppressed. Roles: Can Be Leader, Always Leader, Never Leader, Transition Leader/Follower. Sync markers are
named foot-down events that align clips of different lengths. Alternatives: a foot-contact curve (fire on the
rising edge over a threshold) or a foot-bone height/velocity test.

**Pitfalls.** Every blended clip firing gives doubles. Text keys sit in clip time, so after rate scaling or
IK/warping the sound follows the clip, not the visible foot. Loop wrap: test `(prev, end]` plus `[0, now]`,
half-open, or keys are missed or doubled. A walk-idle blend firing the idle clip's step. Add a per-foot
minimum gap of about 0.15-0.25 s (est.) and a minimum leader weight.

## 4. Ragdoll get-up without a pop

**Technique** ([ALS ragdoll recover](https://seouistudio.notion.site/Ragdoll-with-recover-d4a6a3bb5d6648aaa817e2bc357be1a3),
[Unity](https://www.moddb.com/news/blending-ragdoll-physics-and-animation-in-unity),
[Godot](https://bugnet.io/blog/how-to-fix-godot-blend-from-ragdoll-back-to-animation-snapping)):
1. When the ragdoll is settled (slow for about 0.3-0.5 s, est.), snapshot every bone pose.
2. Pick the clip from the pelvis: dot(pelvis axis, world up) above or below 0 gives the front or back clip.
3. Rotate the root about world up to the pelvis heading, and place it at the pelvis x,z on the ground.
4. Blend snapshot into clip over about 0.2-0.4 s (est.). Inertialization (Bollo, GDC 2018, Gears of War)
   beats a cross-fade: no second pose evaluation, less sliding. The tutorials above use plain cross-fades.

**Pitfalls.** Blending through the root or pelvis collapses the body out of the capsule or spins it: blend
through a spine bone, or keep the root unsimulated. Clip heading not aligned to the pelvis: it snaps around.

## 5. Capsule re-sync after ragdoll

Teleport the capsule to the ground-projected pelvis with the step-3 heading before the clip starts, and keep it
kinematic during get-up. **"Slides to its capsule after get-up" causes:** the capsule stayed at the pre-ragdoll
spot, or the clip's baked root motion moves the mesh but is never applied to the capsule, so the pose
re-syncs with a slide at the end. Fix: apply the clip's root motion to the capsule (preferred, get-up covers
1-2 m), or use an in-place clip. Never both.

## 6. Character freezes or loses its capsule when pushed

[Rapier's controller](https://rapier.rs/docs/user_guides/templates/character_controller/) is kinematic: it
moves only by the translation you give it, and pushes on dynamic bodies only if you enable impulse solving
(with a character mass). Jolt's README contrasts a
[virtual character](https://github.com/jrouwe/JoltPhysics) (outside the simulation, less accurate dynamic
interaction) with a rigid-body character.

**Pitfalls.** A dynamic capsule with a kinematic override: the two fight and the body freezes. Ragdoll trigger
on raw velocity: use impulse (mass x velocity) over a threshold plus a cooldown. A push that never moves the
capsule while the animation keeps "standing".

## 7. Capsule vibrates while walking

[Rapier snap-to-ground](https://rapier.rs/docs/user_guides/bevy_plugin/character_controller_snap_to_ground/)
applies only when the move starts grounded, has a slight downward component, and the gap is within the snap
distance. Defaults: snap about 0.2 of height, offset 0.01 relative, autostep off since v0.19. The snap test
changed between v0.17 and v0.18.

**Causes.** Snap distance flipping on and off at the threshold. Gravity too small to reach contact, so
grounded state alternates: keep a small downward bias while grounded. Skin width too small, so depenetration
fights gravity. Root motion sampled per render frame in a fixed-step controller: use one delta or accumulate.
No interpolation of the visual between fixed steps. Camera reading the capsule before it moves. I found no
Rapier document specific to jitter, so the dominant cause is **unverified**.

## 8. Capsule sizing for wide, low quadrupeds and crabs

An upright capsule is too tall (floats, blocks doors) or too thin for a wide body. Options: a box or low
cylinder for the controller (Rapier shape-casts accept any shape), a capsule on its side (**unverified**
for controllers), or a main body shape plus separate hit shapes per limb. Size from bind-pose bounds:
horizontal extents from the body mesh XZ bounds excluding legs, height from ground to top of back or shell,
step height equal to the leg lift.
