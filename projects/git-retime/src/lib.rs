use clap::Parser;

mod rebase;
mod utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct GitTimeTravel {
    /// commit hash
    pub commit: String,
    /// start date
    pub start_date: String,
    /// end date
    #[arg(short, long, value_name = "END")]
    pub end_date: Option<String>,
    #[arg(short, long, value_name = "BRANCH")]
    pub branch: Option<String>,
}


