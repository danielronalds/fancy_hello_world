use clap::Parser;
use std::time;

use fancy_hello_world;

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


const DEFAULT_STR: &str = "Hello, world!";

const DEFAULT_TIME: u64 = 2;

fn main() {
    let args = ProgramOptions::parse();

    let wanted_string = args.string.unwrap_or(DEFAULT_STR.to_string());

    let time_to_sleep = time::Duration::from_millis(args.time.unwrap_or(DEFAULT_TIME));

    fancy_hello_world::print(wanted_string, time_to_sleep, args.one_line);
}
