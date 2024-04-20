use crate::service::plot_schemas::{BarInput, GraphInput, LineInput, PlotInput};
use charming;
use charming::component::{
    DataView, DataZoom, Feature, Grid, Legend, MagicType, Restore, SaveAsImage, Title, Toolbox,
    ToolboxDataZoom, VisualMap,
};
use charming::datatype::{CompositeValue, DataFrame};
use charming::element::{AxisLabel, Emphasis, EmphasisFocus, ItemStyle, Label, Orient};
use charming::theme::Theme;
use charming::{
    component::Axis, df, element::AxisType, series::Bar, series::Heatmap, Chart, HtmlRenderer,
};

#[derive(Clone)]
pub struct Service {}

impl Service {
    pub fn new() -> Result<Service, Box<dyn std::error::Error>> {
        Ok(Service {})
    }

    fn make_chart(plot: PlotInput) -> (Chart, Axis, Axis) {
        let toolbox = Toolbox::new().show(true).feature(
            Feature::new()
                .data_view(DataView::new().show(true))
                .save_as_image(SaveAsImage::new().show(true))
                .data_zoom(ToolboxDataZoom::new()),
        );

        let mut chart = charming::Chart::new().grid(Grid::new().height("50%").top("10%"));

        if let Some(v) = plot.zoom_end_value {
            chart = chart
                .data_zoom(DataZoom::new().end_value(f64::from(v)))
                .toolbox(toolbox);
        }

        if let Some(v) = plot.title {
            chart = chart.title(Title::new().text(v));
        }

        (
            chart,
            Service::axis_from_title(plot.x_title),
            Service::axis_from_title(plot.y_title),
        )
    }

    fn axis_from_title(title: Option<String>) -> Axis {
        let mut axis = Axis::new().axis_label(AxisLabel::new().font_size(25.0));

        if let Some(v) = title {
            axis = axis.name(v);
        }

        axis
    }

    fn export_to_html(&self, chart: &Chart, plot_input: PlotInput) -> Result<String, String> {
        let renderer =
            HtmlRenderer::new(plot_input.title.unwrap_or("Chart".to_string()), 1200, 800)
                .theme(Theme::Roma);

        match renderer.render(&chart) {
            Ok(v) => Ok(v),
            Err(v) => Err(format!("failed to render chart: {:?}", v)),
        }
    }
    pub fn draw_bar(&self, input: BarInput) -> Result<String, String> {
        let (mut chart, x_axis, y_axis) = Service::make_chart(input.plot.clone());

        let mut values: Vec<(String, f64)> =
            input.values.into_iter().map(|(k, v)| (k, v)).collect();

        if input.asc.is_some_and(|x| x) {
            values.sort_by(|left, right| f64::total_cmp(&left.1, &right.1));
        } else {
            values.sort_by(|left, right| f64::total_cmp(&right.1, &left.1));
        }

        let ks: Vec<_> = values.iter().cloned().map(|(k, _)| k).collect();
        let vs: Vec<_> = values.into_iter().map(|(_, v)| v).collect();

        chart = chart
            .x_axis(
                x_axis
                    .data(ks)
                    .boundary_gap(true)
                    .axis_label(AxisLabel::new().rotate(30).interval(0).font_size(15)),
            )
            .y_axis(y_axis.type_(AxisType::Value));

        chart = chart.series(Bar::new().show_background(true).data(vs));

        self.export_to_html(&chart, input.plot)
    }

    // pub fn draw_linechart(&self, input: LineInput) -> Result<String, String> {
    //     let (mut chart, mut x_axis, mut y_axis) = Service::make_chart(input.plot.clone());
    //
    //
    //
    //     self.export_to_html(&chart, input.plot)
    // }

    pub fn draw_heatmap(&self, input: GraphInput) -> Result<String, String> {
        let (mut chart, x_axis, y_axis) = Service::make_chart(input.plot.clone());

        let data: Vec<DataFrame> = input
            .values
            .into_iter()
            .map(|d| {
                df![
                    d.first,
                    d.second,
                    if d.weight == 0.0 {
                        CompositeValue::from("-")
                    } else {
                        CompositeValue::from(d.weight)
                    }
                ]
            })
            .collect();

        chart = chart
            .series(Heatmap::new().label(Label::new().show(true)).data(data))
            .visual_map(
                VisualMap::new()
                    .min(0)
                    .max(10)
                    .calculable(true)
                    .orient(Orient::Horizontal)
                    .left("center")
                    .bottom("15%"),
            )
            .x_axis(
                x_axis
                    .boundary_gap(true)
                    .axis_label(AxisLabel::new().rotate(30).interval(0).font_size(15)),
            )
            .y_axis(y_axis.type_(AxisType::Value));

        self.export_to_html(&chart, input.plot)
    }
}
