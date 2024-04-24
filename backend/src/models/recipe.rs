use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::ingredient::Ingredient;


#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub id: i64,
    pub name: String,
    pub steps: String,
    pub created_by: Option<i64>,
}

impl Recipe {

    pub fn new(id: i64, name: &str, steps: &str, created_by: i64) -> Self {
        Recipe { id, name: name.to_string(), steps: steps.to_string(), created_by: Some(created_by) }
    }

    fn steps_to_string(steps: Vec<String>) -> String {
        let mut result = String::new();
        for step in steps {
            result.push_str(&step);
            result.push_str("\n");
        }
        result
    }

    fn string_to_steps(steps: String) -> Vec<String> {
        steps.split("\n").map(|s| s.to_string()).collect()
    }

    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn steps(&self) -> &str {
        &self.steps
    }
    pub fn set_id(&mut self, id: i64) {
        self.id = id;
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_steps(&mut self, steps: String) {
        self.steps = steps;
    }

}