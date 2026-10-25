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
    /// `saveload-save-title`, `saveload-load-title`: the save/load screen's heading.
    pub save_title: String,
    pub load_title: String,
    /// `saveload-save`, `saveload-load`, `saveload-delete`, `saveload-back`, `saveload-yes`
    pub save: String,
    pub load: String,
    pub delete: String,
    pub back: String,
    pub yes: String,
    /// `saveload-new`: the new-save row (its generated name follows in brackets).
    pub new_save: String,
    /// `saveload-empty`: no saves to list.
    pub no_saves: String,
    /// `saveload-damaged`: a save that could not be read.
    pub damaged: String,
    /// `saveload-confirm-overwrite`, `saveload-confirm-delete` (`{name}` replaced by the title).
    pub confirm_overwrite: String,
    pub confirm_delete: String,
    /// `pause-title`: the pause menu's heading.
    pub pause_title: String,
    /// `pause-resume`, `pause-save`, `pause-load`, `pause-options`, `pause-quit`: its entries.
    pub resume: String,
    pub pause_save: String,
    pub pause_load: String,
    pub options: String,
    pub quit: String,
    /// `pause-confirm-quit`, `pause-no`: the question before quitting.
    pub confirm_quit: String,
    pub no: String,
    /// `options-keybinds`: the options screen's row that opens the keybinds screen (its title
    /// is `pause-options`, its back button `saveload-back`, its arrows `book-prev`/`-next`).
    pub keybinds: String,
    /// `options-on`, `options-off`: a toggle's value.
    pub on: String,
    pub off: String,
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
            save_title: s("Save game"),
            load_title: s("Load game"),
            save: s("Save"),
            load: s("Load"),
            delete: s("Delete"),
            back: s("Back"),
            yes: s("Yes"),
            new_save: s("New save"),
            no_saves: s("No saves yet."),
            damaged: s("Unreadable"),
            confirm_overwrite: s("Overwrite \"{name}\"?"),
            confirm_delete: s("Delete \"{name}\"? This cannot be undone."),
            pause_title: s("Paused"),
            resume: s("Resume"),
            pause_save: s("Save game"),
            pause_load: s("Load game"),
            options: s("Options"),
            quit: s("Quit"),
            confirm_quit: s("Quit the game? Unsaved progress will be lost."),
            no: s("No"),
            keybinds: s("Key bindings"),
            on: s("On"),
            off: s("Off"),
        }
    }
}
