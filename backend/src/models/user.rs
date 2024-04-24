use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct User {
    id: i64,
    name: String,
    password_hash: String,
}
impl User {
    pub fn new(name: String, password_hash: String) -> Self {
        User {
            id: 0,
            name,
            password_hash,
        }
    }

    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn password_hash(&self) -> &str {
        &self.password_hash
    }
    pub fn set_id(&mut self, id: i64) {
        self.id = id;
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_password_hash(&mut self, password_hash: String) {
        self.password_hash = password_hash;
    }
}
