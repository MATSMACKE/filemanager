#![warn(clippy::all, clippy::pedantic)]

pub mod app;
pub mod terminal;
pub mod filesystem;
pub mod input;
pub mod command;

pub use std::env;
/*pub use app;
pub use window;
pub use terminal;
pub use filesystem;
pub use input;
pub use command;*/

fn main() {
    let mut app = app::App::new(env::current_dir().expect("Couldn't get current directory"));
    app.run();
}
