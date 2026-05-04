//! Slope chart (dumbbell plot) documentation examples.
//!
//! Generates canonical SVG outputs used in the kuva documentation.
//! Run with:
//!
//! ```bash
//! cargo run --example slope
//! ```
//!
//! SVGs are written to `docs/src/assets/slope/`.

use kuva::backend::svg::SvgBackend;
use kuva::plot::slope::SlopePlot;
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;
use kuva::render::render::render_multiple;
use std::fs;

const OUT: &str = "docs/src/assets/slope";

fn write(name: &str, plots: Vec<Plot>, layout: Layout) {
    fs::create_dir_all(OUT).unwrap();
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    fs::write(format!("{OUT}/{name}.svg"), svg).unwrap();
}

fn main() {
    // ── 1. Basic employment rate example (direction colors) ───────────────────
    let sp = SlopePlot::new()
        .with_before_label("2015")
        .with_after_label("2023")
        .with_point("Germany", 68.2, 71.5)
        .with_point("France", 70.1, 68.9)
        .with_point("Italy", 65.3, 69.1)
        .with_point("Spain", 72.4, 74.8)
        .with_point("Poland", 58.6, 63.2)
        .with_point("Netherlands", 74.3, 76.1)
        .with_legend("Direction");
    let plots = vec![Plot::Slope(sp)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Employment Rate 2015–2023")
        .with_x_label("Employment rate (%)");
    write("basic", plots, layout);

    // ── 2. Same data with value labels ────────────────────────────────────────
    let sp = SlopePlot::new()
        .with_before_label("2015")
        .with_after_label("2023")
        .with_point("Germany", 68.2, 71.5)
        .with_point("France", 70.1, 68.9)
        .with_point("Italy", 65.3, 69.1)
        .with_point("Spain", 72.4, 74.8)
        .with_point("Poland", 58.6, 63.2)
        .with_point("Netherlands", 74.3, 76.1)
        .with_values(true);
    let plots = vec![Plot::Slope(sp)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Employment Rate 2015–2023 (with values)")
        .with_x_label("Employment rate (%)");
    write("with_values", plots, layout);

    // ── 3. Gene expression: Control vs Treatment ──────────────────────────────
    let sp = SlopePlot::new()
        .with_before_label("Control")
        .with_after_label("Treatment")
        .with_point("BRCA1", 4.2, 7.8)
        .with_point("TP53", 6.1, 5.4)
        .with_point("MYC", 3.3, 8.9)
        .with_point("EGFR", 7.5, 6.2)
        .with_point("VEGFA", 2.8, 5.1)
        .with_point("CDKN2A", 5.9, 4.3)
        .with_point("KRAS", 8.1, 7.6)
        .with_point("PIK3CA", 3.6, 6.7)
        .with_legend("Direction");
    let plots = vec![Plot::Slope(sp)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Gene Expression: Control vs Treatment")
        .with_x_label("log₂ Expression");
    write("gene_expression", plots, layout);

    // ── 4. Uniform color (no direction encoding) ──────────────────────────────
    let sp = SlopePlot::new()
        .with_direction_colors(false)
        .with_color("steelblue")
        .with_before_label("Before")
        .with_after_label("After")
        .with_point("Germany", 68.2, 71.5)
        .with_point("France", 70.1, 68.9)
        .with_point("Italy", 65.3, 69.1)
        .with_point("Spain", 72.4, 74.8)
        .with_point("Poland", 58.6, 63.2)
        .with_point("Netherlands", 74.3, 76.1);
    let plots = vec![Plot::Slope(sp)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Employment Rate (uniform color)")
        .with_x_label("Employment rate (%)");
    write("uniform_color", plots, layout);
}
