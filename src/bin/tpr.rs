use structopt::StructOpt;

use tpt::cli::tpr_cli::CLP;
use tpt::util::tpr_util;

pub fn main() {
    let opt = CLP::from_args();
    println!("{:?}", opt);

    let file = tpr_util::open_file(&opt);

    match file {
        Ok(inner) => tpr_util::print_from_file(inner, &opt),
        Err(e) => panic!(e),
    }
}
