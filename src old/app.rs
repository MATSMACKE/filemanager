use crate::{get_items_to_vec, Terminal, utils};
use std::env;
use std::path::PathBuf;
use termion::event::Key;

enum Mode {
    Normal,
    Select,
    Command,
}

pub struct App {
    should_quit: bool,
    terminal: Terminal,
    cursor: usize,
    parent_cursor: usize,
    scroll: usize,
    mode: Mode,
    refresh_dir: bool,
    dirs_in_dir: Vec<String>,
    files_in_dir: Vec<String>,
    dirs_in_parent: Vec<String>,
    files_in_parent: Vec<String>,
    current_dir: PathBuf,
    parent_dir: PathBuf,
    repeats: usize,
    command: String,
}

impl App {
    pub fn new() -> Self {
        App {
            should_quit: false,
            terminal: Terminal::default().expect("Terminal initialisation failed."),
            cursor: 0,
            parent_cursor: 0,
            scroll: 0,
            mode: Mode::Normal,
            refresh_dir: true,
            dirs_in_dir: Vec::new(),
            files_in_dir: Vec::new(),
            dirs_in_parent: Vec::new(),
            files_in_parent: Vec::new(),
            current_dir: env::current_dir().expect("Couldn't get current directory"),
            parent_dir: env::current_dir().expect("Couldn't get current directory").parent().expect("Couldn't get parent directory").to_path_buf(),
            repeats: 0,
            command: String::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            // Stop the program if the should_quit flag is "true"
            if self.should_quit {
                self.terminal.clean();
                Terminal::show_cursor(true);
                break;
            }

            if self.refresh_dir {
                if let Ok(items) = get_items_to_vec(&self.current_dir) {
                    self.dirs_in_dir = items.0;
                    self.files_in_dir = items.1;
                }

                self.parent_dir = self.current_dir.parent().expect("Couldn't get parent directory").to_path_buf();

                if let Ok(items) = get_items_to_vec(&self.parent_dir) {
                    self.dirs_in_parent = items.0;
                    self.files_in_parent = items.1;
                }
            }

            if let Err(error) = self.refresh_screen() {
                panic!("{}", error);
            }

            if let Err(error) = self.process_keypress() {
                panic!("{}", error);
            }
        }
        println!("Have a nice day :)) \n\r - The Dev\n\r");
    }    

    /// Do all the required screen drawing
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::show_cursor(false);
        Terminal::move_cursor(0, 0);
        self.terminal.clear_screen();
        Terminal::move_cursor(0, 0);
        self.print_lines();
        self.display_cursor();
        match self.mode {
            Mode::Command => {
                Terminal::move_cursor(0, self.terminal.dimensions().height);
                print!(":{}", self.command);
            }
            _ => (),
        }
        Terminal::move_cursor(0, 0);
        Terminal::flush()
    }

    /// The function that actually displays stuff to the screen
    fn print_lines(&self) {
        for (i, item) in self.dirs_in_dir.iter().enumerate() {
            if i == self.cursor {
                Terminal::print_blue_invert(item);
            } else {
                Terminal::print_blue(item);
            }
        }
        for (i, item) in self.files_in_dir.iter().enumerate() {
            if i + self.dirs_in_dir.len() == self.cursor {
                Terminal::print_invert(item);
            } else {
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
            }
            Mode::Command => {
                self.keypress_command(pressed_key);
            }
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
                        self.move_cursor(-1);
                    }
                    self.repeats = 0;
                } else {
                    self.move_cursor(-1);
                }
            }
            Key::Down | Key::Char('j') => {
                if self.repeats > 0 {
                    for _ in 0..self.repeats {
                        self.move_cursor(1);
                    }
                    self.repeats = 0;
                } else {
                    self.move_cursor(1);
                }
            }
            Key::Right | Key::Char('l') => {
                if self.cursor < self.dirs_in_dir.len() {
                    self.parent_cursor = self.cursor;
                    self.cursor = 0;
                    self.current_dir = self.current_dir.join(&self.dirs_in_dir[self.cursor]);
                    self.refresh_dir();
                }
            }
            Key::Left | Key::Char('h') => {
                self.current_dir = PathBuf::new();
                self.current_dir.push(&self.parent_dir);
                self.cursor = self.parent_cursor;
                self.refresh_dir();
            }
            Key::Char(c) => {
                if c.is_numeric() {
                    self.repeats *= 10;
                    self.repeats += c.to_digit(10).unwrap() as usize;
                } else {
                    match c {
                        ':' => {
                            self.mode = Mode::Command;
                        }
                        'v' => {
                            self.mode = Mode::Select;
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }

    /// Handle a keypress in Command mode
    fn keypress_command(&mut self, key: termion::event::Key) {
        match key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Esc => self.set_mode(Mode::Normal),
            Key::Char('\n') => {
                self.execute_command();
            }
            Key::Backspace => {
                self.command.pop();
            }
            Key::Char(c) => {
                self.command.push(c);
            }
            _ => (),
        }
    }

    fn keypress_select(&mut self, key: termion::event::Key) {
        match key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up => self.move_cursor(-1),
            Key::Down => self.move_cursor(1),
            Key::Esc => self.mode = Mode::Normal,
            Key::Char(c) => match c {
                ':' => {
                    self.set_mode(Mode::Command);
                }
                _ => (),
            },
            _ => (),
        }
    }

    fn execute_command(&mut self) {
        let cmd: &str = &self.command.clone();
        if utils::is_numeric(cmd) {
            self.move_cursor_to(utils::string_to_usize(cmd) - 1);
        }
        match cmd {
            "q" => {
                self.quit()
            }
            _ => {

            }
        }
        self.command = String::from("");
        self.set_mode(Mode::Normal);
    }

    /// Move the app cursor (not necessarily the same as `Terminal::move_cursor`)
    fn move_cursor(&mut self, dif: isize) {
        match dif {
            0 => {}
            1.. => self.cursor = self.cursor.saturating_add(dif as usize),
            _ => self.cursor = self.cursor.saturating_sub((-dif) as usize),
        }
        if self.cursor >= self.dirs_in_dir.len() + self.files_in_dir.len() {
            self.cursor = self.dirs_in_dir.len() + self.files_in_dir.len() - 1;
        }
    }

    fn move_cursor_to(&mut self, y: usize) {
        self.cursor = y;
    }

    fn display_cursor(&self) {
        // To future me: Use self.scroll, and self.cursor.y, shouldn't need any more
        Terminal::move_cursor(0, self.cursor as u16 - self.scroll as u16,
        );
    }

    fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    fn refresh_dir(&mut self) {
        self.refresh_dir = true;
    }

    fn quit(&mut self) {
        self.should_quit = true;
    }
}