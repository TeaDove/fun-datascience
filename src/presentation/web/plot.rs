use crate::service::plot_schemas::{BarInput, GraphInput};
use crate::shared::container::Container;
use rocket::http::Status;
use rocket::post;
use rocket::response::content::RawHtml;
use rocket::response::status;
use rocket::serde::json::Json;

#[post("/histogram", format = "json", data = "<input>")]
pub fn draw_histogram(
    container: &rocket::State<Container>,
    input: Json<BarInput>,
) -> Result<RawHtml<String>, status::Custom<String>> {
    match container.plot_service.draw_bar(input.0) {
        Ok(v) => Ok(RawHtml(v)),
        Err(_v) => Err(status::Custom(Status::new(500), "ISE".to_string())),
    }
}

#[post("/heatmap", format = "json", data = "<input>")]
pub fn draw_heatmap(
    container: &rocket::State<Container>,
    input: Json<GraphInput>,
) -> Result<RawHtml<String>, status::Custom<String>> {
    match container.plot_service.draw_heatmap(input.0) {
        Ok(v) => Ok(RawHtml(v)),
        Err(_v) => Err(status::Custom(Status::new(500), "ISE".to_string())),
    }
}

#[post("/graph", format = "json", data = "<input>")]
pub fn draw_graph(
    container: &rocket::State<Container>,
    input: Json<GraphInput>,
) -> Result<RawHtml<String>, status::Custom<String>> {
    match container.plot_service.draw_graph(input.0) {
        Ok(v) => Ok(RawHtml(v)),
        Err(_v) => Err(status::Custom(Status::new(500), "ISE".to_string())),
    }
}
