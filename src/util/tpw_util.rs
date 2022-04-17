use std::io::{self, Write};

use crate::cli::tpw_cli::CLP;

pub fn run(clp: CLP) -> io::Result<()> {
    match clp.string.clone() {
        None => Ok(()),
        Some(string) => tpw(string, io::stdout(), &clp),
    }
}

fn tpw<W: Write>(mut string: String, mut writer: W, parameters: &CLP) -> io::Result<()> {
    string.push_str("\n");
    if parameters.newline {
        string = String::from(
            string
                .strip_suffix("\n") // Strip newline on Unix
                .or(string.strip_suffix("\r\n")) // Strip newline on Windows
                .unwrap_or(string.as_str()),
        );
    }
    if parameters.escape {
        // Replace \\ with \
        string = string.replace(r"\\", r"\");
        // Replace literal \n with newline
        string = string.replace(r"\n", "\n");
        // Replace literal \r with carriage return
        string = string.replace(r"\r", "\r");
        // Replace literal \t with horizontal tab
        string = string.replace(r"\t", "\t");
    }

    writer.write_fmt(format_args!("{}", string))
}
