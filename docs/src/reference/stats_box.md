# Stats Box

A stats box is a small bordered inset that displays pre-formatted text lines — R², p-values, AUC, sensitivity, or any other metric — inside the plot area. It solves a specific presentation problem: floating text placed directly on the canvas with `.with_equation()` or `.with_correlation()` can overlap data, lacks visual separation from the chart content, and is difficult to reposition without manual coordinate tuning.

The stats box is a `Layout` feature, not a plot-type feature. It works with any plot that uses standard axes.

**Import path:** `kuva::render::layout::Layout` (no additional import needed)

---

## Basic usage

Pass a `Vec` of pre-formatted strings to `.with_stats_box()`. The box is placed in the top-left corner of the plot area by default.

```rust,no_run
use kuva::plot::scatter::{ScatterPlot, TrendLine};
use kuva::backend::svg::SvgBackend;
use kuva::render::render::render_multiple;
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;

let data: Vec<(f64, f64)> = (1..=20)
    .map(|i| (i as f64, i as f64 * 1.9 + (i as f64 * 0.5).sin()))
    .collect();

let plot = ScatterPlot::new()
    .with_data(data)
    .with_color("steelblue")
    .with_size(5.0)
    .with_trend(TrendLine::Linear)
    .with_trend_color("crimson");

let plots = vec![Plot::Scatter(plot)];
let layout = Layout::auto_from_plots(&plots)
    .with_title("Gene Expression vs. Time")
    .with_x_label("Time (h)")
    .with_y_label("Expression (RPKM)")
    .with_stats_box(vec!["R² = 0.971", "p < 0.0001", "y = 1.9x + 0.4"]);

let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
```

<img src="../assets/stats_box/scatter_trend.svg" alt="Scatter plot with trend line and stats box" width="560">

You control the text content entirely — format the strings however your application computes them.

---

## Adding a title

`.with_stats_title()` renders a bold heading above the entries. Useful when the box contains heterogeneous metrics.

```rust,no_run
# use kuva::render::layout::Layout;
# use kuva::render::plots::Plot;
# let plots: Vec<Plot> = vec![];
let layout = Layout::auto_from_plots(&plots)
    .with_stats_title("Linear fit")
    .with_stats_box(vec!["R² = 0.971", "p < 0.0001", "y = 1.9x + 0.4"]);
```

---

## Positioning

`.with_stats_box_at(position, entries)` sets the position and entries in one call. All `LegendPosition` variants are accepted.

```rust,no_run
use kuva::plot::legend::LegendPosition;
use kuva::render::layout::Layout;
# use kuva::render::plots::Plot;
# let plots: Vec<Plot> = vec![];

// Inside variants — overlaid on the plot area with an 8 px inset
let layout = Layout::auto_from_plots(&plots)
    .with_stats_box_at(
        LegendPosition::InsideBottomRight,
        vec!["AUC = 0.883", "95% CI: 0.841–0.925"],
    );

// Outside variants — placed in the margin, same as legend Outside positions
let layout = Layout::auto_from_plots(&plots)
    .with_stats_box_at(
        LegendPosition::OutsideRightTop,
        vec!["n = 240", "R² = 0.847"],
    );
```

The full set of position variants is documented on the [Legends](legends.md) page.

Alternatively, set the position and entries separately:

```rust,no_run
use kuva::plot::legend::LegendPosition;
use kuva::render::layout::Layout;
# use kuva::render::plots::Plot;
# let plots: Vec<Plot> = vec![];
let layout = Layout::auto_from_plots(&plots)
    .with_stats_entry("Sensitivity = 0.843")
    .with_stats_entry("Specificity = 0.779");
```

`.with_stats_entry()` appends one line at a time and is useful when building entries programmatically in a loop.

---

## Hiding the border

The background rect and border are shown by default. Suppress them for a cleaner look when placing the box on a light background with well-separated data:

```rust,no_run
# use kuva::render::layout::Layout;
# use kuva::render::plots::Plot;
# let plots: Vec<Plot> = vec![];
let layout = Layout::auto_from_plots(&plots)
    .with_stats_box(vec!["R² = 0.971", "p < 0.0001"])
    .with_stats_box_border(false);
```

---

## Combining with a legend

When the stats box and the legend are at the same position they stack automatically — the stats box appears below the legend entries. No manual coordinate arithmetic is required.

```rust,no_run
use kuva::plot::scatter::{ScatterPlot, TrendLine};
use kuva::plot::legend::LegendPosition;
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;
# use kuva::backend::svg::SvgBackend;
# use kuva::render::render::render_multiple;

# fn make_data(offset: f64) -> Vec<(f64, f64)> {
#     (1..=20).map(|i| (i as f64, i as f64 * 1.9 + offset)).collect()
# }

let a = ScatterPlot::new()
    .with_data(make_data(0.0))
    .with_color("steelblue")
    .with_legend("Group A")
    .with_trend(TrendLine::Linear);

let b = ScatterPlot::new()
    .with_data(make_data(5.0))
    .with_color("crimson")
    .with_legend("Group B");

let plots = vec![Plot::Scatter(a), Plot::Scatter(b)];
let layout = Layout::auto_from_plots(&plots)
    .with_title("Two Groups")
    .with_stats_box_at(
        LegendPosition::InsideTopRight,
        vec!["R² = 0.971", "slope = 1.9"],
    );
```

---

## ROC curve: sensitivity and specificity at a threshold

The stats box pairs naturally with `RocPlot` to show point metrics at a chosen operating threshold. Compute the values from your data, then format and pass them in:

```rust,no_run
use kuva::plot::{RocPlot, RocGroup};
use kuva::plot::legend::LegendPosition;
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;

# fn logistic_dataset(n: usize, mu: f64, scale: f64) -> Vec<(f64, bool)> { vec![] }

let group = RocGroup::new("Classifier")
    .with_raw(logistic_dataset(150, 1.0, 0.5))
    .with_optimal_point();

let roc = RocPlot::new().with_group(group);
let plots = vec![Plot::Roc(roc)];

// Values computed externally at the Youden-J optimal threshold:
let layout = Layout::auto_from_plots(&plots)
    .with_title("ROC Curve")
    .with_x_label("1 − Specificity")
    .with_y_label("Sensitivity")
    .with_stats_box_at(
        LegendPosition::InsideBottomRight,
        vec![
            "Optimal threshold",
            "Sensitivity = 0.843",
            "Specificity = 0.779",
        ],
    );
```

---

## Scatter + trend line: preferred approach

The `.with_equation()` and `.with_correlation()` methods on `ScatterPlot` render the fit statistics as floating text directly in the data area, which can clash with dense point clouds. The stats box is the preferred approach for any plot where overlap is a concern:

```rust,no_run
use kuva::plot::scatter::{ScatterPlot, TrendLine};
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;

// Preferred: stats box keeps statistics legible at any data density
# let data: Vec<(f64, f64)> = vec![];
let plot = ScatterPlot::new()
    .with_data(data)
    .with_color("steelblue")
    .with_trend(TrendLine::Linear);

let plots = vec![Plot::Scatter(plot)];
let layout = Layout::auto_from_plots(&plots)
    .with_stats_box(vec!["R² = 0.847", "p < 0.0001", "y = 2.1x − 0.3"]);
```

See the [Scatter Plot](../plots/scatter.md) page for the `.with_equation()` / `.with_correlation()` floating-text approach.

---

## API reference

All methods are on `Layout`.

| Method | Default | Description |
|--------|---------|-------------|
| `.with_stats_box(entries)` | — | Set the stats box entries; replaces any previously set entries |
| `.with_stats_entry(entry)` | — | Append a single line to the stats box |
| `.with_stats_box_at(position, entries)` | — | Set position and entries in one call |
| `.with_stats_title(title)` | — | Bold heading rendered above the entries |
| `.with_stats_box_border(bool)` | `true` | Show or hide the background rect and border |

### Position default

The default position is `LegendPosition::InsideTopLeft`. Use `.with_stats_box_at()` to override it.
