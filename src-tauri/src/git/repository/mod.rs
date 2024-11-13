use std::path::PathBuf;

use crate::utils::subprocess::create_git_cli;

pub fn is_git_repository_found(path: &PathBuf) -> bool {
    let output = create_git_cli()
        .args(["rev-parse", "--is-inside-work-tree"])
        .current_dir(path)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn cmd process");

    let output = output.wait_with_output().expect("failed to read stdout");

    if !output.status.success() {
        return false;
    }

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    if !stderr.is_empty() {
        return false;
    }

    if stdout.is_empty() {
        return false;
    }

    stdout.trim() == "true"
}
