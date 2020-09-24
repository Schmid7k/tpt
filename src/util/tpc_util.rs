use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::io::Error;

use crate::cli::tpc_cli::CLP;

pub fn open_file(parameters: &CLP) -> Result<std::fs::File, Error> {
    OpenOptions::new()
            .read(true)
            .open(parameters.file.clone())
}

pub fn print_from_file(mut file: File, parameters: &CLP) {
    let mut output = String::new();
    let bytes = file.read_to_string(&mut output).expect("Unable to parse file content into String");
    if parameters.bytes {
        println!("{} bytes", bytes);
    }
    if parameters.chars {
        println!("{} chars", output.chars().count());
    }
    if parameters.lines {
        println!("{} lines", output.lines().count());
    }
    if parameters.words {
        println!("{} words", output.split_whitespace().count());
    }
}