use std::io::{stdout, Write};
use std::{thread, time::Duration};

const STARTING_CHAR: char = ' ';

const DEFAULT_TIME_DELAY: u64 = 2;

pub struct FancyPrinter {
    time_delay: Duration,
    animation: Animation,
    multi_line: bool,
}

impl FancyPrinter {
    pub fn builder() -> FancyPrinterBuilder {
        FancyPrinterBuilder::new()
    }

    pub fn print<T: ToString>(&self, text: T) {
        let wanted_string = text.to_string();

        if !wanted_string.is_ascii() {
            panic!("String is not ascii!");
        }

        match self.animation {
            Animation::CharacterCycling => self.character_cycling(wanted_string),
            Animation::Typing => self.typing(wanted_string),
        }
    }

    fn typing(&self, text: String) {
        let lines = text.split('\n');

        for line in lines {
            let mut string = String::new();

            for char in line.chars() {
                thread::sleep(self.time_delay);

                string.push(char);

                match self.multi_line {
                    true => println!("{}", &string),
                    false => print!("{}", char),
                }

                stdout().flush().unwrap();
            }

            println!();
        }

        println!();
    }

    fn character_cycling(&self, text: String) {
        let lines = text.split('\n');

        for wanted_string in lines {
            let mut string = String::new();

            for wanted_char in wanted_string.chars() {
                let mut current_letter_as_num: u8 = STARTING_CHAR as u8;

                loop {
                    thread::sleep(self.time_delay);

                    let current_letter = current_letter_as_num as char;

                    match self.multi_line {
                        true => println!("{}{}", &string, &current_letter),
                        false => print!("{}{}\r", &string, &current_letter),
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
}

impl Default for FancyPrinter {
    fn default() -> Self {
        FancyPrinterBuilder::new().build()
    }
}

#[derive(Clone, clap::ValueEnum)]
pub enum Animation {
    CharacterCycling,
    Typing,
}

pub struct FancyPrinterBuilder {
    time_delay: Duration,
    animation: Animation,
    multi_line: bool,
}

impl FancyPrinterBuilder {
    pub fn new() -> Self {
        Self {
            time_delay: Duration::from_millis(DEFAULT_TIME_DELAY),
            multi_line: false,
            animation: Animation::CharacterCycling,
        }
    }

    pub fn time_delay(mut self, time_delay: Duration) -> Self {
        self.time_delay = time_delay;
        self
    }

    pub fn multi_line(mut self, one_line: bool) -> Self {
        self.multi_line = one_line;
        self
    }

    pub fn animation(mut self, animation: Animation) -> Self {
        self.animation = animation;
        self
    }

    pub fn build(self) -> FancyPrinter {
        FancyPrinter {
            time_delay: self.time_delay,
            animation: self.animation,
            multi_line: self.multi_line,
        }
    }
}

impl Default for FancyPrinterBuilder {
    fn default() -> Self {
        Self::new()
    }
}
