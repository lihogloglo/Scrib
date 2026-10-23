//! Typed values and argument kinds: parsing, validation, completion candidates.

use std::fmt;

/// A console value: a variable's state or a parsed argument.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Float(f64),
    /// Free text, or the canonical (lowercase) spelling of a choice.
    Str(String),
}

impl Value {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(i) => Some(*i),
            _ => None,
        }
    }

    /// Floats, and integers widened to float.
    #[allow(clippy::cast_precision_loss)]
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f),
            Value::Int(i) => Some(*i as f64),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Str(s) => Some(s),
            _ => None,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(true) => f.write_str("on"),
            Value::Bool(false) => f.write_str("off"),
            Value::Int(i) => write!(f, "{i}"),
            Value::Float(x) => write!(f, "{x}"),
            Value::Str(s)
                if s.is_empty()
                    || s.contains(|c: char| c.is_whitespace() || c == ';' || c == '"') =>
            {
                write!(f, "\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
            }
            Value::Str(s) => f.write_str(s),
        }
    }
}

/// Which names a [`ArgKind::Name`] argument accepts (and completes).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NameFilter {
    /// Any command or variable.
    Any,
    /// Any variable.
    Vars,
    /// Boolean variables only.
    BoolVars,
    /// A name, or free text that needn't be one (`help <text>` searches); completes like `Any`.
    Search,
}

/// The type of an argument or variable.
#[derive(Clone, Debug, PartialEq)]
pub enum ArgKind {
    /// `on`/`off`, `true`/`false`, `yes`/`no`, `1`/`0`.
    Bool,
    /// Integer in `min..=max`.
    Int { min: i64, max: i64 },
    /// Finite float in `min..=max`.
    Float { min: f64, max: f64 },
    /// Free text (quote it to include spaces).
    Str,
    /// One of a fixed set of lowercase words.
    Choice(Vec<String>),
    /// The name of a command or variable.
    Name(NameFilter),
}

impl ArgKind {
    pub fn int() -> Self {
        ArgKind::Int {
            min: i64::MIN,
            max: i64::MAX,
        }
    }

    pub fn float() -> Self {
        ArgKind::Float {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        }
    }

    pub fn choice(options: &[&str]) -> Self {
        ArgKind::Choice(options.iter().map(|o| o.to_ascii_lowercase()).collect())
    }

    /// What the kind expects, for usage lines and errors.
    pub fn describe(&self) -> String {
        match self {
            ArgKind::Bool => "on|off".into(),
            ArgKind::Int {
                min: i64::MIN,
                max: i64::MAX,
            } => "int".into(),
            ArgKind::Int { min, max } => format!("int {min}..={max}"),
            ArgKind::Float { min, max } if min.is_infinite() && max.is_infinite() => "float".into(),
            ArgKind::Float { min, max } => format!("float {min}..={max}"),
            ArgKind::Str => "text".into(),
            ArgKind::Choice(options) => options.join("|"),
            ArgKind::Name(NameFilter::Any) => "name".into(),
            ArgKind::Name(NameFilter::Vars) => "variable".into(),
            ArgKind::Name(NameFilter::BoolVars) => "on/off variable".into(),
            ArgKind::Name(NameFilter::Search) => "name or text".into(),
        }
    }

    /// Parses one token. `Name` arguments come back as their lowercase text; the
    /// console checks that they exist.
    pub fn parse(&self, text: &str) -> Result<Value, String> {
        let bad = || format!("expected {}, got {text:?}", self.describe());
        match self {
            ArgKind::Bool => parse_bool(text).map(Value::Bool).ok_or_else(bad),
            ArgKind::Int { min, max } => {
                let i: i64 = text.parse().map_err(|_| bad())?;
                (*min..=*max)
                    .contains(&i)
                    .then_some(Value::Int(i))
                    .ok_or_else(bad)
            }
            ArgKind::Float { min, max } => {
                let x: f64 = text.parse().map_err(|_| bad())?;
                (x.is_finite() && (*min..=*max).contains(&x))
                    .then_some(Value::Float(x))
                    .ok_or_else(bad)
            }
            ArgKind::Str => Ok(Value::Str(text.to_owned())),
            ArgKind::Choice(options) => {
                let lower = text.to_ascii_lowercase();
                options
                    .contains(&lower)
                    .then_some(Value::Str(lower))
                    .ok_or_else(bad)
            }
            ArgKind::Name(_) => Ok(Value::Str(text.to_ascii_lowercase())),
        }
    }

    /// Whether a value already fits this kind (defaults, values set from code).
    pub fn accepts(&self, value: &Value) -> bool {
        match (self, value) {
            (ArgKind::Bool, Value::Bool(_)) | (ArgKind::Str | ArgKind::Name(_), Value::Str(_)) => {
                true
            }
            (ArgKind::Int { min, max }, Value::Int(i)) => (*min..=*max).contains(i),
            (ArgKind::Float { min, max }, Value::Float(x)) => {
                x.is_finite() && (*min..=*max).contains(x)
            }
            (ArgKind::Choice(options), Value::Str(s)) => options.contains(s),
            _ => false,
        }
    }

    /// Value completions for this kind (names are completed by the console).
    pub(crate) fn value_candidates(&self) -> Vec<String> {
        match self {
            ArgKind::Bool => vec!["off".into(), "on".into()],
            ArgKind::Choice(options) => options.clone(),
            _ => Vec::new(),
        }
    }
}

/// Accepted spellings of a boolean, case-insensitive.
pub fn parse_bool(text: &str) -> Option<bool> {
    match text.to_ascii_lowercase().as_str() {
        "on" | "true" | "yes" | "1" => Some(true),
        "off" | "false" | "no" | "0" => Some(false),
        _ => None,
    }
}
