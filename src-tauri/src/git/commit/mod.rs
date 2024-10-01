use std::io::Read;

use serde::Serialize;

use crate::utils::subprocess::create_git_cli;

#[derive(Serialize)]
pub struct Commit {
    pub hash: String,
    pub author: String,
    pub timestamp: u64,
    pub message: String,
    pub description: Option<String>,
}

impl Commit {
    pub fn new(
        hash: String,
        author: String,
        timestamp: u64,
        message: String,
        description: Option<String>,
    ) -> Self {
        Self {
            hash,
            author,
            timestamp,
            message,
            description,
        }
    }
}

fn parse_single_commit(output: &str) -> Commit {
    let mut lines = output.split("\n");
    let hash = lines.next().unwrap().to_string();
    let author = lines.next().unwrap().to_string();
    let timestamp = lines.next().unwrap().parse::<u64>().unwrap();
    let message = lines.next().unwrap().to_string();

    Commit::new(hash, author, timestamp, message, None)
}

fn parse_commits(output: &str) -> Vec<Commit> {
    let mut commits = Vec::new();
    output.split("\n\n").for_each(|chunk| {
        let commit = parse_single_commit(chunk);
        commits.push(commit);
    });

    commits
}

pub fn get_commits() -> Vec<Commit> {
    let cli = create_git_cli()
        .args(["log", "--pretty=format:%H%n%aN%n%at%n%s%n"])
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn cmd process");

    let mut output = String::new();

    if let Err(_) = cli.stdout.unwrap().read_to_string(&mut output) {
        return Vec::new();
    }

    parse_commits(&output)
}

#[derive(Serialize)]
pub struct FileDiff {
    file_name: String,
    additions: u64,
    deletions: u64,
}

impl FileDiff {
    pub fn new(file_name: String, additions: u64, deletions: u64) -> Self {
        Self {
            file_name,
            additions,
            deletions,
        }
    }
}

#[derive(Serialize)]
pub struct CommitWithDiff {
    pub commit: Commit,
    pub diff: Vec<FileDiff>,
}

impl CommitWithDiff {
    pub fn new(commit: Commit, diff: Vec<FileDiff>) -> Self {
        Self { commit, diff }
    }
}

fn parse_commit(output: &str, hash: &str) -> CommitWithDiff {
    let mut lines = output.split("\n");
    let author = lines.next().unwrap().to_string();
    let timestamp = lines.next().unwrap().parse::<u64>().unwrap();
    let message = lines.next().unwrap().to_string();

    let mut description = String::new();

    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }

        description.push_str(line);
        description.push_str("\n");
    }

    let mut diff = Vec::new();

    while let Some(line) = lines.next() {
        println!("{}", line);
        if line.trim() == "" {
            continue;
        }

        let parts: Vec<&str> = line.trim().split_whitespace().collect::<Vec<&str>>();

        let additions = parts[0].parse::<u64>().unwrap_or(0);
        let deletions = parts[1].parse::<u64>().unwrap_or(0);
        let file_name = parts[2].to_string();

        diff.push(FileDiff::new(file_name, additions, deletions));
    }

    let commit = Commit::new(
        hash.to_string(),
        author,
        timestamp,
        message,
        Some(description),
    );

    CommitWithDiff::new(commit, diff)
}

pub fn get_commit(hash: &str) -> Option<CommitWithDiff> {
    let cli = create_git_cli()
        .args([
            "show",
            hash,
            "--pretty=format:%aN%n%at%n%s%n%b%n",
            "--numstat",
        ])
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn cmd process");

    let mut output = String::new();

    if let Err(_) = cli.stdout.unwrap().read_to_string(&mut output) {
        return None;
    }

    Some(parse_commit(&output, hash))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_commit_parsing() {
        let output = "797a1acbdec5e0db0edad56ee53e009b65be52ff
Damian Kocjan
1727789190
fix: update tailwindcss file name in shadcn config";

        let commit = parse_single_commit(output);

        assert_eq!(commit.hash, "797a1acbdec5e0db0edad56ee53e009b65be52ff");
        assert_eq!(commit.author, "Damian Kocjan");
        assert_eq!(commit.timestamp, 1727789190);
        assert_eq!(
            commit.message,
            "fix: update tailwindcss file name in shadcn config"
        );
    }

    #[test]
    fn test_commits_parsing() {
        let output = "797a1acbdec5e0db0edad56ee53e009b65be52ff
Damian Kocjan
1727789190
fix: update tailwindcss file name in shadcn config

38c53a749226dc9b7fbc71d0cbe5f5c9cb2f287e
Damian Kocjan
1727786432
feat: get commits

c48207137657e4ecf08563db4ca43e03392cddaf
Damian Kocjan
1727694652
test: add test config

383ceb3b61672aca95f50622afd4e8a6f7b3fe35
Damian Kocjan
1727694637
chore: change tailwindcss config to ts file

3d0dfe5bff45ab179e56902801fead74b73442ba
Damian Kocjan
1727650366
style: change to `New York` shadcn style

86b27dfb437f7d35d9183c7045dd8d8e1fac4864
Damian Kocjan
1727131415
add shadcn

c0ce0b88c373589159ad61deba7e6c1c29f2af02
Damian Kocjan
1727130272
init
";

        let commits = parse_commits(output);

        assert_eq!(commits.len(), 7);

        assert_eq!(commits[0].hash, "797a1acbdec5e0db0edad56ee53e009b65be52ff");
        assert_eq!(commits[0].author, "Damian Kocjan");
        assert_eq!(commits[0].timestamp, 1727789190);
        assert_eq!(
            commits[0].message,
            "fix: update tailwindcss file name in shadcn config"
        );

        assert_eq!(commits[1].hash, "38c53a749226dc9b7fbc71d0cbe5f5c9cb2f287e");
        assert_eq!(commits[1].author, "Damian Kocjan");
        assert_eq!(commits[1].timestamp, 1727786432);
        assert_eq!(commits[1].message, "feat: get commits");
    }

    #[test]
    fn test_commit_parsing() {
        let output = "Damian Kocjan
1727789190
fix: update tailwindcss file name in shadcn config

0      0       src-tauri/src/git/commit/mod.rs
1      1       src-tauri/src/git/commit.rs
10     0       src-tauri/src/utils/subprocess.rs
0      10      src-tauri/src/utils/mod.rs";

        let commit = parse_commit(output, "797a1acbdec5e0db0edad56ee53e009b65be52ff");

        assert_eq!(
            commit.commit.hash,
            "797a1acbdec5e0db0edad56ee53e009b65be52ff"
        );
        assert_eq!(commit.commit.author, "Damian Kocjan");
        assert_eq!(commit.commit.timestamp, 1727789190);
        assert_eq!(
            commit.commit.message,
            "fix: update tailwindcss file name in shadcn config"
        );
        assert_eq!(commit.diff.len(), 4);
        assert_eq!(commit.diff[0].file_name, "src-tauri/src/git/commit/mod.rs");

        assert_eq!(commit.diff[0].additions, 0);
        assert_eq!(commit.diff[0].deletions, 0);

        assert_eq!(commit.diff[1].file_name, "src-tauri/src/git/commit.rs");
        assert_eq!(commit.diff[1].additions, 1);
        assert_eq!(commit.diff[1].deletions, 1);

        assert_eq!(
            commit.diff[2].file_name,
            "src-tauri/src/utils/subprocess.rs"
        );
        assert_eq!(commit.diff[2].additions, 10);
        assert_eq!(commit.diff[2].deletions, 0);

        assert_eq!(commit.diff[3].file_name, "src-tauri/src/utils/mod.rs");
        assert_eq!(commit.diff[3].additions, 0);
        assert_eq!(commit.diff[3].deletions, 10);
    }
}
