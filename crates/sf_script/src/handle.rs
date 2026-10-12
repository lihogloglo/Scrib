//! Entity handles (scripting.md "Entity handles and entity scripts").
//!
//! A script never holds an ECS entity. It holds a [`Handle`]: the 128-bit stable id of a
//! placement (`sf_world::PlacementId`'s bits, or a runtime-spawned id from the placement overlay).
//! This crate stays independent of `sf_world`; `sf_script_bevy` converts and resolves handles
//! (`is_loaded`, position, ...) through API modules. Handles are plain data: they serialize as
//! their id and compare by id.

use mlua::{MetaMethod, UserData, UserDataMethods};
use serde::{Deserialize, Serialize};

/// A stable placement id as seen by scripts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Handle(pub u128);

impl UserData for Handle {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("id", |_, h, ()| Ok(format!("{:032x}", h.0)));
        methods.add_meta_method(MetaMethod::Eq, |_, h, other: mlua::AnyUserData| {
            Ok(other.borrow::<Handle>().is_ok_and(|o| *o == *h))
        });
        methods.add_meta_method(MetaMethod::ToString, |_, h, ()| {
            Ok(format!("Handle({:032x})", h.0))
        });
    }
}
