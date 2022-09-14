# Typing Reader

[leia em português](docs/README-PT_BR.md)

This is a typing trainer program, but also a book reader. I decided
to make this because I wanted to get faster at typing, but also read
a book at the same time, so here is it. It's simple and light weight,
just run the program with the name of the file you want to read as
the first argument in the terminal. 

As you type, the program will show you where you made mistakes, allowing
you to backspace your failures before continuing. You will see your current
characters/second in the bottom of the terminal as well, a moving average
of the last 5 lines typed.

![example](docs/example.png)

## Next steps:
* Overhaul doc-strings
* Add some example books to the repository (considering a new .txt compilation of HPMOR)

## Installation

```bash
cargo install typing-reader
```
Or download the latest release from [GitHub](https://github.com/JaoCR/typing-reader/releases) for your specific platform. It's just a single binary.

## Usage
```bash
typing-reader path/to/file.txt
```
and start typing.

Make sure that your text file doesn't contain weird characters, like emojis, long dashes, fancy quotes, etc. You want to be able to type the text with your own keyboard.