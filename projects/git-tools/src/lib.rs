mod commits;
mod rebase;
mod repos;

use git2::Error;

pub type GitResult<T = ()> = Result<T, Error>;

pub use crate::{commits::reword_root_commit, repos::find_closest_git_repo};
