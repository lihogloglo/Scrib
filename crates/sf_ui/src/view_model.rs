//! MVVM view-models (ui.md "Data model": game systems write them, screens read them).
//!
//! The spec sketched `ViewModel: Reflect`; this crate stays Bevy-free (§8b), so
//! a view-model exposes named fields as [`UiValue`]s instead. That is all
//! bindings (`Bind::text("item.name")`), columns (`ValueSource::Field`) and
//! sorting need; `sf_ui_bevy` may still derive `Reflect` on the same types.

use std::cmp::Ordering;
use std::fmt;

/// A bindable field value.
#[derive(Clone, Debug, PartialEq)]
pub enum UiValue {
    None,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
}

impl UiValue {
    /// Sort order for list columns: numbers by value (ints and floats mixed),
    /// text case-insensitively, `None` last in ascending order.
    pub fn sort_cmp(&self, other: &UiValue) -> Ordering {
        use UiValue as V;
        match (self, other) {
            (V::None, V::None) => Ordering::Equal,
            (V::None, _) => Ordering::Greater,
            (_, V::None) => Ordering::Less,
            (V::Text(a), V::Text(b)) => a
                .to_lowercase()
                .cmp(&b.to_lowercase())
                .then_with(|| a.cmp(b)),
            (a, b) => match (a.as_f64(), b.as_f64()) {
                (Some(x), Some(y)) => x.total_cmp(&y),
                // Mixed text and numbers: numbers first.
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => Ordering::Equal,
            },
        }
    }

    /// Numeric view: ints, floats and bools (0/1).
    #[allow(clippy::cast_precision_loss)] // sort keys; i64 beyond 2^53 is not a UI value
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            UiValue::Int(n) => Some(*n as f64),
            UiValue::Float(x) => Some(*x),
            UiValue::Bool(b) => Some(f64::from(u8::from(*b))),
            UiValue::None | UiValue::Text(_) => None,
        }
    }
}

impl fmt::Display for UiValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UiValue::None => Ok(()),
            UiValue::Bool(b) => write!(f, "{b}"),
            UiValue::Int(n) => write!(f, "{n}"),
            UiValue::Float(x) => write!(f, "{x}"),
            UiValue::Text(s) => f.write_str(s),
        }
    }
}

impl From<bool> for UiValue {
    fn from(v: bool) -> Self {
        UiValue::Bool(v)
    }
}

impl From<i64> for UiValue {
    fn from(v: i64) -> Self {
        UiValue::Int(v)
    }
}

impl From<i32> for UiValue {
    fn from(v: i32) -> Self {
        UiValue::Int(v.into())
    }
}

impl From<u32> for UiValue {
    fn from(v: u32) -> Self {
        UiValue::Int(v.into())
    }
}

impl From<f32> for UiValue {
    fn from(v: f32) -> Self {
        UiValue::Float(v.into())
    }
}

impl From<f64> for UiValue {
    fn from(v: f64) -> Self {
        UiValue::Float(v)
    }
}

impl From<&str> for UiValue {
    fn from(v: &str) -> Self {
        UiValue::Text(v.to_owned())
    }
}

impl From<String> for UiValue {
    fn from(v: String) -> Self {
        UiValue::Text(v)
    }
}

impl<T: Into<UiValue>> From<Option<T>> for UiValue {
    fn from(v: Option<T>) -> Self {
        v.map_or(UiValue::None, Into::into)
    }
}

/// Typed data a screen binds to by field name.
pub trait ViewModel: Send + Sync + 'static {
    /// Kind name screens declare (`ScreenDef::view_models`), e.g. `inventory.item`.
    const KIND: &'static str;

    /// Field names, for diagnostics and binding validation.
    fn fields() -> &'static [&'static str];

    /// A field's current value; `None` for unknown names.
    fn field(&self, name: &str) -> Option<UiValue>;
}
