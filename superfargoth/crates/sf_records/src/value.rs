//! A generic value tree for record patches (mods.md "Native record files").
//!
//! A record of any serde type is serialized into a [`Value`], patch operations
//! edit the tree, and the tree deserializes back into the record type. Values
//! that come from a RON file stay as [`Raw`] text until the record type asks
//! for them, so they are parsed with the type's own hints: enum variants,
//! newtypes and `Option`s read exactly as `ron` reads them. (`ron::Value`
//! cannot carry enum variant names, so it can't round-trip records.)
//!
//! Values from formats without types (TOML) are read leniently: a string can
//! be a unit enum variant, a one-entry table an enum variant with data, a
//! table a struct, and any non-unit value a `Some`.

use std::fmt;

use serde::de::value::BorrowedStrDeserializer;
use serde::de::{self, IntoDeserializer as _};
use serde::ser::{self, Serialize};

/// A serde data-model value, plus [`Raw`] RON text that is typed on use.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Unit,
    Bool(bool),
    I64(i64),
    U64(u64),
    F64(f64),
    Char(char),
    String(String),
    Bytes(Vec<u8>),
    None,
    Some(Box<Value>),
    /// Sequences, tuples and tuple structs.
    Seq(Vec<Value>),
    /// Maps, in serialization order.
    Map(Vec<(Value, Value)>),
    /// Structs, by field name.
    Struct(Vec<(String, Value)>),
    /// An enum variant; `payload` is `None` for unit variants, else a newtype
    /// value, a `Seq` (tuple variant) or a `Struct` (struct variant).
    Enum { variant: String, payload: Option<Box<Value>> },
    /// RON text, parsed when the record type deserializes it.
    Raw(Raw),
}

/// RON text from a record file.
#[derive(Clone, Debug, PartialEq)]
pub struct Raw {
    pub text: String,
    /// Byte offset of `text` in its file, for error positions.
    pub offset: usize,
    /// RON extensions in effect in the file (`#![enable(...)]` plus defaults).
    pub extensions: ron::extensions::Extensions,
}

/// A value that failed to (de)serialize; `at` is a byte offset in the record
/// file when the error is inside file text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValueError {
    pub message: String,
    pub at: Option<usize>,
}

impl ValueError {
    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into(), at: None }
    }
}

impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for ValueError {}

impl de::Error for ValueError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self::new(msg.to_string())
    }
}

impl ser::Error for ValueError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self::new(msg.to_string())
    }
}

impl Value {
    /// Serializes any serde value into a tree.
    pub fn from_serialize<T: Serialize + ?Sized>(value: &T) -> Result<Self, ValueError> {
        value.serialize(ToValue)
    }

    /// Deserializes the tree into `T`.
    pub fn deserialize_into<T: de::DeserializeOwned>(&self) -> Result<T, ValueError> {
        T::deserialize(self)
    }

    /// The key this value has as a path segment, if it is a scalar.
    pub(crate) fn key_text(&self) -> Option<String> {
        Some(match self {
            Self::String(s) => s.clone(),
            Self::I64(n) => n.to_string(),
            Self::U64(n) => n.to_string(),
            Self::Bool(b) => b.to_string(),
            Self::Char(c) => c.to_string(),
            Self::Enum { variant, payload: None } => variant.clone(),
            Self::Raw(raw) => {
                let text = raw.text.trim();
                text.strip_prefix('"').and_then(|t| t.strip_suffix('"')).unwrap_or(text).to_owned()
            }
            _ => return None,
        })
    }
}

impl Raw {
    /// Byte offset in the file of a (line, column) position inside `text`
    /// (both 1-based, columns in characters, as `ron` reports them).
    fn offset_of(&self, position: ron::error::Position) -> usize {
        let mut line_start = 0;
        for _ in 1..position.line {
            match self.text[line_start..].find('\n') {
                Some(n) => line_start += n + 1,
                None => break,
            }
        }
        let column: usize =
            self.text[line_start..].chars().take(position.col.saturating_sub(1)).map(char::len_utf8).sum();
        self.offset + line_start + column
    }

    fn error(&self, error: &ron::error::SpannedError) -> ValueError {
        ValueError { message: error.code.to_string(), at: Some(self.offset_of(error.span.start)) }
    }

    /// Runs one `ron` deserializer call over the text, checking it is all consumed.
    fn with<'de, R>(
        &'de self,
        f: impl FnOnce(&mut ron::Deserializer<'de>) -> Result<R, ron::Error>,
    ) -> Result<R, ValueError> {
        let options = ron::Options::default().with_default_extension(self.extensions);
        let mut de = ron::Deserializer::from_str_with_options(&self.text, &options).map_err(|e| self.error(&e))?;
        let out = f(&mut de).map_err(|e| self.error(&de.span_error(e)))?;
        de.end().map_err(|e| self.error(&de.span_error(e)))?;
        Ok(out)
    }

    /// Deserializes the whole text as `T` (which may borrow from it).
    pub(crate) fn deserialize_ron<'a, T: de::Deserialize<'a>>(&'a self) -> Result<T, ValueError> {
        self.with(|de| T::deserialize(de))
    }

    /// Splits a RON map or struct (`{ "a": 1 }`, `(a: 1)`) into its entries.
    pub(crate) fn entries(&self) -> Option<Vec<(String, Raw)>> {
        let entries: Entries<'_> = self.with(|de| de::Deserialize::deserialize(de)).ok()?;
        Some(entries.0.into_iter().map(|(k, v)| (k, self.sub(v.trim().get_ron()))).collect())
    }

    /// Splits a RON list into its elements.
    pub(crate) fn elements(&self) -> Option<Vec<Raw>> {
        let elements: Vec<&ron::value::RawValue> = self.with(|de| de::Deserialize::deserialize(de)).ok()?;
        Some(elements.into_iter().map(|v| self.sub(v.trim().get_ron())).collect())
    }

    /// A `Raw` for a slice of this one's text.
    pub(crate) fn sub(&self, text: &str) -> Raw {
        Raw { text: text.to_owned(), offset: self.offset + subslice_offset(&self.text, text), extensions: self.extensions }
    }
}

/// Byte offset of `inner` in `outer`, when `inner` is a slice of it (else 0).
pub(crate) fn subslice_offset(outer: &str, inner: &str) -> usize {
    let start = (inner.as_ptr() as usize).wrapping_sub(outer.as_ptr() as usize);
    if start <= outer.len() { start } else { 0 }
}

/// Entries of a RON map or struct, keys as text, values borrowed.
#[derive(Default)]
pub(crate) struct Entries<'a>(pub(crate) Vec<(String, &'a ron::value::RawValue)>);

impl<'de: 'a, 'a> de::Deserialize<'de> for Entries<'a> {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visit<'a>(std::marker::PhantomData<&'a ()>);
        impl<'de: 'a, 'a> de::Visitor<'de> for Visit<'a> {
            type Value = Entries<'a>;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a map or struct")
            }
            fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut out = Vec::new();
                while let Some(key) = map.next_key::<String>()? {
                    out.push((key, map.next_value()?));
                }
                Ok(Entries(out))
            }
        }
        deserializer.deserialize_any(Visit(std::marker::PhantomData))
    }
}

// ---------------------------------------------------------------------------
// Serializer: T -> Value

struct ToValue;

struct SeqSer(Vec<Value>, Option<String>);
struct MapSer(Vec<(Value, Value)>, Option<Value>);
struct StructSer(Vec<(String, Value)>, Option<String>);

impl ser::Serializer for ToValue {
    type Ok = Value;
    type Error = ValueError;
    type SerializeSeq = SeqSer;
    type SerializeTuple = SeqSer;
    type SerializeTupleStruct = SeqSer;
    type SerializeTupleVariant = SeqSer;
    type SerializeMap = MapSer;
    type SerializeStruct = StructSer;
    type SerializeStructVariant = StructSer;

    fn serialize_bool(self, v: bool) -> Result<Value, ValueError> {
        Ok(Value::Bool(v))
    }
    fn serialize_i8(self, v: i8) -> Result<Value, ValueError> {
        Ok(Value::I64(v.into()))
    }
    fn serialize_i16(self, v: i16) -> Result<Value, ValueError> {
        Ok(Value::I64(v.into()))
    }
    fn serialize_i32(self, v: i32) -> Result<Value, ValueError> {
        Ok(Value::I64(v.into()))
    }
    fn serialize_i64(self, v: i64) -> Result<Value, ValueError> {
        Ok(Value::I64(v))
    }
    fn serialize_u8(self, v: u8) -> Result<Value, ValueError> {
        Ok(Value::U64(v.into()))
    }
    fn serialize_u16(self, v: u16) -> Result<Value, ValueError> {
        Ok(Value::U64(v.into()))
    }
    fn serialize_u32(self, v: u32) -> Result<Value, ValueError> {
        Ok(Value::U64(v.into()))
    }
    fn serialize_u64(self, v: u64) -> Result<Value, ValueError> {
        Ok(Value::U64(v))
    }
    fn serialize_f32(self, v: f32) -> Result<Value, ValueError> {
        Ok(Value::F64(v.into()))
    }
    fn serialize_f64(self, v: f64) -> Result<Value, ValueError> {
        Ok(Value::F64(v))
    }
    fn serialize_char(self, v: char) -> Result<Value, ValueError> {
        Ok(Value::Char(v))
    }
    fn serialize_str(self, v: &str) -> Result<Value, ValueError> {
        Ok(Value::String(v.to_owned()))
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<Value, ValueError> {
        Ok(Value::Bytes(v.to_vec()))
    }
    fn serialize_none(self) -> Result<Value, ValueError> {
        Ok(Value::None)
    }
    fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<Value, ValueError> {
        Ok(Value::Some(Box::new(value.serialize(ToValue)?)))
    }
    fn serialize_unit(self) -> Result<Value, ValueError> {
        Ok(Value::Unit)
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Value, ValueError> {
        Ok(Value::Unit)
    }
    fn serialize_unit_variant(self, _name: &'static str, _index: u32, variant: &'static str) -> Result<Value, ValueError> {
        Ok(Value::Enum { variant: variant.to_owned(), payload: None })
    }
    fn serialize_newtype_struct<T: Serialize + ?Sized>(self, _name: &'static str, value: &T) -> Result<Value, ValueError> {
        value.serialize(ToValue)
    }
    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Value, ValueError> {
        Ok(Value::Enum { variant: variant.to_owned(), payload: Some(Box::new(value.serialize(ToValue)?)) })
    }
    fn serialize_seq(self, len: Option<usize>) -> Result<SeqSer, ValueError> {
        Ok(SeqSer(Vec::with_capacity(len.unwrap_or(0)), None))
    }
    fn serialize_tuple(self, len: usize) -> Result<SeqSer, ValueError> {
        Ok(SeqSer(Vec::with_capacity(len), None))
    }
    fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<SeqSer, ValueError> {
        Ok(SeqSer(Vec::with_capacity(len), None))
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<SeqSer, ValueError> {
        Ok(SeqSer(Vec::with_capacity(len), Some(variant.to_owned())))
    }
    fn serialize_map(self, len: Option<usize>) -> Result<MapSer, ValueError> {
        Ok(MapSer(Vec::with_capacity(len.unwrap_or(0)), None))
    }
    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<StructSer, ValueError> {
        Ok(StructSer(Vec::with_capacity(len), None))
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<StructSer, ValueError> {
        Ok(StructSer(Vec::with_capacity(len), Some(variant.to_owned())))
    }
}

fn wrap(payload: Value, variant: Option<String>) -> Value {
    match variant {
        Some(variant) => Value::Enum { variant, payload: Some(Box::new(payload)) },
        None => payload,
    }
}

impl SeqSer {
    fn push<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), ValueError> {
        self.0.push(value.serialize(ToValue)?);
        Ok(())
    }
    fn finish(self) -> Value {
        wrap(Value::Seq(self.0), self.1)
    }
}

impl ser::SerializeSeq for SeqSer {
    type Ok = Value;
    type Error = ValueError;
    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), ValueError> {
        self.push(value)
    }
    fn end(self) -> Result<Value, ValueError> {
        Ok(self.finish())
    }
}

impl ser::SerializeTuple for SeqSer {
    type Ok = Value;
    type Error = ValueError;
    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), ValueError> {
        self.push(value)
    }
    fn end(self) -> Result<Value, ValueError> {
        Ok(self.finish())
    }
}

impl ser::SerializeTupleStruct for SeqSer {
    type Ok = Value;
    type Error = ValueError;
    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), ValueError> {
        self.push(value)
    }
    fn end(self) -> Result<Value, ValueError> {
        Ok(self.finish())
    }
}

impl ser::SerializeTupleVariant for SeqSer {
    type Ok = Value;
    type Error = ValueError;
    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), ValueError> {
        self.push(value)
    }
    fn end(self) -> Result<Value, ValueError> {
        Ok(self.finish())
    }
}

impl ser::SerializeMap for MapSer {
    type Ok = Value;
    type Error = ValueError;
    fn serialize_key<T: Serialize + ?Sized>(&mut self, key: &T) -> Result<(), ValueError> {
        self.1 = Some(key.serialize(ToValue)?);
        Ok(())
    }
    fn serialize_value<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), ValueError> {
        let key = self.1.take().ok_or_else(|| ValueError::new("map value without a key"))?;
        self.0.push((key, value.serialize(ToValue)?));
        Ok(())
    }
    fn end(self) -> Result<Value, ValueError> {
        Ok(Value::Map(self.0))
    }
}

impl StructSer {
    fn push<T: Serialize + ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), ValueError> {
        self.0.push((key.to_owned(), value.serialize(ToValue)?));
        Ok(())
    }
}

impl ser::SerializeStruct for StructSer {
    type Ok = Value;
    type Error = ValueError;
    fn serialize_field<T: Serialize + ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), ValueError> {
        self.push(key, value)
    }
    fn end(self) -> Result<Value, ValueError> {
        Ok(wrap(Value::Struct(self.0), self.1))
    }
}

impl ser::SerializeStructVariant for StructSer {
    type Ok = Value;
    type Error = ValueError;
    fn serialize_field<T: Serialize + ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), ValueError> {
        self.push(key, value)
    }
    fn end(self) -> Result<Value, ValueError> {
        Ok(wrap(Value::Struct(self.0), self.1))
    }
}

// ---------------------------------------------------------------------------
// Deserializer: &Value -> T

/// Forwards a method to `ron` for raw text, else to `deserialize_any`.
macro_rules! forward {
    ($($method:ident ( $($arg:ident : $ty:ty),* )),* $(,)?) => {$(
        fn $method<V: de::Visitor<'de>>(self, $($arg: $ty,)* visitor: V) -> Result<V::Value, ValueError> {
            match self {
                Value::Raw(raw) => raw.with(|de| de::Deserializer::$method(de, $($arg,)* visitor)),
                _ => self.deserialize_any(visitor),
            }
        }
    )*};
}

impl<'de> de::Deserializer<'de> for &'de Value {
    type Error = ValueError;

    fn deserialize_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, ValueError> {
        match self {
            Value::Unit => visitor.visit_unit(),
            Value::Bool(v) => visitor.visit_bool(*v),
            Value::I64(v) => visitor.visit_i64(*v),
            Value::U64(v) => visitor.visit_u64(*v),
            Value::F64(v) => visitor.visit_f64(*v),
            Value::Char(v) => visitor.visit_char(*v),
            Value::String(v) => visitor.visit_borrowed_str(v),
            Value::Bytes(v) => visitor.visit_borrowed_bytes(v),
            Value::None => visitor.visit_none(),
            Value::Some(v) => visitor.visit_some(&**v),
            Value::Seq(v) => visitor.visit_seq(SeqDe(v.iter())),
            Value::Map(v) => visitor.visit_map(MapDe { iter: v.iter(), value: None }),
            Value::Struct(v) => visitor.visit_map(StructDe { iter: v.iter(), value: None }),
            Value::Enum { variant, payload } => visitor.visit_enum(EnumDe { variant, payload: payload.as_deref() }),
            Value::Raw(raw) => raw.with(|de| de::Deserializer::deserialize_any(de, visitor)),
        }
    }

    fn deserialize_option<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, ValueError> {
        match self {
            Value::None | Value::Unit => visitor.visit_none(),
            Value::Some(v) => visitor.visit_some(&**v),
            Value::Raw(raw) => raw.with(|de| de::Deserializer::deserialize_option(de, visitor)),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_enum<V: de::Visitor<'de>>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, ValueError> {
        match self {
            Value::Enum { variant, payload } => visitor.visit_enum(EnumDe { variant, payload: payload.as_deref() }),
            Value::String(variant) => visitor.visit_enum(EnumDe { variant, payload: None }),
            Value::Map(entries) if entries.len() == 1 => match &entries[0] {
                (Value::String(variant), payload) => visitor.visit_enum(EnumDe { variant, payload: Some(payload) }),
                _ => Err(de::Error::custom(format!("expected an enum {name}"))),
            },
            Value::Raw(raw) => raw.with(|de| de::Deserializer::deserialize_enum(de, name, variants, visitor)),
            _ => Err(de::Error::custom(format!("expected an enum {name}"))),
        }
    }

    fn deserialize_newtype_struct<V: de::Visitor<'de>>(self, name: &'static str, visitor: V) -> Result<V::Value, ValueError> {
        match self {
            Value::Raw(raw) => raw.with(|de| de::Deserializer::deserialize_newtype_struct(de, name, visitor)),
            _ => visitor.visit_newtype_struct(self),
        }
    }

    forward! {
        deserialize_bool(), deserialize_i8(), deserialize_i16(), deserialize_i32(), deserialize_i64(),
        deserialize_i128(), deserialize_u8(), deserialize_u16(), deserialize_u32(), deserialize_u64(),
        deserialize_u128(), deserialize_f32(), deserialize_f64(), deserialize_char(), deserialize_str(),
        deserialize_string(), deserialize_bytes(), deserialize_byte_buf(), deserialize_unit(),
        deserialize_unit_struct(name: &'static str), deserialize_seq(), deserialize_tuple(len: usize),
        deserialize_tuple_struct(name: &'static str, len: usize), deserialize_map(),
        deserialize_struct(name: &'static str, fields: &'static [&'static str]),
        deserialize_identifier(), deserialize_ignored_any(),
    }
}

impl<'de> de::IntoDeserializer<'de, ValueError> for &'de Value {
    type Deserializer = Self;
    fn into_deserializer(self) -> Self {
        self
    }
}

struct SeqDe<'de>(std::slice::Iter<'de, Value>);

impl<'de> de::SeqAccess<'de> for SeqDe<'de> {
    type Error = ValueError;
    fn next_element_seed<S: de::DeserializeSeed<'de>>(&mut self, seed: S) -> Result<Option<S::Value>, ValueError> {
        self.0.next().map(|v| seed.deserialize(v)).transpose()
    }
    fn size_hint(&self) -> Option<usize> {
        Some(self.0.len())
    }
}

struct MapDe<'de> {
    iter: std::slice::Iter<'de, (Value, Value)>,
    value: Option<&'de Value>,
}

impl<'de> de::MapAccess<'de> for MapDe<'de> {
    type Error = ValueError;
    fn next_key_seed<S: de::DeserializeSeed<'de>>(&mut self, seed: S) -> Result<Option<S::Value>, ValueError> {
        let Some((key, value)) = self.iter.next() else { return Ok(None) };
        self.value = Some(value);
        seed.deserialize(key).map(Some)
    }
    fn next_value_seed<S: de::DeserializeSeed<'de>>(&mut self, seed: S) -> Result<S::Value, ValueError> {
        seed.deserialize(self.value.take().ok_or_else(|| ValueError::new("map value without a key"))?)
    }
    fn size_hint(&self) -> Option<usize> {
        Some(self.iter.len())
    }
}

struct StructDe<'de> {
    iter: std::slice::Iter<'de, (String, Value)>,
    value: Option<&'de Value>,
}

impl<'de> de::MapAccess<'de> for StructDe<'de> {
    type Error = ValueError;
    fn next_key_seed<S: de::DeserializeSeed<'de>>(&mut self, seed: S) -> Result<Option<S::Value>, ValueError> {
        let Some((key, value)) = self.iter.next() else { return Ok(None) };
        self.value = Some(value);
        seed.deserialize(BorrowedStrDeserializer::new(key)).map(Some)
    }
    fn next_value_seed<S: de::DeserializeSeed<'de>>(&mut self, seed: S) -> Result<S::Value, ValueError> {
        seed.deserialize(self.value.take().ok_or_else(|| ValueError::new("struct value without a field"))?)
    }
    fn size_hint(&self) -> Option<usize> {
        Some(self.iter.len())
    }
}

struct EnumDe<'de> {
    variant: &'de str,
    payload: Option<&'de Value>,
}

impl<'de> de::EnumAccess<'de> for EnumDe<'de> {
    type Error = ValueError;
    type Variant = VariantDe<'de>;
    fn variant_seed<S: de::DeserializeSeed<'de>>(self, seed: S) -> Result<(S::Value, Self::Variant), ValueError> {
        let variant = seed.deserialize(self.variant.into_deserializer())?;
        Ok((variant, VariantDe(self.payload)))
    }
}

struct VariantDe<'de>(Option<&'de Value>);

impl<'de> de::VariantAccess<'de> for VariantDe<'de> {
    type Error = ValueError;
    fn unit_variant(self) -> Result<(), ValueError> {
        match self.0 {
            None | Some(Value::Unit) => Ok(()),
            Some(_) => Err(ValueError::new("expected a unit variant")),
        }
    }
    fn newtype_variant_seed<S: de::DeserializeSeed<'de>>(self, seed: S) -> Result<S::Value, ValueError> {
        seed.deserialize(self.0.ok_or_else(|| ValueError::new("expected a variant with a value"))?)
    }
    fn tuple_variant<V: de::Visitor<'de>>(self, len: usize, visitor: V) -> Result<V::Value, ValueError> {
        let payload = self.0.ok_or_else(|| ValueError::new("expected a tuple variant"))?;
        de::Deserializer::deserialize_tuple(payload, len, visitor)
    }
    fn struct_variant<V: de::Visitor<'de>>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, ValueError> {
        let payload = self.0.ok_or_else(|| ValueError::new("expected a struct variant"))?;
        de::Deserializer::deserialize_struct(payload, "", fields, visitor)
    }
}
