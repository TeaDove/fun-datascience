use std::cmp::Ordering;
use crate::service::plot_schemas::{BarInput, EdgeInput, GraphInput, LineInput, PlotInput};
use charming;
use charming::component::{
    DataView, DataZoom, Feature, Grid, SaveAsImage, Title, Toolbox, ToolboxDataZoom, VisualMap,
};
use charming::datatype::{CompositeValue, DataFrame, Dataset};
use charming::element::{AxisLabel, AxisPointer, DimensionEncode, Emphasis, ItemStyle, Label, LabelLayout, LabelPosition, LineStyle, Orient, ScaleLimit, Tooltip, Trigger};
use charming::series::{GraphData, GraphLayout, GraphLink, GraphNode, Line};
use charming::theme::Theme;
use charming::{
    component::Axis, df, element::AxisType, series::Bar, series::Graph, series::Heatmap, Chart,
    HtmlRenderer,
};
// use rand::Rng;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct Service {}

impl Service {
    pub fn new() -> Result<Service, Box<dyn std::error::Error>> {
        Ok(Service {})
    }

    fn make_chart(plot: PlotInput) -> (Chart, Axis, Axis) {
        let mut feature = Feature::new()
            .data_view(DataView::new().show(true))
            .save_as_image(SaveAsImage::new().show(true));

        let mut chart = charming::Chart::new().grid(Grid::new().height("50%").top("10%"));

        if let Some(v) = plot.zoom_end_value {
            feature = feature.data_zoom(ToolboxDataZoom::new());
            chart = chart.data_zoom(DataZoom::new().end_value(f64::from(v)));
        }

        chart = chart.toolbox(
            Toolbox::new().
                show(true).
                feature(feature)
        ).tooltip(
            Tooltip::new().
                trigger(Trigger::Axis).
                axis_pointer(AxisPointer::new())
        );

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
        let mut axis = Axis::new()
            .axis_label(
                AxisLabel::new()
                    .font_size(25.0)
                    .rotate(30)
                    .interval(0)
                    .font_size(15),
            )
            .boundary_gap(true);

        if let Some(v) = title {
            axis = axis.name(v);
        }

        axis
    }

    fn export_to_html(&self, chart: &Chart, plot_input: PlotInput) -> Result<String, String> {
        let renderer =
            HtmlRenderer::new(plot_input.title.unwrap_or("Chart".to_string()), 1980, 1080)
                .theme(Theme::Default);

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

    fn sort_linechart(values: Vec<(String, CompositeValue, CompositeValue)>) -> HashMap<String, Vec<(CompositeValue)>>{
        let mut map: HashMap<String, Vec<(CompositeValue, CompositeValue)>> = HashMap::new();
        for (legend, x, y) in values{
            if let  Some(mut v) = map.get_mut(&legend){
                v.push((x, y));

                continue
            }

            map.insert(legend, vec!((x, y)));
        }

        let mut res = HashMap::new();

        for (_, mut items) in map{
            items.sort_by(|x, y| if x.0 > y.0 {Ordering::Greater} else {Ordering::Less})
        }

        res
    }

    pub fn draw_linechart(&self, input: LineInput) -> Result<String, String> {
        let (mut chart, mut x_axis, mut y_axis) = Service::make_chart(input.plot.clone());

        let series = Self::sort_linechart(input.values);

        // let mut vec_vec :Vec<Vec<CompositeValue>> = vec!(vec!(
        //     CompositeValue::String("legend".to_string()),
        //     CompositeValue::String("x".to_string()),
        //     CompositeValue::String("y".to_string())
        // ));
        //
        // for (legend, x, y) in input.values{
        //     vec_vec.push(vec!(legend, x, y));
        // }

        chart = chart.
            x_axis(x_axis).
            y_axis(y_axis);

        for (name, data) in series{
            // chart = chart.series(Line::new().data(data).name(name))
        }

        self.export_to_html(&chart, input.plot)
    }

    pub fn sum_duplicates(input: Vec<EdgeInput>) -> Vec<EdgeInput> {
        let mut newvec: Vec<EdgeInput> = vec![];
        let mut edge_to_weight: HashMap<(String, String), f64> = HashMap::new();

        for edge in input.into_iter() {
            let key = (edge.first, edge.second);

            if let Some(v) = edge_to_weight.get(&key.clone()) {
                edge_to_weight.insert(key.clone(), edge.weight + v);

                continue;
            }

            edge_to_weight.insert(key, edge.weight);
        }

        for (k, v) in edge_to_weight.into_iter() {
            newvec.push(EdgeInput {
                first: k.0,
                second: k.1,
                weight: v,
            });
        }

        newvec
    }
    pub fn draw_heatmap(&self, mut input: GraphInput) -> Result<String, String> {
        let (mut chart, x_axis, y_axis) = Service::make_chart(input.plot.clone());

        input.edges = Self::sum_duplicates(input.edges);

        let mut edges_set = HashSet::new();
        for v in input.edges.clone().into_iter() {
            edges_set.insert(v.first);
            edges_set.insert(v.second);
        }

        let axis_data: Vec<String> = edges_set.into_iter().collect();

        let data: Vec<DataFrame> = input
            .edges
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
            .series(
                Heatmap::new()
                    .label(Label::new().show(true))
                    .data(data)
                    .emphasis(
                        Emphasis::new().item_style(
                            ItemStyle::new()
                                .color("rgba(0, 0, 0, 0.5)")
                                .shadow_blur(10)
                                .shadow_color("rgba(0, 0, 0, 0.5)"),
                        ),
                    ),
            )
            .visual_map(
                VisualMap::new()
                    .calculable(true)
                    .orient(Orient::Vertical)
                    .bottom("center")
                    .right("right"),
            )
            .x_axis(
                x_axis
                    .boundary_gap(true)
                    .data(axis_data.clone())
                    .type_(AxisType::Category),
            )
            .y_axis(
                y_axis
                    .type_(AxisType::Category)
                    .data(axis_data)
                    .boundary_gap(true),
            );

        self.export_to_html(&chart, input.plot)
    }

    pub fn draw_graph(&self, mut input: GraphInput) -> Result<String, String> {
        let (mut chart, x_axis, y_axis) = Service::make_chart(input.plot.clone());

        input.edges = Self::sum_duplicates(input.edges);

        let mut data: GraphData = GraphData {
            nodes: vec![],
            links: vec![],
            categories: vec![],
        };

        for v in input.edges {
            data.links.push(GraphLink {
                source: v.first,
                target: v.second,
                value: Some(v.weight),
            })
        }

        for (k, v) in input.nodes {
            data.nodes.push(GraphNode {
                id: k.clone(),
                name: k,
                x: 0.0,
                y: 0.0,
                // x: f64::from(rand::thread_rng().gen_range(0..150)),
                // y: f64::from(rand::thread_rng().gen_range(0..150)),
                value: v.weight.unwrap_or(1.0),
                category: 0,
                symbol_size: v.weight.unwrap_or(1.0),
                label: None,
            })
        }

        chart = chart.series(
            Graph::new()
                .name("Les Miserables")
                .layout(GraphLayout::Force)
                .roam(false)
                .label(
                    Label::new()
                        .show(true)
                        .position(LabelPosition::Right)
                        .formatter("{b}"),
                )
                // .label_layout(LabelLayout::new().hide_overlap(true))
                // .scale_limit(ScaleLimit::new().min(0.4).max(10.0))
                .line_style(LineStyle::new().color("source").curveness(0.3))
                .data(data),
        );

        self.export_to_html(&chart, input.plot)
    }
}
