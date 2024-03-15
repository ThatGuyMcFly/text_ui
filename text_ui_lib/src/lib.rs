use std::{io::stdout, thread};

use crossbeam::channel::{unbounded, Receiver, Sender};
use crossterm::{cursor, execute, terminal};

pub mod ui_display;
pub mod ui_input;

use self::{ui_display::Display, ui_input::Input};

const PIXELS_PER_CHARACTER_WIDTH: usize = 10;
const PIXELS_PER_CHARACTER_HEIGHT: usize = 15;
const DIVIDER_CHARACTER: char = '-';
// const EDGE_CHARACTER: char = '|';

pub struct Ui {
    title: String,
    width: usize,
    display: Display,
    input: Input,
}

impl Ui {
    pub fn new(
        title: &str,
        height: usize,
        width: usize,
        receiver: Receiver<String>,
        sender: Sender<String>,
    ) -> Self {
        let character_height = height / PIXELS_PER_CHARACTER_HEIGHT;
        let character_width = width / PIXELS_PER_CHARACTER_WIDTH;

        Self {
            title: String::from(title),
            width: character_width,
            display: Display::new(10, character_width, receiver),
            input: Input::new(character_width, sender),
        }
    }

    fn draw_divider(&self) {
        let divider = DIVIDER_CHARACTER.to_string().repeat(self.width);

        execute!(stdout(), cursor::MoveToColumn(0)).unwrap();

        println!("{}", divider);
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

    fn reset_cursor(&self) {
        let row: u16 = self.display.get_height().try_into().unwrap();
        execute!(stdout(), cursor::MoveToRow(row + 4)).expect("msg");
        self.input.set_cursor_column();
    }

    fn draw_ui(&self) {
        execute!(
            stdout(),
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        )
        .expect("Failed to set up terminal");

        self.draw_title();
        self.draw_divider();
        self.display.draw();
        self.draw_divider();
        self.input.draw();
        self.draw_divider();
        self.reset_cursor();
    }

    pub fn run_ui(&mut self) {
        loop {
            self.display.update_display();
            self.input.update_input();
            self.draw_ui();
        }
    }
}

pub fn init_ui(
    title: String,
    width: usize,
    height: usize,
    receiver: Receiver<String>,
) -> Receiver<String> {
    let (ui_sender, ui_receiver) = unbounded();

    thread::spawn(move || {
        let mut ui = Ui::new(&title, height, width, receiver, ui_sender);
        ui.run_ui();
    });

    ui_receiver
}

pub fn close_ui() {
    Input::end();

    execute!(
        stdout(),
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )
    .expect("Failed to set up terminal");
}
