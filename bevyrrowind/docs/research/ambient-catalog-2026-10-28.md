# Ambient NPC life catalog for Morrowind (2026-10-28)

Legend. **Src**: have = covered by UAL1/UAL2 clips we own; gen = text-to-motion (Kimodo) or in-betweening (MotionBricks); pack = needs a free/purchased pack; hard = needs hand-precision (retarget + IK + hand-authored).
**IK**: L/M/H = body placement difficulty. **Pt** = interaction point needed. Phases: E = enter, L = loop (variants), X = exit.

## 1. Taxonomy (how the big games classify ambient activity)

Confidence note: GTA/Skyrim/UE are well documented; Cyberpunk workspot detail comes from modding docs; Crimson Desert and KCD2 are thin or fan-sourced (flagged).

| Class | Meaning | Examples in the big games |
|---|---|---|
| Standalone idle | No world object, plays in place | GTA `WORLD_HUMAN_STAND_IMPATIENT`, `_SMOKING`, `_HANG_OUT_STREET`, `_PUSH_UPS`, `_YOGA`, `_JOG_STANDING`, `_MUSICIAN`, `_CHEERING`, `_MOBILE_FILM_SHOCKING`; Skyrim idle markers |
| Object-bound ("interaction point") | A placed marker in the world supplies position/heading and often the clip set | GTA `PROP_HUMAN_SEAT_BENCH/CHAIR/BAR`, `PROP_HUMAN_BBQ`, `PROP_HUMAN_ATM`, `PROP_HUMAN_BUM_BIN`, `_PARKING_METER`; Skyrim furniture (chair, bench, bed, lean, workbench, forge, grindstone, alchemy, cooking pot, mine-wall); Cyberpunk workspots (bar stool, vending machine, counter, wall lean); UE Smart Object slots |
| Prop-carrying | Prop attached to a hand bone for the duration (spawned, then despawned) | GTA `WORLD_HUMAN_DRINKING`, `_SMOKING_POT`, `_CLIPBOARD`, `_HAMMERING`, `_JANITOR`(broom), `_GARDENER_PLANT`, `_MUSICIAN`(guitar), `_BINOCULARS`, `_TOURIST_MAP`; Skyrim sweeping (broom), woodchopping, mining |
| Paired / group | Two+ NPCs sync on one scenario | GTA `WORLD_HUMAN_PROSTITUTE_*`, `CODE_HUMAN_MEDIC_TEND_TO_DEAD`, conversations; RDR2 camp circle and conversation groups; Skyrim "talk-to-each-other" packages (dialogue is cosmetic); Cyberpunk NPC pairs at tables |
| Locomotion style | Same navmesh path, different gait set | GTA move-clipsets (drunk, injured, fat, tough, hurried); RDR2 gaits per personality; Cyberpunk casual/hurried/panicked; Skyrim old/hunched walk replacers |
| Reaction / modal | Overrides on external trigger | Weather (RDR2 rain cover, shelter, umbrella), crime witness, gunfire flee, greetings to player; Cyberpunk "react to V" turns heads |

**Chaining.** Every serious system is enter -> loop(variants) -> exit:
- GTA scenario = base clip + weighted random "variation" clips ("conditional anims") with random loop count, then exit clip; some have an "intro" only for the first time.
- Skyrim furniture = Enter anim event, Idle loop, Exit anim event, via "FurnitureMarker" entry/exit vectors per side; NPC walks to the marker, snaps, plays enter.
- Cyberpunk workspot = graph of "anim sequences" with entry/exit points and idle-state randomizer (e.g. sit -> idle -> look around -> drink); workspot files bind an entity to a slot with a transform.
- UE Smart Objects = definition with slots; claim slot (reservation), use via Gameplay Behavior (any animation/montage/state tree), release. Claim/reserve avoids two NPCs on one stool.

**Interrupts.** Universal pattern: (1) soft interrupt = finish current loop iteration or play exit clip then leave (player talks, hour changes); (2) hard interrupt = blend out immediately and reposition (combat, crime, being shoved, ash storm); (3) look-at layer stays on top of everything. Skyrim: packages have a "conditions + interrupt flags"; GTA: scenario flee flags and "shocking events"; UE: release smart-object claim on state-tree exit.

**Schedules.** Skyrim packages ranked by priority and time window (sleep, eat, sandbox, travel to location); KCD2 per-NPC daily schedules with hand-set work/sleep/eat points and persistent simulation of off-screen NPCs (public Warhorse talks; details of KCD2 not independently verified); Crimson Desert reportedly the same shape (day-night sleep/work cycles, fishing, farming, mining, guards; fan wiki, unverified); RDR2 uses scenario points in the world with time-of-day density and "ambient scenario" spawns near the player only.

**Sources**
- GTA V scenario names (community, partial): https://pastebin.com/wSXf3B6b , https://pastebin.com/8pQAXyir
- Cyberpunk workspots/NPC animation (REDmodding wiki): https://wiki.redmodding.org/cyberpunk-2077-modding/for-mod-creators-theory/files-and-what-they-do/file-formats/animations-.anims/npc-animations ; CDPR on 1000+ handmade routines: https://www.tweaktown.com/news/73048/over-thousand-cyberpunk-2077-npcs-will-have-unique-daily-routines
- Skyrim furniture/packages (Creation Kit wiki): https://ck.uesp.net/wiki/Furniture , https://ck.uesp.net/wiki/Package
- UE Smart Objects: https://dev.epicgames.com/documentation/en-us/unreal-engine/smart-objects-in-unreal-engine
- Crimson Desert (fan wiki, unverified): https://thegameswiki.com/crimson-desert/wiki/npc-daily-routines
- Not verified from a primary source in this pass: RDR2 scenario system internals (beyond common knowledge), KCD2 schedule talks. Treat as to-confirm.

## 2. The catalog

### 2.1 Cross-cutting base moves (used by many settings)

| Activity | Kind | Pt | Props | Phases | IK | Src |
|---|---|---|---|---|---|---|
| Stand idle, weight shift, look around | standalone | none | - | L variants (LookAround, FoldArms, Tired) | L | have |
| Stand talking (gesture) | standalone/paired | none | - | L (Talking) | L | have |
| Sit on chair/stool/bench | object | seat 0.4-0.5 m | - | E, L (Idle/02/03), X | M | have |
| Sit and talk / nod | object/paired | seat | - | L (Talking, Nodding) | M | have |
| Sit on ground | object | floor | - | E, L, X | L | have |
| Lie down / get up | object | bed/floor | - | E (IdleToLay), L, X (LayToIdle) | M | have |
| Lean on rail/wall | object | rail/wall edge | - | L (Rail Loop, Call) | M | have |
| Lean back against wall (foot up) | object | wall | - | L | M | gen |
| Lean elbows on counter | object | counter 1.0 m | - | L (Counter_Idle) | M | have |
| Drink from held mug | prop | none | mug | L (Drink) | M | have |
| Hold lantern / torch while standing | prop | none | lantern, torch | L | L | have |
| Pick up item from floor/table | object | table/floor | - | one-shot (PickUp_Kneeling/Table) | M | have |
| Interact (button, door, shrine touch) | object | any | - | one-shot | M | have |
| Crouch / kneel at task | standalone | floor | - | L | L | have |
| Cast idle (spell hand) | standalone | none | - | L | L | have |

### 2.2 Imperial town (Seyda Neen, Balmora, Caldera)

| Activity | Kind | Pt | Props | Phases | IK | Src |
|---|---|---|---|---|---|---|
| Guard stand at post, arms folded | standalone | none | spear/halberd optional | L variants | L | have |
| Guard patrol march | locomotion | path | weapon sheathed | L | L | gen (stiff march) |
| Guard lean on wall/rail | object | wall | - | L | M | have |
| Guard stop and scan crowd | standalone | none | - | one-shot | L | have |
| Guard salute officer | paired | none | - | one-shot | L | gen |
| Census clerk writes | object | desk seat | quill, ledger | E, L(write, dip quill, look up), X | H | hard |
| Clerk stamps/slides papers | object | desk | paper, stamp | L | H | hard |
| Clerk reads ledger | object | desk/stand | book | L | M | gen |
| Trader behind counter (idle, wave over wares) | object | counter | goods | L | M | have/gen |
| Trader weighs coin, counts | object | counter | coins | L | H | hard |
| Pawnbroker inspects item against light | prop | none | item | L | H | gen |
| Smith hammers anvil | object | anvil | hammer, tongs | E, L(hit, flip, inspect), X | H | pack |
| Smith works bellows / quenches | object | forge/barrel | bellows, tongs | L | H | pack |
| Smith grinds blade | object | grindstone | blade | L | H | pack |
| Alchemist grinds mortar | object | table | mortar, pestle | L | H | gen |
| Alchemist stirs / pours vial | object | table | vial, spoon | L | H | gen |
| Enchanter chanting over item (soul gem glow) | object | table | soul gem | L, VFX | M | gen |
| Tailor stitching / measuring cloth | object | table/seat | needle, cloth | L | H | hard |
| Bookseller shelving / browsing | object | shelf | book | L | M | gen |
| Scribe copies text | object | desk seat | quill, scroll | L | H | hard |
| Customer browses stall, picks and puts back | object | stall | goods | L | M | have (PickUp_Table) |
| Beggar or loiterer on steps | object | step | - | L | L | have |
| Town crier on box | standalone | box | scroll | L | L | gen |

### 2.3 Tavern / cornerclub

| Activity | Kind | Pt | Props | Phases | IK | Src |
|---|---|---|---|---|---|---|
| Bartender wipes counter/mug | object | bar counter | cloth, mug | L | H | gen |
| Bartender pours drink | object | bar | bottle, mug | one-shot | H | gen |
| Bartender sets mug in front of patron | paired | bar | mug | one-shot | M | have (PickUp_Table partial) |
| Drinker sits, sips, sets down | object | seat + table | mug | E, L(Drink, Idle), X | M | have |
| Drinker slumps asleep on table | object | table | - | E, L, X | M | gen |
| Sleeping drunk on floor / bench | object | floor | bottle | E(IdleToLay), L, X | M | have |
| Gamblers: throw dice | paired | table | dice | L | H | hard |
| Gamblers: cheer/groan reaction | paired | table | - | one-shot | L | gen |
| Bard plays lute (seated/standing) | prop | stool | lute | L | H | hard (fingers) |
| Bard sings (no instrument) | standalone | stage | - | L | L | gen |
| Drummer | prop | seat | drum | L | M | pack |
| Dancer | standalone | floor | - | L | L | pack |
| Argument (point, shove, back off) | paired | none | - | L | M | gen |
| Eating (bowl/skewer) | prop | table | bowl, spoon | L | M | gen |
| Toast / clink mugs | group | table | mugs | one-shot | M | gen |
| Patron orders at bar | object | bar | - | L | M | have (Counter_Idle) |

### 2.4 Homes

| Activity | Kind | Pt | Props | Phases | IK | Src |
|---|---|---|---|---|---|---|
| Cooking at pot, stirring | object | hearth/pot | ladle | L | M | gen |
| Chopping ingredients | object | table | knife | L | H | gen |
| Sweeping | prop | floor | broom | L (walk-sweep) | M | gen |
| Laundry: wring, hang | prop | line/basin | cloth | L | H | gen |
| Washing hands/face at basin | object | basin | - | L | M | gen |
| Making the bed | object | bed | blanket | L | H | gen |
| Sleeping in bed | object | bed | - | E, L, X | M | have (Lay) |
| Reading at table / chair | object | seat | book | L (page turn) | M | gen |
| Writing | object | desk | quill | L | H | hard |
| Praying at shrine / kneel | object | shrine | - | E, L, X | M | have (kneel) |
| Warming hands at fire | object | hearth | - | L | L | gen |
| Tidying shelf | object | shelf | items | L | M | have |
| Looking out window | object | window | - | L | L | gen |
| Pacing, pondering | locomotion | none | - | L | L | have |

### 2.5 Farms / plantations / herds

| Activity | Kind | Pt | Props | Phases | IK | Src |
|---|---|---|---|---|---|---|
| Harvest saltrice (bend and cut) | prop | crop patch | sickle, basket | L | M | have (Farm_Harvest) |
| Plant seed / scatter seed | prop | field | seeds | L | L | have |
| Water crops | prop | field | can | L | L | have |
| Kwama egg mine: gather eggs, basket | object | egg sac | basket | L | M | have (PickUp_Kneeling) |
| Netch herding (stand with crook, look up) | prop | herd | crook | L | L | gen |
| Guar handling: feed, walk beside, brush | paired (with creature) | creature | brush, feed | L | H | hard (creature rig) |
| Haul sacks on shoulder | locomotion | path | sack | E, L, X | M | have (Walk_Carry) + gen |
| Stack sacks / crates | object | pile | sack | one-shot | M | have (PickUp) |
| Hoe/weed | prop | field | hoe | L | M | gen |
| Slaves/workers on Dunmer plantations: same labour clips as above, no gratuitous cruelty; show weary gait, supervisor overseer standing and pointing. Framed by in-game fiction as in the source material | locomotion + prop | field | tools | L | M | have/gen |
| Rest, drink from skin | prop | field edge | skin | L | L | have (Drink) |

### 2.6 Mines (ebony, glass, egg)

| Activity | Kind | Pt | Props | Phases | IK | Src |
|---|---|---|---|---|---|---|
| Pick-axe strike | object | rock wall | pick | L | H | gen |
| Shovel into cart | prop | pile | shovel | L | M | gen |
| Push ore cart | prop | rails | cart | E, L, X (Push) | M | have |
| Carry ore basket | locomotion | path | basket | L | M | have |
| Resting miner, seated, wiping brow | object | rock | - | L | L | have (Tired, Sit) |
| Lantern inspection | prop | wall | lantern | L | L | have |
| Sort ore at table | object | table | ore | L | M | have |
| Overseer with ledger | standalone | none | ledger | L | L | gen |

### 2.7 Ports / fishing (Hla Oad, Gnaar Mok, Ebonheart)

| Activity | Kind | Pt | Props | Phases | IK | Src |
|---|---|---|---|---|---|---|
| Fishing with rod: cast, wait, reel | object | water edge/dock | rod | E(Cast), L(Cast_Idle), X(Reel) | M | have |
| Mend nets, seated | object | ground/seat | net | L | H | gen |
| Gut fish at board | object | table | knife, fish | L | H | gen |
| Boat repair (hammer, plane) | prop | boat | hammer | L | M | have(Fixing_Kneeling)+gen |
| Coil rope | prop | dock | rope | L | M | gen |
| Crate loading (carry, set down) | locomotion + object | dock | crate | L | M | have |
| Silt strider porter: carry pack, load | paired (creature) | strider | pack | L | H | hard |
| Caravaner calls out (hand to mouth) | standalone | none | - | L | L | have (Idle_Rail_Call) |
| Bargaining with ship captain | paired | none | - | L | L | have (Talking) |
| Mooring line tie | object | bollard | rope | one-shot | H | hard |
| Dock lookout, hand shading eyes | standalone | none | - | L | L | gen |

### 2.8 Temple / Tribunal / Imperial cult

| Activity | Kind | Pt | Props | Phases | IK | Src |
|---|---|---|---|---|---|---|
| Priest preaches (gesture, hand raised) | standalone | pulpit | book | L | M | gen |
| Pilgrim kneels at altar | object | altar | - | E, L, X | M | have |
| Prostrate prayer | object | floor | - | E, L, X | M | gen |
| Healer lays hands on a patient | paired | patient | VFX | L | M | gen |
| Monk meditation (cross-legged) | object | floor | - | L | L | have (GroundSit) |
| Monk walk with hands folded | locomotion | path | - | L | L | gen |
| Light candles/incense | object | candle | taper | one-shot | M | gen |
| Chanting group in a circle | group | floor | - | L | L | gen |
| Pilgrim sweeps shrine steps | prop | step | broom | L | M | gen |

### 2.9 Great House specifics

| Activity | Kind | Pt | Props | Phases | IK | Src |
|---|---|---|---|---|---|---|
| Telvanni mage levitating (hover pose) | standalone | none | - | E, L, X | L | gen (+ VFX) |
| Telvanni conjuring (summon circle) | standalone | floor | - | E, L, X | L | have (Spell_Double_Idle)+VFX |
| Telvanni reading in tower, hover read | prop | none | tome | L | M | gen |
| Redoran guard drill (sword form, shield) | group | yard | sword | L | M | have (Sword_Idle)+pack |
| Redoran formation stand at attention | group | yard | - | L | L | have |
| Hlaalu merchants haggle (palm gestures) | paired | none | purse | L | M | gen |
| Hlaalu clerk writes contracts | object | desk | quill | L | H | hard |
| Noble receives petitioner | paired | throne | - | L | M | have (Sitting) |

### 2.10 Ashlander camps

| Activity | Kind | Pt | Props | Phases | IK | Src |
|---|---|---|---|---|---|---|
| Sit cross-legged by fire | object | fire/floor | - | E, L, X | L | have (GroundSit) |
| Wise woman gesturing / chanting | standalone | fire | staff | L | L | gen |
| Hunter skins an animal | object | carcass | knife | L | H | gen |
| Guar herder with crook | prop | herd | crook | L | L | gen |
| Drumming | prop | floor/seat | drum | L | M | pack |
| Stir stew pot | object | fire | ladle | L | M | gen |
| Tan hide on frame | object | frame | scraper | L | H | gen |
| Sharpen spear / fletch | prop | seat | spear | L | M | gen |
| Yurt door flap in/out | object | doorway | - | one-shot | M | have (Interact) |
| Sleeping in yurt | object | bed | - | E, L, X | M | have |

### 2.11 Guilds (Mages, Fighters, Thieves)

| Activity | Kind | Pt | Props | Phases | IK | Src |
|---|---|---|---|---|---|---|
| Strike training dummy | object | dummy | sword | L (chain) | M | have (combat clips) |
| Sparring pair (non-lethal) | paired | yard | practice weapons | L | M | gen/pack |
| Cast at target | object | target | VFX | L | L | have |
| Study at table, book | object | desk | book | L | M | gen |
| Alchemy bench stirring | object | bench | flask | L | H | gen |
| Lockpicking practice | object | practice lock | picks | L | H | hard |
| Thieves lookout, leaning, peek around corner | standalone | wall | - | L | M | have (Crouch/Rail)+gen |
| Pocket-palm sleight practice | standalone | none | coin | L | H | hard |
| Guildmaster arms behind back | standalone | none | - | L | L | gen |

### 2.12 Wilderness / travellers

| Activity | Kind | Pt | Props | Phases | IK | Src |
|---|---|---|---|---|---|---|
| Campfire sit and poke fire | object | fire | stick | E, L, X | M | have/gen |
| Sit on rock | object | rock height | - | E, L, X | M | have |
| Map reading | prop | none | map | L | M | gen |
| Forage (kneel, pluck) | object | plant | basket | L | M | have (PickUp_Kneeling) |
| Hunt: crouch, draw bow | standalone | none | bow | L | M | have(Crouch)+gen |
| Bathing/wash in river | object | water edge | - | L | M | gen |
| Wash clothes at river | prop | water edge | cloth | L | H | gen |
| Fill waterskin | object | water edge | skin | L | M | have (Kneel)+gen |
| Rest against tree/pack | object | tree | pack | L | M | gen |
| Check hoof/gear on pack animal | paired (creature) | creature | - | L | H | hard |
| Travel: walk with staff, look-around | locomotion | path | staff | L | L | have |

### 2.13 Nobles / Vivec cantons / street

| Activity | Kind | Pt | Props | Phases | IK | Src |
|---|---|---|---|---|---|---|
| Strolling arm-in-arm | paired locomotion | path | - | L | M | gen |
| Fanning self | prop | none | fan | L | L | gen |
| Conversation group (3-4 turning heads) | group | circle | - | L | L | have (Talking/FoldArms) |
| Beggar sits, hand out | object | ground/wall | bowl | L | L | have(GroundSit)+gen |
| Beggar calls to passer-by | standalone | ground | - | one-shot | L | gen |
| Street performer (juggle/dance/sing) | standalone | plaza | props | L | M | pack |
| Preacher on box | standalone | box | - | L | L | gen |
| Window shopper | standalone | stall | - | L | L | have |
| Courier hurried walk | locomotion | path | satchel | L | L | gen |

### 2.14 Weather / time reactions

| Activity | Kind | Pt | Props | Phases | IK | Src |
|---|---|---|---|---|---|---|
| Ash storm: cover face with arm/cloth, hunched walk | locomotion/reaction | none | - | E, L, X | L | gen |
| Ash storm: duck indoors (pathing to nearest door) | reaction | doorway | - | E | L | have |
| Rain: hood up, hunch; shelter under awning | reaction | awning | hood | E, L | L | gen |
| Evening: light torch/lantern, hang on hook | prop | wall | torch | one-shot | M | have(Idle_Torch)+gen |
| Night: yawn, rub eyes, stretch | standalone | none | - | one-shot | L | gen |
| Morning stretch, shake out | standalone | none | - | one-shot | L | gen |
| Cold: hug self | standalone | none | - | L | L | gen |
| Heat: wipe brow | standalone | none | - | one-shot | L | gen |

### 2.15 Social and paired

| Activity | Kind | Pt | Props | Phases | IK | Src |
|---|---|---|---|---|---|---|
| Handshake | paired | none | - | one-shot | H (hand meet) | hard |
| Hug / clasp forearms | paired | none | - | one-shot | H | hard |
| Argue (point, wave off) | paired | none | - | L | M | gen |
| Point directions to player | standalone | none | - | one-shot | L | gen |
| Haggle (hand wave) | paired | counter | - | L | M | gen |
| Gossip circle, lean in, whisper | group | circle | - | L | M | gen |
| Look-at / glance at player | standalone | none | - | head/eye layer | L | procedural |
| Greet player (nod, raise hand) | standalone | none | - | one-shot | L | gen |
| Laugh | standalone | none | - | one-shot | L | gen |
| Watch fight or crime (shocked, back away) | reaction | none | - | L | L | gen |
(No children exist in Morrowind; no child clips needed.)

### 2.16 Locomotion styles

| Style | Needs | Src |
|---|---|---|
| Tired (slow, droop) | walk variant | have Idle_Tired + gen walk |
| Drunk (sway, stumble) | walk variant | gen |
| Injured (limp, hold side) | walk variant | gen |
| Old (hunched, short stride) | walk variant | gen |
| Carry heavy (sack/crate) | walk | have (Walk_Carry) |
| Hurried (fast, head down) | walk/jog | gen |
| Sneaking thief | crouch walk | have (Crouch) |
| Patrol march | walk | gen |
| Carry lantern/torch | walk | have |
| Shy/furtive (looks over shoulder) | walk | gen |

## 3. Coverage tiers (about 40)

Rule from RDR2/Cyberpunk: density and variety within ~30 m of the player beat rare showpieces. Ship things everybody sees every minute first.

**Tier 1, mostly "have", ship first (about 16)**
1 stand idle variants (LookAround/FoldArms/Tired); 2 stand talking; 3 sit on chair/bench/stool + talk/nod; 4 sit on ground / by fire; 5 sleep in bed; 6 sleeping drunk; 7 lean on rail/wall; 8 lean on counter (trader/bar); 9 drink from mug; 10 guard stand + lean; 11 fishing at water; 12 farm harvest/plant/water; 13 carry sack/basket walk; 14 push cart; 15 hold torch/lantern; 16 kneel at shrine / pray.

**Tier 2, cheap generation, strong payoff (about 14)**
17 sweep with broom; 18 bartender wipe+pour; 19 smith hammer (pack if gen is poor); 20 read book seated; 21 conversation groups; 22 beggar with alms; 23 preacher on box; 24 yawn/stretch/night reactions; 25 ash storm face cover; 26 rain hood/shelter; 27 stir pot; 28 drunk/old/hurried/injured gaits; 29 patrol march; 30 hunter skinning.

**Tier 3, harder or niche (about 10)**
31 lute bard (hard); 32 dice gamblers (hard); 33 writing clerk/scribe (hard); 34 handshake/hug (hard); 35 guar/strider handling (creature rig); 36 Telvanni levitation + conjure (VFX heavy); 37 sparring pairs; 38 drumming; 39 net mending/gut fish; 40 lockpick practice.

Order of work: Tier 1 needs only the interaction-point + scheduling system (no new clips). Build the system and tag content (seats, beds, rails, counters, water edges, fires, shrines) before generating clips.

## 4. What makes it read as alive (beyond clips)

| Layer | What to do |
|---|---|
| Idle variants with random timers | Per NPC: base loop + weighted variants fired on a random 4-15 s timer (GTA conditional anims, Skyrim idle sets). Seed per-NPC so no two share a timeline. |
| Anti-sync | Random start phase on every loop, +/-5-10% playback speed, random clip pick on entry, never same clip at same time for neighbouring NPCs (cooldown per cluster). |
| Look-at | Head/eye layer toward player within ~6 m, brief; guards track longer; neutral NPCs glance then return. Layered over any anim (Cyberpunk, RDR2 do this). |
| Barks | Short contextual lines (weather, time, player faction/reputation, crime). Rate-limit per NPC and globally; proximity-based. Cyberpunk and RDR2 use ambient chatter heavily for density. |
| Props | Spawn on enter, attach to hand bone, despawn on exit (mug, broom, lantern, quill). Hide/swap held items (weapon sheathe) properly; cull when far. |
| Schedules by hour | Sleep, work, eat, socialise windows (Skyrim packages; KCD2 schedules; Crimson Desert reportedly). Doors, lights and tavern crowds change with time. Persist coarse state off-screen. |
| Weather reactions | Ash storm and rain override with hunch/cover/shelter; return afterward. Stagger reaction onset so a crowd does not flip at once. |
| Crowd LOD | Full scenario + IK near (<30 m); simplified loop, no IK, lower tick far; abstract schedule only beyond. Spawn ambient extras near player only (RDR2 style). |
| Claims | Reserve interaction points (UE Smart Object style) so two NPCs never take the same stool; release on interrupt. |
| Interrupts | Soft vs hard (see section 1). Player talk = soft blend to face player; combat = hard. |
| Reaction to events | Witnessing fights/crimes, reacting to player equipment (drawn weapon), greeting. Cheap but disproportionately convincing. |
| Variety | Multiple body/voice/prop sets per role, so ten identical fishers do not read as clones; vary prop type and speed. |
| Density near player | Concentrate richest scenarios where the camera is; lean on Tier 1 repeated with variants over showpieces. |

Sources for section 4 concepts (same as section 1): GTA scenario conditional anims and flags (pastebin lists above), Skyrim package priorities/conditions (https://ck.uesp.net/wiki/Package), UE Smart Object claim/release (Epic docs above), CDPR routine statement (tweaktown link above). KCD2/RDR2 specifics not verified in this pass.
