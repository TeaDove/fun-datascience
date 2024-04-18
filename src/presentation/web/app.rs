use rocket::{Build, get, Rocket, routes};
use rocket::response::content;

#[get("/health")]
fn health() -> content::RawJson<&'static str> {
    content::RawJson("{\"success\":\"true\"}")
}

pub fn build_app() -> Rocket<Build>{
    return rocket::build()
        .mount("/", routes![health])
}
