mod example;
use example::sort_types;
use example::sorts;
use structopt::StructOpt;

#[structopt(
    name = "Search Visualiser",
    about = "Enter the name of a search to visualise how it works -- Best to clear the terminal first"
)]
#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(short="-s", possible_values = &sort_types::Sort::variants(), case_insensitive = true)]
    sort: sort_types::Sort,
}

fn main() {
    //Get arguments from command
    let args = Cli::from_args();
    //println!("{:?}", args);
    sorts::do_hard_work(&args.sort);
}
