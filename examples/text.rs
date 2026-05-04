//! TextPlot documentation examples.
//!
//! Generates canonical SVG outputs used in the kuva documentation.
//! Run with:
//!
//! ```bash
//! cargo run --example text
//! ```
//!
//! SVGs are written to `docs/src/assets/text/`.

use kuva::backend::svg::SvgBackend;
use kuva::plot::{Histogram, ScatterPlot, TextPlot};
use kuva::render::figure::Figure;
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;
use kuva::render::render::render_multiple;

const OUT: &str = "docs/src/assets/text";

fn main() {
    std::fs::create_dir_all(OUT).expect("could not create docs/src/assets/text");

    basic();
    styled();
    headings();
    multiplot_with_description();

    println!("Text SVGs written to {OUT}/");
}

/// Plain text box — no title, no markup.
fn basic() {
    let text = TextPlot::new().with_body(
        "This is a simple text plot. It renders word-wrapped text inside a plot cell, \
            making it easy to add annotations, captions, or methodology notes alongside \
            data visualisations in a figure grid.",
    );

    let plots = vec![Plot::Text(text)];
    let layout = Layout::auto_from_plots(&plots)
        .with_width(480.0)
        .with_height(200.0)
        .with_title("Basic TextPlot");

    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write(format!("{OUT}/basic.svg"), svg).unwrap();
}

/// Text box with a title, background, and inline markup.
fn styled() {
    let text = TextPlot::new()
        .with_title("Study Summary")
        .with_body(
            "**Cohort:** 240 participants across three groups.\n\
            \n\
            **Method:** Bivariate Gaussian sampling with *group-specific* centres \
            and shared covariance structure.\n\
            \n\
            **Finding:** Groups A and B show significant overlap (__p = 0.032__), \
            while Group C is well-separated from both (__p < 0.001__).",
        )
        .with_background("#f8f9fa")
        .with_border("#dee2e6", 1.0)
        .with_padding(20.0);

    let plots = vec![Plot::Text(text)];
    let layout = Layout::auto_from_plots(&plots)
        .with_width(500.0)
        .with_height(260.0);

    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write(format!("{OUT}/styled.svg"), svg).unwrap();
}

/// Demonstrates all markup: headings, bold lines, and horizontal rules.
fn headings() {
    let text = TextPlot::new()
        .with_body(
            "# Results\n\
            Three experimental conditions were evaluated.\n\
            \n\
            ## Primary endpoint\n\
            **Mean response time:** 342 ms (95% CI: 318–366)\n\
            \n\
            ---\n\
            \n\
            ## Secondary endpoint\n\
            **Accuracy:** 94.2% across all conditions.\n\
            Error rate did not differ significantly between groups.",
        )
        .with_padding(20.0);

    let plots = vec![Plot::Text(text)];
    let layout = Layout::auto_from_plots(&plots)
        .with_width(480.0)
        .with_height(300.0)
        .with_title("Markup Showcase");

    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write(format!("{OUT}/headings.svg"), svg).unwrap();
}

/// 2×2 figure: scatter + bar + histogram + TextPlot describing the data.
fn multiplot_with_description() {
    // --- Scatter data: three bivariate clusters ---
    let group_a: Vec<(f64, f64)> = [
        (1.2, 2.1),
        (1.8, 2.9),
        (1.5, 2.4),
        (2.1, 3.1),
        (1.0, 1.8),
        (2.3, 3.5),
        (1.6, 2.6),
        (0.9, 1.6),
        (1.4, 2.2),
        (2.0, 3.0),
    ]
    .into();
    let group_b: Vec<(f64, f64)> = [
        (4.1, 4.8),
        (4.7, 5.3),
        (5.2, 5.9),
        (4.5, 5.1),
        (5.8, 6.4),
        (4.3, 4.6),
        (5.5, 6.0),
        (4.9, 5.5),
        (5.1, 5.7),
        (4.0, 4.3),
    ]
    .into();
    let group_c: Vec<(f64, f64)> = [
        (7.2, 2.2),
        (7.9, 2.8),
        (8.4, 3.3),
        (7.5, 2.5),
        (8.1, 3.0),
        (7.8, 2.7),
        (8.6, 3.5),
        (7.3, 2.0),
        (8.0, 3.1),
        (7.6, 2.4),
    ]
    .into();

    let scatter_a = ScatterPlot::new()
        .with_data(group_a.clone())
        .with_color("steelblue")
        .with_size(5.0)
        .with_legend("Group A");
    let scatter_b = ScatterPlot::new()
        .with_data(group_b.clone())
        .with_color("crimson")
        .with_size(5.0)
        .with_legend("Group B");
    let scatter_c = ScatterPlot::new()
        .with_data(group_c.clone())
        .with_color("seagreen")
        .with_size(5.0)
        .with_legend("Group C");

    let scatter_plots = vec![
        Plot::Scatter(scatter_a),
        Plot::Scatter(scatter_b),
        Plot::Scatter(scatter_c),
    ];
    let scatter_layout = Layout::auto_from_plots(&scatter_plots)
        .with_title("Scatter: Three Groups")
        .with_x_label("X")
        .with_y_label("Y");

    // --- Bar chart: group sizes ---
    let bar = kuva::plot::BarPlot::new()
        .with_group("Group A", vec![(10.0, "steelblue")])
        .with_group("Group B", vec![(10.0, "crimson")])
        .with_group("Group C", vec![(10.0, "seagreen")]);

    let bar_plots = vec![Plot::Bar(bar)];
    let bar_layout = Layout::auto_from_plots(&bar_plots)
        .with_title("Sample Sizes")
        .with_x_label("Group")
        .with_y_label("Count");

    // --- Histogram: all X values pooled ---
    let all_x: Vec<f64> = group_a
        .iter()
        .chain(group_b.iter())
        .chain(group_c.iter())
        .map(|(x, _)| *x)
        .collect();
    let (x_min, x_max) = all_x
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &v| {
            (lo.min(v), hi.max(v))
        });

    let hist = Histogram::new()
        .with_data(all_x)
        .with_bins(10)
        .with_range((x_min, x_max))
        .with_color("mediumpurple");

    let hist_plots = vec![Plot::Histogram(hist)];
    let hist_layout = Layout::auto_from_plots(&hist_plots)
        .with_title("Pooled X Distribution")
        .with_x_label("X")
        .with_y_label("Count");

    // --- TextPlot: description with actual data numbers ---
    let description = TextPlot::new()
        .with_body(
            "# Figure Notes\n\
            \n\
            **Data:** 30 observations across *3 groups* (n = 10 each).\n\
            \n\
            **Group A** clusters near *(1.6, 2.5)* — low X, low Y.\n\
            **Group B** clusters near *(4.9, 5.4)* — mid X, high Y.\n\
            **Group C** clusters near *(7.9, 2.8)* — high X, low Y.\n\
            \n\
            ---\n\
            \n\
            ## Key statistics\n\
            X range: __0.9 – 8.6__ (pooled)\n\
            Mean X: **4.7** | Mean Y: **3.6**\n\
            \n\
            Groups are well-separated in 2D space; B–C separation is primarily \
            driven by X, while A–B and A–C separations are driven by Y.",
        )
        .with_background("#fafafa")
        .with_border("#d0d0d0", 1.0)
        .with_padding(18.0);

    let text_plots = vec![Plot::Text(description)];
    let text_layout = Layout::auto_from_plots(&text_plots).with_title("Description");

    // --- 2×2 Figure ---
    let all_plots = vec![scatter_plots, bar_plots, hist_plots, text_plots];
    let layouts = vec![scatter_layout, bar_layout, hist_layout, text_layout];

    let scene = Figure::new(2, 2)
        .with_plots(all_plots)
        .with_layouts(layouts)
        .with_labels()
        .with_cell_size(480.0, 360.0)
        .render();

    let svg = SvgBackend.render_scene(&scene);
    std::fs::write(format!("{OUT}/multiplot_with_description.svg"), svg).unwrap();
}
