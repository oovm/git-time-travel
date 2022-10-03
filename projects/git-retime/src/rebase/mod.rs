

use git2::{Repository, Signature, Time};

fn modify_root_commit_message_and_rename_branch(repo_path: &str, new_message: &str, new_branch_name: &str) -> Result<(), git2::Error> {
    let repo = Repository::open(repo_path)?;
    let head = repo.head()?;
    let head_commit = head.peel_to_commit()?;
    let mut branch = repo.find_branch("master", git2::BranchType::Local)?;
    branch.move_branch(new_branch_name, true)?;
    let tree = head_commit.tree()?;
    let parent_commit = head_commit.parent(0)?;
    let author = Signature::now("Your Name", "your.email@example.com")?;
    let committer = Signature::now("Your Name", "your.email@example.com")?;
    let new_commit_id = repo.commit(Some("HEAD"), &author, &committer, new_message, &tree, &[&parent_commit])?;
    let new_commit = repo.find_commit(new_commit_id)?;
    repo.set_head_detached(new_commit.id())?;
    Ok(())
}