use std::io::{stdout, Write};
use std::{thread, time::Duration};

const STARTING_CHAR: char = ' ';

pub fn print(wanted_string: String, time_delay: Duration, one_line: bool) {
    let mut string = String::new();

    for wanted_char in wanted_string.chars() {
        let mut current_letter_as_num: u8 = STARTING_CHAR as u8;

        loop {
            thread::sleep(time_delay);

            let current_letter = current_letter_as_num as char;

            match one_line {
                true => print!("{}{}{}", &string, &current_letter, '\r'),
                false => print!("{}{}{}", &string, &current_letter, '\n'),
            }

            stdout().flush().unwrap();

            if current_letter != wanted_char {
                current_letter_as_num = current_letter_as_num.saturating_add(1);
                continue;
            }

            break;
        }

        string.push(current_letter_as_num as char);
    }

    println!();
}
