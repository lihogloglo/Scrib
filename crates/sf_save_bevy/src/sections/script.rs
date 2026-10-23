//! `<mod id>/script`: each script context's `ContextSnapshot` (storage, entity-script states,
//! named timers, RNG, clocks), over `sf_script_bevy::Scripts` (scripting.md "State, timers and
//! saves"). The runtime's snapshot bytes are stored as they are. Restoring queues the mod's
//! `core.on_load { reason = "restore", old_version }`, where the mod migrates its own storage.

use bevy_ecs::prelude::*;
use sf_save::{Family, SectionKey};
use sf_script_bevy::Scripts;

use super::{SCRIPT, SCRIPT_VERSION, order};
use crate::DormantSections;
use crate::section::SaveSection;

/// Every running context's snapshot. A saved context the running app doesn't have (its mod is
/// missing) is kept in [`DormantSections`] and written back unchanged by the next save.
pub fn scripts() -> SaveSection {
    SaveSection::raw_family(
        Family { owner: None, family: SCRIPT.to_owned() },
        SCRIPT_VERSION,
        |w: &World| {
            let Some(s) = w.get_resource::<Scripts>() else { return Ok(Vec::new()) };
            let mut out = Vec::new();
            for ctx in s.runtime.contexts() {
                let key = SectionKey::new(ctx.id(), SCRIPT).map_err(|e| e.to_string())?;
                out.push((key, s.runtime.snapshot(ctx.id()).map_err(|e| format!("{}: {e}", ctx.id()))?));
            }
            Ok(out)
        },
        |w: &mut World, saved: Vec<(SectionKey, Vec<u8>)>| {
            for (key, bytes) in saved {
                let id = key.owner().to_owned();
                let restored = match w.get_resource_mut::<Scripts>() {
                    Some(mut s) if s.runtime.context(&id).is_some() => {
                        s.runtime.restore(&id, &bytes).map_err(|e| format!("{key}: {e}"))?;
                        true
                    }
                    _ => false,
                };
                if !restored {
                    log::warn!("sf_save_bevy: no script context {id:?}; its saved state is kept dormant");
                    w.get_resource_or_insert_with(DormantSections::default).keep_raw(key, SCRIPT_VERSION, bytes);
                }
            }
            Ok(())
        },
    )
    .order(order::SCRIPTS)
}
