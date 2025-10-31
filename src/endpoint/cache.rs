use dashmap::DashMap;
use std::sync::{LazyLock, Mutex};
use std::time::{Duration, Instant};

use crate::grpc::request::Response;

const TTL: Duration = Duration::from_secs(10);

static CACHE: LazyLock<DashMap<(String, String), CacheEntry>> = LazyLock::new(|| DashMap::new());
static CACHE_TTL: LazyLock<Mutex<Vec<(String, String, Instant)>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

struct CacheEntry {
    response: Response,
}

pub fn get_cache_entry(host: String, path: String) -> Option<Response> {
    println!("get_cache_entry");
    try_collect_garbage();

    let cache_ref = (host, path);
    let cache_res = CACHE.get(&cache_ref);

    match cache_res {
        Some(cache_entry) => Some(cache_entry.response.clone()),
        None => None,
    }
}

pub fn insert_cache_entry(host: String, path: String, response: Response) {
    println!("insert_cache_entry");
    let cache_entry = CacheEntry { response };

    try_collect_garbage();

    CACHE.insert((host.clone(), path.clone()), cache_entry);
    let mut cache_ttl = CACHE_TTL.lock().unwrap();
    cache_ttl.push((host, path, Instant::now()));
}

fn try_collect_garbage() {
    println!("try_collect_garbage");
    let mut cache_ttl = CACHE_TTL.lock().unwrap();
    if let Some(entry) = cache_ttl.first() {
        if entry.2.elapsed() > TTL {
            CACHE.remove(&(entry.0.clone(), entry.1.clone()));
            cache_ttl.remove(0);
        }
    }
}
