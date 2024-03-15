use crossbeam::channel::Sender;
use crossterm::event::{poll, read, Event, KeyCode, KeyModifiers};
use std::{char, time::Duration};

pub mod constants;

pub struct Input {
    input: String,
    width: usize,
    sender: Sender<String>,
}

fn _empty_function(_ch: char) {}

impl Input {
    pub fn new(width: usize, sender: Sender<String>) -> Input {
        Input {
            input: String::new(),
            width,
            sender,
        }
    }

    pub fn update_input(&mut self) {
        let user_input = handle_input_event();

        if user_input != constants::NULL {
            self.sender.send(user_input.to_string()).unwrap();
            self.input.push(user_input);
        }
    }
}

fn handle_input_event() -> char {
    crossterm::terminal::enable_raw_mode().expect("Failed to enable raw mode");

    let mut chr: char = constants::NULL;

    if poll(Duration::from_millis(50)).expect("Failed to poll for results") {
        if let Ok(event) = read() {
            match event {
                Event::Key(event) => match event.code {
                    KeyCode::Backspace => chr = constants::BACKSPACE,
                    KeyCode::Enter => chr = constants::CR,
                    KeyCode::Char(c) => {
                        if event.modifiers == KeyModifiers::CONTROL {
                            if c == 'C' || c == 'c' {
                                chr = constants::ESC;
                            }
                        } else {
                            chr = c
                        }
                    }
                    KeyCode::Esc => chr = constants::ESC,
                    _ => {}
                },
                _ => {}
            }
        }
    }

    crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");

    chr
}
