use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub trait Cache {
    fn new() -> Self;
    fn get(&mut self, key: &str) -> Option<String>;
    fn set(&mut self, key: &str, value: &str, ttl: Option<u64>);
    fn remove(&mut self, key: &str);
    fn hash_key(key: &str) -> String {
        // Hash the key with SHA256 and return it as a hexadecimal string
        let mut hasher = Sha256::new();
        hasher.update(key);
        let result = hasher.finalize();

        format!("{:x}", result)
    }
}

#[derive(Debug, Clone)]
pub struct InMemoryCache {
    pub cache: HashMap<String, (String, Option<u64>)>, // Value and Optional TTL timestamp
}

impl Cache for InMemoryCache {
    fn new() -> InMemoryCache {
        InMemoryCache {
            cache: HashMap::new(),
        }
    }

    fn set(&mut self, key: &str, value: &str, ttl: Option<u64>) {
        // calculate ttl_timestamp
        let ttl_timestamp = ttl.map(|ttl| {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs()
                + ttl as u64 // Current time + ttl
        });

        self.cache.insert(
            InMemoryCache::hash_key(key),
            (value.to_string(), ttl_timestamp),
        );
    }

    fn get(&mut self, key: &str) -> Option<String> {
        if let Some((value, ttl)) = self.cache.get(&InMemoryCache::hash_key(key)) {
            // remove expired entries
            if let Some(expiration_time) = ttl {
                let current_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_secs();
                if current_time > *expiration_time {
                    self.remove(key);

                    return None; // TTL expired
                }
            }

            return Some(value.clone()); // Valid cache entry
        }

        None
    }

    fn remove(&mut self, key: &str) {
        self.cache.remove(&InMemoryCache::hash_key(key));
    }
}
