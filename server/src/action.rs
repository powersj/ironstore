use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub async fn del(shared_data: Arc<Mutex<HashMap<String, String>>>, key: String) -> Result<String, String> {
    let mut data = shared_data.lock().unwrap();
    data.remove(&key);
    Ok("Value deleted successfully".to_string())
}

pub async fn get(shared_data: Arc<Mutex<HashMap<String, String>>>, key: String) -> Result<String, String> {
    let data = shared_data.lock().unwrap();
    match data.get(&key) {
        Some(value) => Ok(value.clone()),
        None => Err("Key not found".to_string()),
    }
}

pub async fn set(shared_data: Arc<Mutex<HashMap<String, String>>>, key: String, value: String) -> Result<String, String> {
    let mut data = shared_data.lock().unwrap();
    data.insert(key, value);
    Ok("Value set successfully".to_string())
}
