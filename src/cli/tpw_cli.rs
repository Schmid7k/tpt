use structopt::*;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Text Processing Toolkit",
    about = "Pure Rust implementation of the Unix echo command",
    author = "By Schmid7k",
    version = "0.3.0"
)]
pub struct CLP {
    // Flags
    #[structopt(short, long, help = "Do not print out the trailing newline")]
    pub newline: bool,
    #[structopt(
        short,
        long,
        help = "Enable interpretation of backslash escapes\n\\\\\tbackslash\n\\n\tnewline\n\\r\tcarriage return\n\\t\thorizontal tab"
    )]
    pub escape: bool,
    // Args
    #[structopt()]
    pub string: Option<String>,
}
