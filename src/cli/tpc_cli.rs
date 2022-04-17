use std::path::PathBuf;
use structopt::*;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Text Processing Toolkit",
    about = "Pure Rust implementation of the Unix wordcount command (wc). Print newline, word, character, and byte counts for each file or input given through stdin. A word is a non-zero-lenth sequence of character delimited by white space.\nThe order of counts is always: newline, word, character, byte.",
    author = "By Schmid7k",
    version = "0.3.0"
)]
pub struct CLP {
    // Flags
    #[structopt(short, long, help = "Print byte count")]
    pub bytes: bool,
    #[structopt(short, long, help = "Print line count")]
    pub lines: bool,
    #[structopt(short, long, help = "Print char count")]
    pub chars: bool,
    #[structopt(short, long, help = "Print word count")]
    pub words: bool,
    // Args
    #[structopt(multiple = true, parse(from_os_str))]
    pub file: Option<PathBuf>,
}
