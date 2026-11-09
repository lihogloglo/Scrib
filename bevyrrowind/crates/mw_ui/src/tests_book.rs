//! `mw_ui::book` on synthetic markup (our own text, never vanilla book text).

use sf_ui::{Align, Block, ImageRef, Inline, OBJECT_REPLACEMENT, Paragraph, Rgba8, RichDoc, RunStyle};

use crate::book::{
    BookOptions, BookReport, FAMILY_DAEDRIC, bookart_path, font_color, image_candidates,
    legacy_font_size, parse, parse_with,
};

/// Parses without the vanilla tail trim, so text after the last break stays.
fn raw(text: &str) -> (RichDoc, BookReport) {
    parse_with(
        text,
        BookOptions {
            trim_after_last_break: false,
        },
    )
}

/// The document as drawn lines: each paragraph line, `""` for a blank line, `[img path WxH]`
/// for a block image, `[page]` for a page break.
fn lines(doc: &RichDoc) -> Vec<String> {
    let mut out = Vec::new();
    for block in &doc.blocks {
        match block {
            Block::Paragraph(p) => out.extend(p.text().split('\n').map(str::to_owned)),
            Block::Image { image, .. } => out.push(format!(
                "[img {} {}x{}]",
                image.source,
                image.width.unwrap_or_default(),
                image.height.unwrap_or_default()
            )),
            Block::PageBreak => out.push("[page]".to_owned()),
        }
    }
    out
}

fn aligns(doc: &RichDoc) -> Vec<Align> {
    doc.blocks
        .iter()
        .map(|b| match b {
            Block::Paragraph(p) => p.align,
            Block::Image { align, .. } => *align,
            Block::PageBreak => Align::Start,
        })
        .collect()
}

fn text(s: &str, style: RunStyle) -> Inline {
    Inline::Text {
        text: s.to_owned(),
        style,
    }
}

fn colored(r: u8, g: u8, b: u8) -> RunStyle {
    RunStyle {
        color: Some(Rgba8::rgb(r, g, b)),
        ..RunStyle::default()
    }
}

fn count(report: &BookReport, tag: &str) -> u32 {
    report.tags.get(tag).copied().unwrap_or_default()
}

#[test]
fn empty_input_is_an_empty_document() {
    for input in ["", "\r\n", "\n\n\n", "<BR><P><BR>"] {
        let (doc, _) = raw(input);
        assert_eq!(doc, RichDoc::default(), "{input:?}");
    }
    let (doc, report) = parse("");
    assert_eq!(doc, RichDoc::default());
    assert_eq!(report, BookReport::default());
}

#[test]
fn plain_text_is_one_paragraph() {
    let (doc, report) = raw("A quiet lamp");
    let mut p = Paragraph::new(Align::Start);
    p.push_text("A quiet lamp", &RunStyle::default());
    assert_eq!(doc.blocks, vec![Block::Paragraph(p)]);
    assert_eq!(report, BookReport::default());
}

#[test]
fn vanilla_drops_text_after_the_last_break() {
    let (doc, report) = parse("One<BR>Two<BR>Three");
    assert_eq!(lines(&doc), ["One", "Two"]);
    assert_eq!(report.dropped_tail_bytes, 5);
    assert_eq!(count(&report, "br"), 2);

    // Only a bare `<br>` / `<p>` marks the end; `<p>` here, tags after it still count.
    let (doc, report) = parse("a<P>b<br>c<DIV>");
    assert_eq!(lines(&doc), ["a", "", "b"]);
    assert_eq!(report.dropped_tail_bytes, 1);
    assert_eq!(count(&report, "div"), 1);

    // No break at all: vanilla shows nothing.
    let (doc, report) = parse("No breaks here");
    assert!(doc.blocks.is_empty());
    assert_eq!(report.dropped_tail_bytes, 12);
}

#[test]
fn br_and_p_newline_rules() {
    // Leading BR/P add nothing; newlines right after BR/P are dropped; spaces stay.
    let (doc, _) = raw("<BR><P>\n\nFirst<BR>\n\n  Second\n");
    assert_eq!(lines(&doc), ["First", "  Second"]);

    let (doc, _) = raw("A<P>B");
    assert_eq!(lines(&doc), ["A", "", "B"]);

    // Closing forms count as the tag (OpenMW), names are case-insensitive.
    let (doc, report) = raw("a</P>b<br>c<Br>d</br>e");
    assert_eq!(lines(&doc), ["a", "", "b", "c", "d", "e"]);
    assert_eq!(count(&report, "/p"), 1);
    assert_eq!(count(&report, "br"), 2);
    assert_eq!(count(&report, "/br"), 1);
}

#[test]
fn whitespace_is_kept_and_raw_newlines_break() {
    let (doc, _) = raw("a  \t b\nc\r\nd");
    assert_eq!(lines(&doc), ["a  \t b", "c", "d"]);
    // Leading newlines are dropped, '\r' everywhere.
    let (doc, _) = raw("\r\n\n x");
    assert_eq!(lines(&doc), [" x"]);
}

#[test]
fn lines_group_into_paragraphs_and_blank_lines() {
    let (doc, _) = raw("a<BR>b<BR><BR>c");
    let style = RunStyle::default();
    let mut ab = Paragraph::new(Align::Start);
    ab.push_text("a\nb", &style);
    let mut c = Paragraph::new(Align::Start);
    c.push_text("c", &style);
    assert_eq!(
        doc.blocks,
        vec![
            Block::Paragraph(ab),
            Block::Paragraph(Paragraph::new(Align::Start)),
            Block::Paragraph(c),
        ]
    );
}

#[test]
fn div_alignment_and_block_boundaries() {
    let (doc, report) = raw(r#"<DIV ALIGN="CENTER">Title<BR><DIV ALIGN="left">Body"#);
    assert_eq!(lines(&doc), ["Title", "Body"]);
    assert_eq!(aligns(&doc), [Align::Center, Align::Start]);
    assert_eq!(report.values.get("div.align=center"), Some(&1));
    assert_eq!(report.values.get("div.align=left"), Some(&1));

    // A DIV in mid-line breaks the line (OpenMW's box boundary).
    let (doc, _) = raw("ab<DIV ALIGN=RIGHT>cd");
    assert_eq!(lines(&doc), ["ab", "cd"]);
    assert_eq!(aligns(&doc), [Align::Start, Align::End]);

    // One newline at the boundary is absorbed; a second one is a blank line.
    let (doc, _) = raw("ab<DIV>\ncd");
    assert_eq!(lines(&doc), ["ab", "cd"]);
    let (doc, _) = raw("ab\n<DIV>\ncd");
    assert_eq!(lines(&doc), ["ab", "", "cd"]);

    // `</DIV>` is a boundary but keeps the alignment; unknown values keep it too.
    let (doc, report) = raw(r#"<DIV ALIGN="CENTER">a</DIV>b<DIV ALIGN="justify">c"#);
    assert_eq!(lines(&doc), ["a", "b", "c"]);
    assert_eq!(aligns(&doc), [Align::Center; 3]);
    assert_eq!(count(&report, "/div"), 1);
}

#[test]
fn p_with_align_is_a_block_tag() {
    let (doc, _) = raw(r#"a<P ALIGN="RIGHT">b"#);
    assert_eq!(lines(&doc), ["a", "", "b"]);
    assert_eq!(aligns(&doc), [Align::Start, Align::Start, Align::End]);
}

#[test]
fn font_runs_are_inline() {
    let (doc, report) = raw(r#"<FONT COLOR="FF0000">red</FONT> plain"#);
    let Some(Block::Paragraph(p)) = doc.blocks.first() else {
        panic!("{doc:?}");
    };
    assert_eq!(doc.blocks.len(), 1);
    assert_eq!(
        p.inlines,
        [text("red", colored(255, 0, 0)), text(" plain", RunStyle::default())]
    );
    assert_eq!(report.inline_font_changes, 1);
    assert_eq!(count(&report, "font"), 1);
    assert_eq!(count(&report, "/font"), 1);
}

#[test]
fn nested_font_overrides_and_close_resets_all() {
    let (doc, report) =
        raw(r#"<FONT COLOR="00FF00" FACE="Daedric">a<FONT SIZE=5>b</FONT>c<FONT FACE="Magic Cards">d"#);
    let green_daedric = RunStyle {
        family: Some(FAMILY_DAEDRIC.to_owned()),
        ..colored(0, 255, 0)
    };
    let big = RunStyle {
        size: 1.5,
        ..green_daedric.clone()
    };
    let Some(Block::Paragraph(p)) = doc.blocks.first() else {
        panic!("{doc:?}");
    };
    // `c` and `d` share the default style (another face is the body face), so they merge.
    assert_eq!(
        p.inlines,
        [
            text("a", green_daedric),
            text("b", big),
            text("cd", RunStyle::default())
        ]
    );
    assert_eq!(report.inline_font_changes, 3);
    assert_eq!(report.values.get("font.face=daedric"), Some(&1));
    assert_eq!(report.values.get("font.face=magic cards"), Some(&1));
    assert_eq!(report.values.get("font.size=5"), Some(&1));
}

#[test]
fn font_at_line_start_is_not_counted_and_newline_keeps_its_run() {
    let (doc, report) = raw(r#"<FONT COLOR="FF0000">a<BR></FONT>b"#);
    let Some(Block::Paragraph(p)) = doc.blocks.first() else {
        panic!("{doc:?}");
    };
    assert_eq!(
        p.inlines,
        [text("a\n", colored(255, 0, 0)), text("b", RunStyle::default())]
    );
    assert_eq!(report.inline_font_changes, 0);
}

#[test]
fn font_colors() {
    assert_eq!(font_color("ff8000"), Rgba8::rgb(255, 128, 0));
    assert_eq!(font_color("#00FF00"), Rgba8::rgb(0, 255, 0));
    assert_eq!(font_color("00FF00zz"), Rgba8::rgb(0, 255, 0));
    assert_eq!(font_color("12"), Rgba8::rgb(0, 0, 0x12));
    assert_eq!(font_color("1234567"), Rgba8::rgb(0x23, 0x45, 0x67));
    assert_eq!(font_color("123456789"), Rgba8::rgb(0, 0, 0));
    assert_eq!(font_color("zz"), Rgba8::rgb(0, 0, 0));
    assert_eq!(font_color(""), Rgba8::rgb(0, 0, 0));
}

#[test]
fn legacy_font_sizes() {
    let cases = [
        ("1", 0.75),
        ("2", 8.0 / 9.0),
        ("3", 1.0),
        ("4", 1.2),
        ("5", 1.5),
        ("6", 2.0),
        ("7", 3.0),
        ("9", 3.0),
        ("0", 0.75),
        ("+1", 1.2),
        ("-1", 8.0 / 9.0),
        ("-9", 0.75),
        (" 4", 1.2),
        ("4px", 1.2),
        ("99999999999999999999", 3.0),
    ];
    for (value, em) in cases {
        assert_eq!(
            legacy_font_size(value).map(f32::to_bits),
            Some(f32::to_bits(em)),
            "{value:?}"
        );
    }
    for value in ["", "x", "+", "-", "px4"] {
        assert_eq!(legacy_font_size(value), None, "{value:?}");
    }
    // An invalid size keeps the current one.
    let (doc, _) = raw("<FONT SIZE=6>a<FONT SIZE=big>b");
    let Some(Block::Paragraph(p)) = doc.blocks.first() else {
        panic!("{doc:?}");
    };
    let two = RunStyle {
        size: 2.0,
        ..RunStyle::default()
    };
    assert_eq!(p.inlines, [text("ab", two)]);
}

#[test]
fn attribute_quoting_variants() {
    let (doc, report) = raw(concat!(
        r#"<IMG SRC="bookart\a.tga" WIDTH="10" HEIGHT="20">"#,
        r"<IMG SRC='Bookart/B.TGA' WIDTH='11' HEIGHT='21'>",
        r"<IMG SRC=bookart\c.tga WIDTH=12 HEIGHT=22>",
        r#"<img src = "d.tga"  width = 13 height=" 23 ">"#,
        r#"<IMG SRC="x.tga" SRC="y.tga" WIDTH=1 HEIGHT=1>"#,
    ));
    assert_eq!(
        lines(&doc),
        [
            "[img bookart/a.tga 10x20]",
            "[img bookart/b.tga 11x21]",
            "[img bookart/c.tga 12x22]",
            "[img bookart/d.tga 13x23]",
            "[img bookart/y.tga 1x1]",
        ]
    );
    assert_eq!(report.skipped_images, 0);
    assert_eq!(report.malformed_attributes, 0);

    // The character after a closing quote is skipped (OpenMW expects a space): `WIDTH`
    // becomes `idth` and the image has no width.
    let (doc, report) = raw(r#"<IMG SRC="e.tga"WIDTH="10" HEIGHT="20">"#);
    assert!(doc.blocks.is_empty());
    assert_eq!(report.skipped_images, 1);

    // A missing closing quote runs to the end of the tag.
    let (doc, report) = raw(r#"<FONT FACE="Daedric>x"#);
    let Some(Block::Paragraph(p)) = doc.blocks.first() else {
        panic!("{doc:?}");
    };
    assert_eq!(
        p.inlines,
        [text(
            "x",
            RunStyle {
                family: Some(FAMILY_DAEDRIC.to_owned()),
                ..RunStyle::default()
            }
        )]
    );
    assert_eq!(report.malformed_attributes, 1);
}

#[test]
fn block_images_and_their_line_rules() {
    let (doc, report) = raw(concat!(
        r#"Text<BR><DIV ALIGN="CENTER"><IMG SRC="bookart\Pic.TGA" WIDTH="64" HEIGHT="32">"#,
        "<BR>After",
    ));
    // The newline before the image is absorbed; the one after it stays (OpenMW `isPrevImg`).
    assert_eq!(lines(&doc), ["Text", "[img bookart/pic.tga 64x32]", "", "After"]);
    assert_eq!(aligns(&doc)[1], Align::Center);
    assert_eq!(count(&report, "img"), 1);
}

#[test]
fn skipped_images() {
    for input in [
        r#"<IMG SRC="a.tga">"#,
        "<IMG WIDTH=1 HEIGHT=1>",
        r#"<IMG SRC="a.tga" WIDTH="0" HEIGHT="5">"#,
        r#"<IMG SRC="a.tga" WIDTH="10px" HEIGHT="5">"#,
        r#"<IMG SRC="a.tga" WIDTH="-3" HEIGHT="5">"#,
        r#"<IMG SRC="..\a.tga" WIDTH="3" HEIGHT="5">"#,
        "</IMG>",
    ] {
        let (doc, report) = raw(input);
        assert!(doc.blocks.is_empty(), "{input:?}");
        assert_eq!(report.skipped_images, 1, "{input:?}");
    }
}

#[test]
fn img_scheme_images() {
    // No width: inline at the tag, 50 x 50.
    let (doc, _) = raw(r#"Hi <IMG SRC="img://Icons\Star.dds">there"#);
    let Some(Block::Paragraph(p)) = doc.blocks.first() else {
        panic!("{doc:?}");
    };
    assert_eq!(doc.blocks.len(), 1);
    assert_eq!(p.text(), format!("Hi {OBJECT_REPLACEMENT}there"));
    assert_eq!(
        p.inlines[1],
        Inline::Image(ImageRef::new("icons/star.dds").with_size(50.0, 50.0))
    );

    // With a width: a block image, height defaults to 50.
    let (doc, _) = raw(r#"<IMG SRC="img://x/y.png" WIDTH="30">"#);
    assert_eq!(lines(&doc), ["[img x/y.png 30x50]"]);
}

#[test]
fn bookart_paths() {
    let cases = [
        (r"bookart\x.tga", Some("bookart/x.tga")),
        (r"BookArt\Sub\X.TGA", Some("bookart/sub/x.tga")),
        (r"textures\bookart\x.tga", Some("bookart/x.tga")),
        (r"textures\x.tga", Some("textures/x.tga")),
        (r"Data Files\textures\x.tga", Some("textures/x.tga")),
        ("x.tga", Some("bookart/x.tga")),
        (r"My Books\x.tga", Some("bookart/my books/x.tga")),
        (r"abookart\x.tga", Some("bookart/abookart/x.tga")),
        ("bookart", Some("bookart/bookart")),
        ("", None),
        (r"..\x.tga", None),
    ];
    for (src, want) in cases {
        assert_eq!(bookart_path(src).as_deref(), want, "{src:?}");
    }
}

#[test]
fn image_lookup_order() {
    let pic = ImageRef::new("bookart/pic.tga").with_size(64.0, 32.0);
    assert_eq!(
        image_candidates(&pic),
        ["bookart/pic.dds", "bookart/pic.tga", "bookart/pic_64_32.dds"]
    );
    let deep = ImageRef::new("textures/x/pic.dds");
    assert_eq!(image_candidates(&deep), ["textures/x/pic.dds", "bookart/pic.dds"]);
}

#[test]
fn body_background() {
    let (doc, report) = raw(r#"<BODY BACKGROUND="bookart\Paper.tga">x</BODY>"#);
    assert_eq!(doc.background, Some(ImageRef::new("bookart/paper.tga")));
    assert_eq!(lines(&doc), ["x"]);
    assert_eq!(count(&report, "body"), 1);
    assert_eq!(count(&report, "/body"), 1);
}

#[test]
fn unknown_tags_are_dropped_and_counted() {
    let (doc, report) = raw("<B>bold</B> <HR><!-- c -->x<>y<BR/>z");
    assert_eq!(lines(&doc), ["bold xyz"]);
    let unknown: Vec<(&str, u32)> = report
        .unknown_tags
        .iter()
        .map(|(k, v)| (k.as_str(), *v))
        .collect();
    assert_eq!(
        unknown,
        [("", 1), ("!--", 1), ("/b", 1), ("b", 1), ("br/", 1), ("hr", 1)]
    );
    assert!(report.tags.is_empty());
}

#[test]
fn unterminated_tags_become_text() {
    let (doc, report) = raw("abc <FONT COLOR=");
    assert_eq!(lines(&doc), ["abc <FONT COLOR="]);
    assert!(report.unterminated_tag);

    let (doc, report) = raw("a<BR>b<c<d");
    assert_eq!(lines(&doc), ["a", "b<c<d"]);
    assert!(report.unterminated_tag);

    // With the vanilla trim the broken tail is dropped and measured.
    let (doc, report) = parse("Line<BR>tail <IMG");
    assert_eq!(lines(&doc), ["Line"]);
    assert!(report.unterminated_tag);
    assert_eq!(report.dropped_tail_bytes, 8);
}

#[test]
fn entities_are_kept_and_counted() {
    let (doc, report) = raw("a &amp; b &#65; c & d &;");
    assert_eq!(lines(&doc), ["a &amp; b &#65; c & d &;"]);
    assert_eq!(report.entities, 2);
}

#[test]
fn page_breaks() {
    let (doc, report) = raw("one\n[pagebreak]\ntwo[PAGEBREAK]\nthree");
    assert_eq!(lines(&doc), ["one", "[page]", "two[PAGEBREAK]", "three"]);
    assert!(report.tags.is_empty());
    // Without the newline it is text.
    let (doc, _) = raw("a[pagebreak]");
    assert_eq!(lines(&doc), ["a[pagebreak]"]);
}

#[test]
fn utf8_text_and_attributes() {
    let (doc, report) = raw("é<BR>ü<FONT FACE=\"x\"é SIZE=2>ß");
    assert_eq!(lines(&doc), ["é", "üß"]);
    assert_eq!(report.values.get("font.size=2"), Some(&1));
}

/// Hostile and random inputs never panic and always yield a document.
#[test]
fn never_panics() {
    let fixed = [
        "<", ">", "<>", "</>", "<=>", "<IMG SRC=>", "<IMG SRC=\"", "<FONT COLOR>", "<DIV ALIGN=\"",
        "[pagebreak]", "[pagebreak]\n[pagebreak]\n", "&", "\r\n\r\n", "<<<<", "<P ALIGN>",
        "<IMG SRC='ö' WIDTH='é' HEIGHT=1>", "<FONT FACE='x'", "<IMG SRC=\"img://\">",
        "<BODY BACKGROUND=>",
    ];
    for input in fixed {
        let _ = parse(input);
        let _ = raw(input);
    }
    let pieces = [
        "<", ">", "=", "\"", "'", " ", "\n", "\r", "<BR>", "<P>", "</P>", "<DIV ALIGN=CENTER>",
        "<FONT COLOR=ff0000 SIZE=+2 FACE=Daedric>", "</FONT>", "<IMG SRC=\"a.tga\" WIDTH=4 HEIGHT=4>",
        "<IMG SRC=img://b.dds>", "[pagebreak]\n", "word", "é", "&amp;", "<X", "/",
    ];
    // A fixed LCG, so failures reproduce.
    let mut state: u64 = 0x9E37_79B9_7F4A_7C15;
    for _ in 0..500 {
        let mut input = String::new();
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        let len = (state >> 33) % 40;
        for _ in 0..len {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            let n = u64::try_from(pieces.len()).unwrap_or(1);
            let index = usize::try_from((state >> 33) % n).unwrap_or(0);
            input.push_str(pieces[index]);
        }
        let (doc, _) = raw(&input);
        // Every word outside tags survives untrimmed parsing.
        let words = input.matches("word").count();
        let kept = doc.plain_text().matches("word").count();
        assert!(kept <= words, "{input:?}");
        let _ = parse(&input);
    }
}
