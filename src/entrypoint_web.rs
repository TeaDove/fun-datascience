use fun_datascience::presentation::web::app::build_app;
#[macro_use]
extern crate rocket;

#[launch]
fn run() -> _ {
    build_app()
}
