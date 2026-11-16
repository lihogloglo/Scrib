mod fog;
mod gi_probes; // superfargoth patch (own, GI)
mod surface_weather; // superfargoth patch (own, SW)
mod gpu_preprocess;
mod light;
pub(crate) mod mesh;
mod mesh_bindings;
mod mesh_view_bindings;
mod morph;
mod point_face_cull; // superfargoth patch (own: point shadow face culling)
pub(crate) mod skin;

pub use fog::*;
pub use gi_probes::*; // superfargoth patch (own, GI)
pub use surface_weather::*; // superfargoth patch (own, SW)
pub use gpu_preprocess::*;
pub use light::*;
pub use mesh::*;
pub use mesh_bindings::MeshLayouts;
pub use mesh_view_bindings::*;
pub use morph::*;
pub use point_face_cull::*; // superfargoth patch (own: point shadow face culling)
pub use skin::{
    extract_skins, prepare_skins, skins_use_uniform_buffers, SkinJointBlocks, SkinUniforms, MAX_JOINTS,
    // superfargoth patch (own, CW-7: skip unchanged skins).
    QuerySkinTree, SkinTree, SkinUploadPlan, SkinUploadSettings, SkinUploadStats,
};

pub const CORE_3D_SHADOW_MAP_FORMAT: wgpu_types::TextureFormat =
    wgpu_types::TextureFormat::Depth32Float;
