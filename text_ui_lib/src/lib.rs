use std::{io::stdout, thread, time::Duration};

use crossbeam::channel::{unbounded, Receiver, Sender};
use crossterm::{
    cursor,
    event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind},
    execute, terminal,
};
use ui_input::EventType;

pub mod constants;
pub mod ui_display;
pub mod ui_input;

use self::ui_display::Display;

const PIXELS_PER_CHARACTER_WIDTH: usize = 10;
const PIXELS_PER_CHARACTER_HEIGHT: usize = 15;
const DIVIDER_CHARACTER: char = '-';
const DISPLAY_START_ROW: usize = 2;

pub struct Ui {
    title: String,
    width: usize,
    display: Display,
    input: String,
    input_receiver: Receiver<EventType>,
    sender: Sender<char>,
    quit: bool,
}

impl Ui {
    pub fn new(
        title: &str,
        height: usize,
        width: usize,
        receiver: Receiver<String>,
        sender: Sender<char>,
    ) -> Self {
        let _character_height = height / PIXELS_PER_CHARACTER_HEIGHT;
        let character_width = width / PIXELS_PER_CHARACTER_WIDTH;

        let input_receiver = ui_input::init_input();

        Self {
            title: String::from(title),
            width: character_width,
            display: Display::new(10, character_width, receiver),
            input: String::new(),
            input_receiver,
            sender,
            quit: false,
        }
    }

    /// Draws a line of characters the full width of the UI
    fn draw_divider(&self) {
        let divider = DIVIDER_CHARACTER.to_string().repeat(self.width);

        execute!(stdout(), cursor::MoveToColumn(0)).unwrap();

        println!("{}", divider);
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

    fn draw_title(&self) {
        if self.title.len() > self.width {
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

        let column: u16 = (self.input.len() + 2).try_into().unwrap();
        execute!(stdout(), cursor::MoveToColumn(column)).expect("msg");
    }

    fn draw_input(&self) {
        self.draw_line("Input");
        self.draw_line(&self.input);
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
        self.draw_input();
        self.draw_divider();
        self.reset_cursor();
    }

    fn within_display(&self, col: usize, row: usize) -> bool {
        let within_width = col <= self.display.get_width();
        let within_height =
            row >= DISPLAY_START_ROW && row <= self.display.get_height() + DISPLAY_START_ROW;

        return within_height && within_width;
    }

    fn handle_scroll_event(&mut self, scroll_event: MouseEvent) {
        match scroll_event.kind {
            MouseEventKind::ScrollDown => {
                if self.within_display(
                    scroll_event.column.try_into().unwrap(),
                    scroll_event.row.try_into().unwrap(),
                ) {
                    self.display.shift_view_window("down", 1);
                }
            }
            MouseEventKind::ScrollUp => {
                if self.within_display(
                    scroll_event.column.try_into().unwrap(),
                    scroll_event.row.try_into().unwrap(),
                ) {
                    self.display.shift_view_window("up", 1);
                }
            }
            _ => {}
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Backspace => {
                self.sender.send(constants::BACKSPACE).unwrap();
                self.input.pop();
            }
            KeyCode::Enter => {
                self.sender.send(constants::CR).unwrap();
                self.input = String::new();
            }
            KeyCode::Esc => {
                self.quit = true;
                self.sender.send(constants::ESC).unwrap();
            }
            KeyCode::Char(chr) => {
                if key_event.modifiers == KeyModifiers::CONTROL && (chr == 'c' || chr == 'C') {
                    self.quit = true;
                    self.sender.send(constants::ESC).unwrap();
                } else {
                    self.sender.send(chr).unwrap();
                    self.input.push(chr);
                }
            }
            _ => {}
        }
    }

    fn update_input(&mut self) {
        let ui_event_result = self.input_receiver.try_recv();

        match ui_event_result {
            Ok(ui_event) => match ui_event {
                EventType::Scroll(scroll_event) => {
                    self.handle_scroll_event(scroll_event);
                }
                EventType::Key(key_event) => {
                    self.handle_key_event(key_event);
                }
            },
            Err(_) => {}
        }
    }

    pub fn run_ui(&mut self) {
        while !self.quit {
            self.draw_ui();

            self.display.update_display();

            self.update_input();

            thread::sleep(Duration::from_millis(50));
        }

        close_ui();
    }
}

pub fn init_ui(
    title: String,
    width: usize,
    height: usize,
    receiver: Receiver<String>,
) -> Receiver<char> {
    let (ui_sender, ui_receiver) = unbounded();

    thread::spawn(move || {
        let mut ui = Ui::new(&title, height, width, receiver, ui_sender);
        ui.run_ui();
    });

    ui_receiver
}

pub fn close_ui() {
    execute!(
        stdout(),
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )
    .expect("Failed to set up terminal");
}
