//! Layered record database.
//!
//! Content (base game, DLC, mods) is applied as an ordered sequence of
//! *sources*. Each source can define, patch, or delete records. The database
//! keeps the resolved value of every record plus its full provenance, which
//! drives conflict reports, debugging, and later save-game overlays.
//!
//! The database is game-agnostic: record types are arbitrary Rust types
//! registered under a kind name. Adapters (e.g. the Morrowind content pack)
//! decide which types exist and how their source formats map onto
//! define/patch/delete operations.
//!
//! Kinds registered with serde ([`RecordDb::register_serde`]) can also be
//! written by name: native record files ([`apply_record_file`], RON or TOML)
//! define, patch and delete them, patches edit a generic [`Value`] tree with
//! field paths ([`PatchOp`]), and every patch records the fields it wrote, so
//! the conflict report tells real conflicts (two unrelated sources wrote one
//! field) from automatic merges (disjoint fields). See mods.md "Native record files".

mod file;
mod id;
mod kinds;
mod patch;
mod report;
mod store;
mod value;

pub use file::{FileError, FileOutcome, RecordFormat, apply_record_file};
pub use id::RecordId;
pub use kinds::Fingerprint;
pub use patch::{PatchOp, PathError, paths_overlap};
pub use report::{ConflictReport, KindSummary, RecordConflict, describe};
pub use store::{Entry, Op, Touch};
pub use value::{Raw, Value, ValueError};

use std::any::{Any, TypeId};
use std::collections::HashMap;

use serde::Serialize;
use serde::de::DeserializeOwned;

use kinds::{KindEntry, KindRegistry, PatchFailure, SerdeCodec};
use store::Store;

/// Index of a content source (one mod, plugin, or content file) in apply order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceId(pub u32);

/// Metadata about a registered content source.
#[derive(Clone, Debug)]
pub struct SourceInfo {
    pub name: String,
    /// Sources this one was authored against (its masters / dependencies).
    /// Only earlier sources can be dependencies.
    pub depends_on: Vec<SourceId>,
}

#[derive(Debug, thiserror::Error)]
pub enum RecordError {
    #[error("record kind {0} is not registered")]
    UnregisteredKind(&'static str),
    #[error("record kind name {0:?} is already registered for another type")]
    DuplicateKindName(String),
    #[error("{kind} record {id:?} does not exist and cannot be patched")]
    PatchMissing { kind: String, id: RecordId },
    #[error("unknown record kind {0:?}")]
    UnknownKind(String),
    #[error("record kind {0:?} can't be read from files (registered without serde)")]
    NotSerde(String),
    #[error("record kind {0:?} has no fingerprint function")]
    NoFingerprint(String),
    #[error("{kind} record {id:?}: {error}")]
    Value { kind: String, id: RecordId, error: ValueError },
    #[error("{kind} record {id:?}: {error}")]
    Path { kind: String, id: RecordId, error: PathError },
}

impl RecordError {
    /// Byte offset in the record file, for errors inside file text.
    pub fn offset(&self) -> Option<usize> {
        match self {
            Self::Value { error, .. } => error.at,
            _ => None,
        }
    }
}

/// The record database.
#[derive(Default)]
pub struct RecordDb {
    sources: Vec<SourceInfo>,
    stores: HashMap<TypeId, Box<dyn ErasedStore>>,
    kinds: KindRegistry,
}

impl RecordDb {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a content source. Sources must be added in apply order;
    /// later sources win. `depends_on` lists the sources it was authored
    /// against; it decides whether an override is intentional or a conflict.
    pub fn add_source(&mut self, name: impl Into<String>, depends_on: &[SourceId]) -> SourceId {
        let id = SourceId(u32::try_from(self.sources.len()).expect("too many sources"));
        assert!(depends_on.iter().all(|d| *d < id), "a source can only depend on earlier sources");
        self.sources.push(SourceInfo { name: name.into(), depends_on: depends_on.to_vec() });
        id
    }

    /// `knows[b][a]` is true when source `b` (transitively) depends on `a`.
    fn dependency_closure(&self) -> Vec<Vec<bool>> {
        let n = self.sources.len();
        let mut knows = vec![vec![false; n]; n];
        for (b, info) in self.sources.iter().enumerate() {
            knows[b][b] = true;
            for dep in &info.depends_on {
                let (earlier, row) = knows.split_at_mut(b);
                for (a, known) in earlier[dep.0 as usize].iter().enumerate() {
                    row[0][a] |= *known;
                }
            }
        }
        knows
    }

    pub fn source(&self, id: SourceId) -> &SourceInfo {
        &self.sources[id.0 as usize]
    }

    pub fn sources(&self) -> &[SourceInfo] {
        &self.sources
    }

    /// Registers a record type under a human-readable kind name.
    /// Registering the same type twice is a no-op.
    pub fn register<T: Send + Sync + 'static>(&mut self, kind: &str) -> Result<(), RecordError> {
        let type_id = TypeId::of::<T>();
        match self.kinds.kinds.get(kind) {
            Some(existing) if existing.type_id == type_id => return Ok(()),
            Some(_) => return Err(RecordError::DuplicateKindName(kind.to_owned())),
            None => {}
        }
        self.kinds.kinds.insert(kind.to_owned(), KindEntry { type_id, codec: None, fingerprint: None });
        self.stores.insert(type_id, Box::new(Store::<T>::new(kind.to_owned())));
        Ok(())
    }

    /// Registers a serde record type: like [`Self::register`], and the kind can
    /// be defined and patched by name ([`apply_record_file`],
    /// [`Self::define_value`], [`Self::patch_value`]) and fingerprinted
    /// (blake3 of its postcard bytes unless [`Self::set_fingerprint`] chose
    /// another function). Upgrades a kind registered without serde.
    pub fn register_serde<T>(&mut self, kind: &str) -> Result<(), RecordError>
    where
        T: Serialize + DeserializeOwned + Send + Sync + 'static,
    {
        self.register::<T>(kind)?;
        let entry = self.kinds.kinds.get_mut(kind).expect("registered above");
        entry.codec.get_or_insert_with(|| Box::new(SerdeCodec::<T>::new()));
        entry.fingerprint.get_or_insert_with(kinds::serde_fingerprint::<T>);
        Ok(())
    }

    /// Sets the fingerprint function of a registered kind (bake inputs).
    /// Adapters may hash their source bytes instead of the decoded value.
    pub fn set_fingerprint<T: Send + Sync + 'static>(&mut self, f: fn(&T) -> Fingerprint) -> Result<(), RecordError> {
        let type_id = TypeId::of::<T>();
        let entry = self.kinds.kinds.values_mut().find(|k| k.type_id == type_id);
        let entry = entry.ok_or(RecordError::UnregisteredKind(std::any::type_name::<T>()))?;
        entry.fingerprint = Some(kinds::custom_fingerprint(f));
        Ok(())
    }

    /// Registered kind names, sorted.
    pub fn kind_names(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.kinds.kinds.keys().map(String::as_str).collect();
        names.sort_unstable();
        names
    }

    /// Whether `kind` is registered with serde (writable by name).
    pub fn is_serde_kind(&self, kind: &str) -> bool {
        self.kinds.kinds.get(kind).is_some_and(|k| k.codec.is_some())
    }

    fn kind_entry(&self, kind: &str) -> Result<&KindEntry, RecordError> {
        self.kinds.kinds.get(kind).ok_or_else(|| RecordError::UnknownKind(kind.to_owned()))
    }

    /// Defines (or fully replaces) a record of a serde kind from a value tree.
    pub fn define_value(&mut self, source: SourceId, kind: &str, id: RecordId, value: &Value) -> Result<(), RecordError> {
        let entry = self.kinds.kinds.get(kind).ok_or_else(|| RecordError::UnknownKind(kind.to_owned()))?;
        let codec = entry.codec.as_ref().ok_or_else(|| RecordError::NotSerde(kind.to_owned()))?;
        let store = self.stores.get_mut(&entry.type_id).expect("registered kinds have stores").as_any_mut();
        codec
            .define(store, source, id.clone(), value)
            .map_err(|error| RecordError::Value { kind: kind.to_owned(), id, error })
    }

    /// Patches a record of a serde kind with field-path operations, all or
    /// nothing. The touch records the field paths written.
    pub fn patch_value(&mut self, source: SourceId, kind: &str, id: &RecordId, ops: &[PatchOp]) -> Result<(), RecordError> {
        let entry = self.kinds.kinds.get(kind).ok_or_else(|| RecordError::UnknownKind(kind.to_owned()))?;
        let codec = entry.codec.as_ref().ok_or_else(|| RecordError::NotSerde(kind.to_owned()))?;
        let store = self.stores.get_mut(&entry.type_id).expect("registered kinds have stores").as_any_mut();
        codec.patch(store, source, id, ops).map_err(|failure| {
            let (kind, id) = (kind.to_owned(), id.clone());
            match failure {
                PatchFailure::Missing => RecordError::PatchMissing { kind, id },
                PatchFailure::Path(error) => RecordError::Path { kind, id, error },
                PatchFailure::Value(error) => RecordError::Value { kind, id, error },
            }
        })
    }

    /// Deletes a record of any registered kind by name.
    pub fn delete_by_name(&mut self, source: SourceId, kind: &str, id: RecordId) -> Result<(), RecordError> {
        let type_id = self.kind_entry(kind)?.type_id;
        self.stores.get_mut(&type_id).expect("registered kinds have stores").delete(source, id);
        Ok(())
    }

    /// The record of a serde kind as a value tree (inspection, debugging);
    /// `Ok(None)` when absent or deleted.
    pub fn value_of(&self, kind: &str, id: &RecordId) -> Result<Option<Value>, RecordError> {
        let entry = self.kind_entry(kind)?;
        let codec = entry.codec.as_ref().ok_or_else(|| RecordError::NotSerde(kind.to_owned()))?;
        let store = self.stores[&entry.type_id].as_any();
        codec.to_value(store, id).transpose().map_err(|error| RecordError::Value { kind: kind.to_owned(), id: id.clone(), error })
    }

    /// The record's fingerprint (`Ok(None)` when absent or deleted). A kind
    /// without a fingerprint function can't be a bake input.
    pub fn fingerprint(&self, kind: &str, id: &RecordId) -> Result<Option<Fingerprint>, RecordError> {
        let entry = self.kind_entry(kind)?;
        let f = entry.fingerprint.as_ref().ok_or_else(|| RecordError::NoFingerprint(kind.to_owned()))?;
        Ok(f(self.stores[&entry.type_id].as_any(), id))
    }

    /// The history of a record of any kind, by name.
    pub fn history_of(&self, kind: &str, id: &RecordId) -> Result<Option<Vec<Touch>>, RecordError> {
        let type_id = self.kind_entry(kind)?.type_id;
        Ok(self.stores[&type_id].history(id))
    }

    /// Defines (or fully replaces) a record.
    pub fn define<T: Send + Sync + 'static>(&mut self, source: SourceId, id: RecordId, value: T) -> Result<(), RecordError> {
        self.store_mut::<T>()?.define(source, id, value);
        Ok(())
    }

    /// Modifies an existing record in place. Fails if the record does not
    /// exist or was deleted.
    pub fn patch<T: Send + Sync + 'static>(
        &mut self,
        source: SourceId,
        id: &RecordId,
        patch: impl FnOnce(&mut T),
    ) -> Result<(), RecordError> {
        let store = self.store_mut::<T>()?;
        if store.patch(source, id, patch, None) {
            Ok(())
        } else {
            Err(RecordError::PatchMissing { kind: store.kind.clone(), id: id.clone() })
        }
    }

    /// Defines the record if absent, otherwise patches it. Useful for formats
    /// whose records merge into earlier definitions (e.g. cells).
    pub fn upsert<T: Send + Sync + 'static>(
        &mut self,
        source: SourceId,
        id: RecordId,
        define: impl FnOnce() -> T,
        patch: impl FnOnce(&mut T),
    ) -> Result<(), RecordError> {
        let store = self.store_mut::<T>()?;
        if !store.patch(source, &id, patch, None) {
            store.define(source, id, define());
        }
        Ok(())
    }

    /// Deletes a record. Deleting an unknown record is recorded too, so a
    /// later conflict report can show it.
    pub fn delete<T: Send + Sync + 'static>(&mut self, source: SourceId, id: RecordId) -> Result<(), RecordError> {
        self.store_mut::<T>()?.delete(source, id);
        Ok(())
    }

    /// The resolved value of a record, or `None` if absent or deleted.
    pub fn get<T: Send + Sync + 'static>(&self, id: &RecordId) -> Option<&T> {
        self.store::<T>()?.get(id)
    }

    /// The full entry (resolved value and provenance).
    pub fn entry<T: Send + Sync + 'static>(&self, id: &RecordId) -> Option<&Entry<T>> {
        self.store::<T>()?.entries.get(id)
    }

    /// Iterates live (non-deleted) records of a kind, in first-definition order.
    pub fn iter<T: Send + Sync + 'static>(&self) -> impl Iterator<Item = (&RecordId, &T)> {
        self.store::<T>()
            .into_iter()
            .flat_map(|s| s.entries.iter())
            .filter_map(|(id, e)| e.value.as_ref().map(|v| (id, v)))
    }

    /// Number of live records of a kind.
    pub fn len<T: Send + Sync + 'static>(&self) -> usize {
        self.store::<T>().map_or(0, |s| s.live)
    }

    /// Whether a kind has no live records.
    pub fn is_empty<T: Send + Sync + 'static>(&self) -> bool {
        self.len::<T>() == 0
    }

    /// Summarizes overrides and conflicts across all kinds.
    pub fn conflict_report(&self) -> ConflictReport {
        let knows = self.dependency_closure();
        let mut kinds: Vec<KindSummary> = self.stores.values().map(|s| s.summarize(&knows)).collect();
        kinds.sort_by(|a, b| a.kind.cmp(&b.kind));
        ConflictReport { kinds }
    }

    fn store<T: Send + Sync + 'static>(&self) -> Option<&Store<T>> {
        self.stores.get(&TypeId::of::<T>()).map(|s| {
            s.as_any().downcast_ref::<Store<T>>().expect("store type matches its TypeId key")
        })
    }

    fn store_mut<T: Send + Sync + 'static>(&mut self) -> Result<&mut Store<T>, RecordError> {
        self.stores
            .get_mut(&TypeId::of::<T>())
            .map(|s| s.as_any_mut().downcast_mut::<Store<T>>().expect("store type matches its TypeId key"))
            .ok_or(RecordError::UnregisteredKind(std::any::type_name::<T>()))
    }
}

trait ErasedStore: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn summarize(&self, knows: &[Vec<bool>]) -> KindSummary;
    fn delete(&mut self, source: SourceId, id: RecordId);
    fn history(&self, id: &RecordId) -> Option<Vec<Touch>>;
}

impl<T: Send + Sync + 'static> ErasedStore for Store<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn summarize(&self, knows: &[Vec<bool>]) -> KindSummary {
        self.summarize(knows)
    }
    fn delete(&mut self, source: SourceId, id: RecordId) {
        Store::delete(self, source, id);
    }
    fn history(&self, id: &RecordId) -> Option<Vec<Touch>> {
        self.entries.get(id).map(|e| e.history.to_vec())
    }
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_native;
