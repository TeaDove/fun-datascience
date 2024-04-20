use crate::service::plot_schemas::{BarInput, PlotInput};
use charming;
use charming::component::{Legend, SaveAsImage, Title};
use charming::element::{AxisLabel, Emphasis, EmphasisFocus, ItemStyle};
use charming::series::{Pie, PieRoseType};
use charming::theme::Theme;
use charming::EchartsError::ImageRenderingError;
use charming::{
    component::Axis,
    element::{AxisType, BackgroundStyle},
    series::Bar,
    Chart, HtmlRenderer,
};
use std::fmt::format;

#[derive(Clone)]
pub struct Service {}

impl Service {
    pub fn new() -> Result<Service, Box<dyn std::error::Error>> {
        Ok(Service {})
    }

    fn make_chart(&self, plot: PlotInput) -> charming::Chart {
        let mut chart = charming::Chart::new();

        if let Some(v) = plot.title {
            chart = chart.title(Title::new().text(v));
        }

        chart
    }

    fn axis_from_title(title: Option<String>) -> Axis {
        let mut axis = Axis::new();

        if let Some(v) = title {
            axis = axis.name(v);
        }

        axis
    }
    pub fn draw_bar(&self, bar: BarInput) -> Result<String, String> {
        let mut chart = self.make_chart(bar.plot.clone());

        let mut values: Vec<(String, f64)> = bar.values.into_iter().map(|(k, v)| (k, v)).collect();

        if bar.asc.is_some_and(|x| x) {
            values.sort_by(|left, right| f64::total_cmp(&left.1, &right.1));
        } else {
            values.sort_by(|left, right| f64::total_cmp(&right.1, &left.1));
        }

        if let Some(limit) = bar.limit {
            if limit < values.len() {
                values = values[0..limit].to_vec();
            }
        }

        let ks: Vec<_> = values.iter().cloned().map(|(k, _)| k).collect();
        let vs: Vec<_> = values.iter().cloned().map(|(_, v)| v).collect();

        chart = chart
            .x_axis(
                Service::axis_from_title(bar.plot.x_title.clone())
                    .data(ks)
                    .boundary_gap(true)
                    .axis_label(AxisLabel::new().rotate(50).interval(0)),
            )
            .y_axis(Service::axis_from_title(bar.plot.y_title.clone()).type_(AxisType::Value));

        chart = chart.series(Bar::new().show_background(true).data(vs));

        self.export_to_html(&chart, bar.plot)
    }

    pub fn export_to_html(&self, chart: &Chart, plot_input: PlotInput) -> Result<String, String> {
        let renderer =
            HtmlRenderer::new(plot_input.title.unwrap_or("Chart".to_string()), 1200, 800)
                .theme(Theme::Roma);

        match renderer.render(&chart) {
            Ok(v) => Ok(v),
            Err(v) => Err(format!("failed to render chart: {:?}", v)),
        }
    }
}
