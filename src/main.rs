use clap::Parser;
use std::io::{stdout, Write};
use std::{thread, time};

/// That fancy instagram animation for hello world in rust
#[derive(Parser)]
struct Args {
    #[arg(long, short)]
    // The string to print
    string: Option<String>,

    #[arg(long, short)]
    /// The milliseconds taken inbetween char iterations
    time: Option<u64>,
    
    #[arg(long, short)]
    /// Whether to print all on one line
    one_line: bool,
}

const STARTING_CHAR: char = ' ';

const DEFAULT_STR: &str = "Hello, world!";

const DEFAULT_TIME: u64 = 2;

fn main() {
    let args = Args::parse();

    let wanted_string = args.string.unwrap_or(DEFAULT_STR.to_string());

    let time_to_sleep = time::Duration::from_millis(args.time.unwrap_or(DEFAULT_TIME));

    let mut string = String::new();

    for wanted_char in wanted_string.chars() {
        let mut current_letter_as_num: u8 = STARTING_CHAR as u8;

        loop {
            thread::sleep(time_to_sleep);

            let current_letter = current_letter_as_num as char;

            print!("{}{}", &string, &current_letter);

            if args.one_line {
                print!("{}",'\r');
            } else {
                print!("{}",'\n');
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
