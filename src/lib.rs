use std::io::{stdout, Write};
use std::{thread, time::Duration};

const STARTING_CHAR: char = ' ';

const DEFAULT_TIME_DELAY: u64 = 2;

/// Fancy prints the given string with a time delay of 2ms and on the same line
///
/// Parameters:
/// text   The text to fancy print, must implement ToString
///
/// Panics if the string is not ascii
pub fn default_print<T: ToString>(text: T) {
    let default_time = Duration::from_millis(DEFAULT_TIME_DELAY);

    print(text, default_time, true);
}

/// Fancy prints the given string with a time delay of 2ms and on the same line
///
/// Parameters:
/// text         The text to fancy print, must implement ToString
/// time_delay   The time taken between iterations
/// one_line     Whether iterations should be printed on the same line or not
///
/// Panics if the string is not ascii
pub fn print<T: ToString>(text: T, time_delay: Duration, one_line: bool) {
    let wanted_string = text.to_string();

    if !wanted_string.is_ascii() {
        panic!("String is not ascii!");
    }

    let lines = wanted_string.split('\n');

    for wanted_string in lines {
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
}

mod tests {
    #[test]
    #[should_panic]
    fn print_panics_with_non_ascii_char() {
        let string = "ShȎuld panic";

        super::default_print(string);
    }
}
