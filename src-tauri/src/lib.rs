use std::sync::Mutex;

use tauri::Runtime;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_store::StoreExt;

use git::repository::is_git_repository_found;
use utils::persistent_storage::{update_last_opened_repository, STORE_FILENAME};

mod git;
mod utils;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_commits(state: tauri::State<'_, AppState>) -> Vec<git::commit::Commit> {
    git::commit::get_commits(&state.cwd.lock().unwrap().to_path_buf())
}

#[tauri::command]
fn get_commit(
    state: tauri::State<'_, AppState>,
    hash: &str,
) -> Option<git::commit::CommitWithDiff> {
    git::commit::get_commit(&state.cwd.lock().unwrap().to_path_buf(), hash)
}

#[tauri::command]
fn get_commit_file_changes(
    state: tauri::State<'_, AppState>,
    hash: &str,
    file: &str,
) -> Option<String> {
    git::commit::get_commit_file_changes(&state.cwd.lock().unwrap().to_path_buf(), hash, file)
}

#[tauri::command]
fn get_current_branch(state: tauri::State<'_, AppState>) -> Option<git::branch::CurrentBranch> {
    git::branch::get_current_branch(&state.cwd.lock().unwrap().to_path_buf())
}

#[tauri::command]
fn get_branch_list(state: tauri::State<'_, AppState>) -> Vec<git::branch::Branch> {
    git::branch::get_branch_list(&state.cwd.lock().unwrap().to_path_buf())
}

#[tauri::command]
fn open_repository<R: Runtime>(
    app: tauri::AppHandle<R>,
    state: tauri::State<'_, AppState>,
) -> bool {
    let folder_path = app.dialog().file().blocking_pick_folder();

    if let Some(file_path) = folder_path {
        let folder_path = file_path.into_path().unwrap();

        if !is_git_repository_found(&folder_path) {
            app.dialog()
                .message("Not a git repository")
                .kind(tauri_plugin_dialog::MessageDialogKind::Error)
                .blocking_show();
            return false;
        }

        let store = app.store(STORE_FILENAME).unwrap();

        update_last_opened_repository(&store, folder_path.to_str().unwrap());

        let mut cwd = state.cwd.lock().unwrap();
        *cwd = folder_path;
    }

    true
}

#[tauri::command]
fn change_branch<R: Runtime>(
    app: tauri::AppHandle<R>,
    state: tauri::State<'_, AppState>,
    branch: &str,
) -> bool {
    let output = git::branch::change_branch(&state.cwd.lock().unwrap().to_path_buf(), branch);

    if !output {
        app.dialog()
            .message("Failed to change branch")
            .kind(tauri_plugin_dialog::MessageDialogKind::Error)
            .blocking_show();
        return false;
    }

    true
}

struct AppState {
    cwd: Mutex<std::path::PathBuf>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            cwd: Mutex::new(std::env::current_dir().unwrap()),
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            change_branch,
            open_repository,
            greet,
            get_commits,
            get_commit,
            get_commit_file_changes,
            get_current_branch,
            get_branch_list
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
