# Text-Processing-Toolkit
Text-Processing-Toolkit subsumes simple and lightweight Rust implementation of popular Unix Shell commands such as cat and wc 
that run everywhere thanks to the Rust compiler.

## Installation
Installtion works from source via
```
cargo install tpt
```

## TPR
Text-Processing-Read - short tpr - is a command line utility similar to cat and can be used like this:
```
Text Processing Toolkit 0.1
IceBlockProduction
A Rust implementation of the Unix concatenate command (cat)

USAGE:
    tpr.exe [FLAGS] --file <file>

FLAGS:
    -h, --help        Prints help information
    -n, --numbered    Print lines numbered
    -V, --version     Prints version information

OPTIONS:
    -f, --file <file>    Path to file
```
An example usage would be:
```
tpr -n -f faust.txt   This prints everything from the specified file faust.txt with numbered lines to stdout
```

## TPC
Text-Processing-Count - short tpc - is a command line utility similar to wc and can be used like this:
```
Text Processing Toolkit 0.1
A Rust implementation of the Unix wordcount command (wc)

USAGE:
    tpc.exe [FLAGS] --file <file>

FLAGS:
    -b, --bytes      Print byte count
    -c, --chars      Print char count
    -h, --help       Prints help information
    -l, --lines      Print line count
    -V, --version    Prints version information
    -w, --words      Print word count

OPTIONS:
    -f, --file <file>    Path to file
```
An example usage would be:
```
tpc -bclw -f faust.txt   This prints the byte count, char count, line count and word count of the file faust.txt to stdout
```

## License
Licensed under MIT License.
