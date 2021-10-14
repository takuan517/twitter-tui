use termion::event::{Event, Key};
use super::ui::Renderer;

enum Mode {
    Command,
    Insert,
    Normal,
    InsertConfirm,
}

pub struct Controller {
    renderer: Renderer,
    mode: Mode,
    command_buf: String,
    insert_buf: String,
    normal_buf: String,
    pub exit: bool,
}

impl Controller {
    pub fn new(renderer: Renderer) -> Controller {
        Controller {
            renderer,
            mode: Mode::Normal,
            insert_buf: String::new(),
            command_buf: String::new(),
            normal_buf: String::new(),
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
                    Event::Key(Key::Backspace) => {
                        self.command_buf.pop();
                    },
                    _ => {}
                }
            },
            Mode::Insert => {
                match c {
                    Event::Key(Key::Esc) => {
                        self.mode = Mode::InsertConfirm;
                    },
                    Event::Key(Key::Char(i)) => {
                        self.insert_buf.push(i);
                    },
                    Event::Key(Key::Backspace) => {
                        self.insert_buf.pop();
                    },
                    _ => {}
                }
            },
            Mode::InsertConfirm => {
                self.mode = Mode::Normal;
                match c {
                    Event::Key(Key::Char('y')) => {
                        //ツイート処理
                        self.insert_buf = String::new();
                    },
                    Event::Key(Key::Char('n')) => {
                        self.insert_buf = String::new();
                    },
                    Event::Key(Key::Char('c')) => {
                        self.mode = Mode::Insert;
                    },
                    Event::Key(Key::Char('d')) => {
                        //下書きツイートに保存
                        self.insert_buf = String::new();
                    },
                    _ => {
                        self.mode = Mode::InsertConfirm;
                    }
                }
            },
            Mode::Normal => {
                match c {
                    Event::Key(Key::Char(':')) => {
                        self.normal_buf = String::new();
                        self.mode = Mode::Command;
                    },
                    Event::Key(Key::Char('i')) => {
                        self.normal_buf = String::new();
                        self.mode = Mode::Insert;
                    },
                    Event::Key(Key::Char(i)) => {
                        self.normal_buf.push(i);
                        match self.normal_buf {
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
        self
    }
    pub fn render(&mut self) -> () {
        self.renderer.reflesh();
        self.renderer.clear();
        match self.mode {
            Mode::Command => {
                self.renderer.command_status(&self.command_buf);
            },
            Mode::Insert => {
                self.renderer.insert_status(&self.insert_buf);
            },
            Mode::InsertConfirm => {
                self.renderer.confirm_status(&self.insert_buf);
            },
            Mode::Normal => {
                self.renderer.normal_status(&self.normal_buf);
            }
        }
        self.renderer.flush();
    } 
}
