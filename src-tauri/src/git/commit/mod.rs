use std::io::Read;

use serde::Serialize;

use crate::utils::subprocess::create_git_cli;

static GIT_LOG_PARAMS: &str = "--pretty=format:%H%n%aN%n%at%n%s%n";

#[derive(Serialize)]
pub struct Commit {
    pub hash: String,
    pub author: String,
    pub timestamp: u64,
    pub message: String,
}

impl Commit {
    pub fn new(hash: String, author: String, timestamp: u64, message: String) -> Self {
        Self {
            hash,
            author,
            timestamp,
            message,
        }
    }
}

pub fn get_commits() -> Vec<Commit> {
    let cli = create_git_cli()
        .args(["log", GIT_LOG_PARAMS])
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn cmd process");

    let mut output = String::new();

    if let Err(_) = cli.stdout.unwrap().read_to_string(&mut output) {
        return Vec::new();
    }

    let mut commits = Vec::new();
    output.split("\n\n").for_each(|chunk| {
        let mut lines = chunk.split("\n");
        let hash = lines.next().unwrap().to_string();
        let author = lines.next().unwrap().to_string();
        let timestamp = lines.next().unwrap().parse::<u64>().unwrap();
        let message = lines.next().unwrap().to_string();
        commits.push(Commit::new(hash, author, timestamp, message));
    });

    commits
}
