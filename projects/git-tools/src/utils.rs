use git2::{Error, Oid, Repository, Sort};

// Function to count commits between a commit and HEAD
pub fn count_commits_from(id: Oid, repo: &Repository) -> Result<usize, Error> {
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
