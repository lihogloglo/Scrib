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

mod id;
mod report;
mod store;

pub use id::RecordId;
pub use report::{ConflictReport, KindSummary, RecordConflict};
pub use store::{Entry, Op, Touch};

use std::any::{Any, TypeId};
use std::collections::HashMap;

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
}

/// The record database.
#[derive(Default)]
pub struct RecordDb {
    sources: Vec<SourceInfo>,
    stores: HashMap<TypeId, Box<dyn ErasedStore>>,
    kind_names: HashMap<String, TypeId>,
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
        match self.kind_names.get(kind) {
            Some(existing) if *existing == type_id => return Ok(()),
            Some(_) => return Err(RecordError::DuplicateKindName(kind.to_owned())),
            None => {}
        }
        self.kind_names.insert(kind.to_owned(), type_id);
        self.stores.insert(type_id, Box::new(Store::<T>::new(kind.to_owned())));
        Ok(())
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
        if store.patch(source, id, patch) {
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
        if !store.patch(source, &id, patch) {
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
}

#[cfg(test)]
mod tests;
