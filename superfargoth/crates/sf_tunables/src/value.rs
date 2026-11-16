//! Typed values and kinds: parsing, validation, completion candidates. Shared by tunables and
//! console arguments (`sf_console` re-exports them).

use std::fmt;

/// A tunable's value or a parsed console argument.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Float(f64),
    /// Free text, or the canonical (lowercase) spelling of a choice.
    Str(String),
    /// A fixed pair (a fade start and end). Written `a,b`.
    Float2([f64; 2]),
    /// A fixed triple (a colour, an offset). Written `a,b,c`.
    Float3([f64; 3]),
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

    pub fn as_float2(&self) -> Option<[f64; 2]> {
        match self {
            Value::Float2(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_float3(&self) -> Option<[f64; 3]> {
        match self {
            Value::Float3(v) => Some(*v),
            _ => None,
        }
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::Int(i)
    }
}

impl From<f64> for Value {
    fn from(x: f64) -> Self {
        Value::Float(x)
    }
}

impl From<f32> for Value {
    fn from(x: f32) -> Self {
        Value::Float(f64::from(x))
    }
}

impl From<[f64; 2]> for Value {
    fn from(v: [f64; 2]) -> Self {
        Value::Float2(v)
    }
}

impl From<[f64; 3]> for Value {
    fn from(v: [f64; 3]) -> Self {
        Value::Float3(v)
    }
}

/// Text, or a choice (choices are lowercase).
impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::Str(s.to_owned())
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
            Value::Float2([a, b]) => write!(f, "{a},{b}"),
            Value::Float3([a, b, c]) => write!(f, "{a},{b},{c}"),
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

/// The type of a tunable or a console argument.
#[derive(Clone, Debug, PartialEq)]
pub enum ArgKind {
    /// `on`/`off`, `true`/`false`, `yes`/`no`, `1`/`0`.
    Bool,
    /// Integer in `min..=max`.
    Int { min: i64, max: i64 },
    /// Finite float in `min..=max`.
    Float { min: f64, max: f64 },
    /// Two finite floats, each in `min..=max`, written `a,b`.
    Float2 { min: f64, max: f64 },
    /// Three finite floats, each in `min..=max`, written `a,b,c`.
    Float3 { min: f64, max: f64 },
    /// Free text (quote it to include spaces).
    Str,
    /// One of a fixed set of lowercase words.
    Choice(Vec<String>),
    /// The name of a command or variable (console arguments only).
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
        let floats = |n: usize, min: f64, max: f64| {
            let names = vec!["float"; n].join(",");
            if min.is_infinite() && max.is_infinite() {
                names
            } else {
                format!("{names} {min}..={max}")
            }
        };
        match self {
            ArgKind::Bool => "on|off".into(),
            ArgKind::Int {
                min: i64::MIN,
                max: i64::MAX,
            } => "int".into(),
            ArgKind::Int { min, max } => format!("int {min}..={max}"),
            ArgKind::Float { min, max } => floats(1, *min, *max),
            ArgKind::Float2 { min, max } => floats(2, *min, *max),
            ArgKind::Float3 { min, max } => floats(3, *min, *max),
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
            ArgKind::Float2 { min, max } => parse_floats::<2>(text, *min, *max)
                .map(Value::Float2)
                .ok_or_else(bad),
            ArgKind::Float3 { min, max } => parse_floats::<3>(text, *min, *max)
                .map(Value::Float3)
                .ok_or_else(bad),
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
            (ArgKind::Float { min, max }, Value::Float(x)) => in_range(&[*x], *min, *max),
            (ArgKind::Float2 { min, max }, Value::Float2(v)) => in_range(v, *min, *max),
            (ArgKind::Float3 { min, max }, Value::Float3(v)) => in_range(v, *min, *max),
            (ArgKind::Choice(options), Value::Str(s)) => options.contains(s),
            _ => false,
        }
    }

    /// The numeric bounds, if the kind has any (`Int` widened to float).
    #[allow(clippy::cast_precision_loss)]
    pub fn bounds(&self) -> Option<(f64, f64)> {
        match self {
            ArgKind::Int { min, max } => Some((*min as f64, *max as f64)),
            ArgKind::Float { min, max }
            | ArgKind::Float2 { min, max }
            | ArgKind::Float3 { min, max } => Some((*min, *max)),
            _ => None,
        }
    }

    /// Value completions for this kind (names are completed by the console).
    pub fn value_candidates(&self) -> Vec<String> {
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

fn in_range(v: &[f64], min: f64, max: f64) -> bool {
    v.iter().all(|x| x.is_finite() && (min..=max).contains(x))
}

/// `a,b[,c]`: exactly `N` finite floats, each in `min..=max`.
fn parse_floats<const N: usize>(text: &str, min: f64, max: f64) -> Option<[f64; N]> {
    let mut out = [0.0; N];
    let mut parts = text.split(',');
    for slot in &mut out {
        *slot = parts.next()?.trim().parse().ok()?;
    }
    (parts.next().is_none() && in_range(&out, min, max)).then_some(out)
}
