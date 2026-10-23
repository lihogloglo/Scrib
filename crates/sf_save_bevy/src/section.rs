//! Section providers: how one section family is snapshot from and applied to the app
//! (save.md "Interfaces": `SaveProviders`).
//!
//! A provider pairs `sf_save`'s [`SectionSpec`] (versions and migration steps) with typed
//! accessors. Snapshot clones plain values on the main thread at a tick boundary; encoding runs
//! off-thread. On load, decoding runs off-thread too; only `apply` touches the world.

use std::any::Any;
use std::marker::PhantomData;
use std::sync::Arc;

use bevy_ecs::prelude::*;
use serde::Serialize;
use serde::de::DeserializeOwned;
use sf_save::{Family, SectionFlags, SectionKey, SectionSpec, decode, encode};

/// Encodes one snapshot value (runs off-thread).
pub(crate) type Encoder = Box<dyn FnOnce() -> Vec<u8> + Send>;
/// A decoded section value, typed by its provider.
pub(crate) type Decoded = Box<dyn Any + Send + Sync>;
/// A pre-apply check of one decoded value against the running app.
pub(crate) type Check = Arc<dyn Fn(&World, &(dyn Any + Send + Sync)) -> Result<(), String> + Send + Sync>;

/// The type-erased half of a provider.
pub(crate) trait SectionIo: Send + Sync + 'static {
    fn snapshot(&self, world: &World) -> Result<Vec<(SectionKey, Encoder)>, String>;
    fn decode(&self, raw: &[u8]) -> Result<Decoded, String>;
    /// Every value of the family the save holds (none: the family resets).
    fn apply(&self, world: &mut World, values: Vec<(SectionKey, Decoded)>) -> Result<(), String>;
}

/// Snapshot and apply of one section family, registered with
/// [`SaveAppExt::add_save_section`](crate::SaveAppExt::add_save_section). Framework sections are
/// built by [`crate::sections`]; a game adds its own the same way.
pub struct SaveSection {
    pub(crate) spec: SectionSpec,
    pub(crate) order: i32,
    pub(crate) flags: SectionFlags,
    pub(crate) io: Arc<dyn SectionIo>,
    pub(crate) check: Option<Check>,
}

impl SaveSection {
    /// One section `key` holding a `T`. `get` returns `None` when there is nothing to save (the
    /// resource is absent); on load, a save without the section applies `T::default()` (load is
    /// a full replace).
    pub fn value<T, G, S>(key: SectionKey, version: u32, get: G, set: S) -> Self
    where
        T: Serialize + DeserializeOwned + Default + Send + Sync + 'static,
        G: Fn(&World) -> Option<T> + Send + Sync + 'static,
        S: Fn(&mut World, T) + Send + Sync + 'static,
    {
        let spec = SectionSpec::new(key.owner(), key.name(), version);
        Self::new(spec, Value { key, get, set, _t: PhantomData })
    }

    /// A family of sections (`sf/character/<player key>`, `<mod id>/script`): `get` returns every
    /// instance with its key (each must belong to `family`); `set` receives every instance the
    /// save holds, possibly none.
    pub fn family<T, G, S>(family: Family, version: u32, get: G, set: S) -> Self
    where
        T: Serialize + DeserializeOwned + Send + Sync + 'static,
        G: Fn(&World) -> Result<Vec<(SectionKey, T)>, String> + Send + Sync + 'static,
        S: Fn(&mut World, Vec<(SectionKey, T)>) -> Result<(), String> + Send + Sync + 'static,
    {
        let spec = match &family.owner {
            Some(owner) => SectionSpec::new(owner.clone(), family.family.clone(), version),
            None => SectionSpec::any_owner(family.family.clone(), version),
        };
        Self::new(spec, Many { family, get, set, _t: PhantomData })
    }

    /// A family whose values are already postcard bytes (a script runtime's snapshots): no
    /// decode or re-encode on the way through.
    pub fn raw_family<G, S>(family: Family, version: u32, get: G, set: S) -> Self
    where
        G: Fn(&World) -> Result<Vec<(SectionKey, Vec<u8>)>, String> + Send + Sync + 'static,
        S: Fn(&mut World, Vec<(SectionKey, Vec<u8>)>) -> Result<(), String> + Send + Sync + 'static,
    {
        let spec = match &family.owner {
            Some(owner) => SectionSpec::new(owner.clone(), family.family.clone(), version),
            None => SectionSpec::any_owner(family.family.clone(), version),
        };
        Self::new(spec, Raw { family, get, set })
    }

    fn new(spec: SectionSpec, io: impl SectionIo) -> Self {
        Self { spec, order: 0, flags: SectionFlags::NONE, io: Arc::new(io), check: None }
    }

    /// A check of each decoded `T` against the running app, run for every section before any is
    /// applied: a failure refuses the load with nothing changed.
    #[must_use]
    pub fn check<T: 'static>(mut self, f: impl Fn(&World, &T) -> Result<(), String> + Send + Sync + 'static) -> Self {
        self.check = Some(Arc::new(move |world, value| match value.downcast_ref::<T>() {
            Some(v) => f(world, v),
            None => Err("decoded value of the wrong type".into()),
        }));
        self
    }

    /// Apply order on load, ascending ([`crate::sections::order`]).
    #[must_use]
    pub fn order(mut self, order: i32) -> Self {
        self.order = order;
        self
    }

    /// Flags written with the section (`OPTIONAL`: a corrupt copy is skipped on load).
    #[must_use]
    pub fn flags(mut self, flags: SectionFlags) -> Self {
        self.flags = flags;
        self
    }

    /// A migration step on raw bytes from version `from` to `from + 1`.
    #[must_use]
    pub fn step(mut self, from: u32, f: impl Fn(&[u8]) -> Result<Vec<u8>, String> + Send + Sync + 'static) -> Self {
        self.spec = self.spec.step(from, f);
        self
    }

    /// A typed migration step: decode the frozen `Old` type, convert, encode the next version.
    #[must_use]
    pub fn typed_step<Old, New>(mut self, from: u32, f: impl Fn(Old) -> New + Send + Sync + 'static) -> Self
    where
        Old: DeserializeOwned,
        New: Serialize,
    {
        self.spec = self.spec.typed_step(from, f);
        self
    }

    /// Shown when a save is older than the oldest step.
    #[must_use]
    pub fn too_old_hint(mut self, hint: impl Into<String>) -> Self {
        self.spec = self.spec.too_old_hint(hint);
        self
    }

    pub fn spec(&self) -> &SectionSpec {
        &self.spec
    }
}

struct Value<T, G, S> {
    key: SectionKey,
    get: G,
    set: S,
    _t: PhantomData<fn() -> T>,
}

impl<T, G, S> SectionIo for Value<T, G, S>
where
    T: Serialize + DeserializeOwned + Default + Send + Sync + 'static,
    G: Fn(&World) -> Option<T> + Send + Sync + 'static,
    S: Fn(&mut World, T) + Send + Sync + 'static,
{
    fn snapshot(&self, world: &World) -> Result<Vec<(SectionKey, Encoder)>, String> {
        Ok((self.get)(world).map(|v| (self.key.clone(), Box::new(move || encode(&v)) as Encoder)).into_iter().collect())
    }

    fn decode(&self, raw: &[u8]) -> Result<Decoded, String> {
        decode::<T>(raw).map(|v| Box::new(v) as Decoded).map_err(|e| e.0)
    }

    fn apply(&self, world: &mut World, values: Vec<(SectionKey, Decoded)>) -> Result<(), String> {
        let value = match values.into_iter().next() {
            Some((_, v)) => *v.downcast::<T>().map_err(|_| "decoded value of the wrong type")?,
            None => T::default(),
        };
        (self.set)(world, value);
        Ok(())
    }
}

struct Many<T, G, S> {
    family: Family,
    get: G,
    set: S,
    _t: PhantomData<fn() -> T>,
}

fn check_family(family: &Family, key: &SectionKey) -> Result<(), String> {
    if family.matches(key) { Ok(()) } else { Err(format!("section {key} is outside family {family}")) }
}

impl<T, G, S> SectionIo for Many<T, G, S>
where
    T: Serialize + DeserializeOwned + Send + Sync + 'static,
    G: Fn(&World) -> Result<Vec<(SectionKey, T)>, String> + Send + Sync + 'static,
    S: Fn(&mut World, Vec<(SectionKey, T)>) -> Result<(), String> + Send + Sync + 'static,
{
    fn snapshot(&self, world: &World) -> Result<Vec<(SectionKey, Encoder)>, String> {
        let mut out = Vec::new();
        for (key, v) in (self.get)(world)? {
            check_family(&self.family, &key)?;
            out.push((key, Box::new(move || encode(&v)) as Encoder));
        }
        Ok(out)
    }

    fn decode(&self, raw: &[u8]) -> Result<Decoded, String> {
        decode::<T>(raw).map(|v| Box::new(v) as Decoded).map_err(|e| e.0)
    }

    fn apply(&self, world: &mut World, values: Vec<(SectionKey, Decoded)>) -> Result<(), String> {
        let mut typed = Vec::with_capacity(values.len());
        for (key, v) in values {
            typed.push((key, *v.downcast::<T>().map_err(|_| "decoded value of the wrong type")?));
        }
        (self.set)(world, typed)
    }
}

struct Raw<G, S> {
    family: Family,
    get: G,
    set: S,
}

impl<G, S> SectionIo for Raw<G, S>
where
    G: Fn(&World) -> Result<Vec<(SectionKey, Vec<u8>)>, String> + Send + Sync + 'static,
    S: Fn(&mut World, Vec<(SectionKey, Vec<u8>)>) -> Result<(), String> + Send + Sync + 'static,
{
    fn snapshot(&self, world: &World) -> Result<Vec<(SectionKey, Encoder)>, String> {
        let mut out = Vec::new();
        for (key, bytes) in (self.get)(world)? {
            check_family(&self.family, &key)?;
            out.push((key, Box::new(move || bytes) as Encoder));
        }
        Ok(out)
    }

    fn decode(&self, raw: &[u8]) -> Result<Decoded, String> {
        Ok(Box::new(raw.to_vec()))
    }

    fn apply(&self, world: &mut World, values: Vec<(SectionKey, Decoded)>) -> Result<(), String> {
        let mut raw = Vec::with_capacity(values.len());
        for (key, v) in values {
            raw.push((key, *v.downcast::<Vec<u8>>().map_err(|_| "decoded value of the wrong type")?));
        }
        (self.set)(world, raw)
    }
}

/// `get` for [`SaveSection::value`] over a resource: `None` when the resource is absent.
pub fn read<R: Resource, T>(f: impl Fn(&R) -> T + Send + Sync + 'static) -> impl Fn(&World) -> Option<T> + Send + Sync + 'static {
    move |world| world.get_resource::<R>().map(&f)
}

/// `set` for [`SaveSection::value`] over a resource (inserted with `R::default()` if absent).
pub fn write<R: Resource<Mutability = bevy_ecs::component::Mutable> + Default, T>(
    f: impl Fn(&mut R, T) + Send + Sync + 'static,
) -> impl Fn(&mut World, T) + Send + Sync + 'static {
    move |world, value| f(&mut world.get_resource_or_insert_with(R::default), value)
}
