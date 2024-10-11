use core::{panic, time};
use std::{
    char,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use frame::HandFrame;

mod frame;
mod game;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn run() {
    raw();
    let mut game = game::Game::new(8, HandFrame::default());
    // let (sx, rx): (Sender<char>, Receiver<char>) = mpsc::channel();
    // thread::spawn(move || {
    //     handle_input(sx);
    // });
    loop {
        let input = get_input();
        clear_screen();
        // let input = match rx.recv() {
        //     Ok(ch) => ch,
        //     Err(e) => panic!("{}", e),
        // };
        if input == 'q' {
            break;
        }
        let lost = game.step(input.into());
        game.render();
        println!("score: {}\r", game.score());
        println!("user input: {}\r", input);
        if lost {
            println!("you have lost!");
            break;
        }
        thread::sleep(time::Duration::from_millis(80));
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn handle_input(sx: Sender<char>) {
    loop {
        sx.send(get_input()).unwrap();
    }
}

fn get_input() -> char {
    unsafe {
        let c: char = char::from_u32(read_char() as u32).expect("read char");
        return c;
    }
}

fn raw() {
    unsafe {
        enableRawMode();
    }
}
