use std::path::PathBuf;
use structopt::*;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Text Processing Toolkit",
    about = "A Rust implementation of the Unix concatenate command (cat)",
    author = "IceBlockProduction",
    version = "0.1",
    setting = structopt::clap::AppSettings::ArgRequiredElseHelp
)]
pub struct CLP {
    // Flags
    #[structopt(short, long, help = "Print lines numbered")]
    pub numbered: bool,
    #[structopt(short, long, help = "Open specified file in write mode")]
    pub write: bool,
    // Options
    #[structopt(short, long, help = "Path to file", parse(from_os_str))]
    pub file: PathBuf,
}