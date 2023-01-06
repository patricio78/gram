#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::print_stdout,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else
)]

mod document;
mod editor;
mod row;
mod terminal;
mod filetype;
mod highlighting;

pub use document::Document;
pub use editor::Position;
pub use editor::SearchDirection;
pub use row::Row;
pub use terminal::Terminal;
pub use filetype::FileType;
pub use filetype::HighlightingOptions;

use editor::Editor;

fn main() {
    Editor::default().run();
}
