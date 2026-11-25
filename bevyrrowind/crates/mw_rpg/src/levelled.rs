//! Levelled item lists (`LEVI`): what an inventory entry naming one gives when the inventory is
//! filled (DQ-5f4; the container seed in `mw_rpg_bevy::inventory`).
//!
//! Ported from `OpenMW` (GPL-3.0) at commit `4e718b50`:
//! - `MWMechanics::getLevelledItem` (`apps/openmw/mwmechanics/levelledlist.hpp`, [`levelled_item`]):
//!   one `roll0to99` against the list's `chance_none` first (below it: nothing); the candidates
//!   are the entries at or below the player's level, all of them with "calculate from all levels
//!   <= player" (`CALCULATE_FROM_ALL_LEVELS`), else only those at the highest such level; one
//!   uniform pick among them (`rollDice`). A pick naming another list resolves that list in turn
//!   (its own chance and flags). No eligible entry gives nothing.
//! - `ContainerStore::addInitialItem` / `addInitialItemImp` (`apps/openmw/mwworld/containerstore.cpp`,
//!   [`initial_items`]): a count of 0 gives nothing, a negative (restocking) count counts by its
//!   magnitude; a list entry with |count| > 1 and "calculate for each item"
//!   (`CALCULATE_FOR_EACH_ITEM`) rolls once per piece (one piece each), otherwise one roll gives
//!   the whole count.
//! - **Determinism** (no global RNG, net-ready): every draw comes from a [`LevelledRoll`], a
//!   `splitmix64` stream seeded by the caller's words (a container: its placement key, the entry
//!   index and the player's level), so a read before the fill and the fill itself agree.
//! - Not ported: `OpenMW`'s warning and skip for a pick naming no record (the id is added as
//!   named, like a plain entry), the restocking refill itself (only the first fill).

use sf_records::{RecordDb, RecordId};
use tes3::esp::{LeveledItem, LeveledItemFlags};

use crate::persuasion::splitmix64;

/// Lists nested deeper than this give nothing (a guard against a list naming itself).
// tunables: const (capacity: nesting guard against cyclic lists)
const MAX_DEPTH: usize = 8;

/// A deterministic draw stream for levelled rolls (`splitmix64`: state advances by the golden
/// gamma, each draw is the mixed state).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LevelledRoll(u64);

impl LevelledRoll {
    /// The stream seeded by `words` (folded through `splitmix64` after a salt of its own).
    pub fn new(words: &[u64]) -> Self {
        /// "LEVELLED" in ASCII: keeps this stream apart from other `splitmix64` users.
        const SALT: u64 = 0x4C45_5645_4C4C_4544;
        Self(words.iter().fold(SALT, |h, w| splitmix64(h ^ w)))
    }

    fn next(&mut self) -> u64 {
        let out = splitmix64(self.0);
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        out
    }

    /// `Misc::Rng::roll0to99`.
    fn roll0to99(&mut self) -> u8 {
        u8::try_from(self.next() % 100).unwrap_or(0)
    }

    /// `Misc::Rng::rollDice(n)`: 0..n (`n` > 0).
    fn dice(&mut self, n: usize) -> usize {
        usize::try_from(self.next() % n as u64).unwrap_or(0)
    }
}

/// The item a roll of `LEVI` `list` gives at `player_level` (nested lists resolved), or `None`
/// (chance none, no eligible entry, or not a list's id: then the id itself).
pub fn levelled_item(db: &RecordDb, list: &RecordId, player_level: u16, roll: &mut LevelledRoll) -> Option<RecordId> {
    let mut id = list.clone();
    for _ in 0..MAX_DEPTH {
        let Some(list) = db.get::<LeveledItem>(&id) else { return Some(id) };
        if roll.roll0to99() < list.chance_none {
            return None;
        }
        let eligible = || list.items.iter().filter(|(_, level)| *level <= player_level);
        let highest = eligible().map(|(_, level)| *level).max()?;
        let all = list.leveled_item_flags.contains(LeveledItemFlags::CALCULATE_FROM_ALL_LEVELS);
        let candidates: Vec<&str> = eligible().filter(|(_, level)| all || *level == highest).map(|(item, _)| item.as_str()).collect();
        id = RecordId::new(candidates[roll.dice(candidates.len())]);
    }
    None
}

/// What an inventory entry (`count` of `item`, a record id or a `LEVI` list) gives a fresh fill
/// at `player_level`: `(item, count)` pairs, in roll order (module docs).
pub fn initial_items(db: &RecordDb, item: &RecordId, count: i32, player_level: u16, roll: &mut LevelledRoll) -> Vec<(RecordId, u32)> {
    let pieces = count.unsigned_abs();
    if pieces == 0 {
        return Vec::new();
    }
    let Some(list) = db.get::<LeveledItem>(item) else { return vec![(item.clone(), pieces)] };
    if pieces > 1 && list.leveled_item_flags.contains(LeveledItemFlags::CALCULATE_FOR_EACH_ITEM) {
        (0..pieces).filter_map(|_| levelled_item(db, item, player_level, roll)).map(|it| (it, 1)).collect()
    } else {
        levelled_item(db, item, player_level, roll).map(|it| (it, pieces)).into_iter().collect()
    }
}
