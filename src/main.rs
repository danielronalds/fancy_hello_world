use clap::Parser;
use std::{thread, time};

/// That fancy instagram animation for hello world in rust
#[derive(Parser)]
struct Args {
    // The phrase to print
    phrase: Option<String>,
    /// The milliseconds taken inbetween char iterations
    sleep_time: Option<u64>,
}

fn main() {
    let args = Args::parse();

    let wanted_phrase = args.phrase.unwrap_or("Hello, world!".to_string());

    let time_to_sleep = time::Duration::from_millis(args.sleep_time.unwrap_or(2));

    let mut phrase = String::new();

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
