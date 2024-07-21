use charming::datatype::CompositeValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlotInput {
    pub title: Option<String>,
    pub x_title: Option<String>,
    pub y_title: Option<String>,
    pub zoom_end_value: Option<u32>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BarInput {
    #[serde(flatten)]
    pub plot: PlotInput,

    pub values: HashMap<String, f64>,
    pub asc: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LineInput {
    #[serde(flatten)]
    pub plot: PlotInput,

    pub values: Vec<(String, CompositeValue, CompositeValue)>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EdgeInput {
    pub first: String,
    pub second: String,
    pub weight: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NodeInput {
    pub weight: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GraphInput {
    #[serde(flatten)]
    pub plot: PlotInput,

    pub edges: Vec<EdgeInput>,
    pub nodes: HashMap<String, NodeInput>,
}
