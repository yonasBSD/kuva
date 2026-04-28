//! Date/time axis documentation examples.
//!
//! Demonstrates how to use `ymd()`, `ymd_hms()`, and `DateTimeAxis` to place
//! dates on scatter and line plot axes.
//!
//! Run with:
//!
//! ```bash
//! cargo run --example datetime
//! ```
//!
//! SVGs are written to `docs/src/assets/datetime/`.

use kuva::plot::{LinePlot, ScatterPlot};
use kuva::backend::svg::SvgBackend;
use kuva::render::render::render_multiple;
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;
use kuva::render::datetime::{DateTimeAxis, ymd, ymd_hms};

const OUT: &str = "docs/src/assets/datetime";

fn main() {
    std::fs::create_dir_all(OUT).expect("could not create docs/src/assets/datetime");

    line_monthly();
    scatter_dates();
    line_multiseries();
    line_hourly();

    println!("Datetime SVGs written to {OUT}/");
}

/// Monthly temperature averages — the simplest date-axis use case.
///
/// Data points are keyed by `ymd()`, and the axis is configured with
/// `DateTimeAxis::months()` to format ticks as "Jan 2024", "Feb 2024", etc.
fn line_monthly() {
    // Monthly average temperatures (°C) for a temperate city, 2024
    let data: Vec<(f64, f64)> = [
        ((2024,  1,  1),  3.2),
        ((2024,  2,  1),  4.8),
        ((2024,  3,  1),  8.1),
        ((2024,  4,  1), 12.4),
        ((2024,  5,  1), 16.9),
        ((2024,  6,  1), 20.3),
        ((2024,  7,  1), 22.7),
        ((2024,  8,  1), 22.1),
        ((2024,  9,  1), 17.8),
        ((2024, 10,  1), 12.3),
        ((2024, 11,  1),  7.1),
        ((2024, 12,  1),  3.9),
    ]
    .iter()
    .map(|&((y, m, d), t)| (ymd(y, m, d), t))
    .collect();

    let plot = LinePlot::new()
        .with_data(data)
        .with_color("steelblue")
        .with_stroke_width(2.0);

    let plots = vec![Plot::Line(plot)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Monthly Average Temperature — 2024")
        .with_x_label("Month")
        .with_y_label("Temperature (°C)")
        .with_x_datetime(DateTimeAxis::months("%b %Y"))
        .with_x_tick_rotate(-45.0);

    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write(format!("{OUT}/line_monthly.svg"), svg).unwrap();
}

/// Scatter plot with a date x-axis.
///
/// Each point is an individual observation; `ymd()` converts the calendar date
/// to the internal f64 timestamp. `DateTimeAxis::weeks()` groups ticks by week.
fn scatter_dates() {
    // Simulated daily quality-score measurements for two instruments, Jan–Mar 2024
    let instrument_a: Vec<(f64, f64)> = [
        ((2024, 1,  3), 87.2), ((2024, 1,  8), 89.1), ((2024, 1, 15), 85.7),
        ((2024, 1, 22), 91.3), ((2024, 1, 29), 88.6), ((2024, 2,  5), 90.2),
        ((2024, 2, 12), 86.4), ((2024, 2, 19), 92.8), ((2024, 2, 26), 89.0),
        ((2024, 3,  4), 93.1), ((2024, 3, 11), 90.7), ((2024, 3, 18), 94.2),
        ((2024, 3, 25), 91.8),
    ]
    .iter()
    .map(|&((y, m, d), v)| (ymd(y, m, d), v))
    .collect();

    let instrument_b: Vec<(f64, f64)> = [
        ((2024, 1,  4), 82.1), ((2024, 1, 10), 84.3), ((2024, 1, 17), 81.9),
        ((2024, 1, 24), 85.6), ((2024, 1, 31), 83.2), ((2024, 2,  7), 86.1),
        ((2024, 2, 14), 84.8), ((2024, 2, 21), 87.5), ((2024, 2, 28), 85.9),
        ((2024, 3,  6), 88.3), ((2024, 3, 13), 86.7), ((2024, 3, 20), 89.4),
        ((2024, 3, 27), 87.1),
    ]
    .iter()
    .map(|&((y, m, d), v)| (ymd(y, m, d), v))
    .collect();

    let plots = vec![
        Plot::Scatter(
            ScatterPlot::new()
                .with_data(instrument_a)
                .with_color("steelblue")
                .with_size(5.0)
                .with_legend("Instrument A"),
        ),
        Plot::Scatter(
            ScatterPlot::new()
                .with_data(instrument_b)
                .with_color("coral")
                .with_size(5.0)
                .with_legend("Instrument B"),
        ),
    ];

    let layout = Layout::auto_from_plots(&plots)
        .with_title("Instrument Quality Scores — Q1 2024")
        .with_x_label("Date")
        .with_y_label("Quality Score")
        .with_x_datetime(DateTimeAxis::weeks("%b %d"))
        .with_x_tick_rotate(-45.0);

    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write(format!("{OUT}/scatter_dates.svg"), svg).unwrap();
}

/// Multi-series line plot with a date x-axis.
///
/// Shows `DateTimeAxis::months()` with `.with_step(2)` to draw a tick every
/// two months, keeping the axis readable when the span is long.
fn line_multiseries() {
    // Weekly RNA-seq library yield (ng) for two protocols over 6 months
    let protocol_a: Vec<(f64, f64)> = [
        ((2024,  1,  1), 420.0), ((2024,  2,  1), 455.0), ((2024,  3,  1), 478.0),
        ((2024,  4,  1), 502.0), ((2024,  5,  1), 531.0), ((2024,  6,  1), 548.0),
        ((2024,  7,  1), 562.0),
    ]
    .iter()
    .map(|&((y, m, d), v)| (ymd(y, m, d), v))
    .collect();

    let protocol_b: Vec<(f64, f64)> = [
        ((2024,  1,  1), 388.0), ((2024,  2,  1), 401.0), ((2024,  3,  1), 415.0),
        ((2024,  4,  1), 439.0), ((2024,  5,  1), 461.0), ((2024,  6,  1), 470.0),
        ((2024,  7,  1), 488.0),
    ]
    .iter()
    .map(|&((y, m, d), v)| (ymd(y, m, d), v))
    .collect();

    let plots = vec![
        Plot::Line(
            LinePlot::new()
                .with_data(protocol_a)
                .with_color("steelblue")
                .with_stroke_width(2.0)
                .with_legend("Protocol A"),
        ),
        Plot::Line(
            LinePlot::new()
                .with_data(protocol_b)
                .with_color("coral")
                .with_stroke_width(2.0)
                .with_legend("Protocol B"),
        ),
    ];

    let layout = Layout::auto_from_plots(&plots)
        .with_title("Library Yield by Protocol — H1 2024")
        .with_x_label("Month")
        .with_y_label("Yield (ng)")
        .with_x_datetime(DateTimeAxis::months("%b").with_step(1));

    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write(format!("{OUT}/line_multiseries.svg"), svg).unwrap();
}

/// Sub-day granularity using `ymd_hms()` and `DateTimeAxis::hours()`.
///
/// Each point is an hourly CPU utilisation reading over a single work day.
fn line_hourly() {
    let hours: &[(u32, f64)] = &[
        ( 8, 12.4), ( 9, 34.7), (10, 58.2), (11, 71.5), (12, 45.3),
        (13, 38.1), (14, 62.8), (15, 79.4), (16, 85.1), (17, 66.3),
    ];

    let data: Vec<(f64, f64)> = hours
        .iter()
        .map(|&(h, v)| (ymd_hms(2024, 6, 12, h, 0, 0), v))
        .collect();

    let plot = LinePlot::new()
        .with_data(data)
        .with_color("mediumseagreen")
        .with_stroke_width(2.0);

    let plots = vec![Plot::Line(plot)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("CPU Utilisation — 12 Jun 2024")
        .with_x_label("Time")
        .with_y_label("Utilisation (%)")
        .with_x_datetime(DateTimeAxis::hours("%H:%M"));

    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write(format!("{OUT}/line_hourly.svg"), svg).unwrap();
}
