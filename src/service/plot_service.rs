use plotly::{Plot, Scatter};

#[derive(Clone)]
pub struct Service {}

impl Service {
    pub fn new() -> Result<Service, Box<dyn std::error::Error>> {
        Ok(Service {})
    }

    pub fn draw_bar(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut plot = Plot::new();
        let trace = Scatter::new(vec![0, 1, 2], vec![2, 1, 0]);
        plot.add_trace(trace);

        Ok(plot.to_html())
    }
}
