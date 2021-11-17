#![warn(clippy::all, clippy::pedantic)]
mod app;
mod terminal;
mod filesystem;

use app::App;
pub use terminal::Terminal;
pub use filesystem::{get_items_to_vec, read_file_to_vec};

fn main() {
    App::default().run();
}