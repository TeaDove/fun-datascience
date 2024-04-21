use crate::service::plot_schemas::{BarInput, EdgeInput, GraphInput, LineInput, NodeInput, PlotInput};
use crate::shared::container::Container;
use rand::seq::SliceRandom;
use rand::Rng;
use rstest::{fixture, rstest};
use std::collections::HashMap;
use std::fs;
use std::ops::Deref;
use charming::datatype::CompositeValue;

#[fixture]
pub fn combat_container() -> Container {
    Container::must_new()
}

#[fixture]
pub fn names() -> Vec<String> {
    return vec![
        "maggot",
        "varam",
        "kammus",
        "sarayn",
        "enarvyne",
        "alammu",
        "irethi",
        "neldam",
        "dren",
        "anasour",
        "irarvy",
        "vandal",
        "tadaves",
        "seran",
        "llaalam",
        "worker",
        "dalamus",
        "vandal",
        "gidave",
        "sendal",
        "othralen",
        "tedril",
        "girothran",
        "ararvy",
        "maryon",
        "llaala",
        "faveran",
        "gadaves",
        "uradasou",
        "berendal",
        "maggot",
        "heloth",
        "neldammu",
        "othren",
        "midaves",
        "deras",
        "vedaves",
        "ienevala",
    ]
    .into_iter()
    .map(|x| x.to_string())
    .collect();
}

#[rstest]
pub fn test_plot_service_draw_bar_ok(combat_container: Container, names: Vec<String>) {
    let mut values: HashMap<String, f64> = HashMap::new();
    for _ in 0..50 {
        values.insert(
            names
                .choose(&mut rand::thread_rng())
                .unwrap()
                .deref()
                .to_string(),
            f64::from(rand::thread_rng().gen_range(0..100)),
        );
    }

    let html = combat_container
        .plot_service
        .draw_bar(BarInput {
            plot: PlotInput {
                title: Some("BAR".to_string()),
                x_title: Some("X_TITLE".to_string()),
                y_title: Some("Y_TITLE".to_string()),
                zoom_end_value: Some(10),
            },
            values: values,
            asc: Some(false),
        })
        .unwrap();

    fs::write("draw_bar_ok.html", html).unwrap();
}

#[rstest]
pub fn test_plot_service_draw_heatmap_ok(combat_container: Container, names: Vec<String>) {
    let mut values: Vec<EdgeInput> = Vec::new();
    for _ in 0..500 {
        values.push(EdgeInput {
            first: names
                .choose(&mut rand::thread_rng())
                .unwrap()
                .deref()
                .to_string(),
            second: names
                .choose(&mut rand::thread_rng())
                .unwrap()
                .deref()
                .to_string(),
            weight: f64::from(rand::thread_rng().gen_range(0..100)),
        });
    }

    let html = combat_container
        .plot_service
        .draw_heatmap(GraphInput {
            plot: PlotInput {
                title: Some("HEATMAP".to_string()),
                x_title: Some("X_TITLE".to_string()),
                y_title: Some("Y_TITLE".to_string()),
                zoom_end_value: None,
            },
            edges: values,
            nodes: HashMap::new(),
        })
        .unwrap();

    fs::write("draw_heatmap_ok.html", html).unwrap();
}


#[rstest]
pub fn test_plot_service_draw_lineplot_ok(combat_container: Container, names: Vec<String>) {
    let mut values: Vec<(String,CompositeValue,CompositeValue)> = Vec::new();
    for _ in 0..100 {
        values.push((
            names
                .choose(&mut rand::thread_rng())
                .unwrap()
                .deref()
                .to_string(),
            CompositeValue::from(rand::thread_rng().gen_range(0..100)),
            CompositeValue::from(rand::thread_rng().gen_range(0..100))
        ));
    }

    let html = combat_container
        .plot_service
        .draw_linechart(LineInput {
            plot: PlotInput {
                title: Some("LINEPLOT".to_string()),
                x_title: Some("X_TITLE".to_string()),
                y_title: Some("Y_TITLE".to_string()),
                zoom_end_value: None,
            },
            values
        })
        .unwrap();

    fs::write("draw_lineplot_ok.html", html).unwrap();
}

#[rstest]
pub fn test_plot_service_draw_graph_ok(combat_container: Container, names: Vec<String>) {
    let mut values: Vec<EdgeInput> = Vec::new();
    let mut nodes: HashMap<String, NodeInput> = HashMap::new();
    for _ in 0..200 {
        let edge = EdgeInput {
            first: names
                .choose(&mut rand::thread_rng())
                .unwrap()
                .deref()
                .to_string(),
            second: names
                .choose(&mut rand::thread_rng())
                .unwrap()
                .deref()
                .to_string(),
            weight: f64::from(rand::thread_rng().gen_range(0..100)),
        };
        values.push(edge.clone());

        nodes.insert(
            edge.first,
            NodeInput {
                weight: Some(f64::from(rand::thread_rng().gen_range(0..100))),
            },
        );
        nodes.insert(
            edge.second,
            NodeInput {
                weight: Some(f64::from(rand::thread_rng().gen_range(0..100))),
            },
        );
    }

    let html = combat_container
        .plot_service
        .draw_graph(GraphInput {
            plot: PlotInput {
                title: Some("GRAPH".to_string()),
                x_title: Some("X_TITLE".to_string()),
                y_title: Some("Y_TITLE".to_string()),
                zoom_end_value: None,
            },
            edges: values,
            nodes: nodes,
        })
        .unwrap();

    fs::write("draw_graph_ok.html", html).unwrap();
}
