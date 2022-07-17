use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PayrollDto {
    pub _id: i32,
    pub date: String,
    pub gross: f32,
    pub net: f32,
    pub ss: f32,
    pub irpf: f32,
    pub company_id: i32,
    pub category_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PayrollsResponse {
    pub payrolls: Vec<PayrollDto>,
}