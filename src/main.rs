use std::{thread, time};

fn main() {
    let wanted_phrase = "Hello, world!".to_string();

    let mut phrase = String::new();

    let time_to_sleep = time::Duration::from_millis(2);

    for wanted_char in wanted_phrase.chars() {
        let mut current_letter_as_num: u8 = ' ' as u8;

        loop {
            thread::sleep(time_to_sleep);

            let current_letter = current_letter_as_num as char;

            println!("{}{}", &phrase, &current_letter);

            if current_letter != wanted_char {
                current_letter_as_num = current_letter_as_num.saturating_add(1);
                continue;
            }

            break;
        }

        phrase.push(current_letter_as_num as char);
    }
}
