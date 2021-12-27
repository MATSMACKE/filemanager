#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::must_use_candidate,
    clippy::missing_panics_doc,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation
)]

mod app;
mod filesystem;
mod terminal;
mod utils;

use app::App;
pub use filesystem::{get_items_to_vec, read_file_to_vec};
pub use terminal::Terminal;

fn main() {
    let mut manager = App::new();
    manager.run();
}
