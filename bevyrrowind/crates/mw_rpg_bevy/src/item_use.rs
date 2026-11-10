//! Using an item from the player's inventory (effects-adapter.md "Potion use", slice A2):
//! drinking a potion. Equip, reading books and eating ingredients come later.
//!
//! - **Request, then host** (net.md "Net-ready rules"): the inventory screen's `Use` intent on
//!   the player's own inventory becomes a [`UseItemRequest`] (player id, stack id: no entity, no
//!   record lookup on the client side) in [`request_item_use`]; [`apply_item_use`] is the host
//!   side. It checks the stack is in the player's loadout and is an ALCH record, takes one piece
//!   off it (`Loadout::take`: the last piece removes the stack), and sends the potion's effects
//!   to the player's [`Effects`] as an [`EffectCommand`] (`mw_effects::map_potion` then
//!   `MwSource::inflict` with the player as caster and target, class `Consumable`); then
//!   [`ItemUsed`] goes out. The screen re-syncs from the RPG state (`inventory_ui::sync_open`),
//!   it never changes it. The effects land on the next fixed tick.
//! - **Stacking**: a second drink of the same potion is a new instance beside the first, both
//!   running out on their own (`Consumable` is `StackPolicy::Stack`).
//! - **Not a potion**: nothing happens (equip, books: later); a co-op client's request for another
//!   player's inventory is refused (only the host player has a loadout here).
//!
//! Ported from `OpenMW` `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f` (behaviour, no code):
//! `apps/openmw/mwgui/inventorywindow.cpp` `InventoryWindow::useItem` 610-702 (drag onto the
//! avatar, a quick key or the use button: the class's `use` action runs on the player; potions
//! and ingredients are "consumables"), `apps/openmw/mwclass/potion.cpp` `Potion::use` 122-131
//! (an `ActionApply` with sound `Drink`), `apps/openmw/mwworld/actionapply.cpp` 13-16 and
//! `apps/openmw/mwclass/actor.cpp` `Actor::consume` 79-92 (one piece removed from the
//! container store, then `CastSpell(actor, actor).cast(id)`; a success breaks invisibility),
//! `apps/openmw/mwmechanics/spellcasting.cpp` `CastSpell::cast(const ESM::Potion*)` 315-328
//! (flags `Temporary | Stackable`, every range inflicted on the drinker, no area) and
//! `apps/openmw/mwmechanics/activespells.cpp` `ActiveSpells::addToSpells` 484-501 (a stackable
//! spell skips the same-source search: always a new instance). There is no "you cannot drink
//! while ..." rule in that path (the inventory itself is closed to werewolves).
//!
//! Not yet (leftovers): the `Drink` sound ([`ItemUsed::sound`] names it; audio routing is
//! TODO(audio) as for dialogue sounds), breaking Invisibility (with that effect, adapter A7),
//! a potion's script `OnPCEquip` / `PCSkipEquip` locals (`useItem` 612-653), quick keys.

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_log::prelude::*;
use mw_effects::map_potion;
use mw_game_core::player::HOST_KEY;
use sf_effects_bevy::EffectCommand;
use sf_effects_bevy::sf_effects::RollSeed;
use sf_records::RecordId;
use sf_rpg::{Holder, PlayerId, StackId};
use sf_stats_bevy::StatTick;
use sf_stats_bevy::sf_stats::StatOwner;
use sf_ui_screens::{InventoryEvent, InventoryIntent, InventoryScreen};
use tes3::esp::Alchemy;

use crate::dialogue::Rpg;
use crate::effects::MwEffectsRes;
use crate::inventory::ItemRecords;

/// The sound a drink plays (`Potion::use`: a SOUN record id).
pub const DRINK_SOUND: &str = "Drink";

/// A player asks to use one piece of a stack of their own inventory.
#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct UseItemRequest {
    pub player: PlayerId,
    pub stack: StackId,
}

/// The host used (consumed) an item for a player: for audio, notifications, quest hooks.
#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct ItemUsed {
    pub player: PlayerId,
    pub item: RecordId,
    /// The sound record to play (`Drink`).
    pub sound: Option<RecordId>,
}

/// The request and the host side, in `Update` before the open screens re-sync. Needs the item
/// records and the RPG host (`inventory_ui::add` calls it).
pub fn add(app: &mut App) {
    app.add_message::<InventoryEvent>()
        .add_message::<UseItemRequest>()
        .add_message::<ItemUsed>()
        // Registered by the effects plugin; here too for an app without stats (nothing drinks).
        .add_message::<EffectCommand>()
        .add_systems(
            Update,
            (request_item_use, apply_item_use)
                .chain()
                .before(crate::inventory_ui::sync_open),
        );
}

/// The inventory screen's `Use` on the player's own inventory → [`UseItemRequest`].
#[allow(clippy::needless_pass_by_value)] // system parameters
pub fn request_item_use(
    mut events: MessageReader<InventoryEvent>,
    screens: Query<&InventoryScreen>,
    mut out: MessageWriter<UseItemRequest>,
) {
    for event in events.read() {
        let InventoryIntent::Use(stack) = event.intent else {
            continue;
        };
        let Ok(screen) = screens.get(event.screen) else {
            continue;
        };
        if let Holder::Player(player) = screen.view.holder() {
            out.write(UseItemRequest { player, stack });
        }
    }
}

/// Host side of [`UseItemRequest`] (see the module docs).
#[allow(clippy::needless_pass_by_value)] // system parameters
pub fn apply_item_use(
    mut requests: MessageReader<UseItemRequest>,
    records: Res<ItemRecords>,
    effects: Option<Res<MwEffectsRes>>,
    tick: Option<Res<StatTick>>,
    mut rpg: ResMut<Rpg>,
    mut commands: MessageWriter<EffectCommand>,
    mut used: MessageWriter<ItemUsed>,
) {
    let tick = tick.map_or(0, |t| t.0);
    for (n, request) in requests.read().enumerate() {
        if request.player != PlayerId::HOST {
            warn!(
                "use: player {:?} has no inventory on this host",
                request.player
            );
            continue;
        }
        let Some(item) = rpg
            .state
            .pc
            .loadout
            .inventory()
            .get(request.stack)
            .map(|s| s.item.clone())
        else {
            warn!(
                "use: no stack {:?} in the player's inventory",
                request.stack
            );
            continue;
        };
        let Some(potion) = records.0.get::<Alchemy>(&item) else {
            debug!("use {item}: not a potion (equip and the rest: later)");
            continue;
        };
        let Some(fx) = &effects else {
            warn!("use {item}: no effect table, not drunk");
            continue;
        };
        let (source, notes) = map_potion(&fx.0, potion);
        if !notes.is_empty() {
            debug!("use {item}: mapped with {notes:?}");
        }
        if let Err(e) = rpg.state.pc.loadout.take(request.stack, 1) {
            warn!("use {item}: {e:?}");
            continue;
        }
        let owner = StatOwner::Player(HOST_KEY);
        // One seed per landing: the fixed tick and the request's place in this frame.
        let rolls = RollSeed((u64::from(tick) << 32) | n as u64);
        commands.write(EffectCommand::new(
            owner,
            vec![source.inflict(Some(owner), rolls)],
        ));
        info!("use {item}: drunk");
        used.write(ItemUsed {
            player: request.player,
            item,
            sound: Some(RecordId::new(DRINK_SOUND)),
        });
    }
}

#[cfg(test)]
mod tests;
