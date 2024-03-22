use crossbeam::channel::{self, unbounded};
use text_ui_lib::constants;
struct UserInput {
    input: String,
    sender: channel::Sender<String>,
    receiver: channel::Receiver<char>,
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
                if data == constants::CR {
                    user_input.sender.send(user_input.input).unwrap();
                    user_input.input = String::new();
                } else if data == constants::ESC {
                    text_ui_lib::close_ui();
                    break;
                } else if data == constants::BACKSPACE {
                    user_input.input.pop();
                } else {
                    user_input.input.push(data);
                }
            }
            Err(_) => {}
        }
    }
}
