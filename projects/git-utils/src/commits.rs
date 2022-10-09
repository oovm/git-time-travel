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
    // Find the old branch reference
    let old_branch = repo.find_branch(old, BranchType::Local)?;

    // Get the old branch's commit
    let old_commit = old_branch.get().peel_to_commit()?;

    // Create a new tree based on the old commit's tree
    let old_tree = old_commit.tree()?;
    let new_tree_oid = repo.treebuilder(Some(&old_tree))?.write()?;

    // Create a new commit with the new message and tree
    let new_commit_oid = repo.commit(
        Some("HEAD"),
        &repo.signature()?,
        &repo.signature()?,
        message,
        &repo.find_tree(new_tree_oid)?,
        &[&old_commit],
    )?;

    // Create the new branch pointing to the new commit
    let new_branch = repo.branch(new, &repo.find_commit(new_commit_oid)?, false)?;

    Ok(new_branch)
}
