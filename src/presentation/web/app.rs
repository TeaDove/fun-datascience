use crate::presentation::web::plot::draw_histogram;
use crate::shared::container;
use rocket::response::content;
use rocket::{get, routes, Build, Rocket};

#[get("/health")]
fn health() -> content::RawJson<&'static str> {
    content::RawJson("{\"success\":\"true\"}")
}

pub fn build_app() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![health])
        .mount("/plots", routes![draw_histogram])
        .manage(container::Container::must_new())
}
