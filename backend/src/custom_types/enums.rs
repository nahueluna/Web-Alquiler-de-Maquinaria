use serde::Deserialize;

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
