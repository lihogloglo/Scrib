//! Activation per record class (interaction-adapter.md "Activation per record class"): which verb
//! each Morrowind record class offers, what its default action is and what Classic keeps, as data
//! ([`ROWS`]). `IN32c` registers the offered rows as default verbs on the activation contract
//! (interaction.md "Activation contract").
//!
//! **Behaviour unchanged.** The offered rows give exactly today's prompts
//! (`mw_player_bevy::activatable`, `bevyrrowind/crates/mw_player_bevy/src/lib.rs:798`): NPC "Talk",
//! door and container "Open", activator "Use", book "Read", items "Take", a light "Take" only when
//! carryable, a creature nothing; plus the census furniture's pose verbs (sit, lie, lean). Rows
//! whose default action has no handler yet are kept as data marked [`Availability::NotYet`] with
//! the slice that builds it, and are never offered: no verb without a handler. `OpenMW` citations:
//! commit 4e718b50, `apps/openmw/mwclass/` (the spec's table).

use sf_interaction::TagId;

use crate::furniture::Kind;

/// Verb tags (`TagId::of(name)`), shared with the defs' slots.
pub mod verb {
    pub const TALK: &str = "talk";
    pub const OPEN: &str = "open";
    pub const CLOSE: &str = "close";
    pub const USE: &str = "use";
    pub const READ: &str = "read";
    pub const TAKE: &str = "take";
    pub const SEARCH: &str = "search";
    pub const PICKPOCKET: &str = "pickpocket";
    pub const SNUFF: &str = "snuff";
    pub const RELIGHT: &str = "relight";
    pub const SIT: &str = "sit";
    pub const LIE: &str = "lie";
    pub const LEAN: &str = "lean";
    pub const SLEEP: &str = "sleep";
    pub const BASH: &str = "bash";
    pub const BROWSE: &str = "browse";
    /// An NPC walking through a swing door's doorway slot (`crate::defs::door_def`).
    pub const PASS: &str = "pass";
}

/// A Morrowind record class as activation sees it; furniture is the census kind of a
/// `STAT`/`ACTI`/`CONT` (its own record class's rows apply too).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RecordClass {
    Npc,
    Creature,
    Container,
    Door,
    Book,
    Activator,
    Weapon,
    Armor,
    Clothing,
    Ingredient,
    MiscItem,
    Alchemy,
    Apparatus,
    Lockpick,
    Probe,
    RepairItem,
    Light,
    Furniture(Kind),
}

impl RecordClass {
    /// The loose-item classes (`ALCH INGR MISC WEAP ARMO CLOT APPA LOCK PROB REPA`).
    pub const ITEMS: [Self; 10] = [
        Self::Weapon,
        Self::Armor,
        Self::Clothing,
        Self::Ingredient,
        Self::MiscItem,
        Self::Alchemy,
        Self::Apparatus,
        Self::Lockpick,
        Self::Probe,
        Self::RepairItem,
    ];
}

/// What must hold of the target (or the user, for `Sneaking`) for a row to apply.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Condition {
    Always,
    Dead,
    KnockedOut,
    /// The user is sneaking.
    Sneaking,
    HasDialogue,
    /// A door with no destination (a swing door).
    NoDestination,
    /// A light with `CAN_CARRY`.
    Carryable,
    Locked,
    Lit,
    Unlit,
}

/// The default action a verb runs when no script handled it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DefaultAction {
    /// Dialogue (`mw_rpg_bevy/src/dialogue.rs`).
    Dialogue,
    /// Crime A4.
    Pickpocket,
    /// Loot a body (`ContainerOpened`).
    Loot,
    /// `TakeRequest` → `ContainerOpened` (`mw_rpg_bevy/src/inventory.rs`).
    OpenContainer,
    /// The fade travel of a door with a destination (`mw_doors_bevy`); a door without one does
    /// nothing, as today.
    TravelDoor,
    /// Swing the leaf (IN33).
    SwingDoor,
    /// `TakeRequest`, theft by owner (a book's until the reader exists).
    Take,
    /// Set `Lit` (IN19).
    ToggleLit,
    /// The record's script, if any (DQ-6); else nothing.
    Script,
    /// Claim and use the slot ("Using a slot", `sf_interaction_bevy::seat`).
    Pose,
    /// The rest hook (IN3).
    Rest,
    /// A blow on the lock, crime `LockRequest::Force` (IN37).
    Bash,
    /// Browse a shelf (later).
    Browse,
}

/// What the Classic preset does with a row.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Classic {
    /// As modern.
    Same,
    /// Not offered (its switch off): inert, as in vanilla.
    Off,
    /// The vanilla rest menu (beds' scripts).
    RestMenu,
}

/// The `mw.rules.interaction.*` switch a row depends on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Switch {
    /// `mw.rules.interaction.sit` ([`crate::tunables::SIT`]).
    Sit,
    /// `mw.rules.interaction.snuff` (lands with IN19).
    Snuff,
    /// `mw.rules.interaction.bash` (lands with IN37).
    Bash,
}

/// Whether a row's default action exists.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Availability {
    Offered,
    /// Not offered until this slice builds its default action.
    NotYet(&'static str),
}

/// One row of the table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Row {
    pub class: RecordClass,
    pub when: Condition,
    /// Verb tag name ([`verb`]).
    pub verb: &'static str,
    /// Prompt text.
    pub prompt: &'static str,
    pub action: DefaultAction,
    pub classic: Classic,
    /// The switch that turns it off (`None` = always on).
    pub switch: Option<Switch>,
    pub available: Availability,
}

const DEAD_BODIES: Availability = Availability::NotYet("dead-body search (unscheduled)");

const fn row(
    class: RecordClass,
    when: Condition,
    verb: &'static str,
    prompt: &'static str,
    action: DefaultAction,
) -> Row {
    Row { class, when, verb, prompt, action, classic: Classic::Same, switch: None, available: Availability::Offered }
}

impl Row {
    const fn not_yet(mut self, slice: &'static str) -> Self {
        self.available = Availability::NotYet(slice);
        self
    }

    const fn unavailable(mut self, a: Availability) -> Self {
        self.available = a;
        self
    }

    /// Classic `classic`, and off with `switch`.
    const fn modern(mut self, switch: Switch, classic: Classic) -> Self {
        self.switch = Some(switch);
        self.classic = classic;
        self
    }
}

const fn take(class: RecordClass) -> Row {
    row(class, Condition::Always, verb::TAKE, "Take", DefaultAction::Take)
}

const fn furniture(kind: Kind, verb: &'static str, prompt: &'static str) -> Row {
    row(RecordClass::Furniture(kind), Condition::Always, verb, prompt, DefaultAction::Pose)
        .modern(Switch::Sit, Classic::Off)
}

/// The table, rows of one class in focus order (the first that applies is the prompt's verb; the
/// rest are the hold list).
pub const ROWS: &[Row] = {
    use Condition as C;
    use DefaultAction as A;
    use RecordClass as R;
    &[
        // NPC_ (npc.cpp:798-845): search a body or a knocked-out NPC and pickpocket come first once
        // built; until then talk, as today, whatever its state.
        row(R::Npc, C::Dead, verb::SEARCH, "Search", A::Loot).unavailable(DEAD_BODIES),
        row(R::Npc, C::KnockedOut, verb::SEARCH, "Search", A::Loot).unavailable(DEAD_BODIES),
        row(R::Npc, C::Sneaking, verb::PICKPOCKET, "Pickpocket", A::Pickpocket).not_yet("crime A4"),
        row(R::Npc, C::Always, verb::TALK, "Talk", A::Dialogue),
        // CREA (creature.cpp:441-458): today a creature offers nothing.
        row(R::Creature, C::Dead, verb::SEARCH, "Search", A::Loot).unavailable(DEAD_BODIES),
        row(R::Creature, C::HasDialogue, verb::TALK, "Talk", A::Dialogue).not_yet("creature dialogue (unscheduled)"),
        // CONT (container.cpp:140-200).
        row(R::Container, C::Always, verb::OPEN, "Open", A::OpenContainer),
        row(R::Container, C::Locked, verb::BASH, "Bash", A::Bash).modern(Switch::Bash, Classic::Off).not_yet("IN37"),
        // DOOR (door.cpp:118-229): the swing rows hold for a door without a destination (IN33,
        // `defs::door_def`); the travel row is every door's unconditional row, so a caller that never
        // evaluates `NoDestination` prompts it ("Open").
        row(R::Door, C::NoDestination, verb::OPEN, "Open", A::SwingDoor),
        row(R::Door, C::NoDestination, verb::CLOSE, "Close", A::SwingDoor),
        row(R::Door, C::Always, verb::OPEN, "Open", A::TravelDoor),
        row(R::Door, C::Locked, verb::BASH, "Bash", A::Bash).modern(Switch::Bash, Classic::Off).not_yet("IN37"),
        // BOOK (book.cpp:55-62): read; the default takes it until the reader exists (UI lane).
        row(R::Book, C::Always, verb::READ, "Read", A::Take),
        // Items (misc.cpp:63-66).
        take(R::Weapon),
        take(R::Armor),
        take(R::Clothing),
        take(R::Ingredient),
        take(R::MiscItem),
        take(R::Alchemy),
        take(R::Apparatus),
        take(R::Lockpick),
        take(R::Probe),
        take(R::RepairItem),
        // LIGH (light.cpp:93-150): take when carryable; snuff and relight later (a fixed flame is
        // then a verb-only prompt).
        row(R::Light, C::Carryable, verb::TAKE, "Take", A::Take),
        row(R::Light, C::Lit, verb::SNUFF, "Snuff", A::ToggleLit).modern(Switch::Snuff, Classic::Off).not_yet("IN19"),
        row(R::Light, C::Unlit, verb::RELIGHT, "Relight", A::ToggleLit)
            .modern(Switch::Snuff, Classic::Off)
            .not_yet("IN19"),
        // ACTI (activator.cpp:116-123): use; an `OnActivate` script decides (scripting-adapter.md).
        row(R::Activator, C::Always, verb::USE, "Use", A::Script),
        // Census furniture: pose slots (defs.rs); Classic inert.
        furniture(Kind::Seat, verb::SIT, "Sit"),
        // A static bed's press is Sleep (the rest hook, IN3a; `defs::bed_verb`), lie and sit on hold.
        // Classic: no def (inert); vanilla rests by the rest key and scripted beds' `ShowRestMenu`.
        row(R::Furniture(Kind::Bed), C::Always, verb::SLEEP, "Sleep", A::Rest).modern(Switch::Sit, Classic::RestMenu),
        furniture(Kind::Bed, verb::LIE, "Lie down"),
        furniture(Kind::Bed, verb::SIT, "Sit"),
        furniture(Kind::Table, verb::LEAN, "Lean"),
        furniture(Kind::Counter, verb::LEAN, "Lean"),
        furniture(Kind::Shelf, verb::BROWSE, "Browse").not_yet("later (browse)"),
    ]
};

/// Every row of `class`, in focus order.
pub fn rows(class: RecordClass) -> impl Iterator<Item = &'static Row> {
    ROWS.iter().filter(move |r| r.class == class)
}

/// The rows `class` offers now, in focus order: built ([`Availability::Offered`]), its switch on
/// (`on`), and its condition holding (`holds`; [`Condition::Always`] always does).
pub fn offered(
    class: RecordClass,
    holds: impl Fn(Condition) -> bool,
    on: impl Fn(Switch) -> bool,
) -> impl Iterator<Item = &'static Row> {
    rows(class).filter(move |r| {
        r.available == Availability::Offered
            && r.switch.is_none_or(&on)
            && (r.when == Condition::Always || holds(r.when))
    })
}

/// The prompt's verb: the first offered row.
pub fn primary(
    class: RecordClass,
    holds: impl Fn(Condition) -> bool,
    on: impl Fn(Switch) -> bool,
) -> Option<&'static Row> {
    offered(class, holds, on).next()
}

/// Whether the default handler of `action` runs an approved use (`sf_interaction_bevy::UseApproved`)
/// of `verb`: `None` (the target's primary use: a target with no def) or the tag of an offered row
/// with that action (`open` for a container or a travel door, `open` and `close` for a swing door,
/// `talk`, `take`, `read`). A pose verb
/// (`sit`, `lie`, `lean`) on a def'd container or activator is the seat default's, so its class's
/// default never runs on it (`IN32c2`). The one check every default handler makes.
#[must_use]
pub fn runs_default(action: DefaultAction, verb: Option<TagId>) -> bool {
    verb.is_none_or(|v| {
        ROWS.iter().any(|r| r.action == action && r.available == Availability::Offered && TagId::of(r.verb) == v)
    })
}
