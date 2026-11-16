//! `const` declarations of tunables ([`Def`]) and typed handles ([`Id`]). Spec:
//! `docs/specs/tunables.md` "Definition".

use std::fmt;
use std::marker::PhantomData;

use crate::meta::{Group, Lifetime, Meta, Net, Scope, Source, Unit};
use crate::store::{RegisterError, Spec, TunableId, Tunables};
use crate::value::{ArgKind, Value};

mod sealed {
    pub trait Sealed {}
}

/// A Rust type a tunable can hold: `bool`, integers (`i32`, `i64`, `u32`, `usize`), floats
/// (`f32`, `f64`), pairs and triples (`[f32; 2]`, `[f32; 3]`), and text or a choice
/// (`&'static str`).
pub trait Type: Copy + sealed::Sealed + 'static {
    /// What a read returns (`&str` borrows the store for text).
    type Read<'a>;
    /// The kind for an optional range and, for text, choices.
    fn kind(range: Option<(Self, Self)>, choices: &[&str]) -> ArgKind;
    fn to_value(self) -> Value;
    /// `None` if `value` is another kind (never, through a typed [`Id`]).
    fn read(value: &Value) -> Option<Self::Read<'_>>;
    /// Whether this is a number, which needs a range.
    const NUMBER: bool;
}

/// `f32` through its shortest decimal form, so `0.1f32` reads `0.1`, not `0.100000001...`.
fn widen(x: f32) -> f64 {
    x.to_string().parse().unwrap_or(f64::from(x))
}

impl sealed::Sealed for bool {}
impl Type for bool {
    type Read<'a> = bool;
    const NUMBER: bool = false;
    fn kind(_: Option<(Self, Self)>, _: &[&str]) -> ArgKind {
        ArgKind::Bool
    }
    fn to_value(self) -> Value {
        Value::Bool(self)
    }
    fn read(value: &Value) -> Option<bool> {
        value.as_bool()
    }
}

macro_rules! int_type {
    ($($t:ty),*) => {$(
        impl sealed::Sealed for $t {}
        impl Type for $t {
            type Read<'a> = $t;
            const NUMBER: bool = true;
            fn kind(range: Option<(Self, Self)>, _: &[&str]) -> ArgKind {
                let clamp = |x: $t| i64::try_from(x).unwrap_or(i64::MAX);
                let (min, max) = range.unwrap_or((<$t>::MIN, <$t>::MAX));
                ArgKind::Int { min: clamp(min), max: clamp(max) }
            }
            fn to_value(self) -> Value {
                Value::Int(i64::try_from(self).unwrap_or(i64::MAX))
            }
            fn read(value: &Value) -> Option<$t> {
                value.as_int().and_then(|i| <$t>::try_from(i).ok())
            }
        }
    )*};
}
int_type!(i32, i64, u32, usize);

impl sealed::Sealed for f32 {}
impl Type for f32 {
    type Read<'a> = f32;
    const NUMBER: bool = true;
    fn kind(range: Option<(Self, Self)>, _: &[&str]) -> ArgKind {
        let (min, max) = range.map_or((f64::NEG_INFINITY, f64::INFINITY), |(a, b)| {
            (widen(a), widen(b))
        });
        ArgKind::Float { min, max }
    }
    fn to_value(self) -> Value {
        Value::Float(widen(self))
    }
    #[allow(clippy::cast_possible_truncation)]
    fn read(value: &Value) -> Option<f32> {
        value.as_float().map(|x| x as f32)
    }
}

impl sealed::Sealed for f64 {}
impl Type for f64 {
    type Read<'a> = f64;
    const NUMBER: bool = true;
    fn kind(range: Option<(Self, Self)>, _: &[&str]) -> ArgKind {
        let (min, max) = range.unwrap_or((f64::NEG_INFINITY, f64::INFINITY));
        ArgKind::Float { min, max }
    }
    fn to_value(self) -> Value {
        Value::Float(self)
    }
    fn read(value: &Value) -> Option<f64> {
        value.as_float()
    }
}

/// Pairs and triples: the range bounds every component (`range([0, 0], [500, 500])`; the
/// smallest min and largest max are used).
macro_rules! tuple_type {
    ($n:literal, $variant:ident, $as:ident) => {
        impl sealed::Sealed for [f32; $n] {}
        impl Type for [f32; $n] {
            type Read<'a> = [f32; $n];
            const NUMBER: bool = true;
            fn kind(range: Option<(Self, Self)>, _: &[&str]) -> ArgKind {
                let (min, max) = range.map_or((f64::NEG_INFINITY, f64::INFINITY), |(a, b)| {
                    (
                        a.iter().copied().map(widen).fold(f64::INFINITY, f64::min),
                        b.iter()
                            .copied()
                            .map(widen)
                            .fold(f64::NEG_INFINITY, f64::max),
                    )
                });
                ArgKind::$variant { min, max }
            }
            fn to_value(self) -> Value {
                Value::$variant(self.map(widen))
            }
            #[allow(clippy::cast_possible_truncation)]
            fn read(value: &Value) -> Option<[f32; $n]> {
                value.$as().map(|v| v.map(|x| x as f32))
            }
        }
    };
}
tuple_type!(2, Float2, as_float2);
tuple_type!(3, Float3, as_float3);

impl sealed::Sealed for &'static str {}
impl Type for &'static str {
    type Read<'a> = &'a str;
    const NUMBER: bool = false;
    fn kind(_: Option<(Self, Self)>, choices: &[&str]) -> ArgKind {
        if choices.is_empty() {
            ArgKind::Str
        } else {
            ArgKind::choice(choices)
        }
    }
    fn to_value(self) -> Value {
        Value::Str(self.to_owned())
    }
    fn read(value: &Value) -> Option<&str> {
        value.as_str()
    }
}

/// A tunable declared once, as `const` data, next to the code that reads it. Build it with
/// [`crate::def!`] (which records `file!()`/`line!()`) and the `const fn` builders; register
/// it with [`Tunables::register_def`].
///
/// ```
/// use sf_tunables::{def, Def, Group, Source, Unit};
/// pub const LAMP_FADE_START: Def<f32> = def!(float "shadow.lamp.fade_start_m" = 15.0)
///     .range(0.0, 500.0)
///     .unit(Unit::Meters)
///     .help("Lamp shadows at full strength within this distance of the viewer")
///     .source(Source::Industry("UE r.Shadow.DistanceScale ~ local light fade"))
///     .group(Group::Shadows);
/// ```
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Def<T: Type> {
    name: &'static str,
    default: T,
    range: Option<(T, T)>,
    choices: &'static [&'static str],
    help: &'static str,
    meta: Meta,
}

impl<T: Type> Def<T> {
    pub const fn new(name: &'static str, default: T) -> Self {
        Self {
            name,
            default,
            range: None,
            choices: &[],
            help: "",
            meta: Meta {
                unit: Unit::None,
                source: Source::Guess,
                group: None,
                scope: Scope::Dev,
                net: Net::Presentation,
                lifetime: Lifetime::Live,
                location: None,
                env_alias: None,
            },
        }
    }

    /// Required for numbers: sets outside `min..=max` are refused.
    pub const fn range(mut self, min: T, max: T) -> Self {
        self.range = Some((min, max));
        self
    }

    /// For text: the value must be one of these (lowercase).
    pub const fn choices(mut self, choices: &'static [&'static str]) -> Self {
        self.choices = choices;
        self
    }

    pub const fn help(mut self, help: &'static str) -> Self {
        self.help = help;
        self
    }

    pub const fn unit(mut self, unit: Unit) -> Self {
        self.meta.unit = unit;
        self
    }

    pub const fn source(mut self, source: Source) -> Self {
        self.meta.source = source;
        self
    }

    pub const fn group(mut self, group: Group) -> Self {
        self.meta.group = Some(group);
        self
    }

    pub const fn scope(mut self, scope: Scope) -> Self {
        self.meta.scope = scope;
        self
    }

    pub const fn net(mut self, net: Net) -> Self {
        self.meta.net = net;
        self
    }

    pub const fn lifetime(mut self, lifetime: Lifetime) -> Self {
        self.meta.lifetime = lifetime;
        self
    }

    /// A deprecated environment switch that still sets it, at the `CommandLine` layer, when the
    /// tunable is registered in a Bevy app (`sf_tunables_bevy`); parsed by the tunable's kind
    /// (bools: `1`/`0`, `on`/`off`, `true`/`false`). Shown by the dump and `help`.
    pub const fn env_alias(mut self, name: &'static str) -> Self {
        self.meta.env_alias = Some(name);
        self
    }

    /// Where it is declared ([`crate::def!`] sets it).
    pub const fn at(mut self, file: &'static str, line: u32) -> Self {
        self.meta.location = Some((file, line));
        self
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn default_value(&self) -> T {
        self.default
    }

    pub const fn bounds(&self) -> Option<(T, T)> {
        self.range
    }

    pub const fn meta(&self) -> &Meta {
        &self.meta
    }

    /// The runtime spec [`Tunables::register`] takes.
    pub fn spec(&self) -> Spec {
        Spec::new(
            self.name,
            T::kind(self.range, self.choices),
            self.default.to_value(),
        )
        .help(self.help)
        .meta(self.meta)
    }
}

/// Any [`Def`], for a crate's `TUNABLES: &[&dyn AnyDef]` list.
pub trait AnyDef: Sync {
    fn name(&self) -> &'static str;
    fn spec(&self) -> Spec;
    /// Numbers need a range.
    fn needs_range(&self) -> bool;
}

impl<T: Type + Sync> AnyDef for Def<T> {
    fn name(&self) -> &'static str {
        self.name
    }
    fn spec(&self) -> Spec {
        Def::spec(self)
    }
    fn needs_range(&self) -> bool {
        T::NUMBER && self.range.is_none()
    }
}

/// A typed handle: reads return `T` (or `&str`) with no lookup.
pub struct Id<T> {
    id: TunableId,
    _type: PhantomData<fn() -> T>,
}

impl<T> Id<T> {
    pub fn untyped(self) -> TunableId {
        self.id
    }
}

impl<T> From<Id<T>> for TunableId {
    fn from(id: Id<T>) -> Self {
        id.id
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for Id<T> {}
impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<T> Eq for Id<T> {}
impl<T> fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Id").field(&self.id).finish()
    }
}

impl Tunables {
    /// Registers a [`Def`] (a number without a range is refused).
    pub fn register_def<T: Type + Sync>(&mut self, def: &Def<T>) -> Result<Id<T>, RegisterError> {
        self.register_any(def).map(|id| Id {
            id,
            _type: PhantomData,
        })
    }

    /// Registers one def of a crate's list.
    pub fn register_any(&mut self, def: &dyn AnyDef) -> Result<TunableId, RegisterError> {
        if def.needs_range() {
            return Err(RegisterError::NoRange(def.name().to_owned()));
        }
        self.register(def.spec())
    }

    /// Registers a crate's whole list, stopping at the first error.
    pub fn register_all(&mut self, defs: &[&dyn AnyDef]) -> Result<Vec<TunableId>, RegisterError> {
        defs.iter().map(|d| self.register_any(*d)).collect()
    }

    /// The typed handle of an already registered def (another crate registered it), checking
    /// that the kind matches.
    pub fn id_of<T: Type + Sync>(&self, def: &Def<T>) -> Result<Id<T>, RegisterError> {
        let id = self
            .id(def.name)
            .ok_or_else(|| RegisterError::Unknown(def.name.to_owned()))?;
        let registered = self.var(id).kind();
        let wanted = def.spec().kind;
        if std::mem::discriminant(registered) != std::mem::discriminant(&wanted) {
            return Err(RegisterError::WrongKind {
                name: def.name.to_owned(),
                registered: registered.describe(),
                wanted: wanted.describe(),
            });
        }
        Ok(Id {
            id,
            _type: PhantomData,
        })
    }

    /// The effective value through a typed handle.
    pub fn read<T: Type>(&self, id: Id<T>) -> T::Read<'_> {
        T::read(self.get(id.id)).expect("a typed id always matches its kind")
    }

    /// Sets a typed value on `layer` (see [`Tunables::set_by`]).
    pub fn write<T: Type>(
        &mut self,
        id: Id<T>,
        value: T,
        layer: crate::SetBy,
    ) -> Result<crate::SetOutcome, String> {
        self.set_by(id.id, value.to_value(), layer)
    }
}

/// Declares a [`Def`] with its source location. The kind word fixes the Rust type: `bool`,
/// `int` (`i64`), `count` (`u32`), `float` (`f32`), `float2`/`float3` (`[f32; N]`), `text`
/// and `choice` (`&'static str`; give a choice its options with `.choices(&[..])`). Without a
/// kind word the type comes from the const's annotation.
#[macro_export]
macro_rules! def {
    (bool $name:literal = $default:expr) => { $crate::def!(@ bool, $name, $default) };
    (int $name:literal = $default:expr) => { $crate::def!(@ i64, $name, $default) };
    (count $name:literal = $default:expr) => { $crate::def!(@ u32, $name, $default) };
    (float $name:literal = $default:expr) => { $crate::def!(@ f32, $name, $default) };
    (float2 $name:literal = $default:expr) => { $crate::def!(@ [f32; 2], $name, $default) };
    (float3 $name:literal = $default:expr) => { $crate::def!(@ [f32; 3], $name, $default) };
    (text $name:literal = $default:expr) => { $crate::def!(@ &'static str, $name, $default) };
    (choice $name:literal = $default:expr) => { $crate::def!(@ &'static str, $name, $default) };
    ($name:literal = $default:expr) => { $crate::Def::new($name, $default).at(file!(), line!()) };
    (@ $t:ty, $name:literal, $default:expr) => {
        $crate::Def::<$t>::new($name, $default).at(file!(), line!())
    };
}
