use std::process::Command;
use git2::{Error, RebaseOptions, Repository, Signature, Sort, Time};
use clap::Parser;

#[test]
fn test() {
    let repo = find_closest_git_repo().unwrap();
    let c = modify_commit("88ac480a", &repo, &[Time::new(622505600, 0), Time::new(622505600, 0), Time::new(622505600, 0), Time::new(622505600, 0)]);
    println!("count: {:?}", c);
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct GitTimeTravel {
    /// commit hash
    commit: String,
    /// start date
    start: String,
    /// end date
    #[arg(short, long, value_name = "FILE")]
    end: Option<String>,
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
        let mut parents = commit.parents().collect::<Vec<_>>();
        // Ensure the current tip is the first parent before creating the commit
        if let Some(first_parent) = parents.get(0) {
            if repo.head()?.peel_to_commit()?.id() == first_parent.id() {
                let _ = repo.commit(
                    Some("HEAD"),
                    &author,
                    &committer,
                    commit.message().unwrap_or(""),
                    &commit.tree()?,
                    &parents.iter().map(|s| s).collect::<Vec<_>>(),
                )?;
            } else {
                println!("Failed to create commit: current tip is not the first parent");
            }
        }
        // Finish the rebase
        rebase.commit(Some(&author), &committer, commit.message())?;
    }

    rebase.finish(None)?;
    Ok(())
}


fn new_sign(old: Signature, date: Time) -> Result<Signature, Error> {
    let name = String::from_utf8_lossy(old.name_bytes());
    let email = String::from_utf8_lossy(old.email_bytes());
    Signature::new(name.as_ref(), email.as_ref(), &date)
}


impl GitTimeTravel {
    pub fn run(&self) -> Result<(), Error> {
        let rebase = Command::new("git")
            .arg("rebase")
            .arg("-i")
            .arg("--root")
            .spawn()
            .expect("failed to execute process");
        rebase.wait_with_output().expect("failed to wait on child");
        // println!("{:?}", rebase);
        Ok(())
    }
}

// Function to find the closest git repository in ancestors and return the Repository object
fn find_closest_git_repo() -> Result<Repository, Error> {
    let mut current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(_) => return Err(Error::from_str("Can not get current directory")),
    };
    loop {
        if current_dir.join(".git").exists() {
            return Ok(Repository::open(current_dir)?);
        }
        if !current_dir.pop() {
            break;
        }
    }
    Err(Error::from_str("No git repository found"))
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
