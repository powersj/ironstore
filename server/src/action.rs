use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Append the key value into the database.
pub fn append(shared_data: Arc<Mutex<HashMap<String, String>>>, key: &String, value: &String) -> Result<String, String> {
    let mut data = shared_data.lock().unwrap();
    data.entry(key.to_string())
        .and_modify(|existing| existing.push_str(value))
        .or_insert(value.to_string());

    Ok("Value appended successfully".to_string())
}

/// Delete a key in the database.
pub fn del(shared_data: Arc<Mutex<HashMap<String, String>>>, key: &String) -> Result<String, String> {
    let mut data = shared_data.lock().unwrap();
    data.remove(&key.to_string());
    Ok("Value deleted successfully".to_string())
}


/// Flush all items from database.
pub fn flushall(shared_data: Arc<Mutex<HashMap<String, String>>>) -> Result<String, String> {
    shared_data.lock().unwrap().clear();
    Ok("All values deleted successfully".to_string())
}

/// Get a single key from database.
pub fn get(shared_data: Arc<Mutex<HashMap<String, String>>>, key: &String) -> Result<String, String> {
    let data = shared_data.lock().unwrap();
    match data.get(&key.to_string()) {
        Some(value) => Ok(value.clone()),
        None => Err("Key not found".to_string()),
    }
}

/// Return if a key exists
pub fn exists(shared_data: Arc<Mutex<HashMap<String, String>>>, key: &String) -> Result<String, String> {
    let data = shared_data.lock().unwrap();
    match data.get(&key.to_string()) {
        Some(_) => Ok("1".to_string()),
        None => Err("0".to_string()),
    }
}

/// Return info of server.
pub fn info() -> Result<String, String> {
    Ok("Server OK".to_string())
}

/// Return all known keys.
pub fn keys(shared_data: Arc<Mutex<HashMap<String, String>>>) -> Result<String, String> {
    let data: std::sync::MutexGuard<'_, HashMap<String, String>> = shared_data.lock().unwrap();
    if data.is_empty() {
        return Ok("no keys".to_string());
    }

    let keys: Vec<String> = data.keys().cloned().collect();
    let keys_str = keys.join("\n");
    Ok(keys_str)
}

/// Ping the server.
pub fn ping() -> Result<String, String> {
    Ok("pong".to_string())
}

/// Set the key value into the database.
pub fn set(shared_data: Arc<Mutex<HashMap<String, String>>>, key: &String, value: &String) -> Result<String, String> {
    let mut data = shared_data.lock().unwrap();
    data.insert(key.to_string(), value.to_string());
    Ok("Value set successfully".to_string())
}
