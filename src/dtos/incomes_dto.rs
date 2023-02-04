use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct IncomeDto {
    pub _id: i32,
    pub name: String,
    pub amount: f32,
    pub date: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IncomesResponse {
    pub incomes: Vec<IncomeDto>,
    pub total: f32,
}
