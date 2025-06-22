use serde::{Deserialize, Serialize};
use std::fmt;

pub enum RunningEnv {
    Production,
    Testing,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderByField {
    Price,
    Rating,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderDirection {
    Asc,
    Desc,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    Approved,
    Pending,
    Rejected,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UnitStatusEvents {
    Available,
    Maintenance,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StatType {
    Rentals,
    Income,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum StatGroupBy {
    Month,
    Employee,
    Category,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StatOrder {
    Asc,
    Desc,
}

impl fmt::Display for StatOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            StatOrder::Asc => "ASC",
            StatOrder::Desc => "DESC",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ReviewOrder{
    MoreRating,
    LessRating,
    Recent,
}
