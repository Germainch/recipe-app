
#[derive(Debug)]
pub struct Ingredient {
    pub id: u32,
    pub name: String,
}

impl Ingredient {
    pub fn new(name: String) -> Self {
        Ingredient {
            id : 1,
            name,
        }
    }
}



