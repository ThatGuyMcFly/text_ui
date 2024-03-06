use std::{char, io::stdout, time::Duration};

use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyModifiers},
    execute, terminal, ExecutableCommand,
};

const STRING_ARRAY_LENGTH: usize = 10;

const DIVIDER: &str = "-----------------------------------------";
const TITLE: &str = "Welcome to Rust Text UI Example";

const BACKSPACE: char = 0x8 as char;
const ESC: char = 0x1b as char;
const NULL: char = '\0';
const CR: char = '\n';

const EMPTY_STRING: String = String::new();

/**
 * Prints the title as centered as possible
 */
fn print_title() {
    let title_divider_length_difference = DIVIDER.len() - TITLE.len();

    if DIVIDER.len() <= TITLE.len() {
        println!("{TITLE}");
    } else {
        let indent = title_divider_length_difference / 2;
        let indent_string = " ".repeat(indent);
        print!("{indent_string}");
        println!("{TITLE}");
    }
}

/**
 * prints a formatted new line
 *
 * @param :
 *      line : &String - reference to a string to be printed
 */
fn print_line(line: &String) {
    print!("| ");

    let mut count = 2;

    for chr in line.chars() {
        if chr != 0xA as char {
            print!("{chr}");
            count += 1;
        }

        if count == DIVIDER.len() - 5 && line.len() > DIVIDER.len() - 4 {
            print!("...");
            count += 3;
            break;
        }
    }

    let trailing_space = " ".repeat(DIVIDER.len() - 1 - count);

    // for _x in count..DIVIDER.len() - 1 {
    //     print!(" ");
    // }

    println!("{trailing_space}|");
}

/**
 * Shifts the values in a string one to the left, removing the first element
 *
 * @param:
 *      user_inputs: String[] - array of strings
 *
 * @return
 *      String[]
 */
fn shift_input_values(
    mut user_inputs: [String; STRING_ARRAY_LENGTH],
) -> [String; STRING_ARRAY_LENGTH] {
    for i in 0..user_inputs.len() - 1 {
        user_inputs[i] = String::from(&user_inputs[i + 1]);
    }

    user_inputs
}

/**
 * Inserts a new string into an array of strings. Will replace the first element in the array if full
 *
 * @param:
 *      user_inputs: String[] - a mutable array into which to insert a new string
 *      input: String - a string to be inserted into the array
 *
 * @return
 *      String[] - an array of strings
 */
fn insert_new_input(
    mut user_inputs: [String; STRING_ARRAY_LENGTH],
    input: String,
) -> [String; STRING_ARRAY_LENGTH] {
    for i in 0..user_inputs.len() {
        if user_inputs[i] == "" {
            user_inputs[i] = input;
            return user_inputs;
        }
    }

    user_inputs = shift_input_values(user_inputs);

    user_inputs[user_inputs.len() - 1] = input;

    user_inputs
}

/**
 * Create an array of empty strings
 *
 * @return
 *      String[] - an array of empty strings
 */
fn create_empty_strings() -> [String; STRING_ARRAY_LENGTH] {
    let user_inputs: [String; STRING_ARRAY_LENGTH] = [EMPTY_STRING; STRING_ARRAY_LENGTH];

    user_inputs
}

/**
 * clears and sets up the UI
 *
 * @params
 *      user_inputs : &String[] - a reference to an array of user inputs to be printed to the UI
 *      user_input: &String - the current input from the user
 */
fn refresh_ui(user_inputs: &[String; STRING_ARRAY_LENGTH], user_input: &String) {
    execute!(
        stdout(),
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )
    .expect("Failed to set up terminal");

    print_title();

    println!("{DIVIDER}");

    for user_input in user_inputs {
        print_line(&user_input);
    }

    println!("{DIVIDER}");

    print_line(&String::from("User Input"));
    print_line(user_input);

    println!("{DIVIDER}");

    print_line(&"Ctlr+C or Esc to exit".to_string());

    println!("{DIVIDER}");

    execute!(
        stdout(),
        cursor::MoveTo(
            (user_input.len() + 2).try_into().unwrap(),
            (STRING_ARRAY_LENGTH + 4).try_into().unwrap()
        ),
    )
    .expect("Failed to set up terminal");
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

    let mut chr: char = NULL;

    if poll(Duration::from_millis(50)).expect("Failed to poll for results") {
        if let Ok(event) = read() {
            match event {
                Event::Key(event) => match event.code {
                    KeyCode::Backspace => chr = BACKSPACE,
                    KeyCode::Enter => chr = '\n',
                    KeyCode::Char(c) => {
                        if event.modifiers == KeyModifiers::CONTROL {
                            if c == 'C' || c == 'c' {
                                chr = ESC;
                            }
                        } else {
                            chr = c
                        }
                    }
                    KeyCode::Esc => chr = ESC,
                    _ => {}
                },
                _ => {}
            }
        }
    }

    crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");

    chr
}

fn main() {
    // Set up terminal
    let mut user_inputs = create_empty_strings();

    let mut user_input = String::new();

    refresh_ui(&user_inputs, &"".to_string());

    loop {
        let input = handle_input_event();
        if input != NULL {
            if input == CR {
                stdout().execute(cursor::MoveToColumn(0)).expect("msg");

                if user_input == "/clear" {
                    user_inputs = create_empty_strings();
                } else {
                    user_inputs = insert_new_input(user_inputs, user_input.clone());
                }

                user_input.clear();
            } else if input == ESC {
                execute!(
                    stdout(),
                    terminal::Clear(terminal::ClearType::All),
                    cursor::MoveTo(0, 0),
                )
                .expect("msg");
                break;
            } else if input == BACKSPACE {
                user_input.pop();
            } else {
                stdout().execute(cursor::MoveRight(2)).expect("msg");
                user_input.push(input);
            }
        }

        refresh_ui(&user_inputs, &user_input);
    }
}
