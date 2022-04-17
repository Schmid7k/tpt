# Text Processing Toolkit (tpt)
The Text Processing Toolkit subsumes a few simple and lightweight pure Rust implementations of popular Unix shell commands such as cat, wc and echo.

## Installation
Installation via cargo:
```
cargo install tpt
tpc --help
tpr --help
```

Or build it from source:
```
git clone https://github.com/Schmid7k/tpt.git
cd tpt
cargo build --release
./target/release/tpc --help
./target/release/tpr --help
./target/release/tpw --help
```

## tpr
Text Processing Read - short tpr - is a command line utility similar to cat:
```
Text Processing Toolkit 0.3.0
By Schmid7k
Pure Rust implementation of the Unix concatenate command (cat)

USAGE:
    tpr [FLAGS] [file]...

FLAGS:
    -h, --help        Prints help information
    -n, --numbered    Print lines numbered
    -V, --version     Prints version information

ARGS:
    <file>...
```
Example usage:
```
tpr -n faust.txt   This prints everything from the specified file faust.txt with numbered lines to stdout
```

You can also pipe in text from other commands:
```
echo "Hello World" | tpr
```

## tpc
Text Processing Count - short tpc - is a command line utility similar to wc:
```
Text Processing Toolkit 0.3.0
By Schmid7k
Pure Rust implementation of the Unix wordcount command (wc). Print newline, word, character, and byte counts for each
file or input given through stdin. A word is a non-zero-lenth sequence of character delimited by white space.
The order of counts is always: newline, word, character, byte.

USAGE:
    tpc [FLAGS] [file]...

FLAGS:
    -b, --bytes      Print byte count
    -c, --chars      Print char count
    -h, --help       Prints help information
    -l, --lines      Print line count
    -V, --version    Prints version information
    -w, --words      Print word count

ARGS:
    <file>...
```
Example usage:
```
tpc -bclw faust.txt   This prints the byte count, char count, line count and word count of the file faust.txt to stdout
```

You can also pipe in text from other commands:
```
echo "Hello World" | tpc -bclw
```

## tpw
Text Processing Write - short tpw - is a command line utility similar to echo:
```
Text Processing Toolkit 0.3.0
By Schmid7k
Pure Rust implementation of the Unix echo command

USAGE:
    tpw [FLAGS] [string]

FLAGS:
    -e, --escape     Enable interpretation of backslash escapes
                     \\	backslash
                     \n	newline
                     \r	carriage return
                     \t	horizontal tab
    -h, --help       Prints help information
    -n, --newline    Do not print out the trailing newline
    -V, --version    Prints version information

ARGS:
    <string>
```

Example usage:
```
tpw "Hello World"
```

Or pipe in the output into other commands:
```
tpw "Hello World" | tpc
```

## License
Licensed under MIT License.
