use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct User {
    id: uuid::Uuid,
    name: String,
    password_hash: String,
    inserted_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl User {
    pub fn new(name: String, password_hash: String) -> Self {
        User {
            id: uuid::Uuid::new_v4(),
            name,
            password_hash,
            inserted_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    pub fn id(&self) -> uuid::Uuid {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn password_hash(&self) -> &str {
        &self.password_hash
    }
    pub fn inserted_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.inserted_at
    }
    pub fn updated_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.updated_at
    }
    pub fn set_id(&mut self, id: uuid::Uuid) {
        self.id = id;
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_password_hash(&mut self, password_hash: String) {
        self.password_hash = password_hash;
    }
    pub fn set_inserted_at(&mut self, inserted_at: chrono::DateTime<chrono::Utc>) {
        self.inserted_at = inserted_at;
    }
    pub fn set_updated_at(&mut self, updated_at: chrono::DateTime<chrono::Utc>) {
        self.updated_at = updated_at;
    }
}
