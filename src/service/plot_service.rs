use crate::service::plot_schemas::{Bar, Plot};
use plotly;
use plotly::common::Title;
use plotly::layout::Axis;

#[derive(Clone)]
pub struct Service {}

impl Service {
    pub fn new() -> Result<Service, Box<dyn std::error::Error>> {
        Ok(Service {})
    }

    fn make_plot(&self, plot: Plot) -> plotly::Plot {
        let mut plotly_plot = plotly::Plot::new();

        let mut layout = plotly::Layout::new();

        if let Some(v) = plot.title {
            layout = layout.title(Title::new(v.as_str()));
        }

        if let Some(v) = plot.x_title {
            let mut x_axis = Axis::new();

            x_axis = x_axis.title(Title::new(v.as_str()));

            layout = layout.x_axis(x_axis);
        }

        if let Some(v) = plot.y_title {
            let mut y_axis = Axis::new();

            y_axis = y_axis.title(Title::new(v.as_str()));

            layout = layout.y_axis(y_axis);
        }

        plotly_plot.set_layout(layout);

        plotly_plot
    }
    pub fn draw_bar(&self, bar: Bar) -> Result<String, Box<dyn std::error::Error>> {
        let mut plot = self.make_plot(bar.plot);

        let mut values: Vec<(String, f64)> = bar.values.into_iter().map(|(k, v)| (k, v)).collect();

        values.sort_by(|left, right| f64::total_cmp(&left.1, &right.1));

        let ks: Vec<_> = values.iter().cloned().map(|(k, _)| k).collect();
        let vs: Vec<_> = values.iter().cloned().map(|(_, v)| v).collect();

        plot.add_trace(plotly::Bar::new(ks.clone(), vs.clone()));

        Ok(plot.to_html())
    }
}
