use termion::event::Key;
use termion::input::TermRead;
use std::io::{self};

/// Wait for a key to be pressed and return it
///
/// # Errors
/// - Once again, no clue, just there to be safe
pub fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}