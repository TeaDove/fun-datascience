use crate::shared::container::Container;
use rocket::post;
use rocket::response::content::RawHtml;

#[post("/histogram")]
pub fn draw_histogram(container: &rocket::State<Container>) -> RawHtml<String> {
    RawHtml(container.plot_service.draw_bar().unwrap())
}
