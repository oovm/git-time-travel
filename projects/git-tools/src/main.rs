use clap::Parser;
use git2::Error;
use git_retime::GitTimeTravel;

pub fn main() -> Result<(), Error> {
    let app = GitTimeTravel::parse();
    app.run()
}
