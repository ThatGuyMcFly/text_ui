use std::io::stdout;

use crossbeam::channel::Receiver;
use crossterm::{cursor, execute};

use crate::constants::{CR, NULL};

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
        if self.stick_to_bottom {
            if self.data.len() > self.display_height {
                self.view_window.0 += 1;
                self.view_window.1 += 1;
            }
        }
    }

    pub fn shift_view_window(&mut self, direction: &str, amount: usize) {
        match direction {
            "up" => {
                if self.view_window.0 != 0 {
                    self.view_window.0 -= amount;
                    self.view_window.1 -= amount;
                }

                self.stick_to_bottom = false;
            }
            "down" => {
                if self.view_window.1 == self.data.len() {
                    self.stick_to_bottom = true;
                } else if self.view_window.1 != self.data.len()
                    && self.data.len() > self.display_height
                {
                    self.view_window.0 += amount;
                    self.view_window.1 += amount;
                }
            }
            _ => {}
        }
    }

    pub fn update_display(&mut self) {
        let received_data = self.receiver.try_recv();

        match received_data {
            Ok(data) => self.add_data(&data),
            Err(_) => {}
        }
    }

    fn draw_line(&self, line: &str) {
        execute!(stdout(), cursor::MoveToColumn(0)).expect("msg");
        print!("| ");

        let mut count = 2;

        for chr in line.chars() {
            if chr != NULL && chr != CR {
                print!("{}", chr);
                count += 1;
            }

            if count == self.display_width - 5 && line.len() > self.display_width - 4 {
                print!("...");
                count += 3;

                break;
            }
        }

        let trailing_spaces = " ".repeat(self.display_width - 1 - count);

        println!("{}|", trailing_spaces);
    }

    pub fn draw(&self) {
        for index in self.view_window.0..self.view_window.1 {
            let datum = self.data.get(index);

            match datum {
                Some(value) => self.draw_line(&value),
                None => self.draw_line(" "),
            }
        }
    }

    pub fn get_height(&self) -> usize {
        self.display_height
    }

    pub fn get_width(&self) -> usize {
        self.display_width
    }
}
