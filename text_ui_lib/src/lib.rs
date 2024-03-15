use std::io::stdout;

use crossbeam::channel::{Receiver, Sender};
use crossterm::{cursor, execute, terminal};

pub mod ui_display;
pub mod ui_input;

use self::{ui_display::Display, ui_input::Input};

const PIXELS_PER_CHARACTER_WIDTH: usize = 10;
const PIXELS_PER_CHARACTER_HEIGHT: usize = 15;
const DIVIDER_CHARACTER: char = '-';
const EDGE_CHARACTER: char = '|';

pub struct Ui {
    title: String,
    width: usize,
    pub display: Display,
    pub input: Input,
}

impl Ui {
    pub fn new(
        title: &str,
        height: usize,
        width: usize,
        sender: Sender<String>,
        receiver: Receiver<String>,
    ) -> Ui {
        let character_height = height / PIXELS_PER_CHARACTER_HEIGHT;
        let character_width = width / PIXELS_PER_CHARACTER_WIDTH;

        Ui {
            title: String::from(title),
            width: character_width,
            display: Display::new(character_height, character_width, receiver),
            input: Input::new(character_width, sender),
        }
    }

    fn draw_divider(&self) {
        let divider = DIVIDER_CHARACTER.to_string().repeat(self.width);

        println!("{}", divider);
        execute!(stdout(), cursor::MoveToColumn(0)).unwrap();
    }

    fn draw_title(&self) {
        if self.title.len() < self.width {
            println!("{}", self.title);
        } else {
            let title_ui_width_difference = self.width - self.title.len();
            let indent = " ".repeat(title_ui_width_difference / 2);
            print!("{}", indent);
            println!("{}", self.title);
        }
        execute!(stdout(), cursor::MoveToColumn(0)).unwrap();
    }

    fn draw_display(&self) {}

    fn draw_input(&self) {}

    fn reset_cursor(&self) {}

    fn draw_ui(&self) {
        execute!(
            stdout(),
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        )
        .expect("Failed to set up terminal");

        self.draw_title();
        self.draw_divider();
        self.draw_display();
        self.draw_divider();
        self.draw_input();
        self.draw_divider();
        self.reset_cursor();
    }

    pub fn run_ui(&mut self) {
        self.draw_ui();
        loop {
            self.display.update_display();
            self.input.update_input();
        }
    }
}
