use crossbeam::channel::Sender;
use crossterm::{
    cursor,
    event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
};
use std::{io::stdout, time::Duration};
pub mod constants;

pub struct Input {
    input: String,
    width: usize,
    quit_input: bool,
    sender: Sender<String>,
}

pub enum EventType {
    Key(crossterm::event::KeyEvent),
    Scroll(crossterm::event::MouseEvent),
}

impl Input {
    pub fn new(width: usize, sender: Sender<String>) -> Input {
        crossterm::terminal::enable_raw_mode().expect("Failed to enable raw mode");
        execute!(stdout(), EnableMouseCapture).expect("msg");
        Self {
            input: String::new(),
            width,
            quit_input: false,
            sender,
        }
    }

    pub fn end() {
        crossterm::terminal::disable_raw_mode().expect("Failed to enable raw mode");
    }

    fn update_input(&mut self, key_event: crossterm::event::KeyEvent) {
        match key_event.code {
            KeyCode::Backspace => {
                self.sender.send(constants::BACKSPACE.to_string()).unwrap();
                self.input.pop();
            }
            KeyCode::Enter => {
                self.sender.send(constants::CR.to_string()).unwrap();
                self.input = String::new();
            }
            KeyCode::Esc => {
                self.quit_input = true;
                self.sender.send(constants::ESC.to_string()).unwrap();
            }
            KeyCode::Char(chr) => {
                if key_event.modifiers == KeyModifiers::CONTROL && (chr == 'c' || chr == 'C') {
                    self.quit_input = true;
                    execute!(stdout(), DisableMouseCapture).expect("msg");
                    self.sender.send(constants::ESC.to_string()).unwrap();
                } else {
                    self.sender.send(chr.to_string()).unwrap();
                    self.input.push(chr);
                }
            }
            _ => {}
        }
    }

    pub fn get_input(&mut self) -> Option<EventType> {
        let user_input_event = handle_input_event();

        match user_input_event {
            Some(ref user_input) => match user_input {
                EventType::Key(key_event) => {
                    self.update_input(*key_event);
                }
                EventType::Scroll(_scroll_event) => {}
            },
            None => {}
        }

        user_input_event
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

    pub fn get_quit_input(&self) -> bool {
        self.quit_input
    }
}

fn handle_input_event() -> Option<EventType> {
    if poll(Duration::from_millis(50)).expect("Failed to poll for results") {
        if let Ok(event) = read() {
            match event {
                Event::Key(event) => return Some(EventType::Key(event)),
                Event::Mouse(event) => return Some(EventType::Scroll(event)),
                _ => {}
            }
        }
    }

    None
}
