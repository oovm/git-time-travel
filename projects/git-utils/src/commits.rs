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

/// Reword the root commit of the old branch and create a new branch
///
/// # Arguments
///
/// * `repo`:  The git repository
/// * `message`:  The initial commit message
/// * `old`:  The old branch name
/// * `new`: The new branch name
pub fn reword_root_commit<'a>(repo: &'a Repository, old: &str, new: &str, message: &str) -> GitResult<Branch<'a>> {
    let old_branch = repo.find_branch(old, BranchType::Local)?;
    let old_commit = old_branch.get().peel_to_commit()?;
    let old_tree = old_commit.tree()?;
    let init_tree = repo.treebuilder(Some(&old_tree))?.write()?;
    let init_oid = repo.commit(None, &repo.signature()?, &repo.signature()?, message, &repo.find_tree(init_tree)?, &[])?;
    let new_branch = repo.branch(new, &repo.find_commit(init_oid)?, false)?;
    Ok(new_branch)
}
