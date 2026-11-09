//! `mw_ui::message`: `OpenMW`'s own test vectors (`apps/openmw_tests/mwscript/testscripts.cpp`
//! at `ca5e2576`, `mwscript_test_messagebox_format` and `mwscript_test_8129`), then the edge
//! cases of `MessageFormatParser::process` and `std::format` on an `f32`.
//!
//! Status: written 2026-11-08 without compiling: UNTESTED (owner test pass 2026-11-10).

use crate::message::{
    MAX_BUTTONS, MAX_FIELD, Message, MessageArg, MessageReport, Placeholder, format_message,
    message_box, message_box_from_args, placeholders,
};

fn n(v: f64) -> MessageArg {
    MessageArg::Num(v)
}

fn s(text: &str) -> MessageArg {
    MessageArg::Str(text.to_owned())
}

fn w(text: &str) -> MessageArg {
    MessageArg::Word(text.to_owned())
}

/// The formatted text only.
fn fmt(format: &str, values: &[MessageArg]) -> String {
    format_message(format, values).0
}

/// One number through a format.
fn one(format: &str, v: f64) -> String {
    fmt(format, &[n(v)])
}

fn labels(message: &Message) -> Vec<&str> {
    message.buttons.iter().map(String::as_str).collect()
}

#[test]
fn openmw_messagebox_format_vectors() {
    // `set fVal to 12.34` (a float local: f32), then `MessageBox "<format>" fVal`.
    let cases = [
        ("%.0f", "12"),
        ("%.f", "12"),
        ("a %03.0f b", "a 012 b"),
        ("%+04.0f", "+012"),
        ("%+4.0f", " +12"),
        ("%+ 4.0f", " +12"),
        ("%0+ 4.0f", "+012"),
        ("%0+ #4.0f", "+12."),
        ("%- 5.0f", " 12  "),
        ("%g", "12.34"),
        ("%.3g", "12.3"),
        ("%.5g", "12.34"),
        ("%#.5g", "12.340"),
        ("%-5g", "12.34"),
        ("%- 5g", " 12.34"),
        ("%.1b", "b"),
    ];
    for (format, expected) in cases {
        assert_eq!(one(format, 12.34), expected, "{format}");
    }
    assert_eq!(fmt("hello world", &[]), "hello world");
}

#[test]
fn openmw_issue_8129_buttons() {
    let m = message_box_from_args("must include all buttons", &[s("A"), s("B"), s("C")]);
    assert_eq!(m.text, "must include all buttons");
    assert_eq!(labels(&m), ["A", "B", "C"]);
    assert!(m.is_modal());

    let m = message_box_from_args("must ignore all buttons", &[w("A"), w("B"), w("C")]);
    assert!(m.buttons.is_empty());
    assert!(!m.is_modal());

    let m = message_box_from_args(
        "the number of buttons must match the number of quoted arguments (two)",
        &[w("A"), w("B"), s("C"), w("D"), s("E")],
    );
    assert_eq!(labels(&m), ["A", "B"]);

    let m = message_box_from_args(
        "use strings that start with numbers, - and .",
        &[
            w("1"),
            w("-2"),
            w("3.14pi"),
            w(".todd"),
            s("A"),
            s("B"),
            s("C"),
            s("D"),
        ],
    );
    assert_eq!(labels(&m), ["1", "-2", "3.14pi", ".todd"]);

    let m = message_box_from_args(
        "use keywords",
        &[w("messagebox"), w("set"), s("messagebox2"), s("set2")],
    );
    assert_eq!(labels(&m), ["messagebox", "set"]);

    let eleven: Vec<MessageArg> = (1..=11).map(|i| s(&i.to_string())).collect();
    let m = message_box_from_args("don't use more than 10 buttons", &eleven);
    assert_eq!(labels(&m), ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]);
    assert_eq!(m.report.dropped_buttons, 1);
}

#[test]
fn values_come_before_buttons() {
    let m = message_box_from_args(
        "Hi %s, you have %g gold.",
        &[s("Bob"), n(5.5), s("Yes"), s("No")],
    );
    assert_eq!(
        m,
        Message {
            text: "Hi Bob, you have 5.5 gold.".into(),
            buttons: vec!["Yes".into(), "No".into()],
            report: MessageReport::default(),
        }
    );
    // A `%s` takes a quoted string as its value, so it is not a button.
    let m = message_box_from_args("Go %s?", &[s("north")]);
    assert_eq!((m.text.as_str(), m.buttons.len()), ("Go north?", 0));
    // A bare word in a `%s` position is its own text (`OpenMW`'s string argument parser).
    assert_eq!(message_box_from_args("%s", &[w("Fargoth")]).text, "Fargoth");
    // Numbers in button positions keep a label and are counted.
    let m = message_box_from_args("Pick", &[n(1.0), n(-2.0), s("A"), s("B")]);
    assert_eq!(labels(&m), ["1", "-2"]);
    assert_eq!(m.report.numeric_buttons, 2);
}

#[test]
fn message_box_caps_buttons() {
    let buttons: Vec<String> = (0..12).map(|i| format!("b{i}")).collect();
    let m = message_box("Choose", &[], &buttons);
    assert_eq!(m.buttons.len(), MAX_BUTTONS);
    assert_eq!(m.buttons[9], "b9");
    assert_eq!(m.report.dropped_buttons, 2);
    let m = message_box("%g%%", &[n(50.0)], &["Ok".to_owned()]);
    assert_eq!((m.text.as_str(), labels(&m)), ("50%", vec!["Ok"]));
}

#[test]
fn placeholder_kinds() {
    use Placeholder::{Float, Int, Str};
    assert_eq!(
        placeholders("%g %s %d %i %f %F %e %E %G %a %A %S %% %x %5.1q %-08.3f %"),
        [
            Float, Str, Int, Int, Float, Float, Float, Float, Float, Float, Float, Str, Float
        ]
    );
    assert!(placeholders("no values, 100%% sure").is_empty());
}

#[test]
fn percent_and_unknown_conversions() {
    assert_eq!(fmt("100%%", &[]), "100%");
    assert_eq!(fmt("100%", &[]), "100", "a trailing % is dropped");
    assert_eq!(fmt("a%5", &[]), "a", "cut off after the width");
    assert_eq!(fmt("a%.", &[]), "a", "cut off after the point");
    assert_eq!(fmt("a%-", &[]), "a", "cut off after a flag");
    assert_eq!(fmt("%x", &[]), "x");
    assert_eq!(fmt("%5.2x|%l|%-%", &[]), "x|l|%");
    assert_eq!(fmt("%é", &[]), "é");
    assert_eq!(fmt("%PCName", &[]), "PCName", "%P is unknown: only P stays");
    // An unknown conversion takes no value.
    let (text, report) = format_message("%x %g", &[n(1.0)]);
    assert_eq!(text, "x 1");
    assert_eq!(report, MessageReport::default());
}

#[test]
fn general_notation() {
    assert_eq!(one("%g", 0.5), "0.5");
    assert_eq!(one("%g", 1e7), "1e+07");
    assert_eq!(one("%g", 123_456.7), "123457");
    assert_eq!(one("%g", 100_000.0), "100000");
    assert_eq!(one("%g", 1_000_000.0), "1e+06");
    assert_eq!(one("%g", 0.0001), "0.0001");
    assert_eq!(one("%G", 0.000_01), "1E-05");
    assert_eq!(one("%g", 12.345), "12.345", "f32 12.345000267 at 6 digits");
    assert_eq!(one("%g", 0.0), "0");
    assert_eq!(one("%g", -0.0), "-0");
    assert_eq!(one("%+g", 0.0), "+0");
    assert_eq!(one("%g", -2.5), "-2.5");
    assert_eq!(one("%.3g", 1.23456), "1.23");
    assert_eq!(one("%.0g", 3.7), "4", "precision 0 counts as 1");
    assert_eq!(one("%#g", 1.0), "1.00000");
    assert_eq!(one("%#.1g", 1.0), "1.");
    assert_eq!(one("%g", f64::from(f32::MAX)), "3.40282e+38");
}

#[test]
fn fixed_notation() {
    assert_eq!(one("%f", 1.5), "1.500000");
    assert_eq!(one("%f", 0.1), "0.100000");
    assert_eq!(one("%.10f", 0.1), "0.1000000015", "the value is an f32");
    assert_eq!(one("%.1f", 12.345), "12.3");
    assert_eq!(one("%.f", 2.7), "3");
    assert_eq!(one("%.05f", 1.0), "1.00000", "leading zeros in the precision");
    assert_eq!(one("%#.0f", 3.0), "3.");
    assert_eq!(one("%F", 2.0), "2.000000");
    assert_eq!(one("%8.2f", 5.678), "    5.68");
    assert_eq!(fmt("%-8.2f|", &[n(5.678)]), "5.68    |");
    assert_eq!(one("%08.2f", -5.678), "-0005.68");
    assert_eq!(one("%+.1f", 2.0), "+2.0");
    assert_eq!(one("% .1f", 2.0), " 2.0");
    assert_eq!(
        fmt("You have %g gems, %.1f%% done.", &[n(3.0), n(12.345)]),
        "You have 3 gems, 12.3% done."
    );
}

#[test]
fn rounding_ties_to_even() {
    // Exact binary ties: `std::to_chars` rounds them to even, as glibc's printf does.
    assert_eq!(one("%.0f", 0.5), "0");
    assert_eq!(one("%.0f", 1.5), "2");
    assert_eq!(one("%.0f", 2.5), "2");
    assert_eq!(one("%.0f", 3.5), "4");
    assert_eq!(one("%.1f", 0.25), "0.2");
    assert_eq!(one("%.2f", 0.125), "0.12");
    assert_eq!(one("%.0e", 2.5), "2e+00");
}

#[test]
fn scientific_notation() {
    assert_eq!(one("%e", 1234.5), "1.234500e+03");
    assert_eq!(one("%E", 0.000_12), "1.200000E-04");
    assert_eq!(one("%e", 0.0), "0.000000e+00");
    assert_eq!(one("%.0e", 5.0), "5e+00");
    assert_eq!(one("%#.0e", 5.0), "5.e+00");
    assert_eq!(one("%.2e", -1e-20), "-1.00e-20");
    assert_eq!(one("%12.3e", 1e30), "   1.000e+30");
}

#[test]
fn hex_notation() {
    assert_eq!(one("%a", 1.0), "1p+0");
    assert_eq!(one("%a", 0.5), "1p-1");
    assert_eq!(one("%a", 3.0), "1.8p+1");
    assert_eq!(one("%a", -2.0), "-1p+1");
    assert_eq!(one("%a", 0.0), "0p+0");
    assert_eq!(one("%A", 0.1), "1.99999AP-4");
    assert_eq!(one("%.2a", 0.1), "1.9ap-4");
    assert_eq!(one("%.1a", 1.0), "1.0p+0");
    assert_eq!(one("%.8a", 1.0), "1.00000000p+0");
    assert_eq!(one("%#a", 1.0), "1.p+0");
    assert_eq!(
        one("%a", f64::from(f32::from_bits(1))),
        "0.000002p-126",
        "smallest subnormal"
    );
}

#[test]
fn non_finite_values() {
    assert_eq!(one("%f", f64::INFINITY), "inf");
    assert_eq!(one("%F", f64::INFINITY), "inf", "%F is fixed: lowercase");
    assert_eq!(one("%G", f64::NEG_INFINITY), "-INF");
    assert_eq!(one("%e", f64::INFINITY), "inf");
    assert_eq!(one("%f", f64::NAN), "nan");
    assert_eq!(one("%05f", f64::INFINITY), "  inf", "no zero padding");
    assert_eq!(fmt("%-6g|", &[n(f64::INFINITY)]), "inf   |");
    assert_eq!(one("%g", 1e300), "inf", "beyond f32");
}

#[test]
fn integers() {
    assert_eq!(one("%d", 7.9), "7");
    assert_eq!(one("%d", -7.9), "-7");
    assert_eq!(one("%i", 3.0), "3");
    assert_eq!(one("%5d", 42.0), "   42");
    assert_eq!(fmt("%-5d|", &[n(42.0)]), "42   |");
    assert_eq!(one("%05d", -42.0), "-0042");
    assert_eq!(fmt("%-05d|", &[n(42.0)]), "42   |", "0 is ignored when left-aligned");
    assert_eq!(one("%+d", 5.0), "+5");
    assert_eq!(one("% d", 5.0), " 5");
    assert_eq!(one("%+ d", 5.0), "+5", "+ wins over space");
    assert_eq!(one("%.3d", 5.0), "5", "precision is ignored");
    assert_eq!(one("%#d", 5.0), "5");
    assert_eq!(one("%d", 3e10), "2147483647", "saturates");
    assert_eq!(one("%d", f64::NAN), "0");
}

#[test]
fn strings() {
    let bob = [s("Bob")];
    assert_eq!(fmt("%s", &bob), "Bob");
    assert_eq!(fmt("%S", &bob), "Bob");
    assert_eq!(fmt("%.2s", &bob), "Bo");
    assert_eq!(fmt("%.0s|", &bob), "|");
    assert_eq!(fmt("%5s", &[s("ab")]), "   ab");
    assert_eq!(fmt("%-5s|", &[s("ab")]), "ab   |");
    assert_eq!(fmt("%05s", &[s("ab")]), "000ab", "0 is the fill character");
    assert_eq!(fmt("%-05s", &[s("ab")]), "ab000");
    assert_eq!(fmt("%+ #s", &[s("ab")]), "ab", "sign and # do nothing");
    assert_eq!(fmt("%1s", &[s("long")]), "long", "never cut by the width");
    assert_eq!(fmt("%.3s", &[s("Ébène")]), "Ébè", "precision counts chars");
}

#[test]
fn missing_mistyped_and_extra_values() {
    let (text, report) = format_message("%g|%s|%d", &[]);
    assert_eq!(text, "0||0");
    assert_eq!(report.missing_args, 3);
    let (text, report) = format_message("%g|%s|%d", &[s("a"), n(1.0), w("b")]);
    assert_eq!(text, "0||0");
    assert_eq!(report.type_mismatches, 3);
    assert_eq!(report.missing_args, 0);
    let (text, report) = format_message("Hi", &[n(1.0), s("x")]);
    assert_eq!(text, "Hi");
    assert_eq!(report.extra_args, 2);
    // `message_box_from_args` with too few arguments: all go to values, no buttons.
    let m = message_box_from_args("%g and %g", &[n(1.0)]);
    assert_eq!(m.text, "1 and 0");
    assert_eq!(m.report.missing_args, 1);
}

#[test]
fn field_limits() {
    assert_eq!(one("%2000d", 5.0).len(), MAX_FIELD, "width clamped");
    assert_eq!(one("%99999999999d", 5.0), "5", "an overflowing width is dropped");
    assert_eq!(
        one("%.2000f", 1.0).len(),
        2 + MAX_FIELD,
        "precision clamped"
    );
    assert_eq!(one("%.99999999999f", 1.5), "2", "an overflowing precision reads 0");
}
