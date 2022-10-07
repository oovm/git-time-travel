use crate::{errors::runtime_error, GitResult};
use git2::{
    Branch, BranchType, Commit, Error, ErrorClass, ErrorCode, Index, IndexAddOption, IndexEntry, IndexTime, ObjectType, Oid,
    Repository, RepositoryInitMode, RepositoryInitOptions, Signature, TreeEntry,
};
use globset::{Glob, GlobSet, GlobSetBuilder};
use std::{
    collections::HashSet,
    ffi::OsStr,
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path,
    str::FromStr,
};

pub struct GitCleaner {
    /// The git-utils repository to clean.
    repo: Repository,
    /// size to purge
    purge_size: Option<usize>,
    purge_path: GlobSet,
}

impl GitCleaner {
    pub fn new(project: Repository) -> Self {
        Self { repo: project, purge_size: None, purge_path: GlobSet::empty() }
    }
    pub fn set_max_size(&mut self, size: usize) {
        if size == 0 {
            self.purge_size = None;
        }
        else {
            self.purge_size = Some(size);
        }
    }
    pub fn set_pattern(&mut self, pattern: &str) -> GitResult<()> {
        let mut set = GlobSetBuilder::new();
        for line in pattern.lines() {
            for item in line.split(',') {
                match Glob::from_str(item.trim()) {
                    Ok(o) => {
                        set.add(o);
                    }
                    Err(e) => runtime_error(e.to_string())?,
                }
            }
        }
        match set.build() {
            Ok(o) => self.purge_path = o,
            Err(e) => runtime_error(e.to_string())?,
        }
        Ok(())
    }
}

impl GitCleaner {
    /// Create a new branch with does not contain the files that match the purge conditions.
    ///
    /// # Arguments
    ///
    /// * `old`: old branch name
    /// * `start`: commit to start from, None means initial commit
    /// * `new`: create new branch name
    fn prune(&mut self, old: &str, start: Oid, new: &str) -> Result<Branch, Error> {
        let old_branch = self.repo.find_branch(old, BranchType::Local)?;
        let target = old_branch.get().peel_to_commit()?;
        let new_branch = self.repo.branch(new, &target, true)?;
        let old_tree = old_branch.get().peel_to_tree()?;
        let mut tree_builder = self.repo.treebuilder(Some(&old_tree))?;
        // Iterate through the old tree entries
        for entry in old_tree.iter() {
            if let Ok(path) = self.should_remove(&entry) {
                tree_builder.remove(path)?;
            }
        }
        // Write the new tree
        let new_tree_oid = tree_builder.write()?;
        let new_tree = self.repo.find_tree(new_tree_oid)?;
        // Create a new commit for the new branch
        let old_commit = old_branch.get().peel_to_commit()?;
        let new_commit_oid = self.repo.commit(
            Some("HEAD"),
            &old_commit.author(),
            &old_commit.committer(),
            "Purge commit",
            &new_tree,
            &[&old_commit],
        )?;
        // Update the new branch reference to point to the new commit
        // new_branch.set_target(new_commit_oid, "Update new branch after purge")?;
        Ok(new_branch)
    }

    pub fn should_remove<'a>(&self, entry: &'a TreeEntry) -> GitResult<&'a [u8]> {
        let object = entry.to_object(&self.repo)?;
        let file_bytes = entry.name_bytes();
        let file_utf8 = String::from_utf8_lossy(file_bytes);
        let file_path = Path::new(file_utf8.as_ref());
        // Check if the file matches the purge conditions
        let should_remove = match self.purge_path.is_match(file_path) {
            true => true,
            false => {
                // TODO: check file size
                false
            }
        };
        if should_remove {
            return Ok(file_bytes);
        }
        Err(Error::from_str("File does not match purge conditions"))
    }
}
