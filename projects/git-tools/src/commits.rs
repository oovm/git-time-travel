use crate::GitResult;
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
    let head = repo.head()?.peel(ObjectType::Commit)?;
    let mut commit = match head.into_commit() {
        Ok(commit) => commit,
        Err(_) => return Err(Error::from_str("HEAD is not a commit")),
    };
    let tree = commit.tree()?;
    let new_commit_id = repo.commit(Some("HEAD"), &commit.author(), &commit.committer(), new_message, &tree, &[&commit])?;
    let new_commit = repo.find_commit(new_commit_id)?;
    let new = repo.branch(branch_name, &new_commit, false)?;
    Ok(new)
}
