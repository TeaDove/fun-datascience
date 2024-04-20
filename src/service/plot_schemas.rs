use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Plot {
    pub title: Option<String>,
    pub x_title: Option<String>,
    pub y_title: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Bar {
    #[serde(flatten)]
    pub plot: Plot,

    pub values: HashMap<String, f64>,
    pub limit: Option<u32>,
    pub asc: Option<bool>,
}
