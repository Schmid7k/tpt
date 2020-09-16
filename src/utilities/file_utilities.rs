use std::fs::OpenOptions;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

use crate::cli::tpt_cli::CLP;

pub fn open_file(parameters: &CLP) -> Result<std::fs::File, Error> {
    match parameters.write {
        true => OpenOptions::new()
            .read(true)
            .open(parameters.file.clone()),
        false => OpenOptions::new()
            .read(true)
            .write(true)
            .open(parameters.file.clone()),
    }
}

pub fn print_from_file(mut file: File, parameters: &CLP) {
    let mut output = String::new();
    file.read_to_string(&mut output).expect("Unable to parse file content into String");
    match parameters.numbered {
        true => {
            for (count, line) in output.lines().enumerate() {
                println!("{}: {}", count, line);
            }
        }
        false => {
            print!("{}", output);
        }
    }
}