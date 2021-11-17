use std::io::{self, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

/// Simple struct to store the size of a terminal
pub struct Size {
    pub width: u16,
    pub height: u16,
}

/// This struct interfaces with the terminal to display all there is to display
pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<std::io::Stdout>
}

impl Terminal {
    /// Get the size of the current terminal
    /// # Errors
    /// No known possible errors, but better safe than sorry
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
            _stdout: stdout().into_raw_mode()?
        })
    }

    /// Getter for the size of the terminal
    pub fn size(&self) -> &Size {
        &self.size
    }

    /// Clears the whole terminal
    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    /// Clears the line the cursor is currently on
    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    /// Moves the terminal cursor (not necessarily the same as the editor cursor)
    pub fn move_cursor(x: u16, y: u16) {
        let x = x.saturating_add(1);
        let y = y.saturating_add(1);
        print!("{}", termion::cursor::Goto(x, y));
    }

    pub fn move_cursor_to(x: u16, y: u16) {
        let x = x.saturating_add(1);
        let y = y.saturating_add(1);
        print!("{}", termion::cursor::Goto(x, y));
    }

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    /// Wait for a key to be pressed and return it
    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    /// Draw square brackets down the left side of the screen, my take on the vim classic
    pub fn draw_brackets(&self) {
        for _ in 0..self.size().height - 1 {
            Terminal::clear_current_line();
            println!("]\r");
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
