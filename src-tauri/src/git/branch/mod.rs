use std::path::PathBuf;

use serde::Serialize;

use crate::utils::subprocess::create_git_cli;

#[derive(Debug, Serialize)]
pub struct Branch {
    pub name: String,
    pub is_current: bool,
    pub is_remote: bool,
    pub is_head: bool,
    pub remote: Option<String>,
}

fn parse_branches(output: &str) -> Vec<Branch> {
    let mut branches = Vec::new();

    let mut lines = output.lines();

    while let Some(line) = lines.next() {
        let line = line.trim();

        // 1. Check if the line is empty
        if line.is_empty() {
            continue;
        }

        // 2. Parse the branch
        // 2.1. Check if branch is current
        let is_current = line.starts_with('*');

        // 2.2. Check if branch is remote
        let is_remote = line.starts_with("remotes/");

        // 2.3. Check if branch is HEAD
        let is_head = line.contains("HEAD ->");

        // 2.4. Get the branch name
        let name = if is_remote {
            let name = line.split_whitespace().last().unwrap();
            let last_slash = name.rfind('/');

            if let Some(last_slash) = last_slash {
                let name = &name[last_slash + 1..];
                name
            } else {
                name
            }
        } else {
            let name = if is_current { &line[1..] } else { line };
            name
        };
        let name = name.trim();

        // 2.5. Get the remote name
        let remote = if is_head {
            let remote = line.split(" -> ").collect::<Vec<&str>>()[0];
            Some(remote.to_string())
        } else if is_remote {
            Some(line.to_string())
        } else {
            None
        };

        // 2.6. Create the branch
        let branch = Branch {
            name: name.to_string(),
            is_current,
            is_remote,
            is_head,
            remote,
        };

        branches.push(branch);
    }

    branches
}

pub fn get_branch_list(dir: &PathBuf) -> Vec<Branch> {
    let output = create_git_cli()
        .args(["branch", "-a"])
        .current_dir(dir)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn cmd process");

    let output = output.wait_with_output().expect("failed to read stdout");

    if !output.status.success() {
        return Vec::new();
    }

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    if !stderr.is_empty() {
        return Vec::new();
    }

    if stdout.is_empty() {
        return Vec::new();
    }

    parse_branches(&stdout)
}

pub type CurrentBranch = String;

fn parse_current_branch(output: &str) -> Option<CurrentBranch> {
    let output = output.trim();

    if output.is_empty() {
        return None;
    }

    Some(output.to_string())
}

pub fn get_current_branch(dir: &PathBuf) -> Option<CurrentBranch> {
    let output = create_git_cli()
        .args(["branch", "--show-current"])
        .current_dir(dir)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn cmd process");

    let output = output.wait_with_output().expect("failed to read stdout");

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    if !stderr.is_empty() {
        return None;
    }

    if stdout.is_empty() {
        return None;
    }

    parse_current_branch(&stdout)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_branches() {
        let output =
            "* main\n  feature-branch\n  remotes/origin/main\n  remotes/origin/feature-branch";
        let branches = parse_branches(output);

        assert_eq!(branches.len(), 4);
        assert_eq!(branches[0].name, "main");
        assert_eq!(branches[0].is_current, true);
        assert_eq!(branches[0].is_remote, false);
        assert_eq!(branches[0].is_head, false);
        assert_eq!(branches[0].remote.is_none(), true);

        assert_eq!(branches[1].name, "feature-branch");
        assert_eq!(branches[1].is_current, false);
        assert_eq!(branches[1].is_remote, false);
        assert_eq!(branches[1].is_head, false);
        assert_eq!(branches[1].remote.is_none(), true);

        assert_eq!(branches[2].name, "main");
        assert_eq!(branches[2].is_current, false);
        assert_eq!(branches[2].is_remote, true);
        assert_eq!(branches[2].is_head, false);
        assert_eq!(branches[2].remote, Some("remotes/origin/main".to_string()));

        assert_eq!(branches[3].name, "feature-branch");
        assert_eq!(branches[3].is_current, false);
        assert_eq!(branches[3].is_remote, true);
        assert_eq!(branches[3].is_head, false);
        assert_eq!(
            branches[3].remote,
            Some("remotes/origin/feature-branch".to_string())
        );
    }

    #[test]
    fn test_parse_current_branch() {
        let output = "main";
        let current_branch = parse_current_branch(output);

        assert!(current_branch.is_some());
        assert_eq!(current_branch.unwrap(), "main");
    }
}
