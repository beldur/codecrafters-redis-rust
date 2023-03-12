use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

pub struct Store {
    data: HashMap<String, Entry>,
}

struct Entry {
    value: String,
    t: Option<Instant>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, Entry { value, t: None });
    }

    pub fn get(&mut self, key: String) -> Option<String> {
        match self.data.get(key.as_str()) {
            Some(entry) => {
                if let Some(t) = entry.t {
                    if Instant::now() > t {
                        self.data.remove(key.as_str());
                        return None;
                    }
                }

                Some(entry.value.clone())
            }
            None => None,
        }
    }

    pub fn set_with_expiry(&mut self, key: String, value: String, px: u64) {
        let entry = Entry {
            value,
            t: Some(Instant::now() + Duration::from_millis(px)),
        };

        self.data.insert(key, entry);
    }
}
