use crate::GitResult;
use git2::{Error, Repository};

// Function to find the closest git repository in ancestors and return the Repository object
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
