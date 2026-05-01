//! Figure (multi-plot layout) documentation examples.
//!
//! Generates SVG outputs used in the kuva documentation.
//! Run with:
//!
//! ```bash
//! cargo run --example figure
//! ```
//!
//! SVGs are written to `docs/src/assets/figure/`.

use kuva::plot::{ScatterPlot, LinePlot, LegendEntry, LegendShape, LegendPlot};
use kuva::backend::svg::SvgBackend;
use kuva::render::figure::Figure;
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;

const OUT: &str = "docs/src/assets/figure";

fn main() {
    std::fs::create_dir_all(OUT).expect("could not create docs/src/assets/figure");
    basic();
    merged();
    tall_panel();
    shared_axes();
    shared_legend();
    figure_size();
    per_row_col_sizing();
    println!("Figure SVGs written to {OUT}/");
}

fn scatter(data: Vec<(f64, f64)>, color: &str, legend: Option<&str>) -> Plot {
    let mut p = ScatterPlot::new().with_data(data).with_color(color).with_size(4.5);
    if let Some(l) = legend { p = p.with_legend(l); }
    Plot::Scatter(p)
}

fn line(data: Vec<(f64, f64)>, color: &str, legend: Option<&str>) -> Plot {
    let mut p = LinePlot::new().with_data(data).with_color(color);
    if let Some(l) = legend { p = p.with_legend(l); }
    Plot::Line(p)
}

/// Basic 1×2 grid with panel labels.
fn basic() {
    let scatter_data = vec![
        (1.0_f64, 2.3), (2.1, 4.1), (3.4, 3.2), (4.2, 5.8),
        (5.0, 5.1), (6.3, 7.2), (7.1, 6.9), (8.5, 8.8),
    ];
    let line_data: Vec<(f64, f64)> = (0..=20)
        .map(|i| { let x = i as f64 * 0.5; (x, x * 0.8 + (x * 0.4).sin() * 1.5) })
        .collect();

    let all_plots = vec![
        vec![scatter(scatter_data, "steelblue", None)],
        vec![line(line_data, "crimson", None)],
    ];

    let layouts: Vec<Layout> = all_plots.iter().zip([
        ("Scatter", "X", "Y"),
        ("Line",    "Time", "Value"),
    ]).map(|(cell, (title, xl, yl))| {
        Layout::auto_from_plots(cell)
            .with_title(title)
            .with_x_label(xl)
            .with_y_label(yl)
    }).collect();

    let scene = Figure::new(1, 2)
        .with_plots(all_plots)
        .with_layouts(layouts)
        .with_labels()
        .render();

    std::fs::write(format!("{OUT}/basic.svg"), SvgBackend.render_scene(&scene)).unwrap();
}

/// 2×3 grid with a wide panel spanning the full bottom row.
fn merged() {
    let colors  = ["steelblue", "crimson", "seagreen"];
    let titles  = ["Sample A",  "Sample B", "Sample C"];

    let mut all_plots: Vec<Vec<Plot>> = colors.iter().zip(titles.iter())
        .map(|(color, title)| {
            let data: Vec<(f64, f64)> = (0..12)
                .map(|i| { let x = i as f64; (x, x * 0.9 + (x * 0.7).sin() * 2.0) })
                .collect();
            vec![line(data, color, Some(title))]
        })
        .collect();

    let wide_data: Vec<(f64, f64)> = (0..40)
        .map(|i| { let x = i as f64 * 0.25; (x, (x * 0.5).sin() * 3.0 + x * 0.3) })
        .collect();
    all_plots.push(vec![line(wide_data, "darkorange", None)]);

    let mut layouts: Vec<Layout> = all_plots[..3].iter().zip(titles.iter())
        .map(|(cell, title)| Layout::auto_from_plots(cell).with_title(*title))
        .collect();
    layouts.push(
        Layout::auto_from_plots(&all_plots[3])
            .with_title("Combined")
            .with_x_label("Time")
            .with_y_label("Value"),
    );

    let scene = Figure::new(2, 3)
        .with_structure(vec![
            vec![0], vec![1], vec![2],
            vec![3, 4, 5],
        ])
        .with_plots(all_plots)
        .with_layouts(layouts)
        .with_labels()
        .with_cell_size(380.0, 300.0)
        .render();

    std::fs::write(format!("{OUT}/merged.svg"), SvgBackend.render_scene(&scene)).unwrap();
}

/// 2×2 grid with a tall left panel spanning both rows.
fn tall_panel() {
    // cell indices for a 2×2 grid:
    //   0  1
    //   2  3
    // group [0,2] → tall left; [1] → top-right; [3] → bottom-right
    let tall_data: Vec<(f64, f64)> = (0..30)
        .map(|i| { let x = i as f64 * 0.3; (x, x * 0.6 + (x * 0.8).cos() * 2.5) })
        .collect();
    let top_right: Vec<(f64, f64)> = (1..=8).map(|i| (i as f64, i as f64 * 1.2 + 0.5)).collect();
    let bot_right: Vec<(f64, f64)> = (1..=8).map(|i| (i as f64, i as f64 * 0.6 + 1.0)).collect();

    let all_plots = vec![
        vec![line(tall_data, "steelblue", None)],
        vec![scatter(top_right, "crimson", None)],
        vec![scatter(bot_right, "seagreen", None)],
    ];

    let layouts = vec![
        Layout::auto_from_plots(&all_plots[0])
            .with_title("Full Series").with_x_label("Time").with_y_label("Value"),
        Layout::auto_from_plots(&all_plots[1]).with_title("Period 1"),
        Layout::auto_from_plots(&all_plots[2]).with_title("Period 2"),
    ];

    let scene = Figure::new(2, 2)
        .with_structure(vec![vec![0, 2], vec![1], vec![3]])
        .with_plots(all_plots)
        .with_layouts(layouts)
        .with_labels()
        .with_cell_size(420.0, 320.0)
        .render();

    std::fs::write(format!("{OUT}/tall_panel.svg"), SvgBackend.render_scene(&scene)).unwrap();
}

/// 2×2 grid demonstrating shared Y axis per row.
fn shared_axes() {
    let make = |offset: f64, color: &str| -> Vec<Plot> {
        let data: Vec<(f64, f64)> = (0..10)
            .map(|i| { let x = i as f64; (x, x * 1.1 + offset + (x * 0.5).sin()) })
            .collect();
        vec![scatter(data, color, None)]
    };

    let all_plots = vec![
        make(0.0,  "steelblue"),
        make(1.5,  "steelblue"),
        make(10.0, "crimson"),
        make(11.0, "crimson"),
    ];

    let layouts = vec![
        Layout::auto_from_plots(&all_plots[0]).with_title("Group A — rep 1").with_y_label("Value"),
        Layout::auto_from_plots(&all_plots[1]).with_title("Group A — rep 2"),
        Layout::auto_from_plots(&all_plots[2]).with_title("Group B — rep 1").with_x_label("X").with_y_label("Value"),
        Layout::auto_from_plots(&all_plots[3]).with_title("Group B — rep 2").with_x_label("X"),
    ];

    let scene = Figure::new(2, 2)
        .with_plots(all_plots)
        .with_layouts(layouts)
        .with_shared_y(0)
        .with_shared_y(1)
        .with_cell_size(420.0, 320.0)
        .render();

    std::fs::write(format!("{OUT}/shared_axes.svg"), SvgBackend.render_scene(&scene)).unwrap();
}

/// 2×3 grid sized to a fixed total width × height via with_figure_size.
fn figure_size() {
    let colors = ["steelblue", "crimson", "seagreen", "darkorange", "mediumpurple", "teal"];
    let titles = ["Alpha", "Beta", "Gamma", "Delta", "Epsilon", "Zeta"];

    let all_plots: Vec<Vec<Plot>> = colors.iter().zip(titles.iter())
        .map(|(color, title)| {
            let data: Vec<(f64, f64)> = (0..10)
                .map(|i| { let x = i as f64; (x, x * 0.8 + (x * 0.6 + title.len() as f64).sin() * 2.0) })
                .collect();
            vec![scatter(data, color, None)]
        })
        .collect();

    let layouts: Vec<Layout> = all_plots.iter().zip(titles.iter())
        .map(|(cell, title)| {
            Layout::auto_from_plots(cell).with_title(*title)
        })
        .collect();

    let scene = Figure::new(2, 3)
        .with_plots(all_plots)
        .with_layouts(layouts)
        .with_labels()
        .with_title("Fixed 900 × 560 figure — cells auto-sized")
        .with_figure_size(900.0, 560.0)
        .render();

    std::fs::write(format!("{OUT}/figure_size.svg"), SvgBackend.render_scene(&scene)).unwrap();
}

/// 3×2 grid showing per-row and per-column size overrides.
///
/// Layout:
///   Row 0 (350 px): two normal scatter plots
///   Row 1 ( 90 px): a thin legend-only row spanning both columns
///   Row 2 (350 px): narrow left column (250 px) + wide right column (600 px)
///
/// Demonstrates:
///   - `with_row_height(row, px)` — sets an individual row height
///   - `with_col_width(col, px)` — sets an individual column width
///   - `LegendPlot` auto-sizing: many entries in the 90 px row reflow into
///     multiple columns automatically to avoid overflow
fn per_row_col_sizing() {
    let data_a: Vec<(f64, f64)> = (0..12)
        .map(|i| { let x = i as f64; (x, x * 0.9 + (x * 0.7).sin() * 1.5) })
        .collect();
    let data_b: Vec<(f64, f64)> = (0..12)
        .map(|i| { let x = i as f64; (x, x * 0.6 + (x * 0.5).cos() * 2.0) })
        .collect();

    // Build a LegendPlot with 8 entries — at 18 px/row these would overflow the
    // 90 px row as a single column; the renderer will reflow into multiple columns.
    let entries: Vec<LegendEntry> = [
        ("Control A", "steelblue"), ("Treatment A", "crimson"),
        ("Control B", "seagreen"),  ("Treatment B", "darkorange"),
        ("Control C", "mediumpurple"), ("Treatment C", "teal"),
        ("Reference", "#888888"),   ("Baseline", "#333333"),
    ]
    .iter()
    .map(|&(label, color)| LegendEntry {
        label: label.into(),
        color: color.into(),
        shape: LegendShape::Circle,
        dasharray: None,
    })
    .collect();

    // cell indices for a 3×2 grid:
    //   0  1    ← row 0 (data)
    //   2  3    ← row 1 (legend, spanned)
    //   4  5    ← row 2 (data)
    // No legend labels on the scatter series — the LegendPlot row carries them.
    let all_plots: Vec<Vec<Plot>> = vec![
        vec![scatter(data_a.clone(), "steelblue", None)],
        vec![scatter(data_b.clone(), "crimson",   None)],
        vec![Plot::LegendPlot(LegendPlot::from_entries(entries))],
        vec![scatter(data_a, "seagreen",   None)],
        vec![scatter(data_b, "darkorange", None)],
    ];

    let layouts: Vec<Layout> = vec![
        Layout::auto_from_plots(&all_plots[0]).with_title("Scatter A").with_y_label("Value"),
        Layout::auto_from_plots(&all_plots[1]).with_title("Scatter B"),
        Layout::new((0.0, 1.0), (0.0, 1.0)),  // legend cell — auto layout unused
        Layout::auto_from_plots(&all_plots[3]).with_title("Scatter C").with_x_label("X").with_y_label("Value"),
        Layout::auto_from_plots(&all_plots[4]).with_title("Scatter D").with_x_label("X"),
    ];

    let scene = Figure::new(3, 2)
        .with_structure(vec![
            vec![0], vec![1],      // row 0: two independent cells
            vec![2, 3],            // row 1: legend spans both columns
            vec![4], vec![5],      // row 2: two independent cells
        ])
        .with_plots(all_plots)
        .with_layouts(layouts)
        .with_row_height(1, 90.0)   // thin legend row
        .with_col_width(0, 250.0)   // narrow left column in row 2
        .with_col_width(1, 600.0)   // wide right column in row 2
        .render();

    std::fs::write(format!("{OUT}/per_row_col_sizing.svg"), SvgBackend.render_scene(&scene)).unwrap();
}

/// 1×2 grid with a shared legend collected from all panels.
fn shared_legend() {
    let make_panel = |x_offset: f64| -> Vec<Plot> {
        let ctrl: Vec<(f64, f64)> = (0..8)
            .map(|i| { let x = i as f64 + x_offset; (x, x * 0.9 + 1.0) })
            .collect();
        let trt: Vec<(f64, f64)> = (0..8)
            .map(|i| { let x = i as f64 + x_offset; (x, x * 1.4 + 0.5) })
            .collect();
        vec![
            scatter(ctrl, "steelblue", Some("Control")),
            scatter(trt,  "crimson",   Some("Treatment")),
        ]
    };

    let all_plots = vec![make_panel(0.0), make_panel(0.5)];

    let layouts = vec![
        Layout::auto_from_plots(&all_plots[0])
            .with_title("Experiment 1").with_x_label("Time").with_y_label("Response"),
        Layout::auto_from_plots(&all_plots[1])
            .with_title("Experiment 2").with_x_label("Time"),
    ];

    let scene = Figure::new(1, 2)
        .with_plots(all_plots)
        .with_layouts(layouts)
        .with_shared_legend()
        .with_cell_size(420.0, 340.0)
        .render();

    std::fs::write(format!("{OUT}/shared_legend.svg"), SvgBackend.render_scene(&scene)).unwrap();
}
