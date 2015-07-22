extern crate rustbox;
extern crate rustc_serialize;
extern crate gag;

use std::env::args;
use std::path::Path;
use rustbox::{RustBox, Event, InputMode, InitOptions};
use rustbox::keyboard::Key;
use gag::Hold;

#[macro_use] mod signals;
mod ui;
mod buffer;
mod util;
mod segment;

use ui::view::HexEdit;

fn main() {
    let mut args = args();
    let mut edit = HexEdit::new();

    if args.len() > 1 {
        edit.open(&Path::new(&args.nth(1).unwrap()));
    }

    let hold = (Hold::stdout().unwrap(), Hold::stderr().unwrap());

    let rb = RustBox::init(InitOptions{
        buffer_stderr: false,
        input_mode: InputMode::Esc,
    }).unwrap();

    edit.resize(rb.width() as i32, rb.height() as i32);
    edit.draw(&rb);
    rb.present();
    loop {
        let event = rb.poll_event(false).unwrap();
        match event {
            // This case is here, since we want to have a 'way ouy' till we fixed bugs
            Event::KeyEvent(Some(Key::Char('\u{0}'))) => break,  /** This should be Ctrl-` */
            Event::KeyEvent(Some(key)) => edit.input(key),
            Event::ResizeEvent(w, h) => { edit.resize(w, h) }
            _ => ()
        };
        rb.clear();
        edit.draw(&rb);
        rb.present();
    }
    drop(rb);
    drop(hold);
}
