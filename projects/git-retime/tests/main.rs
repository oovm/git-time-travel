use git_retime::GitTimeTravel;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
#[ignore]
fn test() {
    GitTimeTravel {
        commit: "2a990148".to_string(),
        start_date: "2022-8-22".to_string(),
        end_date: None,
        branch: Some("dev".to_string()),
    }
    .run()
    .unwrap();
}
