use ffidji;
use structopt::StructOpt;

fn main() {
    let opts: ffidji::Opts = ffidji::Opts::from_args();
    ffidji::execute(&opts);
}