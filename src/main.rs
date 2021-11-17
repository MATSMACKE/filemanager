#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;
mod filesystem;

use editor::Editor;
pub use terminal::Terminal;

fn main() {
    Editor::default().run();
}