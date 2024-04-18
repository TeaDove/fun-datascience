use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Plot {
    pub title: String,
    pub x_title: String,
    pub y_title: String,
}
