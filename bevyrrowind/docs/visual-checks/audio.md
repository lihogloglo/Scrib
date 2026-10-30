# Visual checks: audio

Open visual checks: NPC voices, footsteps, doors (AU-*). Index and how-to: [`../visual-checks.md`](../visual-checks.md).

## Sound: NPC voices, boots, swinging doors (superfargoth `audio.md`; ROADMAP 2.6, session voices)

Dev builds only (`cargo dev ...`): the glue is behind the `dev` feature until these pass. Agents cannot hear; the
synthetic and app tests cover selection, timing and routing (`mw_audio` voice tests, `audio::tests`). Run
`RUST_LOG=bevyrrowind::audio=debug` to see each line's subtitle (`audio: <npc> says Hello: ...`).

### AU-3: NPC greetings, idle chatter, lip-sync level (`cargo dev view "Seyda Neen, Census and Excise Office"`)
- Fly the camera up to a standing NPC: within ~2-3 m (AI Hello x 6 units) they say a Hello line matching their
  race and sex, from their position, once; stay close: no repeat; back off past ~7 m and return: greets again.
- Stand among NPCs a few minutes: now and then an idle line (about one per NPC every ~3 minutes on average).
- Music dips (~8 dB) under speech and comes back over ~1 s. Never two lines at once from one NPC.
- Known gaps: no line-of-sight test (voices reach through walls), quest/local filters see stage 0 and no locals.
- Owner 2026-10-30 (Census office): most NPCs neither greet nor turn toward the player; only some say a Hello line.
  Wanted: NPCs face the player when greeting. Status: rejected (greetings mostly missing).
- Owner 2026-10-30, later (`scene door-balmora-guild`): NPCs do say their Hello lines, but only when you are very close
  (the trigger range feels too short; the first run missed it for that reason). They don't stop or turn toward the
  player (wanted). Status: partial (range too short, no stop/turn).

### AU-4: Footsteps by boots (`cargo dev scene door-balmora-guild`)
- Walkers in heavy boots (guards) step heavy, light/medium armour boots light/medium, shoes and bare feet bare.
- Owner 2026-10-30 (`scene door-balmora-guild`): footsteps by boots work. Status: confirmed.

### AU-5: Swinging-door sounds (`cargo dev view "Seyda Neen, Census and Excise Office"`, any inner door)
- Activate a non-load door: its open sound at the door; again: its close sound. The door itself does not move
  yet (the swing is a doors.rs follow-up), so the open/close order is the only state.
- Owner 2026-10-30: inner doors show the use prompt but don't open (no swing) and play no sound. Status: rejected.

### AU-6: Third-person listener sits in the character's head, not the camera (owner report)
- Kind: game location. Any scene in third person with sounds around (`scene door-balmora-guild`).
- Seen 2026-10-30: sounds pan and fade as if heard from the character's head; owner wants them heard from the camera.
- Note: shipped third-person games often put the listener at the camera and take distance attenuation from the
  character (a hybrid); research and pick in `audio` spec before changing.
- Status: known bug (owner pass 2026-10-30).
