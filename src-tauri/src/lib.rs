mod git;
mod utils;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_commits() -> Vec<git::commit::Commit> {
    git::commit::get_commits()
}

#[tauri::command]
fn get_commit(hash: &str) -> Option<git::commit::CommitWithDiff> {
    git::commit::get_commit(hash)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, get_commits, get_commit])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
