use std::collections::HashMap;

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Self {
        Database {
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }

    fn update(&mut self, key: String, value: String) {
        if self.map.contains_key(&key) {
            self.map.insert(key, value);
        } else {
            println!("Error: Key {} not found", key);
        }
    }

    fn delete(&mut self, key: &str) {
        self.map.remove(key);
    }
}


fn main() {
    let mut db = Database::new();
    db.insert("key1".to_string(), "value1".to_string());
    db.insert("key2".to_string(), "value2".to_string());
    println!("{:?}", db.get("key1"));
    db.update("key1".to_string(), "newvalue1".to_string());
    println!("{:?}", db.get("key1"));
    db.delete("key2");
    println!("{:?}", db.get("key2"));
}