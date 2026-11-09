//! Morrowind's UI adapter core on `sf_ui` (`docs/specs/ui-adapter.md`). Bevy-free.
//!
//! - [`book`]: BOOK record text (books, scrolls, notes) to the framework's rich-text IR
//!   ([`sf_ui::RichDoc`]), ported from `OpenMW`'s `BookTextParser` (the module cites file and
//!   commit).
//! - [`message`]: `MessageBox` text formatting (`%g`, `%.2f`, `%s`, ...) and button selection,
//!   ported from `OpenMW`'s `MessageFormatParser` / `RuntimeMessageFormatter` (the module cites
//!   file and commit).
//!
//! Proposed later in this crate (ui-adapter.md): topic linking, the bitmap `.fnt` reader.

pub mod book;
pub mod message;

#[cfg(test)]
mod tests_book;
#[cfg(test)]
mod tests_message;
