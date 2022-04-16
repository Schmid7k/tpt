# text_processing_toolkit
text_processing_toolkit subsumes a few simple and lightweight pure Rust implementations of popular Unix shell commands such as cat and wc.

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
```

## tpr
text_processing_read - short tpr - is a command line utility similar to cat:
```
Text Processing Toolkit 0.2.1
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
An example usage would be:
```
tpr -n faust.txt   This prints everything from the specified file faust.txt with numbered lines to stdout
```

You can also pipe in text from other commands:
```
echo "Hello World" | tpr
```

## tpc
text_processing_count - short tpc - is a command line utility similar to wc:
```
Text Processing Toolkit 0.2.1
By Schmid7k
Pure Rust implementation of the Unix wordcount command (wc)

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
An example usage would be:
```
tpc -bclw faust.txt   This prints the byte count, char count, line count and word count of the file faust.txt to stdout
```

You can also pipe in text from other commands:
```
echo "Hello World!" | tpc -bclw
```

## License
Licensed under MIT License.
