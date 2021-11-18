#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::must_use_candidate, clippy::missing_panics_doc, clippy::cast_sign_loss, clippy::cast_possible_truncation)]
mod app;
mod terminal;
mod filesystem;

use app::App;
pub use terminal::Terminal;
pub use filesystem::{get_items_to_vec, read_file_to_vec};

fn main() {
    App::default().run();
}