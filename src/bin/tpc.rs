use structopt::StructOpt;

use tpt::cli::tpc_cli::CLP;
use tpt::util::tpc_util;

pub fn main() {
    let opt = CLP::from_args();
    println!("{:?}", opt);

    let file = tpc_util::open_file(&opt);

    match file {
        Ok(inner) => tpc_util::print_from_file(inner, &opt),
        Err(e) => panic!(e),
    }
}

