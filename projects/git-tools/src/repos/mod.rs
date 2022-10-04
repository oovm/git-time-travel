use crate::GitResult;
use git2::{Commit, Error, Repository, Sort};

// Function to find the closest git repository in ancestors and return the Repository object if exists.
pub fn find_closest_git_repo() -> GitResult<Repository> {
    let mut current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(_) => return Err(Error::from_str("Can not get current directory")),
    };
    loop {
        if current_dir.join(".git").exists() {
            return Ok(Repository::open(current_dir)?);
        }
        if !current_dir.pop() {
            break;
        }
    }
    Err(Error::from_str("No git repository found"))
}

pub fn find_initial_commit(repo: &Repository) -> GitResult<Commit> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(Sort::TIME)?;
    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        if commit.parent_count() == 0 {
            return Ok(commit);
        }
    }
    Err(Error::from_str("No initial commit found"))
}
