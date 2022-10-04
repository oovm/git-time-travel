use git_utils::{find_closest_git_repo, reword_root_commit};

#[test]
fn test_reword() {
    let repo = find_closest_git_repo().unwrap();
    reword_root_commit(&repo, "new message", "new_branch").unwrap();
}
