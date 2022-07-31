use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TransactionDto {
    pub _id: i32,
    pub name: String,
    pub date: String,
    pub amount: f32,
    pub tag: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionsResponse {
    pub transactions: Vec<TransactionDto>,
    pub total: f32,
    pub tag_data: HashMap<i32, f32>,
    pub tag_count: HashMap<i32, u32>,
}
