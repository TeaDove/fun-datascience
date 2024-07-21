use crate::tests_plot::combat_container;
use fun_datascience::shared::container::Container;
use rstest::rstest;

#[rstest]
pub fn test_plot_service_draw_bar_ok(combat_container: Container) {
    let f = combat_container.anime_prediction_service.predict().unwrap();
    println!("{}", f)
}
