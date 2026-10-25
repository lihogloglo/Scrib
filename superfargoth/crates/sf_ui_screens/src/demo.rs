//! Synthetic data and a tiny host for every stock screen (feature `demo`; the `screens`
//! example and the tests). No game content: invented names, a few rules, generated items.

use std::collections::HashMap;

use bevy_ecs::prelude::*;
use sf_records::{RecordDb, RecordFormat, apply_record_file};
use sf_rpg::{
    ActorKey, AttrValue, Attrs, Choice, ConversationCtx, DialogueDb, DialogueOutput, DialogueRules,
    DialogueSession, EntryDraft, EquipPlacement, EquipRules, GameTime, HeardLine, Holder,
    ItemStack, LinkRules, Loadout, PlayerId, RecordId, RpgCommand, SlotTable, StageFlags,
    TopicEntry, TopicKind, WorldRpg,
};
use sf_rpg_ui::{DialogueRequest, DialogueView, JournalLabels};
use sf_ui::{ActionDef, ActionRef, Keymap, UiRegistry, register_kinds};
use sf_ui_inventory::{Better, InventoryTab, InventoryView, ItemCatalog, ItemInfo, StatDef};

use crate::hud::{HudBar, HudView, UsePrompt};
use crate::journal::JournalSource;

/// Screen records of the demo: the HUD and the menus (the pause menu pauses the game).
pub const SCREENS: &str = r#"
[[define]]
kind = "ui.screen"
id = "hud"
value = { layer = "Hud", implementation = { Template = "hud" } }

[[define]]
kind = "ui.screen"
id = "dialogue"
value = { layer = "Menu", implementation = { Template = "dialogue" } }

[[define]]
kind = "ui.screen"
id = "journal"
value = { layer = "Menu", implementation = { Template = "journal" } }

[[define]]
kind = "ui.screen"
id = "inventory"
value = { layer = "Menu", implementation = { Template = "inventory" } }

[[define]]
kind = "ui.screen"
id = "keybinds"
value = { layer = "Menu", implementation = { Template = "keybinds" } }

[[define]]
kind = "ui.screen"
id = "saveload"
value = { layer = "Menu", implementation = { Template = "saveload" } }

[[define]]
kind = "ui.screen"
id = "pause"
value = { layer = "Menu", pause = "WhileOpen", implementation = { Template = "pause" } }
"#;

/// The registry of [`SCREENS`].
pub fn registry() -> UiRegistry {
    let mut db = RecordDb::new();
    register_kinds(&mut db).expect("UI record kinds register");
    let base = db.add_source("demo", &[]);
    let outcome =
        apply_record_file(&mut db, base, SCREENS, RecordFormat::Toml).expect("demo screens parse");
    assert!(outcome.errors.is_empty(), "{:?}", outcome.errors);
    UiRegistry::build(&db).0
}

fn id(s: &str) -> RecordId {
    RecordId::new(s)
}

// ---------------------------------------------------------------- world and journal

/// The shared world state (quests, journal, known topics) and its labels.
#[derive(Resource)]
pub struct DemoWorld {
    pub world: WorldRpg,
    labels: DemoLabels,
}

struct DemoLabels {
    quests: HashMap<RecordId, String>,
}

impl JournalLabels for DemoLabels {
    fn quest_name(&self, quest: &RecordId) -> Option<String> {
        self.quests.get(quest).cloned()
    }

    fn topic_name(&self, topic: &RecordId) -> Option<String> {
        let mut name = topic.as_str().to_owned();
        if let Some(first) = name.get_mut(0..1) {
            first.make_ascii_uppercase();
        }
        Some(name)
    }

    fn day_heading(&self, day: u32) -> String {
        format!("Day {}", day + 1)
    }
}

impl JournalSource for DemoWorld {
    fn world(&self) -> &WorldRpg {
        &self.world
    }

    fn labels(&self) -> &dyn JournalLabels {
        &self.labels
    }
}

const PROSE: &str = "The road was longer than the map promised, and the rain had not stopped \
since the ferry. I wrote down what the keeper said, word for word, because nothing else \
in this town is written down at all.";

/// Three quests (one finished), entries over several days, `topics` known topics with heard
/// lines.
pub fn world(entries: usize, topics: usize) -> DemoWorld {
    let mut world = WorldRpg::default();
    let quests = [
        ("lost_ring", "The Lost Ring"),
        ("ferry", "Passage on the Ferry"),
        ("ledger", "The Keeper's Ledger"),
    ];
    for i in 0..entries {
        let (quest, _) = quests[i % quests.len()];
        let finished = quest == "ferry" && i + quests.len() >= entries;
        let stage = i32::try_from(i).unwrap_or(0) * 10 + 10;
        world
            .apply(RpgCommand::SetStage {
                quest: id(quest),
                stage,
                flags: StageFlags {
                    finish: finished,
                    restart: false,
                },
                entry: Some(EntryDraft {
                    source: None,
                    text: format!("Entry {}. {PROSE}", i + 1),
                    time: GameTime {
                        day: u32::try_from(i / 3).unwrap_or(0),
                        minute: 0,
                    },
                }),
            })
            .expect("demo stage applies");
    }
    for t in 0..topics {
        let topic = id(&topic_name(t));
        world
            .apply(RpgCommand::AddTopic(topic.clone()))
            .expect("demo topic applies");
        if t % 2 == 0 {
            world
                .apply(RpgCommand::HearLine {
                    topic,
                    line: HeardLine {
                        source: id(&format!("heard_{t}")),
                        speaker: "Keeper".into(),
                        text: format!("What the keeper said about topic {t}. {PROSE}"),
                    },
                })
                .expect("demo line applies");
        }
    }
    DemoWorld {
        world,
        labels: DemoLabels {
            quests: quests
                .iter()
                .map(|(k, v)| (id(k), (*v).to_owned()))
                .collect(),
        },
    }
}

/// Invented topic names spread over the alphabet.
pub fn topic_name(i: usize) -> String {
    const STEMS: [&str; 12] = [
        "amber", "bridges", "copper", "dunes", "embers", "ferries", "glass", "harbor", "inks",
        "ledgers", "rain", "wells",
    ];
    if i < STEMS.len() {
        STEMS[i].to_owned()
    } else {
        format!("{} {}", STEMS[i % STEMS.len()], i / STEMS.len())
    }
}

// ---------------------------------------------------------------- dialogue

/// Dialogue rules of the demo: every conversation topic has one line naming other topics
/// (links); "ferries" offers two choices; "wells" forces goodbye.
pub struct DemoRules;

impl DialogueRules for DemoRules {
    fn accepts(&self, ctx: &ConversationCtx, _topic: &RecordId, response: &RecordId) -> bool {
        match response.as_str() {
            "ferries_yes" => ctx.choice == Some(2),
            "ferries_no" => ctx.choice == Some(1),
            "ferries" => ctx.choice.is_none(),
            _ => true,
        }
    }

    fn render(&self, _ctx: &ConversationCtx, _topic: &RecordId, response: &RecordId) -> String {
        match response.as_str() {
            "greeting" => {
                "Rain again. Ask me about the harbor, or the ferries, or rain itself.".into()
            }
            "ferries" => "The ferry leaves at dawn. Will you carry a letter across?".into(),
            "ferries_yes" => "Good. The keeper of ledgers will pay you on the far bank.".into(),
            "ferries_no" => "Then I will ask someone at the harbor.".into(),
            "wells" => "I have nothing more to say. Leave me be.".into(),
            other => format!(
                "About {other}: the amber from the dunes, glass from the harbor, copper for the bridges."
            ),
        }
    }
}

/// A conversation: its database and session (host side).
#[derive(Resource)]
pub struct DemoTalk {
    pub db: DialogueDb,
    pub session: DialogueSession,
}

/// Opens a conversation over `world` and its view (fed with the greeting).
pub fn open_dialogue(world: &mut WorldRpg) -> (DemoTalk, DialogueView) {
    let mut db = DialogueDb::new();
    db.insert_topic(
        id("greeting"),
        TopicEntry {
            kind: TopicKind::Greeting,
            name: "Greeting".into(),
            responses: vec![id("greeting")],
        },
    );
    for t in 0..12 {
        let name = topic_name(t);
        let responses = if name == "ferries" {
            vec![id("ferries_yes"), id("ferries_no"), id("ferries")]
        } else {
            vec![id(&name)]
        };
        db.insert_topic(
            id(&name),
            TopicEntry {
                kind: TopicKind::Conversation,
                name: name.clone(),
                responses,
            },
        );
    }
    db.set_greetings(vec![id("greeting")]);
    db.build_linker(LinkRules {
        word_start: true,
        word_end: true,
    })
    .expect("demo topics link");
    let ctx = ConversationCtx {
        player: PlayerId::HOST,
        speaker: ActorKey(1),
        speaker_base: id("keeper"),
        choice: None,
    };
    let (session, out) = DialogueSession::open(&db, &DemoRules, ctx);
    let mut talk = DemoTalk { db, session };
    let mut view = DialogueView::new("The Keeper");
    talk.run(world, &mut view, out);
    (talk, view)
}

impl DemoTalk {
    /// Applies a request from the window, as a host would, and feeds the view.
    pub fn apply(
        &mut self,
        world: &mut WorldRpg,
        view: &mut DialogueView,
        request: &DialogueRequest,
    ) {
        if let Ok(out) = request.apply(&mut self.session, &self.db, &DemoRules) {
            self.run(world, view, out);
        }
    }

    fn run(&mut self, world: &mut WorldRpg, view: &mut DialogueView, out: Vec<DialogueOutput>) {
        view.apply_outputs(&self.db, &out);
        for o in out {
            match o {
                DialogueOutput::Line {
                    topic,
                    response,
                    text,
                } => {
                    let line = HeardLine {
                        source: response,
                        speaker: "The Keeper".into(),
                        text,
                    };
                    let _ = world.apply(RpgCommand::HearLine { topic, line });
                }
                DialogueOutput::Learned(topic) => {
                    let _ = world.apply(RpgCommand::AddTopic(topic));
                }
                DialogueOutput::RunResult { response, .. } => match response.as_str() {
                    "ferries" => {
                        let _ = self.session.offer_choices(vec![
                            Choice {
                                label: "No".into(),
                                value: 1,
                            },
                            Choice {
                                label: "Yes, I will carry it".into(),
                                value: 2,
                            },
                        ]);
                    }
                    "ferries_yes" => {
                        let events = world
                            .apply(RpgCommand::SetStage {
                                quest: id("ferry"),
                                stage: 1000,
                                flags: StageFlags::default(),
                                entry: Some(EntryDraft {
                                    source: None,
                                    text: "I agreed to carry a letter across the water.".into(),
                                    time: GameTime { day: 9, minute: 0 },
                                }),
                            })
                            .unwrap_or_default();
                        view.apply_events(&events);
                    }
                    "wells" => self.session.force_goodbye(),
                    _ => {}
                },
                DialogueOutput::NoResponse { .. } => {}
            }
        }
        view.refresh_from(&self.session, &self.db, &DemoRules, &world.topics);
    }
}

// ---------------------------------------------------------------- inventory

/// Item names, categories, weights, values and stats from the item id.
pub struct DemoCatalog {
    stats: Vec<StatDef>,
}

impl Default for DemoCatalog {
    fn default() -> Self {
        let stat = |s: &str, better| StatDef {
            id: id(s),
            label: s.into(),
            better,
        };
        Self {
            stats: vec![
                stat("damage", Better::Higher),
                stat("armor", Better::Higher),
            ],
        }
    }
}

impl ItemCatalog for DemoCatalog {
    fn describe(&self, stack: &ItemStack) -> ItemInfo {
        let item = stack.item.as_str();
        let (category, weight, value, stats): (&str, f32, i64, Vec<(&str, f64)>) =
            if item.starts_with("blade") {
                (
                    "weapon",
                    8.0,
                    40,
                    vec![("damage", 6.0 + f64::from(n_of(item) % 7))],
                )
            } else if item.starts_with("helm") {
                (
                    "armor",
                    4.0,
                    25,
                    vec![("armor", 3.0 + f64::from(n_of(item) % 5))],
                )
            } else if item.starts_with("bread") {
                ("food", 0.5, 2, Vec::new())
            } else {
                ("misc", 1.0, 1 + i64::from(n_of(item) % 50), Vec::new())
            };
        let condition = match stack.attrs.get(&id("condition")) {
            Some(AttrValue::Int(c)) => format!(" ({c}%)"),
            _ => String::new(),
        };
        ItemInfo {
            name: format!("{}{condition}", item.replace('_', " ")),
            category: id(category),
            weight,
            value,
            icon: None,
            stats: stats.into_iter().map(|(k, v)| (id(k), v)).collect(),
        }
    }

    fn stats(&self) -> &[StatDef] {
        &self.stats
    }
}

fn n_of(item: &str) -> u32 {
    item.rsplit('_')
        .next()
        .and_then(|n| n.parse().ok())
        .unwrap_or(0)
}

/// Blades go in the hand, helms on the head.
pub struct DemoEquip {
    pub table: SlotTable,
}

impl Default for DemoEquip {
    fn default() -> Self {
        Self {
            table: SlotTable::new(["hand", "head"]).expect("demo slots"),
        }
    }
}

impl EquipRules for DemoEquip {
    fn placement(&self, stack: &ItemStack) -> Option<EquipPlacement> {
        let slot = if stack.item.as_str().starts_with("blade") {
            "hand"
        } else if stack.item.as_str().starts_with("helm") {
            "head"
        } else {
            return None;
        };
        Some(EquipPlacement {
            slots: std::iter::once(self.table.id(slot)?).collect(),
            whole_stack: false,
        })
    }
}

/// The demo player's inventory: `stacks` generated stacks, the first blade and helm worn.
#[derive(Resource)]
pub struct DemoInventory {
    pub loadout: Loadout,
    pub catalog: DemoCatalog,
    pub equip: DemoEquip,
}

pub const HOLDER: Holder = Holder::Player(PlayerId::HOST);

impl DemoInventory {
    pub fn new(stacks: usize) -> Self {
        let mut loadout = Loadout::new();
        for i in 0..stacks {
            let (item, count) = match i % 4 {
                0 => (format!("blade_{i}"), 1),
                1 => (format!("helm_{i}"), 1),
                2 => ("bread".to_owned(), 5),
                _ => (
                    format!("trinket_{i}"),
                    1 + u32::try_from(i % 3).unwrap_or(0),
                ),
            };
            let attrs = if item == "bread" {
                // Distinct stacks of bread (instance data keeps them apart).
                Attrs::new().with("condition", AttrValue::Int(i64::try_from(i).unwrap_or(0)))
            } else {
                Attrs::new()
            };
            loadout
                .add(id(&item), count, attrs)
                .expect("demo item adds");
        }
        let equip = DemoEquip::default();
        let first = |prefix: &str| {
            loadout
                .inventory()
                .stacks()
                .iter()
                .find(|s| s.item.as_str().starts_with(prefix))
                .map(|s| s.id)
        };
        for stack in [first("blade"), first("helm")].into_iter().flatten() {
            let _ = loadout.equip(stack, &equip);
        }
        Self {
            loadout,
            catalog: DemoCatalog::default(),
            equip,
        }
    }

    /// A synced view with the demo tabs.
    pub fn view(&self) -> InventoryView {
        // Labels are Fluent keys; the demo has no strings, so plain words stand in.
        let mut all = InventoryTab::all();
        "All".clone_into(&mut all.label);
        let mut worn = InventoryTab::equipped();
        "Worn".clone_into(&mut worn.label);
        let tabs = vec![
            all,
            InventoryTab::categories("gear", "Gear", ["weapon", "armor"]),
            InventoryTab::categories("food", "Food", ["food"]),
            worn,
        ];
        let mut view = InventoryView::new(HOLDER, tabs, 1.0, 1.0);
        view.set_capacity(Some(200.0));
        view.sync(&self.loadout, &self.equip.table, &self.catalog);
        view
    }

    /// Applies an item command as the host would and re-syncs `view`.
    pub fn apply(&mut self, view: &mut InventoryView, command: RpgCommand) {
        let mut pc = sf_rpg::CharacterRpg {
            loadout: std::mem::take(&mut self.loadout),
        };
        let events = pc.apply(command).unwrap_or_default();
        self.loadout = pc.loadout;
        let mut resync = false;
        for e in &events {
            resync |= view.on_event(e);
        }
        if resync {
            view.sync(&self.loadout, &self.equip.table, &self.catalog);
        }
    }
}

// ---------------------------------------------------------------- HUD and keymap

/// The game's activate action in the demo keymap (context `gameplay`).
pub fn activate_action() -> ActionRef {
    ActionRef::new("gameplay", "activate")
}

/// Registers the demo's gameplay actions (activate, jump) in a keymap.
pub fn register_gameplay(keymap: &mut Keymap) {
    let _ = keymap.register(
        ActionDef::button("gameplay", "activate", ["key:KeyE", "pad:West"]).label("activate"),
    );
    let _ = keymap
        .register(ActionDef::button("gameplay", "jump", ["key:Space", "pad:South"]).label("jump"));
    keymap.link_scopes("gameplay", sf_ui_bevy::HUD_SCOPE);
}

/// A HUD with three bars, a heading and a ready prompt.
pub fn hud() -> HudView {
    let bar = |id: &str, current, max| HudBar {
        id: id.into(),
        label: String::new(),
        current,
        max,
    };
    HudView {
        bars: vec![
            bar("health", 80.0, 100.0),
            bar("magicka", 35.0, 120.0),
            bar("fatigue", 200.0, 220.0),
        ],
        heading: Some(48.0),
        crosshair: true,
        prompt: Some(UsePrompt {
            verb: "Open".into(),
            target: Some("Door".into()),
            ready: true,
            action: activate_action(),
        }),
    }
}

// ---------------------------------------------------------------- host systems

/// Host side of the demo dialogue: applies the window's requests to [`DemoTalk`] over
/// [`DemoWorld`], and closes the screen when the conversation ends.
pub fn host_dialogue(
    mut requests: MessageReader<crate::DialogueRequestEvent>,
    talk: Option<ResMut<DemoTalk>>,
    world: Option<ResMut<DemoWorld>>,
    mut screens: Query<(&sf_ui_bevy::UiScreenRoot, &mut crate::DialogueScreen)>,
    mut stack: ResMut<sf_ui_bevy::ScreenStackRes>,
) {
    let (Some(mut talk), Some(mut world)) = (talk, world) else {
        requests.clear();
        return;
    };
    for event in requests.read() {
        if let Ok((root, mut screen)) = screens.get_mut(event.screen) {
            talk.apply(&mut world.world, &mut screen.view, &event.request);
            if screen.view.is_closed() {
                stack.0.close(root.0.as_str());
            }
        }
    }
}

/// Host side of the demo inventory: splits, merges, the split picker and comparisons.
pub fn host_inventory(
    mut events: MessageReader<crate::InventoryEvent>,
    inventory: Option<ResMut<DemoInventory>>,
    mut screens: Query<&mut crate::InventoryScreen>,
) {
    let Some(mut inventory) = inventory else {
        events.clear();
        return;
    };
    for event in events.read() {
        let Ok(mut screen) = screens.get_mut(event.screen) else {
            continue;
        };
        let screen = &mut *screen;
        let inv = &mut *inventory;
        match &event.intent {
            crate::inventory::InventoryIntent::Focused(Some(stack)) => {
                screen.compare =
                    screen
                        .view
                        .compare(*stack, &inv.loadout, &inv.equip, &inv.catalog);
            }
            crate::inventory::InventoryIntent::SplitRequested(stack) => {
                if let Some(picker) = screen.view.split_picker(&inv.loadout, *stack) {
                    screen.open_split(*stack, picker);
                }
            }
            crate::inventory::InventoryIntent::Split { stack, count } => {
                if let Ok(cmd) = screen.view.split_command(&inv.loadout, *stack, *count) {
                    inv.apply(&mut screen.view, cmd);
                }
            }
            crate::inventory::InventoryIntent::Merge { from, into } => {
                if let Ok(cmd) = screen.view.merge_command(&inv.loadout, *from, *into) {
                    inv.apply(&mut screen.view, cmd);
                }
            }
            crate::inventory::InventoryIntent::Focused(None)
            | crate::inventory::InventoryIntent::Use(_) => {}
        }
    }
}
