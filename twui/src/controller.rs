use termion::event::{Event, Key};

enum Mode {
    Command,
    Insert,
    Normal,
}

pub struct Controller {
    renderer: Renderer,
    mode: Mode,
    command_buf: String,
    insert_buf: String,
    pub exit: bool,
}

impl Controller {
    pub fn new(renderer: Renderer) -> Controller {
        Controller {
            renderer,
            mode: Mode::Normal,
            insert_buf: String::new(),
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
                    Event::Key(Key::Backspace) => {
                        self.command_buf.pop(i);
                    },
                    _ => {}
                }
            },
            Mode::Insert => {
                match c {
                    Event::Key(Key::Esc) => {
                        self.mode = Mode::Normal;
                    },
                    Event::Key(Key::Ctrl('\n')) => {

                        self.mode = Mode::Normal;
                        self.insert_buf = String::new();
                        //ツイート処理
                    }
                    Event::Key(Key::Char(i)) => {
                        self.insert_buf.push(i);
                    },
                    Event::Key(Key::Backspace) => {
                        self.insert_buf.pop(i);
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
        self.renderer.reflesh();
        self.renderer.clear();
        match self.mode {
            Mode::Command => {
                self.renderer.command_status(self.command_buf);
            },
            Mode::Insert => {
                self.renderer.insert_status(self.insert_buf);
            },
            Mode::Normal => {
                self.renderer.normal_status();
            }
        }
        self.renderer.flush();
    } 
}
