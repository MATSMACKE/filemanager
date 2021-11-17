use crate::Terminal;
use termion::event::Key;
use std::fs;
use std::path::Path;

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
    text: String,
    refresh_dir: bool
}

impl App {
    pub fn run(&mut self) {
        loop {
            if self.refresh_dir {
                
            }

            if let Err(error) = self.refresh_screen() {
                die(&error);
            }

            // Stop the program if the should_quit flag is "true"
            if self.should_quit {
                self.terminal.clean();
                break
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
            text: String::from("Does not exist"),
            refresh_dir: true,
        }
    }

    /// Do all the required screen drawing
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::show_cursor(false);
        Terminal::move_cursor_to(0, 0);
        if self.should_quit {
            Terminal::show_cursor(true);
        }
        else {
            self.terminal.clear_screen();
            self.display_cursor();
            print!("{}", self.cursor.y);
        }
        print!("{}", self.text);
        Terminal::flush()
    }

    /// Get keypress and handle it appropriately
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match self.mode {
            Mode::Normal => {
                match pressed_key {
                    Key::Ctrl('q') => self.should_quit = true,
                    Key::Up => self.move_cursor(0, -1),
                    Key::Down => self.move_cursor(0, 1),
                    Key::Char(c) => {
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
                    },
                    _ => ()
                }
            },
            Mode::Command => {
                match pressed_key {
                    Key::Ctrl('q') => self.should_quit = true,
                    Key::Left => self.move_cursor(-1, 0),
                    Key::Right => self.move_cursor(1, 0),
                    Key::Esc => self.mode = Mode::Normal,
                    Key::Char('\n') => {

                    }
                    Key::Char(c) => {
                        
                    },
                    _ => {

                    }
                }
            },
            Mode::Insert => {
                match pressed_key {
                    Key::Ctrl('q') => self.should_quit = true,
                    Key::Up => self.move_cursor(0, -1),
                    Key::Down => self.move_cursor(0, 1),
                    Key::Esc => self.mode = Mode::Normal,
                    Key::Char(c) => {
                        match c {
                            _ => ()
                        }
                    },
                    _ => ()
                }
            },
            Mode::Select => {
                match pressed_key {
                    Key::Ctrl('q') => self.should_quit = true,
                    Key::Up => self.move_cursor(0, -1),
                    Key::Down => self.move_cursor(0, 1),
                    Key::Esc => self.mode = Mode::Normal,
                    Key::Char(c) => {
                        match c {
                            ':' => {
                                self.mode = Mode::Command;
                            },
                            'i' => {
                                self.mode = Mode::Insert;
                            },
                            _ => ()
                        }
                    },
                    _ => ()
                }
            }
        }
        Ok(())
    }

    /// Move the app cursor (not necessarily the same as `Terminal::move_cursor`)
    fn move_cursor(&mut self, x: isize, y: isize) {
        match x {
            0 => {},
            1.. => self.cursor.x = self.cursor.x.saturating_add(x as usize),
            _ => self.cursor.x = self.cursor.x.saturating_sub((-1*x) as usize)
        }
        match y {
            0 => {},
            1.. => self.cursor.y = self.cursor.y.saturating_add(y as usize),
            _ => self.cursor.y = self.cursor.y.saturating_sub((-1*y) as usize)
        }
    }

    fn display_cursor(&self) {
        Terminal::move_cursor_to(self.cursor.x as u16, self.cursor.y as u16 - self.scroll as u16);
    }
}

/// In case everything goes wrong
fn die(e: &std::io::Error) {
    panic!("{}", e)
}