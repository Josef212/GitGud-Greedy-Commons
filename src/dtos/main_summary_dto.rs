use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct GeneralDataDto {
    pub avg_expenses: f32,
    pub avg_savings: f32,
    pub financial_freedom: f32,
}

pub struct AccountDto {
    pub name: String,
    pub description: String,
    pub amount: f32,
    pub periodic_in_or_out: Option<f32>,
}

pub struct AccountsDataDto {
    pub total: f32,
    pub accounts: Vec<AccountDto>,
}

#[derive(Serialize, Deserialize)]
pub struct LoanDto {
    pub name: String,
    pub description: String,
    pub amount_per_month: f32,
    pub total: f32,
    pub pending_amount: f32,
    pub pending_months: u16,
}

#[derive(Serialize, Deserialize)]
pub struct LoansDataDto {
    pub loans: Vec<LoanDto>,
}

// TODO: Investments should be improved in general. Would be nice to have a table of investment expenses related to each account/investment to keep track of the actual profit of each investment
// So we would have the data of: 
//      - Initial amount
//      - All added money
//      - Current amount
//      - current - (initial + sum(added_money)) => actual_profit
#[derive(Serialize, Deserialize)]
pub struct InvestmentDto {
    pub name: String,
    pub description: String,
    pub initial: f32,
    pub current: f32,
    pub periodic_in_or_out: Option<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct InvestmentDataDto {
    pub investments: Vec<InvestmentDto>,
}

#[derive(Serialize, Deserialize)]
pub struct MainSummaryDto {
    pub date: String,
    pub general: GeneralDataDto,
    pub loans: LoansDataDto,
    pub investments: InvestmentDataDto,
}