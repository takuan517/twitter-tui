use termion::clear;
use termion::cursor;
use std::io::{Stdout, Write};
use termion::raw::RawTerminal;

pub struct Renderer {
    stdout: RawTerminal<Stdout>,
    width: u16,
    height: u16,
}

impl Renderer {
    pub fn new(stdout: RawTerminal<Stdout>) -> Renderer {
        width, height = terminal_size().unwrap();
        Renderer {
            stdout,
            width,
            height
        }
    }
    pub fn reflesh(mut self) -> Renderer {
        self.width, self.height = terminal_size().unwrap();
        self
    }
    pub fn clear(self) -> Renderer {
        write!(self.stdout, "{}", clear::All);
    }
    pub fn render(&mut self) -> Renderer {
        write!(self.stdout, "{}", cursor::Goto(1, self.height));
    }
    pub fn flush(&mut self) -> Renderer {
        self.stdout.flush().unwrap();
    }
    pub fn normal_status(&mut self) {
        write!(self.stdout, "{}", cursor::Goto(1, self.height));
    }
    pub fn insert_status(&mut self, insert_buf: &str) {
        write!(self.stdout, "{}", cursor::Goto(1, self.height));
        write!(self.stdout, "Tweet: {}", insert_buf);
    }
    pub fn command_status(&mut self, command_buf: &str) {
        write!(self.stdout, "{}", cursor::Goto(1, self.height));
        write!(self.stdout, ":{}", command_buf);
    }
}
