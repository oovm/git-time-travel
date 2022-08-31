use std::collections::VecDeque;

use git2::{Error, IndexAddOption, RebaseOptions, Repository, Signature, Sort, Time};
use crate::errors::find_closest_git_repo;
use crate::GitTimeTravel;

#[test]
fn test() {
    let repo = find_closest_git_repo().unwrap();
    let c = modify_commit("69b31666", &repo, &[Time::new(622505600, 0), Time::new(622505600, 0), Time::new(622505600, 0), Time::new(622505600, 0)]);
    println!("count: {:?}", c);
}

// modify all commits from hash to head's time to date
fn modify_commit(hash: &str, repo: &Repository, dates: &[Time]) -> Result<(), Error> {
    let id = repo.revparse_single(hash)?.id();
    let annotated = repo.find_annotated_commit(id)?;
    let mut rebase_options = RebaseOptions::new();
    rebase_options.inmemory(true);
    let mut rebase = repo.rebase(None, Some(&annotated), None, Some(&mut rebase_options))?;
    let mut dates = dates.iter();
    while let Some(operation) = rebase.next() {
        let commit = repo.find_commit(operation?.id())?;
        let mut author = commit.author();
        let mut committer = commit.committer();

        // Update the author and committer dates
        match dates.next() {
            Some(s) => {
                author = new_sign(author, *s)?;
                committer = new_sign(committer, *s)?;
            }
            None => {
                println!("rest {} commits will be ignored", rebase.len());
                break;
            }
        }
        rebase.commit(Some(&author), &committer, commit.message())?;
    }
    rebase.finish(None)?;
    // Add the changes to the index and create a new tree
    let mut index = repo.index()?;
    index.add_all(["."].iter(), IndexAddOption::DEFAULT, None)?;
    index.write()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    // Update the reference to point to the new tree
    let head = repo.head()?;
    let parent_commit = repo.find_commit(head.target().unwrap())?;
    let sig = repo.signature()?;
    repo.commit(Some("HEAD"), &sig, &sig, "Update commit dates", &tree, &[&parent_commit])?;


    Ok(())
}


fn new_sign(old: Signature, date: Time) -> Result<Signature, Error> {
    let name = String::from_utf8_lossy(old.name_bytes());
    let email = String::from_utf8_lossy(old.email_bytes());
    Signature::new(name.as_ref(), email.as_ref(), &date)
}


impl GitTimeTravel {
    pub fn run(&self) -> Result<(), Error> {
        // println!("{:?}", rebase);
        Ok(())
    }

    // Function to count commits between a commit and HEAD
    pub fn count_commits_between(commit: &str, repo: &Repository) -> Result<usize, Error> {
        let mut count = 0;
        let mut revwalk = repo.revwalk()?;
        revwalk.push_head()?;
        revwalk.set_sorting(Sort::TIME)?;

        let commit_oid = git2::Oid::from_str(commit)?;

        for commit_id in revwalk {
            let commit_id = commit_id?;
            if commit_id == commit_oid {
                break;
            }
            count += 1;
        }

        Ok(count)
    }
}

