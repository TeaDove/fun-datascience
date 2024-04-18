use crate::service::plot_service;

#[derive(Clone)]
pub(crate) struct Container {
    pub plot_service: plot_service::Service,
}

impl Container {
    pub fn must_new() -> Self {
        let plot_service = plot_service::Service::new().unwrap();

        Container { plot_service }
    }
}
