use std::io::{self, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

/// Simple struct to store the size of a terminal
pub struct Dimensions {
    pub width: u16,
    pub height: u16,
}

/// This struct interfaces with the terminal to display all there is to display
pub struct Terminal {
    dimensions: Dimensions,
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    /// Get the size of the current terminal
    /// # Errors
    /// No known possible errors, but better safe than sorry
    pub fn default() -> Result<Self, std::io::Error> {
        let dimensions = termion::terminal_size()?;
        Ok(Self {
            dimensions: Dimensions {
                width: dimensions.0,
                height: dimensions.1,
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }

    /// Getter for the size of the terminal
    pub fn dimensions(&self) -> &Dimensions {
        &self.dimensions
    }

    pub fn print_invert(text: &str) {
        print!(
            "{}{}{}\n\r",
            termion::style::Invert,
            text,
            termion::style::Reset
        );
    }

    pub fn print_blue_invert(text: &str) {
        print!(
            "{}{}{}{}{}\n\r",
            termion::style::Invert,
            termion::color::Fg(termion::color::Blue),
            termion::color::Fg(termion::color::White),
            text,
            termion::style::Reset
        );
    }

    pub fn print_blue(text: &str) {
        print!(
            "{}{}{}\n\r",
            termion::color::Fg(termion::color::Blue),
            text,
            termion::style::Reset
        );
    }

    /// Clears the line the cursor is currently on
    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    /// Clear the screen
    pub fn clear_screen(&self) {
        for _ in 0..self.dimensions().height {
            Terminal::clear_current_line();
            println!("\r");
        }
    }

    /// Moves the terminal cursor (not necessarily the same as the app cursor)
    pub fn move_cursor(x: u16, y: u16) {
        let x = x.saturating_add(1);
        let y = y.saturating_add(1);
        print!("{}", termion::cursor::Goto(x, y));
    }

    /// Reset the terminal, removing any artifacts
    pub fn clean(&self) {
        self.clear_screen();
        Terminal::move_cursor(0, 0);
    }

    /// A wrapper around `io::stdout().flush()` to make stuff nice over in app.rs
    ///
    /// # Errors
    /// - Honestly no clue, just there for safety
    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

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

    pub fn show_cursor(show: bool) {
        if show {
            print!("{}", termion::cursor::Show);
        } else {
            print!("{}", termion::cursor::Hide);
        }
    }
}
