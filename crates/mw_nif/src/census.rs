//! What a NIF contains, and how much of it the conversion understands.
//!
//! Used by the asset sweep (`bevyrrowind nif-sweep`) to rank missing
//! features by how much content needs them. [`support`] is maintained by
//! hand next to the converter: when a feature lands, move its types to
//! [`Support::Converted`].

use std::collections::BTreeMap;

use tes3::nif::NiStream;

/// How the converter treats an object type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Support {
    /// Read and turned into model data.
    Converted,
    /// Read, but only partly honored (the note says what's missing).
    Partial(&'static str),
    /// Deliberately skipped; it has no effect worth reproducing (yet).
    Ignored(&'static str),
    /// Not handled. Content using it renders wrong or incomplete.
    Unsupported,
}

/// Object type name -> number of objects, for one stream.
pub fn object_types(stream: &NiStream) -> BTreeMap<&'static str, usize> {
    let mut counts = BTreeMap::new();
    for object in stream.objects.values() {
        let name = std::str::from_utf8(object.type_name()).unwrap_or("<invalid>");
        *counts.entry(name).or_default() += 1;
    }
    counts
}

/// How the converter treats objects of `type_name`.
pub fn support(type_name: &str) -> Support {
    use Support::{Converted, Ignored, Partial, Unsupported};
    match type_name {
        // Scene graph and geometry.
        "NiNode" | "NiBSAnimationNode" | "NiBSParticleNode" | "RootCollisionNode" | "NiSwitchNode"
        | "NiBillboardNode" | "NiTriShape" | "NiTriShapeData" | "NiTriStrips" | "NiTriStripsData"
        // Properties.
        | "NiTexturingProperty" | "NiSourceTexture" | "NiMaterialProperty" | "NiAlphaProperty"
        | "NiVertexColorProperty" | "NiStencilProperty" | "NiZBufferProperty"
        // Extra data.
        | "NiStringExtraData"
        // Controllers and their data.
        | "NiKeyframeController" | "NiKeyframeData" | "NiVisController" | "NiVisData" | "NiAlphaController"
        | "NiFloatData" | "NiMaterialColorController" | "NiPosData" | "NiUVController" | "NiUVData"
        // Particles.
        | "NiParticles" | "NiParticlesData" | "NiRotatingParticles" | "NiRotatingParticlesData"
        | "NiAutoNormalParticles" | "NiAutoNormalParticlesData" | "NiParticleSystemController" | "NiGravity"
        | "NiParticleGrowFade" | "NiParticleColorModifier" | "NiColorData" | "NiParticleRotation"
        | "NiBSPArrayController"
        // Skins and geometry morphs (`skin`).
        | "NiSkinInstance" | "NiSkinData" | "NiGeomMorpherController" | "NiMorphData" => Converted,

        "NiLODNode" => Partial("active level only, no distance switching"),
        "NiTextKeyExtraData" => Partial("only the Idle group, looped on objects"),
        "NiSkinPartition" => Ignored("GPU split of NiSkinData's weights; the cook builds its own"),

        "AvoidNode" => Ignored("AI pathing hint"),
        "NiSpecularProperty" => Ignored("PBR specular replaces it"),
        "NiWireframeProperty" | "NiDitherProperty" | "NiShadeProperty" => Ignored("fixed-function state"),
        "NiFogProperty" => Ignored("fog is a scene setting"),
        "TES3ObjectExtraData" | "NiExtraData" | "NiVertWeightsExtraData" | "NiCamera" => Ignored("no render effect"),
        "NiPointLight" | "NiSpotLight" | "NiAmbientLight" | "NiDirectionalLight" => {
            Ignored("lights come from LIGH records")
        }
        "NiTextureEffect" => Ignored("projected texture effects"),
        _ => Unsupported,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_types_are_unsupported() {
        assert_eq!(support("NiFancyNewThing"), Support::Unsupported);
        assert_eq!(support("NiNode"), Support::Converted);
    }
}
