use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CompanyDto {
    pub _id: u32,
    pub name: String,
    pub description: String,
}
