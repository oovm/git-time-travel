use std::path::PathBuf;
use std::process::Command;
use git2::{Error, Oid, Repository, Sort};
use clap::Parser;

#[test]
fn test() {
    let repo = find_closest_git_repo().unwrap();
    let c = count_commits_between("dd99ab69", &repo);
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

#[test]
fn test2() {
    GitTimeTravel {
        commit: "".to_string(),
        start: "".to_string(),
        end: None,
    }.run();
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
fn count_commits_between(commit: &str, repo: &Repository) -> Result<usize, Error> {
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
