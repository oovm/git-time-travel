use clap::Parser;
use git2::Error;
use git_retime::GitRootCommit;

pub fn main() -> Result<(), Error> {
    let app = GitRootCommit::parse();
    app.run()
}
