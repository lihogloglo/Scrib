//! Serde forms for foreign types that don't implement serde themselves.

/// `Option<VfsPath>` as an optional normalized string.
pub(crate) mod opt_path {
    use serde::{Deserialize, Deserializer, Serializer};
    use sf_vfs::VfsPath;

    // Serde's `with` hands the field by reference.
    #[allow(clippy::ref_option)]
    pub(crate) fn serialize<S: Serializer>(path: &Option<VfsPath>, s: S) -> Result<S::Ok, S::Error> {
        match path {
            Some(p) => s.serialize_some(p.as_str()),
            None => s.serialize_none(),
        }
    }

    pub(crate) fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<VfsPath>, D::Error> {
        Option::<String>::deserialize(d)?
            .map(|raw| VfsPath::new(&raw).map_err(serde::de::Error::custom))
            .transpose()
    }
}
