//! Names of the far-field bake products, shared by the bake (feature
//! `bake`) and the runtime that loads them through `baked://`.

use sf_world::WorldspaceId;

use crate::node::NodeKey;

pub const POLICY_STEP: &str = "lod.policy";
pub const IMPOSTOR_STEP: &str = "lod.impostor";
pub const L1_STEP: &str = "lod.l1";
pub const L2_STEP: &str = "lod.l2";
pub const L3_STEP: &str = "lod.l3";
pub const DIR_STEP: &str = "lod.dir";

/// File extensions the runtime loaders register, per step (pass them to
/// `sf_model_bevy::baked_source`).
pub const EXTENSIONS: [(&str, &str); 5] =
    [(L1_STEP, "sfnode"), (L2_STEP, "sfnode"), (L3_STEP, "sfnode"), (DIR_STEP, "sffar"), (IMPOSTOR_STEP, "sfimp")];

/// The step making nodes of `level`.
pub fn level_step(level: u8) -> &'static str {
    match level {
        1 => L1_STEP,
        2 => L2_STEP,
        _ => L3_STEP,
    }
}

/// Subject of a node product: `<ws>/<x>,<y>,<z>`.
pub fn node_subject(ws: &WorldspaceId, key: NodeKey) -> String {
    format!("{}/{}", ws.as_str(), key.subject_tail())
}

/// Subject of the far directory: the worldspace id.
pub fn directory_subject(ws: &WorldspaceId) -> String {
    ws.as_str().to_owned()
}

#[cfg(feature = "bake")]
pub(crate) fn parse_node(level: u8, subject: &str) -> Option<(WorldspaceId, NodeKey)> {
    let (ws, tail) = subject.rsplit_once('/')?;
    Some((WorldspaceId::new(ws), NodeKey::parse_tail(level, tail)?))
}
