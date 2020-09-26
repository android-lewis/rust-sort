mod example;
use example::sorts;
use structopt::StructOpt;
/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
#[structopt(
    name = "Search Visualiser",
    about = "Enter the name of a search to visualise how it works -- Best to clear the terminal first"
)]
struct Cli {
    /// The sort to apply
    sort: String,
    //list: String
}

fn main() {
    //Get arguments from command
    let args = Cli::from_args();
    sorts::do_hard_work(&args.sort);
}
