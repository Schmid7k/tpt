use std::fs::File;
use std::io;
use std::io::prelude::*;

use crate::cli::tpc_cli::CLP;

pub fn run(clp: CLP) -> io::Result<()> {
    match clp.file.clone() {
        None => tpc(io::BufReader::new(io::stdin()), io::stdout(), &clp)?,
        Some(file) => tpc(
            io::BufReader::new(File::open(file)?),
            io::stdout(),
            &clp,
        )?,
    }
    Ok(())
}

fn tpc<R: BufRead, W: Write>(mut reader: R, mut writer: W, parameters: &CLP) -> io::Result<()> {
    let mut buf = String::new();
    let bytes = reader.read_to_string(&mut buf).expect("Unable to parse content into String");

    
    if parameters.bytes {
        writer.write_fmt(format_args!("{} bytes ", bytes))?;
    }
    if parameters.chars {
        writer.write_fmt(format_args!("{} chars ", buf.chars().count()))?;
    }
    if parameters.lines {
        writer.write_fmt(format_args!("{} lines ", buf.lines().count()))?;
    }
    if parameters.words {
        writer.write_fmt(format_args!("{} words ", buf.split_whitespace().count()))?;
    }
    Ok(())
}
