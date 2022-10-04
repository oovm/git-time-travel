use crate::{find_initial_commit, GitResult};
use git2::{Branch, BranchType, Error, ObjectType, Oid, Repository, Signature, Sort};

// Function to count commits between a commit and HEAD
pub fn count_commits_from(id: Oid, repo: &Repository) -> GitResult<usize> {
    let mut count: usize = 0;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(Sort::TIME)?;

    for commit_id in revwalk {
        let commit_id = commit_id?;
        if commit_id == id {
            break;
        }
        count += 1;
    }
    // count = count.saturating_sub(1);
    Ok(count)
}

/// Reword the root commit of the current branch and create a new branch with the same name.
///
/// # Arguments
///
/// * `repo`:
/// * `new_message`:
/// * `branch_name`:
///
/// returns: Result<Branch, Error>
///
/// # Examples
///
/// ```no_run
/// # use git_tools::{find_closest_git_repo, reword_root_commit};
/// let repo = find_closest_git_repo().unwrap();
/// let new_branch = reword_root_commit(&repo, "new message", "new_branch").unwrap();
/// ```
pub fn reword_root_commit<'a>(repo: &'a Repository, new_message: &str, branch_name: &str) -> GitResult<Branch<'a>> {
    let root = find_initial_commit(repo)?;
    let root_tree = root.tree()?;
    let init_id = repo.commit(None, &root.author(), &root.committer(), new_message, &root_tree, &[])?;
    let init = repo.find_commit(init_id)?;
    let new = repo.branch(branch_name, &init, false)?;
    Ok(new)
}
