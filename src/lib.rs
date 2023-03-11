//! # fancy_print
//!
//! Easily print animated ASCII text to the terminal!
//!
//! ## Example
//!
//! To start using this crate you need to create a `FancyPrinter`. The easiest way to do this is
//! calling the `builder()` function, and defining your desired options. Instead of calling the
//! `builder()` function with the `FancyBuilder` struct you can also call
//! `FancyPrinterBuilder::new()`, they are equivalent.
//!
//! After you have a `FancyPrinter` you can call the `print()` method to print animated text.
//!
//! The following code block demonstrates this
//!
//! ```
//! use fancy_print::{FancyPrinter, Animation};
//! use std::time::Duration;
//!
//! let printer = FancyPrinter::builder()
//!     .animation(Animation::CharacterCycling)
//!     .time_delay(Duration::from_millis(2))
//!     .multi_line(false)
//!     .ignore_newlines(false)
//!     .build();
//!
//! printer.print("Hello, world!");
//! ```
//!
//! ## Defaults
//!
//! The above example is actually the default configuration of a `FancyPrinter` which can be seen
//! in the following test
//!
//! ```
//! # use fancy_print::{FancyPrinter, Animation};
//! # use std::time::Duration;
//! let printer = FancyPrinter::builder()
//!     .animation(Animation::CharacterCycling)
//!     .time_delay(Duration::from_millis(2))
//!     .multi_line(false)
//!     .ignore_newlines(false)
//!     .build();
//!
//! let default_printer = FancyPrinter::builder().build();
//!
//! assert_eq!(printer, default_printer);
//! ```
//!
//! As a result the first example can be shortened to the following
//!
//! ```
//! use fancy_print::{FancyPrinter, Animation};
//! use std::time::Duration;
//!
//! let printer = FancyPrinter::builder().build();
//!
//! printer.print("Hello, world!");
//! ```
//!
//! ## Animations
//!
//! The currently listed animations are listed below with descriptions, however the best way to
//! understand them is to see them in action, so copy the code blocks and run them!
//!
//! #### Character Cycling
//!
//! Cycles each the current character until it matches the desired one, then moving onto the next
//! character in the text. Works by incrementing a u8 and converting it to a char, and as a result
//! *all chars in the text must be ASCII*.
//!
//! To enforce this `print()` panics! if there are any non-ASCII chars in the provided text
//!
//! ```
//! use fancy_print::{Animation, FancyPrinter};
//!
//! let printer = FancyPrinter::builder()
//!     .animation(Animation::CharacterCycling)
//!     .build();
//!
//! printer.print("Hello, world!");
//! ```
//!
//! #### Typing
//!
//! Prints the given text to the terminal one character at a time until it has printed all
//! characters in the text. Pretty much exactly what you would expect. This animation runs a lot
//! faster than the Character Cycling one as there are no "iterations" per char, so I'd advice
//! changing the `time_delay` when building a `FancyPrinter` with this animation
//!
//! ```
//! use fancy_print::{Animation, FancyPrinter};
//! use std::time::Duration;
//!
//! let printer = FancyPrinter::builder()
//!     .animation(Animation::Typing)
//!     .time_delay(Duration::from_millis(100))
//!     .build();
//!
//! printer.print("Hello, world!");
//! ```

use std::io::{stdout, Write};
use std::{thread, time::Duration};

const STARTING_CHAR: char = ' ';

const DEFAULT_TIME_DELAY: u64 = 2;

/// Struct to print text to the stdout using various animations
///
/// # Example
///
/// The example below builds a `FancyPrinter` that uses the `CharacterCycling` animation with a
/// delay of 2ms between frames, with frames being printed on the same line and respects newline
/// characters. It then prints "Hello, world!".
/// ```
/// use fancy_print::{FancyPrinter, Animation};
/// use std::time::Duration;
///
/// let printer = FancyPrinter::builder()
///     .animation(Animation::CharacterCycling)
///     .time_delay(Duration::from_millis(2))
///     .multi_line(false)
///     .ignore_newlines(false)
///     .build();
///
/// printer.print("Hello, world!");
/// ```
///
/// The options used in the above example are the defaults for the `FancyPrinterBuilder` struct
/// which is returned when the `builder()` function is called.
///
/// ```
/// # use fancy_print::{FancyPrinter, Animation};
/// # use std::time::Duration;
/// let printer = FancyPrinter::builder()
///     .animation(Animation::CharacterCycling)
///     .time_delay(Duration::from_millis(2))
///     .multi_line(false)
///     .ignore_newlines(false)
///     .build();
///
/// let default_printer = FancyPrinter::builder().build();
///
/// assert_eq!(printer, default_printer);
/// ```

#[derive(Debug, PartialEq, Eq)]
pub struct FancyPrinter {
    time_delay: Duration,
    animation: Animation,
    multi_line: bool,
    ignore_newlines: bool,
}

impl FancyPrinter {
    /// Returns a `FancyPrinterBuilder` to construct a new FancyPrinter with
    pub fn builder() -> FancyPrinterBuilder {
        FancyPrinterBuilder::new()
    }

    /// Prints the given text with the `FancyPrinter`
    ///
    /// # Panics
    /// Panics if `text` isn't ASCII
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

            if !self.ignore_newlines {
                println!();
            }
        }
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
                        false => print!("\r{}{}", &string, &current_letter),
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

            if !self.ignore_newlines {
                println!();
            }
        }
    }
}

impl Default for FancyPrinter {
    fn default() -> Self {
        FancyPrinterBuilder::new().build()
    }
}

#[derive(Debug, PartialEq, Eq)]
/// The types of animations that text can be printed with
pub enum Animation {
    /// The text is cycled one character at a time, staring from the space char in ASCII
    CharacterCycling,
    /// The text is printed one character at a time
    Typing,
}

/// Builder for the `FancyPrinter` struct
///
/// # Example
/// ```
/// use fancy_print::{Animation, FancyPrinterBuilder};
/// use std::time::Duration;
///
/// let builder = FancyPrinterBuilder::new()
///     .animation(Animation::CharacterCycling)
///     .time_delay(Duration::from_millis(2))
///     .multi_line(false)
///     .ignore_newlines(false)
///     .build();
/// ```
pub struct FancyPrinterBuilder {
    time_delay: Duration,
    animation: Animation,
    multi_line: bool,
    ignore_newlines: bool,
}

impl FancyPrinterBuilder {
    /// Returns a new `FancyPrinterBuilder` struct with the defaults shown in the example
    pub fn new() -> Self {
        Self {
            time_delay: Duration::from_millis(DEFAULT_TIME_DELAY),
            multi_line: false,
            animation: Animation::CharacterCycling,
            ignore_newlines: false,
        }
    }

    /// The duration of time between frames of the animation. Defaults to 2ms
    pub fn time_delay(mut self, time_delay: Duration) -> Self {
        self.time_delay = time_delay;
        self
    }

    /// Whether the `FancyPrinter` should print new frames on new lines. Defaults to `false`
    pub fn multi_line(mut self, one_line: bool) -> Self {
        self.multi_line = one_line;
        self
    }

    /// The type of animation the `FancyPrinter` will use. Defaults to `CharacterCycling`
    pub fn animation(mut self, animation: Animation) -> Self {
        self.animation = animation;
        self
    }

    /// Whether the `FancyPrinter` should ignore newline characters. Defaults to `false`
    pub fn ignore_newlines(mut self, ignore_newlines: bool) -> Self {
        self.ignore_newlines = ignore_newlines;
        self
    }

    /// Builds a `FancyPrinter` struct with the builders options
    pub fn build(self) -> FancyPrinter {
        FancyPrinter {
            time_delay: self.time_delay,
            animation: self.animation,
            multi_line: self.multi_line,
            ignore_newlines: self.ignore_newlines,
        }
    }
}

impl Default for FancyPrinterBuilder {
    fn default() -> Self {
        Self::new()
    }
}
