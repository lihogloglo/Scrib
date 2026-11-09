//! Every BOOK record of the installed game files (all files, overridden versions included)
//! parses without a panic. Prints tag counts, dropped tags, mapped attribute values and
//! repairs, and the IDs of books that hit them; never prints book text. Asserts nothing else.
//! Skips (passes) when no Morrowind install is detected.

use std::collections::BTreeMap;

use mw_install::MorrowindInstall;
use mw_ui::book::{self, BookReport};
use sf_ui::Block;
use tes3::esp::{Plugin, TES3Object};

const MAX_IDS: usize = 30;

fn add(into: &mut BTreeMap<String, u32>, from: &BTreeMap<String, u32>) {
    for (key, n) in from {
        *into.entry(key.clone()).or_default() += n;
    }
}

fn print_counts(title: &str, counts: &BTreeMap<String, u32>) {
    eprintln!("{title}:");
    for (key, n) in counts {
        eprintln!("  {key:<32} {n}");
    }
}

fn print_ids(title: &str, ids: &[String]) {
    let shown: Vec<&str> = ids.iter().take(MAX_IDS).map(String::as_str).collect();
    let more = ids.len().saturating_sub(MAX_IDS);
    eprintln!("{title}: {} {shown:?}{}", ids.len(), if more > 0 { format!(" +{more}") } else { String::new() });
}

#[test]
#[ignore = "real install"]
fn every_book_parses() {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let mut total = BookReport::default();
    let mut books = 0_u32;
    let (mut paragraphs, mut images, mut page_breaks, mut backgrounds) = (0_usize, 0_usize, 0_usize, 0_u32);
    let mut with_unknown = Vec::new();
    let mut with_tail = Vec::new();
    let mut with_repairs = Vec::new();
    let mut with_skipped_images = Vec::new();
    let mut empty = Vec::new();
    for file in install.game_files() {
        let bytes = std::fs::read(install.data_files.join(&file)).unwrap();
        let mut plugin = Plugin::new();
        plugin.load_bytes_filtered(&bytes, |tag| matches!(&tag, b"TES3" | b"BOOK")).unwrap();
        for object in &plugin.objects {
            let TES3Object::Book(record) = object else {
                continue;
            };
            books += 1;
            let (doc, report) = book::parse(&record.text);
            let id = format!("{file}:{}", record.id);
            for block in &doc.blocks {
                match block {
                    Block::Paragraph(_) => paragraphs += 1,
                    Block::Image { .. } => images += 1,
                    Block::PageBreak => page_breaks += 1,
                }
            }
            backgrounds += u32::from(doc.background.is_some());
            if !report.unknown_tags.is_empty() {
                let names: Vec<&str> = report.unknown_tags.keys().map(String::as_str).collect();
                with_unknown.push(format!("{id} {names:?}"));
            }
            if report.dropped_tail_bytes > 0 {
                with_tail.push(format!("{id} ({} B)", report.dropped_tail_bytes));
            }
            if report.unterminated_tag || report.malformed_attributes > 0 {
                with_repairs.push(id.clone());
            }
            if report.skipped_images > 0 {
                with_skipped_images.push(id.clone());
            }
            if doc.plain_text().trim().is_empty() && !record.text.trim().is_empty() {
                empty.push(id);
            }
            add(&mut total.tags, &report.tags);
            add(&mut total.unknown_tags, &report.unknown_tags);
            add(&mut total.values, &report.values);
            total.unterminated_tag |= report.unterminated_tag;
            total.malformed_attributes += report.malformed_attributes;
            total.skipped_images += report.skipped_images;
            total.inline_font_changes += report.inline_font_changes;
            total.dropped_tail_bytes += report.dropped_tail_bytes;
            total.entities += report.entities;
        }
    }
    eprintln!(
        "{books} BOOK records: {paragraphs} paragraphs, {images} block images, {page_breaks} page breaks, \
         {backgrounds} backgrounds"
    );
    print_counts("tags", &total.tags);
    print_counts("unknown tags (dropped)", &total.unknown_tags);
    print_counts("attribute values", &total.values);
    eprintln!(
        "repairs: unterminated tag in some book: {}, malformed attributes {}; skipped images {}; \
         inline FONT changes {}; entities {}; tail text dropped {} B",
        total.unterminated_tag,
        total.malformed_attributes,
        total.skipped_images,
        total.inline_font_changes,
        total.entities,
        total.dropped_tail_bytes
    );
    print_ids("books with unknown tags", &with_unknown);
    print_ids("books with text after the last break (dropped, as vanilla)", &with_tail);
    print_ids("books with repaired markup", &with_repairs);
    print_ids("books with skipped images", &with_skipped_images);
    print_ids("books with text that show nothing", &empty);
}
