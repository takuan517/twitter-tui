use std::io::{Stdout, Write};
use termion::clear;
use termion::cursor;
use termion::event::{Event, Key};
use termion::raw::RawTerminal;

enum Mode {
    Command,
    Insert,
    Normal,
}

pub struct Controller {
    stdout: RawTerminal<Stdout>,
    mode: Mode,
    command_buf: String,
    pub exit: bool,
}

impl Controller {
    pub fn new(stdout: RawTerminal<Stdout>) -> Controller {
        Controller {
            stdout,
            mode: Mode::Normal,
            command_buf: String::new(),
            exit: false,
        }
    }
    pub fn input(mut self, c: Event) -> Controller {
        match self.mode {
            Mode::Command => {
                match c {
                    Event::Key(Key::Char('\n')) => {

                        self.mode = Mode::Normal;
                        match self.command_buf.as_str() {
                            "q" => {
                                self.exit = true;
                            },
                            &_ => {}
                        }
                        self.command_buf = String::new();
                    },
                    Event::Key(Key::Char(i)) => {
                        self.command_buf.push(i);
                    },
                    _ => {}
                }
            },
            Mode::Insert => {
                match c {
                    Event::Key(Key::Esc) => {
                        self.mode = Mode::Normal;
                    },
                    Event::Key(Key::Char(i)) => {
                    },
                    _ => {}
                }
            },
            Mode::Normal => {
                match c {
                    Event::Key(Key::Char(':')) => {
                        self.mode = Mode::Command;
                    },
                    Event::Key(Key::Char('i')) => {
                        self.mode = Mode::Insert;
                    },
                    _ => {

                    }
                }
            }
        }
        self
    }
    pub fn render(&mut self) -> () {
        write!(self.stdout, "{}", clear::All);
        write!(self.stdout, "{}", cursor::Goto(1,1));
        match self.mode {
            Mode::Command => {
                write!(self.stdout, "CMD:{}", self.command_buf);
            },
            Mode::Insert => {
                write!(self.stdout, "INS:");
            },
            Mode::Normal => {
                write!(self.stdout, "NOR:");
            }
        }
        self.stdout.flush().unwrap();
    } 
}
