#[cfg(target_os = "windows")]
const SHELL: &str = "cmd";

#[cfg(not(target_os = "windows"))]
const SHELL: &str = "sh";

#[cfg(target_os = "windows")]
const SHELL_FLAG: &str = "/C";

#[cfg(not(target_os = "windows"))]
const SHELL_FLAG: &str = "-c";

pub fn create_git_cli() -> std::process::Command {
    let mut shell = std::process::Command::new(SHELL);
    shell.arg(SHELL_FLAG).arg("git");
    shell
}
