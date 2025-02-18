use std::sync::Arc;

use tauri::Runtime;
use tauri_plugin_store::Store;

pub const STORE_FILENAME: &str = "store.json";

const LAST_OPENED_REPOSITORY_KEY: &str = "lastOpenedRepository";
const OPENED_REPOSITORIES_KEY: &str = "openedRepositories";

pub type LastOpenedRepository = String;
pub type OpenedRepositories = Vec<String>;

pub fn update_last_opened_repository<R: Runtime>(store: &Arc<Store<R>>, repository: &str) {
    let last_opened_repository_option_value = store.get(LAST_OPENED_REPOSITORY_KEY);

    if let Some(last_opened_repository_value) = last_opened_repository_option_value {
        let last_opened_repository = serde_json::from_str::<LastOpenedRepository>(
            last_opened_repository_value.as_str().unwrap(),
        )
        .unwrap_or_default();

        if last_opened_repository != repository {
            store.set(LAST_OPENED_REPOSITORY_KEY, repository);
        }
    } else {
        store.set(LAST_OPENED_REPOSITORY_KEY, repository);
    }

    add_opened_repository(store, repository);
}

pub fn get_last_opened_repository<R: Runtime>(store: &Arc<Store<R>>) -> Vec<String> {
    let opened_repositories = store.get(OPENED_REPOSITORIES_KEY);

    if let Some(opened_repositories) = opened_repositories {
        serde_json::from_str::<OpenedRepositories>(opened_repositories.as_str().unwrap())
            .unwrap_or(vec![])
    } else {
        vec![]
    }
}

pub fn add_opened_repository<R: Runtime>(store: &Arc<Store<R>>, repository: &str) {
    let opened_repositories_option_value = store.get(OPENED_REPOSITORIES_KEY);

    if let Some(opened_repositories_value) = opened_repositories_option_value {
        let mut opened_repositories =
            serde_json::from_str::<OpenedRepositories>(opened_repositories_value.as_str().unwrap())
                .unwrap_or_default();

        if !opened_repositories.contains(&repository.to_string()) {
            opened_repositories.push(repository.to_string());
            store.set(OPENED_REPOSITORIES_KEY, opened_repositories);
        }
    } else {
        let mut opened_repositories = OpenedRepositories::new();
        opened_repositories.push(repository.to_string());

        store.set(OPENED_REPOSITORIES_KEY, opened_repositories);
    }
}

pub fn remove_opened_repository<R: Runtime>(store: &Arc<Store<R>>, repository: &str) {
    let opened_repositories_option_value = store.get(OPENED_REPOSITORIES_KEY);

    if let Some(opened_repositories_value) = opened_repositories_option_value {
        let mut opened_repositories =
            serde_json::from_str::<OpenedRepositories>(opened_repositories_value.as_str().unwrap())
                .unwrap_or_default();

        if let Some(index) = opened_repositories.iter().position(|r| r == repository) {
            opened_repositories.remove(index);
            store.set(OPENED_REPOSITORIES_KEY, opened_repositories);
        }
    }
}

pub fn clear_opened_repositories<R: Runtime>(store: &Arc<Store<R>>) {
    store.set(OPENED_REPOSITORIES_KEY, OpenedRepositories::new());
}
