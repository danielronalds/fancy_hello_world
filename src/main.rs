use clap::Parser;
use std::io::{stdout, Write};
use std::{thread, time};

#[derive(Parser)]
#[clap(author, version, about)]
/// That fancy instagram animation for hello world in rust
struct ProgramOptions {
    #[arg(long, short)]
    /// The string to print, the default being "Hello, world!"
    string: Option<String>,

    #[arg(long, short)]
    /// The milliseconds taken inbetween char iterations, the default being 2ms
    time: Option<u64>,

    #[arg(long, short)]
    /// Whether new iterations should be printed on the same line
    one_line: bool,
}

const STARTING_CHAR: char = ' ';

const DEFAULT_STR: &str = "Hello, world!";

const DEFAULT_TIME: u64 = 2;

fn main() {
    let args = ProgramOptions::parse();

    let wanted_string = args.string.unwrap_or(DEFAULT_STR.to_string());

    let time_to_sleep = time::Duration::from_millis(args.time.unwrap_or(DEFAULT_TIME));

    let mut string = String::new();

    for wanted_char in wanted_string.chars() {
        let mut current_letter_as_num: u8 = STARTING_CHAR as u8;

        loop {
            thread::sleep(time_to_sleep);

            let current_letter = current_letter_as_num as char;

            match args.one_line {
                true => print!("{}{}{}", &string, &current_letter, '\r'),
                false => print!("{}{}{}", &string, &current_letter, '\n') 
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
