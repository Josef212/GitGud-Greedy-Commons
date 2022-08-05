use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CategoryDto {
    pub _id: i32,
    pub name: String,
    pub description: String,
}
