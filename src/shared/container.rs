use crate::service::anime_detection_service;
use crate::service::plot_service;

#[derive(Clone, Debug)]
pub struct Container {
    pub plot_service: plot_service::Service,
    pub anime_prediction_service: anime_detection_service::Service,
}

impl Container {
    pub fn must_new() -> Self {
        let plot_service = plot_service::Service::new().unwrap();
        let anime_prediction_service = anime_detection_service::Service::new().unwrap();

        Container {
            plot_service,
            anime_prediction_service,
        }
    }
}
