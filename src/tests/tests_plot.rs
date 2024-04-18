use crate::shared::container::Container;
use rstest::{fixture, rstest};
use std::fs;

#[fixture]
pub fn combat_container() -> Container {
    Container::must_new()
}

#[rstest]
pub fn test_plot_service_draw_bar_ok(combat_container: Container) {
    let html = combat_container.plot_service.draw_bar().unwrap();

    fs::write("draw_bar_ok.html", html).unwrap();
}
