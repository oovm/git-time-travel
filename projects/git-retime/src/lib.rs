
use clap::Parser;

mod rebase;
mod utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct GitTimeTravel {
    /// commit hash
    commit: String,
    /// start date
    start_date: String,
    /// end date
    #[arg(short, long, value_name = "END")]
    end_date: Option<String>,
    #[arg(short, long, value_name = "BRANCH")]
    branch: Option<String>,
}
