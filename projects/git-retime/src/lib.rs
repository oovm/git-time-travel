mod errors;

pub use errors::{Error, Result};

use git2::Repository;

#[test]
fn test() {
    let repo = match Repository::open("/path/to/a/repo") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
}