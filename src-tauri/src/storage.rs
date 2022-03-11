trait Storage {
    fn get_item(&mut self, key: &str) -> Option<String>;
    fn set_item(&mut self, key: &str, value: &str);
    fn remove_item(&mut self, key: &str);
    fn clear(&mut self);
    fn length(&self) -> usize;
}

pub struct LocalStorage {
    pub space: String,
}

impl Default for LocalStorage {
    fn default() -> Self {
        LocalStorage {
            space: "".to_string(),
        }
    }
}

impl Storage for LocalStorage {
    fn get_item(&mut self, key: &str) -> Option<String> {
        todo!()
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
}
