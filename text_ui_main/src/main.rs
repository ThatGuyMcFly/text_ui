use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

use crossbeam::channel::{self, unbounded};
use crossterm::{cursor, event::poll, execute, terminal};
use text_ui_lib::{ui_input::constants, Ui};

struct UserInput {
    input: String,
    sender: channel::Sender<String>,
    receiver: channel::Receiver<String>,
}

fn main() {
    let (s, r) = unbounded();
    let (sender, receiver) = (s.clone(), r.clone());

    let mut user_input = UserInput {
        input: String::new(),
        sender: s,
        receiver: r,
    };

    // let (sender, receiver) = (s.clone(), r.clone());

    thread::spawn(move || {
        let mut ui = Ui::new("Test Text UI", 600, 800, sender, receiver);

        ui.run_ui();
    });

    loop {
        if poll(Duration::from_millis(50)).expect("msg") {
            if let Ok(message) = user_input.receiver.recv() {
                print!("{}", message);
                stdout().flush().expect("Couldn't flush");
                if message == constants::CR.to_string() {
                    execute!(stdout(), cursor::MoveToColumn(0)).expect("msg");
                    println!("input: {}", user_input.input);
                    execute!(stdout(), cursor::MoveToColumn(0)).expect("msg");
                    user_input.sender.send(user_input.input).unwrap();
                    user_input.input = String::new();
                } else if message == constants::ESC.to_string() {
                    break;
                } else {
                    user_input.input += &message;
                }
            }
        }
    }
}
