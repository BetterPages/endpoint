use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex};

const STORAGE_PATH: &str = ".";
static MAPPINGS: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(init_mappings()));

pub fn init_mappings() -> HashMap<String, String> {
    let mut mappings = HashMap::new();
    mappings.insert("localhost/index.html".into(), "index.html".into());
    mappings.insert("localhost/404.html".into(), "404.html".into());
    mappings.insert("localhost/about.html".into(), "about.html".into());

    mappings
}

pub fn resolve(host: &str, path: &str) -> Option<PathBuf> {
    let storage_path = Path::new(STORAGE_PATH);
    let path = host.to_string() + path;

    let mappings = MAPPINGS.lock().unwrap();

    if let Some(path) = mappings.get(&path) {
        Some(storage_path.join(path))
    } else if let Some(path) = mappings.get(&format!("{path}.html")) {
        Some(storage_path.join(path))
    } else if let Some(path) = mappings.get(&format!("{path}index.html")) {
        Some(storage_path.join(path))
    } else if let Some(path) = mappings.get(&format!("{host}/404.html")) {
        Some(storage_path.join(path))
    } else {
        None
    }
}
