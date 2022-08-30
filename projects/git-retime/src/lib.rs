
use clap::Parser;

mod rebase;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct GitTimeTravel {
    /// commit hash
    commit: String,
    /// start date
    start: String,
    /// end date
    #[arg(short, long, value_name = "FILE")]
    end: Option<String>,
}
