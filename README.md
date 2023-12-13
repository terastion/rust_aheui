# Rust Aheui
This is my attempt at an implementation of the [Aheui](https://aheui.readthedocs.io/ko/latest/specs.en.html) programming language in Rust. Aheui is an esolang written using the Korean Hangul script. Aheui code is as a two-dimensional grid of instructions, and code execution direction is controlled by the vowel of each Hangul syllable.

This repository includes a library for representing an Aheui program's execution state (known as `libaheui`), and an interpreter to run Aheui program files (known as `rsaheui`).

## Differences from Reference Implementation
This implementation was written from scratch, only making use of the Aheui documentation and the [reference JavaScript implementation](http://aheui.github.io/jsaheui/jsaheui_en.html) without consulting its code. Because this implementation uses Rust and runs exclusively in the command line, it has some differences from the reference implementation:
* The extension protocol, which currently has no defined behavior in the documentation, acts as another queue storage structure.
* When a program prompts for user input (either a number or single character), the entire output is flushed with a newline character before prompting the user for an input.
* If the user provides an invalid input (e.g. blank input or a non-number input when prompted for a number), the interpreter will warn the user about their invalid input and prompt them once again.
* Attempting to push an invalid UTF-8 character to output is a fatal error that will terminate
program execution prematurely.
* Arithmetic operations that cause overflow or underflow for `isize` are fatal errors that will terminate program execution prematurely. This may be subject to change.
* Terminated programs are flushed with an additional newline character.

## Usage
To build the program, simply run:
```console
$ cargo build --release
```

The resulting interpreter binary `rsaheui` will be located at `target/release/rsaheui`. To run the interpreter binary:

```console
$ ./rsaheui FILE
```

### Options
* `FILE`: the Aheui program file to run

## License
This project is licensed under the terms of the GNU GPL-3.0 license. See the `LICENSE` file for more information.
