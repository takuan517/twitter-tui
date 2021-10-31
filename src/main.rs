extern crate twui;
extern crate termion;
extern crate chrono;
extern crate chrono_tz;

use std::io::{stdin, stdout};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use twui::controller::Controller;
use twui::ui::Renderer;


fn main() {
    let std_in = stdin();
    let mut std_out = stdout().into_raw_mode().unwrap();
    let mut state = Controller::new(Renderer::new(std_out));
    for evt in std_in.events() {
        state.render();
        let c = evt.unwrap();
        state = state.input(c);
        if state.exit {
            return;
        }
    }
}
