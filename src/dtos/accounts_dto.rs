use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AccountDto {
    pub _id: i32,
    pub name: String,
    pub amount: f32,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountsResponse {
    pub payrolls: Vec<AccountDto>,
}
