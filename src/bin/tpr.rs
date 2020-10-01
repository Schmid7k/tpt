use structopt::StructOpt;

use tpt::cli::tpr_cli::CLP;
use tpt::util::tpr_util::run;

fn main() {
    run(CLP::from_args()).expect("Something unexpected happened!");
}
