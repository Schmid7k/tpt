use structopt::StructOpt;

use tpt::cli::tpw_cli::CLP;
use tpt::util::tpw_util::run;

fn main() {
    run(CLP::from_args()).expect("Something unexpected happened!");
}
