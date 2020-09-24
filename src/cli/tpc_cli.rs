use std::path::PathBuf;
use structopt::*;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Text Processing Toolkit",
    about = "A Rust implementation of the Unix wordcount command (wc)",
    author = "IceBlockProduction",
    version = "0.1",
    setting = structopt::clap::AppSettings::ArgRequiredElseHelp
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
    // Options
    #[structopt(short, long, help = "Path to file", parse(from_os_str))]
    pub file: PathBuf,
}