# fancy_print

Easily print animated ASCII text to the terminal!

## Example

To start using this crate you need to create a `FancyPrinter`. The easiest way to do this is
calling the `builder()` function, and defining your desired options. Instead of calling the
`builder()` function with the `FancyBuilder` struct you can also call
`FancyPrinterBuilder::new()`, they are equivalent.

After you have a `FancyPrinter` you can call the `print()` method to print animated text.

The following code block demonstrates this

```rust
use fancy_print::{FancyPrinter, Animation};
use std::time::Duration;

let printer = FancyPrinter::builder()
    .animation(Animation::CharacterCycling)
    .time_delay(Duration::from_millis(2))
    .multi_line(false)
    .ignore_newlines(false)
    .build();

printer.print("Hello, world!");
```

## Defaults

The above example is actually the default configuration of a `FancyPrinter` which can be seen
in the following test

```rust
use fancy_print::{FancyPrinter, Animation};
use std::time::Duration;
let printer = FancyPrinter::builder()
    .animation(Animation::CharacterCycling)
    .time_delay(Duration::from_millis(2))
    .multi_line(false)
    .ignore_newlines(false)
    .build();

let default_printer = FancyPrinter::builder().build();

assert_eq!(printer, default_printer);
```

As a result the first example can be shortened to the following

```rust
use fancy_print::{FancyPrinter, Animation};
use std::time::Duration;

let printer = FancyPrinter::builder().build();

printer.print("Hello, world!");
```

## Animations

The currently listed animations are listed below with descriptions, however the best way to
understand them is to see them in action, so copy the code blocks and run them!

#### Character Cycling

Cycles each the current character until it matches the desired one, then moving onto the next
character in the text. Works by incrementing a u8 and converting it to a char, and as a result
*all chars in the text must be ASCII*.

To enforce this `print()` panics! if there are any non-ASCII chars in the provided text

```rust
use fancy_print::{Animation, FancyPrinter};

let printer = FancyPrinter::builder()
    .animation(Animation::CharacterCycling)
    .build();

printer.print("Hello, world!");
```

#### Typing

Prints the given text to the terminal one character at a time until it has printed all
characters in the text. Pretty much exactly what you would expect. This animation runs a lot
faster than the Character Cycling one as there are no "iterations" per char, so I'd advice
changing the `time_delay` when building a `FancyPrinter` with this animation

```rust
use fancy_print::{Animation, FancyPrinter};
use std::time::Duration;

let printer = FancyPrinter::builder()
    .animation(Animation::Typing)
    .time_delay(Duration::from_millis(100))
    .build();

printer.print("Hello, world!");
```
