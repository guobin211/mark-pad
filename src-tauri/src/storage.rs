use std::borrow::Borrow;
use std::collections::HashMap;

trait Storage {
    fn get_item(&mut self, key: &str) -> Option<&String>;
    fn set_item(&mut self, key: &str, value: &str);
    fn remove_item(&mut self, key: &str);
    fn clear(&mut self);
    fn length(&self) -> usize;
    fn for_each(&self, f: &mut dyn FnMut(&str, &str));
}

pub struct LocalStorage {
    store: HashMap<String, String>,
    pub space: String,
}

impl Default for LocalStorage {
    fn default() -> Self {
        LocalStorage {
            space: "".to_string(),
            store: HashMap::new(),
        }
    }
}

impl Storage for LocalStorage {
    fn get_item(&mut self, key: &str) -> Option<&String> {
        self.store.borrow().get(key)
    }

    fn set_item(&mut self, key: &str, value: &str) {
        todo!()
    }

    fn remove_item(&mut self, key: &str) {
        todo!()
    }

    fn clear(&mut self) {
        todo!()
    }

    fn length(&self) -> usize {
        todo!()
    }

    fn for_each(&self, f: &mut dyn FnMut(&str, &str)) {
        self.store.borrow().iter().for_each(|(k, v)| f(k, v));
    }
}
