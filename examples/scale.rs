//! Scale reference documentation examples.
//!
//! Generates canonical SVG outputs used in docs/src/reference/layout.md.
//! Run with:
//!
//! ```bash
//! cargo run --example scale
//! ```
//!
//! SVGs are written to `docs/src/assets/scale/`.

use kuva::backend::svg::SvgBackend;
use kuva::plot::line::LinePlot;
use kuva::plot::scatter::ScatterPlot;
use kuva::render::annotations::{ReferenceLine, TextAnnotation};
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;
use kuva::render::render::render_multiple;

const OUT: &str = "docs/src/assets/scale";

// ── Shared dataset ───────────────────────────────────────────────────────────
// A small scatter + trend line with a title, axis labels, a reference line,
// and a text annotation — exercises every piece of chrome that scales.

fn base_plots() -> Vec<Plot> {
    let scatter_pts: Vec<(f64, f64)> = vec![
        (1.0, 2.1), (2.0, 3.8), (3.0, 5.5), (4.0, 7.0), (5.0, 9.2),
        (6.0, 10.8), (7.0, 12.1), (8.0, 14.5), (9.0, 15.9), (10.0, 18.0),
    ];
    let line_pts: Vec<(f64, f64)> = vec![(1.0, 1.9), (10.0, 18.2)];

    vec![
        Plot::Scatter(
            ScatterPlot::new()
                .with_data(scatter_pts)
                .with_color("steelblue")
                .with_legend("Observations"),
        ),
        Plot::Line(
            LinePlot::new()
                .with_data(line_pts)
                .with_color("crimson")
                .with_legend("Trend"),
        ),
    ]
}

fn base_layout(plots: &[Plot]) -> Layout {
    Layout::auto_from_plots(plots)
        .with_title("Growth Rate")
        .with_x_label("Time (weeks)")
        .with_y_label("Count")
        .with_annotation(
            TextAnnotation::new("Peak", 9.0, 15.9)
                .with_arrow(9.0, 15.9)
                .with_font_size(11)
                .with_color("dimgray"),
        )
        .with_reference_line(
            ReferenceLine::horizontal(10.0)
                .with_color("orange")
                .with_label("threshold"),
        )
}

fn write(name: &str, svg: String) {
    let path = format!("{OUT}/{name}.svg");
    std::fs::write(&path, svg).unwrap();
    println!("  wrote {path}");
}

// ── Comparison strip: four scale factors side by side ────────────────────────
// Each is rendered at the default canvas size so differences in chrome
// density are easy to see.

fn scale_comparison() {
    for (label, scale) in [
        ("scale_0_5x", 0.5_f64),
        ("scale_1x",   1.0),
        ("scale_1_5x", 1.5),
        ("scale_2x",   2.0),
    ] {
        let plots = base_plots();
        let layout = base_layout(&plots).with_scale(scale);
        let scene = render_multiple(plots, layout);
        write(label, SvgBackend.render_scene(&scene));
    }
}

// ── Annotation limitation example ────────────────────────────────────────────
// Shows what happens when with_scale is used but annotation font_size and
// reference line stroke_width are NOT adjusted manually.  The chrome scales
// but the annotation text and reference line stay at their default pixel size.

fn annotation_not_scaled() {
    let plots = base_plots();
    let layout = base_layout(&plots).with_scale(2.0);
    // annotation font_size (11) and reference line stroke_width (1.0) stay fixed.
    let scene = render_multiple(plots, layout);
    write("annotation_not_scaled", SvgBackend.render_scene(&scene));
}

// ── Annotation limitation fixed ───────────────────────────────────────────────
// The corrected version: scale=2 AND manually doubled annotation font_size
// and reference line stroke_width.

fn annotation_scaled_manually() {
    let scatter_pts: Vec<(f64, f64)> = vec![
        (1.0, 2.1), (2.0, 3.8), (3.0, 5.5), (4.0, 7.0), (5.0, 9.2),
        (6.0, 10.8), (7.0, 12.1), (8.0, 14.5), (9.0, 15.9), (10.0, 18.0),
    ];
    let line_pts: Vec<(f64, f64)> = vec![(1.0, 1.9), (10.0, 18.2)];

    let plots = vec![
        Plot::Scatter(
            ScatterPlot::new()
                .with_data(scatter_pts)
                .with_color("steelblue")
                .with_legend("Observations"),
        ),
        Plot::Line(
            LinePlot::new()
                .with_data(line_pts)
                .with_color("crimson")
                .with_legend("Trend"),
        ),
    ];

    let scale = 2.0_f64;
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Growth Rate")
        .with_x_label("Time (weeks)")
        .with_y_label("Count")
        .with_annotation(
            TextAnnotation::new("Peak", 9.0, 15.9)
                .with_arrow(9.0, 15.9)
                .with_font_size((11.0 * scale).round() as u32)  // scaled manually
                .with_color("dimgray"),
        )
        .with_reference_line(
            ReferenceLine::horizontal(10.0)
                .with_color("orange")
                .with_label("threshold")
                .with_stroke_width(1.0 * scale),  // scaled manually
        )
        .with_scale(scale);

    let scene = render_multiple(plots, layout);
    write("annotation_scaled_manually", SvgBackend.render_scene(&scene));
}

// ── Scale + larger canvas ─────────────────────────────────────────────────────
// scale=2 makes chrome proportionally larger; combine with an explicit canvas
// size to keep the same data-to-chrome ratio as the default.

fn scale_with_larger_canvas() {
    let plots = base_plots();
    let layout = base_layout(&plots)
        .with_scale(2.0)
        .with_width(1200.0)
        .with_height(900.0);
    let scene = render_multiple(plots, layout);
    write("scale_with_larger_canvas", SvgBackend.render_scene(&scene));
}

// ── 3× scale ─────────────────────────────────────────────────────────────────

fn scale_3x() {
    let plots = base_plots();
    let layout = base_layout(&plots).with_scale(3.0);
    let scene = render_multiple(plots, layout);
    write("scale_3x", SvgBackend.render_scene(&scene));
}

// ─────────────────────────────────────────────────────────────────────────────

fn main() {
    std::fs::create_dir_all(OUT).unwrap();
    scale_comparison();
    annotation_not_scaled();
    annotation_scaled_manually();
    scale_with_larger_canvas();
    scale_3x();
    println!("Scale SVGs written to {OUT}/");
}
