//! `MessageBox` text formatting and button selection (ui-adapter.md "MWScript UI calls").
//!
//! Status: written 2026-11-08 without compiling: UNTESTED (owner test pass 2026-11-10).
//!
//! [`format_message`] substitutes the argument values into a `MessageBox` format string exactly
//! as `OpenMW` does; [`message_box`] adds the button labels and returns the [`Message`] model
//! (text and buttons) the UI shows; [`message_box_from_args`] also applies `OpenMW`'s
//! compile-time split of a call's arguments into values and buttons. [`placeholders`] gives the
//! value kinds a format string asks for, which the script lowering needs at parse time. Nothing
//! here panics: every input yields a message, and oddities are counted in [`MessageReport`].
//!
//! # Provenance
//!
//! Ported from `OpenMW` (GPL-3.0), commit `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`
//! (github.com/OpenMW/openmw):
//! - `components/misc/messageformatparser.cpp` lines 7-19 (`parseNumber`) and 26-97
//!   (`MessageFormatParser::process`): the format-string scanner.
//! - `components/interpreter/miscopcodes.hpp` lines 17-121 (`RuntimeMessageFormatter`, which
//!   builds a `std::format` spec per placeholder), 123-131 (`formatMessage`) and 133-160
//!   (`OpMessageBox`).
//! - `components/compiler/lineparser.cpp` lines 145-180 (argument and button parsing), 461-473
//!   (the 10-button cap) and 501-519 (`GetArgumentsFromMessageFormat`: one typed argument per
//!   placeholder); `components/compiler/generator.cpp` lines 436-447 (float to integer
//!   conversion).
//! - Test vectors from `apps/openmw_tests/mwscript/testscripts.cpp` (`sScript5` lines 133-158
//!   with `mwscript_test_messagebox_format` lines 633-660; `sIssue8129` lines 512-520 with
//!   `mwscript_test_8129` lines 1031-1043).
//!
//! # Format strings (as `OpenMW`)
//!
//! - `%%` is a `%`. A `%` at the very end of the text is dropped.
//! - A placeholder is `%`, flags (`-` left-align, `+` sign, space, `0` zero-pad, `#` alternate
//!   form, any order and number), an optional width, an optional `.precision` (`.` alone is 0),
//!   then a conversion: `s`/`S` string, `d`/`i` integer, `f`/`F` fixed, `e`/`E` scientific,
//!   `g`/`G` general, `a`/`A` hexadecimal float. Each placeholder takes the next value.
//! - Any other conversion character is written as itself and the `%`, flags, width and
//!   precision before it are dropped (`%x` → `x`, `%5.2q` → `q`); it takes no value. A
//!   placeholder cut off by the end of the text (`%5`, `%.`) writes nothing.
//! - Numbers follow C++ `std::format` (not C `printf`): floats are `f32` (the interpreter's
//!   float slot), `%g` and `%f` default to 6 digits, `%g` drops trailing zeros unless `#`,
//!   exponents have a sign and at least two digits (`1e+07`), `%a` has no `0x` prefix
//!   (`1.8p+1`), `%F` prints like `%f` (lowercase `inf`/`nan`, `OpenMW` maps it to fixed).
//!   Rounding is to nearest, ties to even, on the exact binary value. Integers truncate
//!   toward zero (`OpenMW`'s float to integer conversion); precision is ignored on `%d`.
//! - `0` pads numbers with zeros after the sign, unless left-aligned or not finite. On a
//!   string, `0` makes `'0'` the fill character (`%05s` → `000ab`, `%-05s` → `ab000`, as
//!   `OpenMW`'s `{:0>N}`); `+`, space and `#` do nothing on strings; precision cuts a string to
//!   that many characters.
//! - Text defines (`^PCName`, `%PCName`) are not expanded here: `OpenMW` expands them after
//!   formatting (`Interpreter::fixDefinesMsgBox`, `miscopcodes.hpp` line 129), so the caller
//!   does it on [`Message::text`]. A `%PCName` in a `MessageBox` text is already eaten by the
//!   formatter (`%P` is an unknown conversion → `P`), as in `OpenMW`; vanilla uses `^PCName`.
//!
//! # Deviations (documented, deliberate)
//!
//! - Width and precision are capped at [`MAX_FIELD`] (`OpenMW` would allocate whatever the
//!   script asks for).
//! - Characters, not bytes: precision on `%s` counts `char`s and widths count `char`s
//!   (`OpenMW` counts bytes of the Windows-1252 text and `std::format`'s display width).
//! - Arity and type: `OpenMW` compiles exactly one argument per placeholder, converted to its
//!   type, so a missing or mistyped value cannot happen there. Here a missing value reads 0
//!   (or `""`) and a value of the wrong kind reads 0 (or `""`); both are counted.
//! - A NaN prints `nan` or `-nan` by its sign bit (`MSVC`'s `to_chars` may print `-nan(ind)`).
//! - A float above `i32::MAX` saturates on `%d` (C++ leaves it undefined).

use std::fmt::Write as _;

/// The most buttons a `MessageBox` keeps (`OpenMW` warns and drops the rest,
/// `lineparser.cpp` lines 463-467).
pub const MAX_BUTTONS: usize = 10;

/// The largest width or precision honoured; larger ones are clamped to this.
pub const MAX_FIELD: usize = 1024;

/// One argument of a `MessageBox` call after the format string, as the interpreter supplies it.
#[derive(Clone, Debug, PartialEq)]
pub enum MessageArg {
    /// A numeric value (a literal, variable or expression, already evaluated).
    Num(f64),
    /// A quoted string literal.
    Str(String),
    /// An unquoted word kept as written (`A`, `3.14pi`, `.todd`, a keyword): `OpenMW` reads it
    /// as text in a `%s` position and as a potential button label after the values.
    Word(String),
}

/// What kind of value a placeholder takes (`OpenMW`'s `Placeholder`; the compiler's argument
/// codes `S`, `l`, `f`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Placeholder {
    /// `%s`, `%S`: a string.
    Str,
    /// `%d`, `%i`: an integer.
    Int,
    /// `%f`, `%F`, `%e`, `%E`, `%g`, `%G`, `%a`, `%A`: a float.
    Float,
}

/// What formatting met that `OpenMW`'s compiler would have rejected or trimmed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MessageReport {
    /// Placeholders with no value left (read as 0 or `""`).
    pub missing_args: u32,
    /// Values of the wrong kind for their placeholder (read as 0 or `""`).
    pub type_mismatches: u32,
    /// Values beyond the last placeholder (ignored; [`format_message`] only).
    pub extra_args: u32,
    /// Buttons beyond [`MAX_BUTTONS`] (dropped).
    pub dropped_buttons: u32,
    /// Numeric values used as button labels ([`message_box_from_args`]; labelled by their
    /// shortest decimal form, since the source token is gone).
    pub numeric_buttons: u32,
}

/// A formatted `MessageBox`: what the UI shows.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Message {
    /// The text after substitution (text defines not yet expanded, see the module docs).
    pub text: String,
    /// Button labels in order; empty means a timed notification, not a modal.
    pub buttons: Vec<String>,
    /// What formatting met.
    pub report: MessageReport,
}

impl Message {
    /// Whether the message waits for a button (`GetButtonPressed`), i.e. has buttons.
    pub fn is_modal(&self) -> bool {
        !self.buttons.is_empty()
    }
}

/// Flag bits, as `OpenMW`'s `MessageFormatParser::Flags`.
const POSITIVE_SPACE: u8 = 1;
const POSITIVE_SIGN: u8 = 2;
const ALIGN_LEFT: u8 = 4;
const PREPEND_ZERO: u8 = 8;
const ALTERNATE_FORM: u8 = 16;

/// A float conversion (`OpenMW`'s `Notation`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Notation {
    Fixed,
    SciLower,
    SciUpper,
    GeneralLower,
    GeneralUpper,
    HexLower,
    HexUpper,
}

/// One parsed placeholder.
#[derive(Clone, Copy, Debug)]
struct Spec {
    kind: Placeholder,
    flags: u8,
    width: Option<usize>,
    precision: Option<usize>,
    notation: Notation,
}

impl Spec {
    fn has(&self, flag: u8) -> bool {
        self.flags & flag != 0
    }
}

/// What the scanner visits (`visitedCharacter` / `visitedPlaceholder`).
enum Piece {
    Char(char),
    Spec(Spec),
}

/// `OpenMW`'s `parseNumber`: reads the run of ASCII digits at `*i`. No digits, or a run that
/// overflows `int` (consumed all the same, as `std::from_chars` does), gives `None`; the
/// caller applies its fallback.
fn parse_number(m: &[char], i: &mut usize) -> Option<usize> {
    let start = *i;
    let mut value: i64 = 0;
    while let Some(digit) = m.get(*i).and_then(|ch| ch.to_digit(10)) {
        // Clamped just above `i32::MAX` so the accumulator never overflows.
        value = (value * 10 + i64::from(digit)).min(i64::from(i32::MAX) + 1);
        *i += 1;
    }
    if *i == start || value > i64::from(i32::MAX) {
        return None;
    }
    usize::try_from(value).ok()
}

/// `MessageFormatParser::process`: scans `format`, visiting characters and placeholders.
fn walk(format: &str, mut visit: impl FnMut(Piece)) {
    let m: Vec<char> = format.chars().collect();
    let mut i = 0;
    while i < m.len() {
        if m[i] != '%' {
            visit(Piece::Char(m[i]));
            i += 1;
            continue;
        }
        i += 1;
        if i == m.len() {
            break;
        }
        if m[i] == '%' {
            visit(Piece::Char('%'));
            i += 1;
            continue;
        }
        let mut flags = 0u8;
        while let Some(&c) = m.get(i) {
            let flag = match c {
                '-' => ALIGN_LEFT,
                '+' => POSITIVE_SIGN,
                ' ' => POSITIVE_SPACE,
                '0' => PREPEND_ZERO,
                '#' => ALTERNATE_FORM,
                _ => break,
            };
            flags |= flag;
            i += 1;
        }
        let width = parse_number(&m, &mut i).map(|width| width.min(MAX_FIELD));
        if i < m.len() {
            let mut precision = None;
            if m[i] == '.' {
                i += 1;
                precision = Some(parse_number(&m, &mut i).unwrap_or(0).min(MAX_FIELD));
            }
            if let Some(&c) = m.get(i) {
                let spec = |kind: Placeholder, notation: Notation| {
                    Piece::Spec(Spec {
                        kind,
                        flags,
                        width,
                        precision,
                        notation,
                    })
                };
                visit(match c {
                    's' | 'S' => spec(Placeholder::Str, Notation::Fixed),
                    'd' | 'i' => spec(Placeholder::Int, Notation::Fixed),
                    'f' | 'F' => spec(Placeholder::Float, Notation::Fixed),
                    'e' => spec(Placeholder::Float, Notation::SciLower),
                    'E' => spec(Placeholder::Float, Notation::SciUpper),
                    'g' => spec(Placeholder::Float, Notation::GeneralLower),
                    'G' => spec(Placeholder::Float, Notation::GeneralUpper),
                    'a' => spec(Placeholder::Float, Notation::HexLower),
                    'A' => spec(Placeholder::Float, Notation::HexUpper),
                    other => Piece::Char(other),
                });
            }
        }
        // Past the conversion character (or past the end, which ends the loop).
        i += 1;
    }
}

/// The value kinds `format` asks for, in order (`GetArgumentsFromMessageFormat`): the number
/// of arguments `OpenMW` compiles before the buttons, and the type each is converted to.
pub fn placeholders(format: &str) -> Vec<Placeholder> {
    let mut kinds = Vec::new();
    walk(format, |piece| {
        if let Piece::Spec(spec) = piece {
            kinds.push(spec.kind);
        }
    });
    kinds
}

/// Substitutes `values` into `format` (`RuntimeMessageFormatter`). Values are taken in order,
/// one per placeholder; see the module docs for the rules.
pub fn format_message(format: &str, values: &[MessageArg]) -> (String, MessageReport) {
    let mut report = MessageReport::default();
    let mut out = String::with_capacity(format.len());
    let mut values = values.iter();
    walk(format, |piece| match piece {
        Piece::Char(c) => out.push(c),
        Piece::Spec(spec) => {
            let value = values.next();
            if value.is_none() {
                report.missing_args += 1;
            }
            match spec.kind {
                Placeholder::Str => {
                    let text = match value {
                        Some(MessageArg::Str(s) | MessageArg::Word(s)) => s.as_str(),
                        Some(MessageArg::Num(_)) => {
                            report.type_mismatches += 1;
                            ""
                        }
                        None => "",
                    };
                    push_string(&mut out, text, &spec);
                }
                Placeholder::Int => {
                    let v = match value {
                        Some(MessageArg::Num(v)) => to_i32(*v),
                        Some(_) => {
                            report.type_mismatches += 1;
                            0
                        }
                        None => 0,
                    };
                    push_int(&mut out, v, &spec);
                }
                Placeholder::Float => {
                    let v = match value {
                        Some(MessageArg::Num(v)) => to_f32(*v),
                        Some(_) => {
                            report.type_mismatches += 1;
                            0.0
                        }
                        None => 0.0,
                    };
                    push_float(&mut out, v, &spec);
                }
            }
        }
    });
    report.extra_args = u32::try_from(values.len()).unwrap_or(u32::MAX);
    (out, report)
}

/// A `MessageBox` from its format string, values and button labels (`OpMessageBox`): the
/// formatted text and the first [`MAX_BUTTONS`] buttons.
pub fn message_box(format: &str, values: &[MessageArg], buttons: &[String]) -> Message {
    let (text, mut report) = format_message(format, values);
    let kept = buttons.len().min(MAX_BUTTONS);
    report.dropped_buttons = u32::try_from(buttons.len() - kept).unwrap_or(u32::MAX);
    Message {
        text,
        buttons: buttons[..kept].to_vec(),
        report,
    }
}

/// A `MessageBox` from all of a call's arguments after the format string, split as `OpenMW`'s
/// compiler does (`lineparser.cpp` lines 145-180 and 461-473): the first
/// `placeholders(format).len()` arguments are the values; the rest are potential buttons. The
/// number of buttons is the number of quoted ones ([`MessageArg::Str`]) among them, capped at
/// [`MAX_BUTTONS`], and the buttons are that many potential buttons from the front, quoted or
/// not (`A B "C" D "E"` → `A`, `B`). A [`MessageArg::Num`] there is labelled by its shortest
/// decimal form and counted in [`MessageReport::numeric_buttons`].
pub fn message_box_from_args(format: &str, args: &[MessageArg]) -> Message {
    let split = placeholders(format).len().min(args.len());
    let (values, rest) = args.split_at(split);
    let quoted = rest
        .iter()
        .filter(|a| matches!(a, MessageArg::Str(_)))
        .count();
    let mut numeric = 0u32;
    let potential: Vec<String> = rest
        .iter()
        .take(quoted)
        .map(|a| match a {
            MessageArg::Str(s) | MessageArg::Word(s) => s.clone(),
            MessageArg::Num(v) => {
                numeric += 1;
                v.to_string()
            }
        })
        .collect();
    let mut message = message_box(format, values, &potential);
    message.report.numeric_buttons = numeric;
    message
}

/// An `f64` interpreter value in `OpenMW`'s `f32` float slot.
#[allow(clippy::cast_possible_truncation)] // Rounds to the nearest f32: OpenMW's float type.
fn to_f32(v: f64) -> f32 {
    v as f32
}

/// An `f64` interpreter value as an integer: truncated toward zero, NaN → 0, saturating.
#[allow(clippy::cast_possible_truncation)] // Truncation is the point (OpenMW's float to int).
fn to_i32(v: f64) -> i32 {
    v as i32
}

/// Appends `n` copies of `c`.
fn push_repeat(out: &mut String, c: char, n: usize) {
    for _ in 0..n {
        out.push(c);
    }
}

/// The sign prefix for a number: `-` when negative, else `+` / space by the flags.
fn sign(negative: bool, spec: &Spec) -> &'static str {
    if negative {
        "-"
    } else if spec.has(POSITIVE_SIGN) {
        "+"
    } else if spec.has(POSITIVE_SPACE) {
        " "
    } else {
        ""
    }
}

/// Appends a number padded to the width: left-aligned with spaces, zero-padded after the sign
/// (when `zero_ok`, i.e. finite), or right-aligned with spaces.
fn push_number(out: &mut String, sign: &str, body: &str, spec: &Spec, zero_ok: bool) {
    let len = sign.chars().count() + body.chars().count();
    let pad = spec.width.unwrap_or(0).saturating_sub(len);
    if spec.has(ALIGN_LEFT) {
        out.push_str(sign);
        out.push_str(body);
        push_repeat(out, ' ', pad);
    } else if spec.has(PREPEND_ZERO) && zero_ok {
        out.push_str(sign);
        push_repeat(out, '0', pad);
        out.push_str(body);
    } else {
        push_repeat(out, ' ', pad);
        out.push_str(sign);
        out.push_str(body);
    }
}

/// `%s`: cut to the precision, then padded with `'0'` (zero flag) or spaces.
fn push_string(out: &mut String, value: &str, spec: &Spec) {
    let value: String = match spec.precision {
        Some(p) => value.chars().take(p).collect(),
        None => value.to_owned(),
    };
    let Some(width) = spec.width else {
        out.push_str(&value);
        return;
    };
    let fill = if spec.has(PREPEND_ZERO) { '0' } else { ' ' };
    let pad = width.saturating_sub(value.chars().count());
    if spec.has(ALIGN_LEFT) {
        out.push_str(&value);
        push_repeat(out, fill, pad);
    } else {
        push_repeat(out, fill, pad);
        out.push_str(&value);
    }
}

/// `%d`: decimal, precision ignored (`OpenMW` resets it), `#` has no effect.
fn push_int(out: &mut String, v: i32, spec: &Spec) {
    let body = v.unsigned_abs().to_string();
    push_number(out, sign(v < 0, spec), &body, spec, true);
}

/// The float conversions (`std::format` on an `f32`).
fn push_float(out: &mut String, v: f32, spec: &Spec) {
    let alt = spec.has(ALTERNATE_FORM);
    let body = if v.is_nan() {
        "nan".to_owned()
    } else if v.is_infinite() {
        "inf".to_owned()
    } else {
        let a = v.abs();
        match spec.notation {
            Notation::Fixed => fixed(a, spec.precision.unwrap_or(6), alt),
            Notation::SciLower | Notation::SciUpper => {
                scientific(a, spec.precision.unwrap_or(6), alt)
            }
            Notation::GeneralLower | Notation::GeneralUpper => {
                general(a, spec.precision.unwrap_or(6), alt)
            }
            Notation::HexLower | Notation::HexUpper => hex(a, spec.precision, alt),
        }
    };
    let upper = matches!(
        spec.notation,
        Notation::SciUpper | Notation::GeneralUpper | Notation::HexUpper
    );
    let body = if upper {
        body.to_ascii_uppercase()
    } else {
        body
    };
    push_number(out, sign(v.is_sign_negative(), spec), &body, spec, v.is_finite());
}

/// Fixed notation of a non-negative finite value with `p` decimals; `#` keeps the point.
fn fixed(a: f32, p: usize, alt: bool) -> String {
    let mut s = format!("{a:.p$}");
    if alt && p == 0 {
        s.push('.');
    }
    s
}

/// Splits Rust's `{:e}` output (`1.5e3`, `1.5e-3`) into mantissa and exponent.
fn split_exponent(s: &str) -> (&str, i32) {
    match s.split_once('e') {
        Some((mantissa, exp)) => (mantissa, exp.parse().unwrap_or(0)),
        None => (s, 0),
    }
}

/// Appends a C-style exponent: `e`, a sign, at least two digits.
fn push_exponent(out: &mut String, exp: i32) {
    let sign = if exp < 0 { '-' } else { '+' };
    let _ = write!(out, "e{sign}{:02}", exp.unsigned_abs());
}

/// Scientific notation with `p` decimals (`1.234500e+03`); `#` keeps the point.
fn scientific(a: f32, p: usize, alt: bool) -> String {
    let s = format!("{a:.p$e}");
    let (mantissa, exp) = split_exponent(&s);
    let mut out = mantissa.to_owned();
    if alt && p == 0 {
        out.push('.');
    }
    push_exponent(&mut out, exp);
    out
}

/// General notation with `precision` significant digits (0 counts as 1): fixed when the
/// rounded exponent X satisfies -4 <= X < P, else scientific; trailing zeros (and a bare
/// point) dropped unless `#`, which also keeps the point.
fn general(value: f32, precision: usize, alt: bool) -> String {
    let digits = precision.max(1);
    let probe = format!("{value:.prec$e}", prec = digits - 1);
    let (_, rounded_exp) = split_exponent(&probe);
    let digits_i = i32::try_from(digits).unwrap_or(i32::MAX);
    let text = if (-4..digits_i).contains(&rounded_exp) {
        let decimals = usize::try_from(digits_i - 1 - rounded_exp).unwrap_or(0);
        format!("{value:.decimals$}")
    } else {
        scientific(value, digits - 1, false)
    };
    let (num, exp) = text
        .find('e')
        .map_or((text.as_str(), ""), |at| text.split_at(at));
    let num = if alt {
        if num.contains('.') {
            num.to_owned()
        } else {
            format!("{num}.")
        }
    } else if num.contains('.') {
        num.trim_end_matches('0').trim_end_matches('.').to_owned()
    } else {
        num.to_owned()
    };
    format!("{num}{exp}")
}

/// Hexadecimal float of a non-negative finite `f32` as `std::to_chars(.., chars_format::hex)`:
/// no `0x` prefix, `1.xxxxxxp±d` (subnormals `0.xxxxxxp-126`, zero `0p+0`); shortest digits
/// without a precision, else rounded (ties to even) or zero-extended to `precision` digits.
fn hex(value: f32, precision: Option<usize>, alt: bool) -> String {
    let bits = value.to_bits();
    let exp_bits = (bits >> 23) & 0xff;
    // The 23 fraction bits shifted to 24: six hex digits.
    let mantissa = (bits & 0x7f_ffff) << 1;
    let (mut lead, exp) = if bits == 0 {
        (0u32, 0i32)
    } else if exp_bits == 0 {
        (0, -126)
    } else {
        (1, i32::try_from(exp_bits).unwrap_or(0) - 127)
    };
    let digits = match precision {
        None => format!("{mantissa:06x}").trim_end_matches('0').to_owned(),
        Some(wanted) if wanted >= 6 => {
            let mut padded = format!("{mantissa:06x}");
            push_repeat(&mut padded, '0', wanted - 6);
            padded
        }
        Some(wanted) => {
            // Lead digit and fraction rounded together, so a carry (`1.f8p+0` at `.1` →
            // `2.0p+0`) and the lead's parity at precision 0 come out right.
            let full = (lead << 24) | mantissa;
            let shift = 4 * (6 - wanted);
            let mut rounded = full >> shift;
            let rem = full & ((1u32 << shift) - 1);
            let half = 1u32 << (shift - 1);
            if rem > half || (rem == half && rounded & 1 == 1) {
                rounded += 1;
            }
            lead = rounded >> (4 * wanted);
            let frac = rounded & ((1u32 << (4 * wanted)) - 1);
            if wanted == 0 {
                String::new()
            } else {
                format!("{frac:0wanted$x}")
            }
        }
    };
    let mut out = lead.to_string();
    if !digits.is_empty() || alt {
        out.push('.');
    }
    out.push_str(&digits);
    let sign = if exp < 0 { '-' } else { '+' };
    let _ = write!(out, "p{sign}{}", exp.unsigned_abs());
    out
}
