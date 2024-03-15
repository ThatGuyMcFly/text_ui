use crossbeam::channel::Sender;
use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyModifiers},
    execute,
};
use std::{char, io::stdout, time::Duration};
pub mod constants;

pub struct Input {
    input: String,
    width: usize,
    sender: Sender<String>,
}

fn _empty_function(_ch: char) {}

impl Input {
    pub fn new(width: usize, sender: Sender<String>) -> Input {
        crossterm::terminal::enable_raw_mode().expect("Failed to enable raw mode");

        Input {
            input: String::new(),
            width,
            sender,
        }
    }

    pub fn end() {
        crossterm::terminal::disable_raw_mode().expect("Failed to enable raw mode");
    }

    pub fn update_input(&mut self) {
        let user_input = handle_input_event();

        match user_input {
            constants::CR => {
                self.sender.send(user_input.to_string()).unwrap();
                self.input = String::new();
            }
            constants::BACKSPACE => {
                self.sender.send(user_input.to_string()).unwrap();
                self.input.pop();
            }
            constants::ESC => {
                self.sender.send(user_input.to_string()).unwrap();
            }
            constants::NULL => {}
            _ => {
                self.sender.send(user_input.to_string()).unwrap();
                self.input.push(user_input);
            }
        }
    }

    fn draw_line(&self, line: &str) {
        execute!(stdout(), cursor::MoveToColumn(0)).expect("msg");
        print!("| ");

        let mut count = 2;

        for chr in line.chars() {
            if chr != constants::NULL {
                print!("{}", chr);
                count += 1;
            }

            if count == self.width - 5 && line.len() > self.width - 4 {
                print!("...");
                count += 3;

                break;
            }
        }

        let trailing_spaces = " ".repeat(self.width - 1 - count);

        println!("{}|", trailing_spaces);
    }

    pub fn set_cursor_column(&self) {
        let column: u16 = (self.input.len() + 2).try_into().unwrap();
        execute!(stdout(), cursor::MoveToColumn(column)).expect("msg");
    }

    pub fn draw(&self) {
        self.draw_line("Input");
        self.draw_line(&self.input);
    }
}

fn handle_input_event() -> char {
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

    chr
}
