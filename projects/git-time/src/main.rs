use clap::Parser;
use git2::Error;
use git_time::GitTimeTravel;

pub fn main() -> Result<(), Error> {
    let app = GitTimeTravel::parse();
    app.run()
}
