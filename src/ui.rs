use termion::{clear, cursor, terminal_size};
use std::io::{Stdout, Write};
use termion::raw::RawTerminal;

pub struct Renderer {
    stdout: RawTerminal<Stdout>,
    width: u16,
    height: u16,
}

impl Renderer {
    pub fn new(stdout: RawTerminal<Stdout>) -> Renderer {
        let (width, height) = terminal_size().unwrap();
        Renderer {
            stdout,
            width,
            height
        }
    }
    pub fn reflesh(&mut self) {
        let (width, height) = terminal_size().unwrap();
        self.width = width;
        self.height = height;
    }
    pub fn clear(&mut self) {
        write!(self.stdout, "{}", clear::All);
    }
    pub fn render(&mut self) {
        write!(self.stdout, "{}", cursor::Goto(1, self.height));
    }
    pub fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }
    pub fn normal_status(&mut self, normal_buf: &str) {
        write!(self.stdout, "{}", cursor::Goto(self.width - (normal_buf.len() as u16), self.height));
        write!(self.stdout, "{}", normal_buf);
        write!(self.stdout, "{}", cursor::Goto(1, self.height));
    }
    pub fn insert_status(&mut self, insert_buf: &str) {
        write!(self.stdout, "{}", cursor::Goto(1, self.height));
        write!(self.stdout, "Tweet: {}", insert_buf);
    }
    pub fn confirm_status(&mut self, insert_buf: &str) {
        write!(self.stdout, "{}", cursor::Goto(1, self.height - 1));
        write!(self.stdout, "Are you sure to tweet?: {}", insert_buf);
        write!(self.stdout, "{}", cursor::Goto(1, self.height));
        write!(self.stdout, "Yes(y) / Stash(n) / Cancel(c) / Save as draft(d)");
    }
    pub fn command_status(&mut self, command_buf: &str) {
        write!(self.stdout, "{}", cursor::Goto(1, self.height));
        write!(self.stdout, ":{}", command_buf);
    }
}
