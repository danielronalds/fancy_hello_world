use clap::Parser;
use std::time;

use fancy_print::{Animation, FancyPrinter};

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
    /// The type of animation to use, the default being character-cycling
    animation: Option<AnimationOptions>,

    #[arg(long, short)]
    /// Whether new iterations should be printed on the different lines
    multi_line: bool,

    #[arg(long, short)]
    /// Whether to ignore new line characters or not
    ignore_newlines: bool,
}

#[derive(Clone, clap::ValueEnum)]
/// Intermediate enum so that no clap code is in the lib package
enum AnimationOptions {
    CharacterCycling,
    Typing,
}

impl AnimationOptions {
    pub fn to_animation(&self) -> Animation {
        match self {
            AnimationOptions::CharacterCycling => Animation::CharacterCycling,
            AnimationOptions::Typing => Animation::Typing,
        }
    } 
}

const DEFAULT_STR: &str = "Hello, world!";

const DEFAULT_ANIMATION: AnimationOptions = AnimationOptions::CharacterCycling;

const DEFAULT_TIME: u64 = 2;

fn main() {
    let args = ProgramOptions::parse();

    let string = args.string.unwrap_or_else(|| DEFAULT_STR.to_string());

    let time = time::Duration::from_millis(args.time.unwrap_or(DEFAULT_TIME));

    let animation = args.animation.unwrap_or(DEFAULT_ANIMATION).to_animation();

    let printer = FancyPrinter::builder()
        .time_delay(time)
        .animation(animation)
        .multi_line(args.multi_line)
        .ignore_newlines(args.ignore_newlines)
        .build();

    printer.print(string);
}
