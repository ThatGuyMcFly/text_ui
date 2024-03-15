use std::io::{stdout, Write};

use crossbeam::channel::{self, unbounded};
use text_ui_lib::ui_input::constants;

struct UserInput {
    input: String,
    sender: channel::Sender<String>,
    receiver: channel::Receiver<String>,
}

fn main() {
    let (s, r) = unbounded();

    let ui_receiver = text_ui_lib::init_ui("Test UI".to_string(), 800, 600, r);

    let mut user_input = UserInput {
        input: String::new(),
        sender: s,
        receiver: ui_receiver,
    };

    loop {
        let received_data = user_input.receiver.try_recv();

        match received_data {
            Ok(data) => {
                print!("{}", data);
                stdout().flush().expect("Couldn't flush");
                if data == constants::CR.to_string() {
                    user_input.sender.send(user_input.input).unwrap();
                    user_input.input = String::new();
                } else if data == constants::ESC.to_string() {
                    break;
                } else {
                    user_input.input += &data;
                }
            }
            Err(_) => {}
        }
    }
}
