use std::io::stdout;

use crossbeam::channel::{self, Receiver, Sender};
use crossterm::{cursor, execute, terminal};

use self::{
    ui_display::Display,
    ui_input::{constants, Input},
};

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

pub mod ui_display {
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
        pub fn new(
            display_height: usize,
            display_width: usize,
            receiver: Receiver<String>,
        ) -> Display {
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
}

pub mod ui_input {
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

        // fn add_input(&mut self, ch: char) {
        //     self.input.push(ch);
        // }

        pub fn update_input(&mut self) {
            let user_input = handle_input_event();

            if user_input != constants::NULL {
                self.sender.send(user_input.to_string()).unwrap();
                self.input.push(user_input);
            }
        }
    }

    /**
     * Handles keyboard input
     *
     * @return
     *      char - the character input from from the keyboard
     *             returns '/0' if no input was recevied from the user
     */
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
}
