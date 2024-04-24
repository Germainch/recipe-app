use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub password_hash: String,
}
impl User {
    pub fn new(name: &str, password_hash: &str) -> Self {
        User {
            id: 0,
            name: name.to_string(),
            password_hash: password_hash.to_string(),
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
