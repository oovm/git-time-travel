use clap::Parser;

mod rebase;
mod utils;

/// Configs for `git-retime`
#[derive(Parser)]
#[command(author, version, about, long_about = include_str!("../readme.md"))]
pub struct GitTimeTravel {
    /// commit hash
    pub commit: String,
    /// start date
    pub start_date: String,
    /// end date, default is `start date + commit count`
    #[arg(short, long, value_name = "END")]
    pub end_date: Option<String>,
    /// custom branch name, default is `time-travel`
    #[arg(short, long, value_name = "BRANCH")]
    pub branch: Option<String>,
}


