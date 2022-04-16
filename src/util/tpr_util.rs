use std::fs::File;
use std::io;
use std::io::prelude::*;

use crate::cli::tpr_cli::CLP;

pub fn run(clp: CLP) -> io::Result<()> {
    match clp.file.clone() {
        None => tpr(io::BufReader::new(io::stdin()), io::stdout(), &clp)?,
        Some(file) => tpr(io::BufReader::new(File::open(file)?), io::stdout(), &clp)?,
    }
    Ok(())
}

fn tpr<R: BufRead, W: Write>(reader: R, mut writer: W, parameters: &CLP) -> io::Result<()> {
    if parameters.numbered {
        for (line, bytes) in reader.lines().enumerate() {
            writer.write_fmt(format_args!(
                "{} {}\n",
                line + 1,
                bytes.expect("Unable to parse characters into UTF-8 format")
            ))?;
        }
    } else {
        for line in reader.lines() {
            writer.write_fmt(format_args!(
                "{}\n",
                line.expect("Unable to parse characters into UTF-8 format")
            ))?;
        }
    }

    Ok(())
}
