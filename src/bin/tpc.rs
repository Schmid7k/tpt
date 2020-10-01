use structopt::StructOpt;

use tpt::cli::tpc_cli::CLP;
use tpt::util::tpc_util::run;

fn main() {
    run(CLP::from_args()).expect("Something unexpected happened!");
}
