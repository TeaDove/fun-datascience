use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlotInput {
    pub title: Option<String>,
    pub x_title: Option<String>,
    pub y_title: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BarInput {
    #[serde(flatten)]
    pub plot: PlotInput,

    pub values: HashMap<String, f64>,
    pub limit: Option<u32>,
    pub asc: Option<bool>,
}
