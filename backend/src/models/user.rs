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
}
