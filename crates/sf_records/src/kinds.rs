//! The kind registry: record kinds by name, their Rust type, and optionally
//! a serde codec (native record files, generic patches) and a fingerprint
//! function (bake inputs, bake.md "Inputs and fingerprints").

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::marker::PhantomData;

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::patch::{PatchOp, PathError, apply_ops, path_exists};
use crate::store::Store;
use crate::value::{Value, ValueError};
use crate::{RecordId, SourceId};

/// A 32-byte content fingerprint (blake3 for the default).
pub type Fingerprint = [u8; 32];

pub(crate) type FingerprintFn = Box<dyn Fn(&dyn Any, &RecordId) -> Option<Fingerprint> + Send + Sync>;

pub(crate) struct KindEntry {
    pub(crate) type_id: TypeId,
    pub(crate) codec: Option<Box<dyn Codec>>,
    pub(crate) fingerprint: Option<FingerprintFn>,
}

/// Kinds by name. Several names may not share one type: a type's store has one kind name.
#[derive(Default)]
pub(crate) struct KindRegistry {
    pub(crate) kinds: HashMap<String, KindEntry>,
}


/// Why a generic patch failed.
pub(crate) enum PatchFailure {
    Missing,
    Path(PathError),
    Value(ValueError),
}

/// Type-erased serde access to one kind's store.
pub(crate) trait Codec: Send + Sync {
    fn define(&self, store: &mut dyn Any, source: SourceId, id: RecordId, value: &Value) -> Result<(), ValueError>;
    fn patch(&self, store: &mut dyn Any, source: SourceId, id: &RecordId, ops: &[PatchOp]) -> Result<(), PatchFailure>;
    fn to_value(&self, store: &dyn Any, id: &RecordId) -> Option<Result<Value, ValueError>>;
}

pub(crate) struct SerdeCodec<T>(PhantomData<fn() -> T>);

impl<T> SerdeCodec<T> {
    pub(crate) fn new() -> Self {
        Self(PhantomData)
    }
}

fn store<T: 'static>(store: &dyn Any) -> &Store<T> {
    store.downcast_ref::<Store<T>>().expect("codec matches its store type")
}

fn store_mut<T: 'static>(store: &mut dyn Any) -> &mut Store<T> {
    store.downcast_mut::<Store<T>>().expect("codec matches its store type")
}

impl<T: Serialize + DeserializeOwned + Send + Sync + 'static> Codec for SerdeCodec<T> {
    fn define(&self, store: &mut dyn Any, source: SourceId, id: RecordId, value: &Value) -> Result<(), ValueError> {
        let value: T = value.deserialize_into()?;
        store_mut::<T>(store).define(source, id, value);
        Ok(())
    }

    fn patch(&self, s: &mut dyn Any, source: SourceId, id: &RecordId, ops: &[PatchOp]) -> Result<(), PatchFailure> {
        let current = store::<T>(s).get(id).ok_or(PatchFailure::Missing)?;
        let mut tree = Value::from_serialize(current).map_err(PatchFailure::Value)?;
        let written = apply_ops(&mut tree, ops).map_err(PatchFailure::Path)?;
        let patched: T = tree.deserialize_into().map_err(PatchFailure::Value)?;
        // Serde ignores unknown struct fields unless the type denies them, so
        // check that everything written made it into the record.
        let result = Value::from_serialize(&patched).map_err(PatchFailure::Value)?;
        if let Some(path) = written.values.iter().find(|p| !path_exists(&result, p)) {
            let message = "does not exist in this record kind".to_owned();
            return Err(PatchFailure::Path(PathError { path: path.clone(), message }));
        }
        store_mut::<T>(s).patch(source, id, |v| *v = patched, Some(written.all()));
        Ok(())
    }

    fn to_value(&self, s: &dyn Any, id: &RecordId) -> Option<Result<Value, ValueError>> {
        store::<T>(s).get(id).map(Value::from_serialize)
    }
}

/// The default fingerprint of serde kinds: blake3 of the postcard bytes.
pub(crate) fn serde_fingerprint<T: Serialize + 'static>() -> FingerprintFn {
    Box::new(|s, id| {
        let value = store::<T>(s).get(id)?;
        let bytes = postcard::to_stdvec(value).ok()?;
        Some(*blake3::hash(&bytes).as_bytes())
    })
}

pub(crate) fn custom_fingerprint<T: 'static>(f: fn(&T) -> Fingerprint) -> FingerprintFn {
    Box::new(move |s, id| store::<T>(s).get(id).map(f))
}
