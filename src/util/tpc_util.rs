use std::fs::File;
use std::io;
use std::io::prelude::*;

use crate::cli::tpc_cli::CLP;

pub fn run(clp: CLP) -> io::Result<()> {
    match clp.file.clone() {
        None => tpc(io::BufReader::new(io::stdin()), io::stdout(), &clp)?,
        Some(file) => tpc(io::BufReader::new(File::open(file)?), io::stdout(), &clp)?,
    }
    Ok(())
}

fn tpc<R: BufRead, W: Write>(mut reader: R, mut writer: W, parameters: &CLP) -> io::Result<()> {
    let mut buf = String::new();
    let mut output = String::new();
    let bytes = reader
        .read_to_string(&mut buf)
        .expect("Unable to parse content into String");

    if !parameters.bytes && !parameters.chars && !parameters.lines && !parameters.words {
        output = format!(
            " {} {} {}",
            buf.lines().count(),
            buf.split_whitespace().count(),
            bytes
        );
    } else {
        if parameters.lines {
            output.push_str(&format!(" {}", buf.lines().count()));
        }
        if parameters.words {
            output.push_str(&format!(" {}", buf.split_whitespace().count()));
        }
        if parameters.chars {
            output.push_str(&format!(" {}", buf.chars().count()));
        }
        if parameters.bytes {
            output.push_str(&format!(" {}", bytes));
        }
    }

    writer.write_fmt(format_args!("{}\n", output))
}
