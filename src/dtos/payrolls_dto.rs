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
    pub gross_total: f32,
    pub net_total: f32,
    pub ss_total: f32,
    pub irpf_total: f32,
    
    // TODO: Per company and category data
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MultiplePayrollsResponse {
    pub data: Vec<PayrollsResponse>,
}