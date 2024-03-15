use std::time::Duration;

use crossbeam::channel::Receiver;
use crossterm::event::poll;

pub struct Display {
    data: Vec<String>,
    view_window: (usize, usize),
    display_height: usize,
    display_width: usize,
    receiver: Receiver<String>,
    stick_to_bottom: bool,
}

impl Display {
    pub fn new(display_height: usize, display_width: usize, receiver: Receiver<String>) -> Display {
        Display {
            data: vec![],
            view_window: (0, display_height),
            display_height,
            display_width,
            receiver,
            stick_to_bottom: true,
        }
    }

    pub fn add_data(&mut self, new_data: &str) {
        self.data.push(new_data.to_string());
        // println!("{}", self.data.get(0).unwrap());
        if self.stick_to_bottom {
            if self.data.len() > self.display_height {
                self.view_window.0 += 1;
            }
            self.view_window.1 += 1;
        }
    }

    pub fn shift_view_window(&mut self, direction: &str, amount: usize) {
        match direction {
            "up" => {
                self.view_window.0 -= amount;
                self.view_window.1 -= amount;
            }
            "down" => {
                self.view_window.0 += amount;
                self.view_window.1 += amount;
            }
            _ => {}
        }
    }

    pub fn update_display(&mut self) {
        if poll(Duration::from_millis(50)).expect("msg") {
            if let Ok(received_message) = self.receiver.recv() {
                self.add_data(&received_message);
            }
        };
    }
}
