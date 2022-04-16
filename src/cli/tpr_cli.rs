use std::path::PathBuf;
use structopt::*;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Text Processing Toolkit",
    about = "Pure Rust implementation of the Unix concatenate command (cat)",
    author = "By Schmid7k",
    version = "0.2.1"
)]
pub struct CLP {
    // Flags
    #[structopt(short, long, help = "Print lines numbered")]
    pub numbered: bool,
    // Args
    #[structopt(multiple = true, parse(from_os_str))]
    pub file: Option<PathBuf>,
}
