use std::path::PathBuf;

use termion::event::Key;

use crate::{*};


struct AppFlags {
    quit: bool,
    update_dir: bool
}

struct Cursor {
    from_top: usize,
    total: usize
}

enum Mode {
    Normal,
    Command
}

pub struct App {
    terminal: terminal::Terminal,
    location: PathBuf,
    flags: AppFlags,
    contents: (Vec<String>, Vec<String>),
    mode: Mode,
    cursor: Cursor
}

impl App {
    pub fn new(location: PathBuf) -> Self {
        App {
            terminal: terminal::Terminal::new().expect("Couldn't initialize terminal"),
            location,
            flags: AppFlags {
                quit: false,
                update_dir: true
            },
            contents: (Vec::new(), Vec::new()),
            mode: Mode::Normal,
            cursor: Cursor{total: 0, from_top: 0}
        }
    }

    pub fn run(&mut self) {
        loop {
            if self.flags.quit {
                terminal::Terminal::flush().expect("Couldnt update terminal");
                self.terminal.clean();
                break
            }

            if self.flags.update_dir {
                if let Ok(contents) = filesystem::get_contents(&self.location) {
                    self.contents = contents;
                }
            }

            self.update_screen();

            self.process_input();
        }
    }

    pub fn process_input(&mut self) {
        let key = input::read_key().expect("Couldn't read keypress");

        match self.mode {
            Mode::Normal => {
                match key {
                    Key::Ctrl('q' | 'c') => {
                        self.flags.quit = true;
                    }
                    Key::Char('j') => {
                        self.cursor_down(1);
                    }
                    _ => {}
                }
            },
            _ => {

            }
        }
    }

    pub fn cursor_down(&mut self, movement: usize) {
        let height = self.terminal.dimensions().height;

        for _ in 0..movement {
            self.cursor.total += 1;
            if self.cursor.from_top != height as usize {
                self.cursor.from_top += 1;
            }
        }
    }

    pub fn update_screen(&mut self) {
        let height = self.terminal.dimensions().height as usize;

        self.terminal.clear_screen();
        terminal::Terminal::move_cursor(0, 0);

        let folders = self.contents.0.len();

        let mut printed_lines = 0;
;
        for (i, item) in self.contents.0.iter().enumerate() {
            if i < self.cursor.total - self.cursor.from_top || i > printed_lines {
                continue
            }
            print!("{}\n\r", item);
            printed_lines += 1;
        }

        for (i, item) in self.contents.1.iter().enumerate() {
            if i < self.cursor.total - self.cursor.from_top || i > printed_lines {
                continue
            }
            print!("{}\n\r", item);
            printed_lines += 1;
        }

        terminal::Terminal::flush().expect("Couldn't update terminal");
    }
}