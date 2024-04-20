use crate::service::plot_schemas::Bar;
use crate::shared::container::Container;
use rocket::http::Status;
use rocket::post;
use rocket::response::content::RawHtml;
use rocket::response::status;
use rocket::serde::json::Json;

#[post("/histogram", format = "json", data = "<input>")]
pub fn draw_histogram(
    container: &rocket::State<Container>,
    input: Json<Bar>,
) -> Result<RawHtml<String>, status::Custom<String>> {
    match container.plot_service.draw_bar(input.0) {
        Ok(v) => Ok(RawHtml(v)),
        Err(_v) => Err(status::Custom(Status::new(500), "ISE".to_string())),
    }
}
