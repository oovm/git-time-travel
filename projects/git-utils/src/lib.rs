mod commits;
mod rebases;
mod repos;

pub use crate::{
    commits::{count_commits_from, reword_root_commit},
    repos::{find_closest_git_repo, find_initial_commit},
};
mod errors;

pub use crate::errors::GitResult;
