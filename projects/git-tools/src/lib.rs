mod rebase;
mod repos;
mod utils;

use git2::Error;

pub type GitResult<T = ()> = Result<T, Error>;

pub use crate::repos::find_closest_git_repo;
