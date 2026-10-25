//! Fixed strings of the stock screens, already localized by the app (Fluent ids in the
//! comments; English fallbacks here).

use bevy_ecs::prelude::*;

#[derive(Resource, Clone, Debug)]
pub struct ScreenLabels {
    /// `dialogue-goodbye`
    pub goodbye: String,
    /// `dialogue-journal-updated`
    pub journal_updated: String,
    /// `journal-tab-journal`, `-quests`, `-topics`
    pub journal: String,
    pub quests: String,
    pub topics: String,
    /// `journal-quests-active`, `-finished`, `-all`
    pub active: String,
    pub finished: String,
    pub all: String,
    /// `book-prev`, `book-next`
    pub prev: String,
    pub next: String,
    /// `keybinds-press`: shown while capturing.
    pub press_input: String,
    /// `keybinds-conflict`: "{input} is also bound to {action}." (`{input}`, `{action}` replaced)
    pub conflict: String,
    pub swap: String,
    pub unbind: String,
    pub keep_both: String,
    pub cancel: String,
    pub reset: String,
    pub reset_all: String,
    /// The hold / toggle cell of an action that offers the choice.
    pub hold: String,
    pub toggle: String,
    /// `keybinds-unbound`: an empty slot.
    pub unbound: String,
    /// `keybinds-kbm`, `keybinds-gamepad`: column headings.
    pub kbm: String,
    pub gamepad: String,
    /// `inventory-split`, `inventory-split-ok`
    pub split: String,
    pub ok: String,
    /// `inventory-weight`, `inventory-value`
    pub weight: String,
    pub value: String,
}

impl Default for ScreenLabels {
    fn default() -> Self {
        let s = |t: &str| t.to_owned();
        Self {
            goodbye: s("Goodbye"),
            journal_updated: s("Your journal has been updated."),
            journal: s("Journal"),
            quests: s("Quests"),
            topics: s("Topics"),
            active: s("Active"),
            finished: s("Finished"),
            all: s("All"),
            prev: s("<"),
            next: s(">"),
            press_input: s("Press an input..."),
            conflict: s("{input} is also bound to {action}."),
            swap: s("Swap"),
            unbind: s("Unbind the other"),
            keep_both: s("Keep both"),
            cancel: s("Cancel"),
            reset: s("Reset"),
            reset_all: s("Reset all"),
            hold: s("Hold"),
            toggle: s("Toggle"),
            unbound: s("-"),
            kbm: s("Keyboard / mouse"),
            gamepad: s("Gamepad"),
            split: s("Split stack"),
            ok: s("OK"),
            weight: s("Weight"),
            value: s("Value"),
        }
    }
}
