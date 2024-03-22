use crossbeam::channel::{unbounded, Receiver, Sender};
use crossterm::{
    event::{
        read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
        MouseEventKind,
    },
    execute,
};
use std::{io::stdout, thread};

pub struct Input {
    sender: Sender<EventType>,
}

pub enum EventType {
    Key(crossterm::event::KeyEvent),
    Scroll(crossterm::event::MouseEvent),
}

impl Input {
    fn new(sender: Sender<EventType>) -> Input {
        crossterm::terminal::enable_raw_mode().expect("Failed to enable raw mode");
        execute!(stdout(), EnableMouseCapture).expect("msg");
        Self { sender }
    }

    fn quit_input(&self, key_event: KeyEvent) -> bool {
        let mut quit = false;

        match key_event.code {
            KeyCode::Char(c) => {
                if (c == 'C' || c == 'c') && key_event.modifiers == KeyModifiers::CONTROL {
                    quit = true;
                }
            }
            _ => {}
        }

        quit
    }

    pub fn run_input(&mut self) {
        let mut quit = false;

        while !quit {
            if let Ok(event) = read() {
                match event {
                    Event::Key(key_event) => {
                        self.sender.send(EventType::Key(key_event)).unwrap();
                        quit = self.quit_input(key_event);
                    }
                    Event::Mouse(mouse_event) => match mouse_event.kind {
                        MouseEventKind::ScrollDown => {
                            self.sender.send(EventType::Scroll(mouse_event)).unwrap();
                        }
                        MouseEventKind::ScrollUp => {
                            self.sender.send(EventType::Scroll(mouse_event)).unwrap();
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }

        crossterm::terminal::disable_raw_mode().expect("Failed to enable raw mode");
        execute!(stdout(), DisableMouseCapture).expect("msg");
    }
}

pub fn init_input() -> Receiver<EventType> {
    let (s, r) = unbounded();

    thread::spawn(move || {
        let mut input = Input::new(s);

        input.run_input();
    });

    r
}
