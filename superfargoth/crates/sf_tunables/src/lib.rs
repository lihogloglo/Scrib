//! Tunables core, Bevy-free. Spec: `docs/specs/tunables.md`.
//!
//! A tunable is a named, typed, ranged value with a unit, a source (provenance), a preset group,
//! a scope, a net role and a lifetime (what a change costs).
//!
//! - [`Def`]: the `const` declaration next to the code that reads it ([`def!`] adds
//!   `file!()`/`line!()`); a crate lists its defs as `TUNABLES: &[&dyn AnyDef]`.
//! - [`Tunables`]: the registry. Each tunable holds one optional value per layer ([`SetBy`]:
//!   default < mod < preset < user < command line < code < console) and the strongest one set
//!   is in effect; sets are validated by kind and range; effective changes are tracked
//!   ([`Tunables::take_changes`], [`Tunables::revision`]). Reads by [`Id`] / [`TunableId`] are
//!   an index, fine every frame.
//! - [`Tunables::rows`] / [`Tunables::dump_markdown`]: the stable, sorted dump.
//!
//! `sf_console` uses this store for its variables (re-exported there as `Vars`, `Var`,
//! `VarId`, `VarSpec`).

mod def;
mod dump;
mod meta;
mod store;
mod value;

#[cfg(test)]
mod tests;

pub use def::{AnyDef, Def, Id, Type};
pub use dump::Row;
pub use meta::{Group, Lifetime, Meta, Net, Scope, Source, Unit};
pub use store::{
    Layer, RegisterError, SetBy, SetOutcome, Spec, Tunable, TunableId, Tunables, check_name,
};
pub use value::{ArgKind, NameFilter, Value, parse_bool};
