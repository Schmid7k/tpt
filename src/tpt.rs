mod utilities;
mod cli;

use structopt::StructOpt;

use utilities::file_utilities;
use cli::tpt_cli;

pub fn main() {
    let opt = tpt_cli::CLP::from_args();
    println!("{:?}", opt);

    let file = file_utilities::open_file(&opt);

    match file {
        Ok(inner) => file_utilities::print_from_file(inner, &opt),
        Err(e) => panic!(e),
    }
}
