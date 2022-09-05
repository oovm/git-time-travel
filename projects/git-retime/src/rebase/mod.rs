use std::collections::BTreeSet;
use git2::{Error, Oid, RebaseOptions, Repository, Signature, Time};
use crate::utils::{count_commits_from, find_closest_git_repo};
use crate::GitTimeTravel;

use rand::Rng;
use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};

use rand::thread_rng;

impl GitTimeTravel {
    pub fn run(&self) -> Result<(), Error> {
        let repo = find_closest_git_repo()?;
        let oid = repo.revparse_single(&self.commit)?.id();
        let commits = count_commits_from(oid, &repo)?;
        let start_time = self.start_time()?;
        let end_time = self.end_time(commits)?;
        let mut dates = BTreeSet::new();
        let mut rng = thread_rng();
        for _ in 0..commits {
            let random_time = rng.gen_range(start_time.timestamp()..end_time.timestamp());
            dates.insert(Time::new(random_time, 0));
        }
        rebase_to_branch(&self.branch_name(), oid, &repo, &dates.into_iter().collect::<Vec<_>>())?;
        Ok(())
    }
    fn start_time(&self) -> Result<NaiveDateTime, Error> {
        let date = match NaiveDate::parse_from_str(&self.start_date, "%Y-%m-%d") {
            Ok(o) => { o }
            Err(_) => { Err(Error::from_str("date parse failed"))? }
        };
        Ok(date.and_time(NaiveTime::MIN))
    }
    fn end_time(&self, days: usize) -> Result<NaiveDateTime, Error> {
        match &self.end_date {
            Some(s) => {
                let date = match NaiveDate::parse_from_str(&s, "%Y-%m-%d") {
                    Ok(o) => { o }
                    Err(_) => { Err(Error::from_str("date parse failed"))? }
                };
                Ok(date.and_time(NaiveTime::MIN))
            }
            None => {
                let start_time = self.start_time()?;
                Ok(start_time + Duration::days(days as i64))
            }
        }
    }
    fn branch_name(&self) -> String {
        match &self.branch {
            Some(s) => {
                s.to_string()
            }
            None => {
                "time-travel".to_string()
            }
        }
    }
}


// modify all commits from hash to head's time to date and rebase into a new branch
fn rebase_to_branch(name: &str, id: Oid, repo: &Repository, dates: &[Time]) -> Result<(), Error> {
    let annotated = repo.find_annotated_commit(id)?;
    let mut rebase_options = RebaseOptions::new();
    rebase_options.inmemory(true);
    let mut last = id;
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
                panic!("Not enough dates provided")
            }
        }
        last = rebase.commit(Some(&author), &committer, commit.message())?;
    }
    rebase.finish(None)?;
    // Create a new branch with the rebased commits
    let target = repo.find_commit(last)?;
    repo.branch(name, &target, true)?;
    Ok(())
}


fn new_sign(old: Signature, date: Time) -> Result<Signature, Error> {
    let name = String::from_utf8_lossy(old.name_bytes());
    let email = String::from_utf8_lossy(old.email_bytes());
    Signature::new(name.as_ref(), email.as_ref(), &date)
}