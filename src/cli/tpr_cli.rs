use std::path::PathBuf;
use structopt::*;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Text Processing Toolkit",
    about = "A Rust implementation of the Unix concatenate command (cat)",
    author = "IceBlockProduction",
    version = "0.2.0"
)]
pub struct CLP {
    // Flags
    #[structopt(short, long, help = "Print lines numbered")]
    pub numbered: bool,
    #[structopt(short, long, help = "Input file", parse(from_os_str))]
    pub file: Option<PathBuf>,
}
