use crate::{Terminal, get_items_to_vec};
use termion::event::Key;
use std::env;
use std::path::{PathBuf};

struct Position {
    x: usize,
    y: usize
}

enum Mode {
    Normal,
    Select,
    Command,
    Insert
}

pub struct App {
    should_quit: bool,
    terminal: Terminal,
    cursor: Position,
    scroll: usize,
    mode: Mode,
    refresh_dir: bool,
    dirs_in_dir: Vec<String>,
    files_in_dir: Vec<String>,
    current_dir: PathBuf,
    repeats: usize,
    command: String,
}

impl App {
    pub fn run(&mut self) {
        loop {
            // Stop the program if the should_quit flag is "true"
            if self.should_quit {
                self.terminal.clean();
                Terminal::show_cursor(true);
                break
            }

            if self.refresh_dir {
                if let Ok(items) = get_items_to_vec(&self.current_dir) {
                    self.dirs_in_dir = items.0;
                    self.files_in_dir = items.1;

                    self.dirs_in_dir.sort_by_key(|s| s.to_lowercase());
                    self.files_in_dir.sort_by_key(|s| s.to_lowercase());

                }
            }

            if let Err(error) = self.refresh_screen() {
                die(&error);
            }

            if let Err(error) = self.process_keypress() {
                die(&error);
            }
        }
        println!("Have a nice day :)) \n\r - The Dev\n\r");
    }

    pub fn default() -> Self {
        App{
            should_quit: false,
            terminal: Terminal::default().expect("Terminal initialisation failed."),
            cursor: Position {x: 0, y: 0},
            scroll: 0,
            mode: Mode::Normal,
            refresh_dir: true,
            dirs_in_dir: Vec::new(),
            files_in_dir: Vec::new(),
            current_dir: env::current_dir().unwrap(),
            repeats: 0,
            command: String::new(),
        }
    }

    /// Do all the required screen drawing
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::show_cursor(false);
        Terminal::move_cursor_to(0, 0);
        if self.should_quit {
        }
        else {
            self.terminal.clear_screen();
            Terminal::move_cursor_to(0, 0);
            self.print_lines();
            self.display_cursor();
            match self.mode {
                Mode::Command => {
                    Terminal::move_cursor_to(0, self.terminal.dimensions().height);
                    print!(":{}", self.command);
                },
                Mode::Insert => {

                }
                _ => ()
            }
            Terminal::move_cursor_to(0, 0);
        }
        Terminal::flush()
    }

    /// The function that actually displays stuff to the screen
    fn print_lines(&self) {
        for (i, item) in self.dirs_in_dir.iter().enumerate() {
            if i == self.cursor.y {
                Terminal::print_blue_invert(item);
            }
            else {
                Terminal::print_blue(item);
            }
        }
        for (i, item) in self.files_in_dir.iter().enumerate() {
            if i + self.dirs_in_dir.len() == self.cursor.y {
                Terminal::print_invert(item);
            }
            else {
                print!("{}\n\r", item);
            }
        }
    }

    /// Get keypress and handle it appropriately
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match self.mode {
            Mode::Normal => {
                self.keypress_normal(pressed_key);
            },
            Mode::Command => {
                self.keypress_command(pressed_key);
            },
            Mode::Insert => {
                self.keypress_insert(pressed_key);
            },
            Mode::Select => {
                self.keypress_select(pressed_key);
            }
        }
        Ok(())
    }

    /// Handle a keypress in Normal mode
    fn keypress_normal(&mut self, key: termion::event::Key) {
        match key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up | Key::Char('k') => {
                if self.repeats > 0 {
                    for _ in 0..self.repeats {
                        self.move_cursor(0, -1);
                    }
                    self.repeats = 0;
                }
                else {
                    self.move_cursor(0, -1);
                }
            },
            Key::Down | Key::Char('j') => {
                if self.repeats > 0 {
                    for _ in 0..self.repeats {
                        self.move_cursor(0, 1);
                    }
                    self.repeats = 0;
                }
                else {
                    self.move_cursor(0, 1);
                }
            },
            Key::Char(c) => {
                if c.is_numeric() {
                    self.repeats *= 10;
                    self.repeats += c.to_digit(10).unwrap() as usize;
                }
                else {
                    match c {
                        ':' => {
                            self.mode = Mode::Command;
                        },
                        'i' => {
                            self.mode = Mode::Insert;
                        },
                        'v' => {
                            self.mode = Mode::Select;
                        }
                        _ => ()
                    }
                }
            },
            _ => ()
        }
    }

    /// Handle a keypress in Command mode
    fn keypress_command(&mut self, key: termion::event::Key) {
        match key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Left => self.move_cursor(-1, 0),
            Key::Right => self.move_cursor(1, 0),
            Key::Esc => self.set_mode(Mode::Normal),
            Key::Char('\n') => {
                self.set_mode(Mode::Normal);
                self.execute_command();
            }
            Key::Char(c) => {
                self.command.push(c);
            },
            _ => ()
        }
    }

    /// Handle a keypress in Insert mode
    fn keypress_insert(&mut self, key: termion::event::Key) {
        match key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up => self.move_cursor(0, -1),
            Key::Down => self.move_cursor(0, 1),
            Key::Esc => self.set_mode(Mode::Normal),
            Key::Char(c) => {
                match c {
                    _ => ()
                }
            },
            _ => ()
        }
    }

    fn keypress_select(&mut self, key: termion::event::Key) {
        match key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up => self.move_cursor(0, -1),
            Key::Down => self.move_cursor(0, 1),
            Key::Esc => self.mode = Mode::Normal,
            Key::Char(c) => {
                match c {
                    ':' => {
                        self.set_mode(Mode::Command);
                    },
                    'i' => {
                        self.set_mode(Mode::Insert);
                    },
                    _ => ()
                }
            },
            _ => ()
        }
    }

    fn execute_command(&mut self) {
        if self.command == "q" {
            self.should_quit = true;
        }
        self.command = String::from("");
    }

    /// Move the app cursor (not necessarily the same as `Terminal::move_cursor`)
    fn move_cursor(&mut self, x: isize, y: isize) {
        match x {
            0 => {},
            1.. => self.cursor.x = self.cursor.x.saturating_add(x as usize),
            _ => self.cursor.x = self.cursor.x.saturating_sub((-x) as usize)
        }
        
        match y {
            0 => {},
            1.. => self.cursor.y = self.cursor.y.saturating_add(y as usize),
            _ => self.cursor.y = self.cursor.y.saturating_sub((-y) as usize)
        }
        if self.cursor.y >= self.dirs_in_dir.len() + self.files_in_dir.len() {
            self.cursor.y = self.dirs_in_dir.len() + self.files_in_dir.len() - 1;
        }
    }

    fn display_cursor(&self) {
        // To future me: Use self.scroll, self.cursor.x and self.cursor.y, shouldn't need any more
        Terminal::move_cursor_to(self.cursor.x as u16, self.cursor.y as u16 - self.scroll as u16);
    }

    fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }
}

/// In case everything goes wrong
fn die(e: &std::io::Error) {
    panic!("{}", e)
}