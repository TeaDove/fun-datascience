use crate::service::plot_service;

pub(crate) struct Container{
    plot_service: *plot_service::Service
}

impl Container{
    pub fn must_new() -> Self{
        let plot_service = plot_service::Service::new().unwrap();

        return Container{plot_service}
    }
}
