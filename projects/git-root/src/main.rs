use clap::Parser;
use git2::Error;
use git_root::GitRootCommit;

pub fn main() -> Result<(), Error> {
    let app = GitRootCommit::parse();
    app.run()
}
