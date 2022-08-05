use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TagDto {
    pub _id: i32,
    pub name: String,
    pub description: String,
}