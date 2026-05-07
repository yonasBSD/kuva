/// How to format value labels next to dots in a SlopePlot.
pub enum SlopeValueFormat {
    /// Smart default: integers as "5", minimal decimals, scientific for extremes.
    Auto,
    /// Exactly `n` decimal places.
    Fixed(usize),
    /// Round to the nearest integer.
    Integer,
}

/// A single row in a slope / dumbbell chart.
pub struct SlopePoint {
    /// Row label shown on the y-axis.
    pub label: String,
    /// Value at the left endpoint (e.g. "before" timepoint).
    pub before: f64,
    /// Value at the right endpoint (e.g. "after" timepoint).
    pub after: f64,
}

/// A slope chart (also called a dumbbell plot).
///
/// Each row shows a labelled entity with a dot at the `before` value, a dot
/// at the `after` value, and a horizontal segment connecting them.  By default
/// the line and dots are coloured green when `after > before` and red when
/// `after < before`, making up/down trends immediately apparent.
///
/// # Example
///
/// ```rust,no_run
/// use kuva::prelude::*;
///
/// let sp = SlopePlot::new()
///     .with_before_label("2015")
///     .with_after_label("2023")
///     .with_point("Germany",      68.2, 71.5)
///     .with_point("France",       70.1, 68.9)
///     .with_point("Spain",        72.4, 74.8)
///     .with_values(true);
///
/// let plots = vec![Plot::from(sp)];
/// let layout = Layout::auto_from_plots(&plots)
///     .with_title("Employment Rate")
///     .with_x_label("Rate (%)");
/// ```
pub struct SlopePlot {
    /// Data rows, in the order they will appear top-to-bottom.
    pub points: Vec<SlopePoint>,

    // ── Column header labels ───────────────────────────────────────────────
    /// Label for the left (before) column, drawn above the plot area.
    pub before_label: Option<String>,
    /// Label for the right (after) column, drawn above the plot area.
    pub after_label: Option<String>,

    // ── Direction-based coloring ───────────────────────────────────────────
    /// Color used when `after > before`. Default `"#2ca02c"` (green).
    pub color_up: String,
    /// Color used when `after < before`. Default `"#d62728"` (red).
    pub color_down: String,
    /// Color used when `after == before`. Default `"#aaaaaa"`.
    pub color_flat: String,
    /// When `true` (default), color each row by direction (up/down/flat).
    /// When `false`, use the uniform `color` field.
    pub color_by_direction: bool,
    /// Uniform color used when `color_by_direction` is `false`. Default `"steelblue"`.
    pub color: String,
    /// Per-point color overrides indexed by row.  When set, takes precedence
    /// over both direction coloring and the uniform `color` field.
    pub group_colors: Option<Vec<String>>,

    // ── Appearance ─────────────────────────────────────────────────────────
    /// Dot radius in pixels. Default `6.0`.
    pub dot_radius: f64,
    /// Connecting segment stroke width in pixels. Default `2.5`.
    pub line_width: f64,
    /// Fill opacity for dots. Default `1.0`.
    pub dot_opacity: f64,
    /// Stroke opacity for the connecting segment. Default `0.7`.
    pub line_opacity: f64,
    /// When `true`, draw numeric labels beside each dot. Default `false`.
    pub show_values: bool,
    /// Format for value labels. Default [`SlopeValueFormat::Auto`].
    pub value_format: SlopeValueFormat,

    // ── Legend ─────────────────────────────────────────────────────────────
    /// Legend title / trigger.  When `Some`:
    /// - `color_by_direction = true` → two entries: "Increase" and "Decrease".
    /// - `color_by_direction = false` → one entry per row using group / uniform color.
    pub legend_label: Option<String>,
}

impl Default for SlopePlot {
    fn default() -> Self {
        Self::new()
    }
}

impl SlopePlot {
    /// Create a slope chart with default settings.
    pub fn new() -> Self {
        Self {
            points: vec![],
            before_label: None,
            after_label: None,
            color_up: "#2ca02c".into(),
            color_down: "#d62728".into(),
            color_flat: "#aaaaaa".into(),
            color_by_direction: true,
            color: "steelblue".into(),
            group_colors: None,
            dot_radius: 6.0,
            line_width: 2.5,
            dot_opacity: 1.0,
            line_opacity: 0.7,
            show_values: false,
            value_format: SlopeValueFormat::Auto,
            legend_label: None,
        }
    }

    /// Add a single row with the given label, before value, and after value.
    pub fn with_point(
        mut self,
        label: impl Into<String>,
        before: impl Into<f64>,
        after: impl Into<f64>,
    ) -> Self {
        self.points.push(SlopePoint {
            label: label.into(),
            before: before.into(),
            after: after.into(),
        });
        self
    }

    /// Add multiple rows from an iterator of `(label, before, after)` triples.
    pub fn with_points(
        mut self,
        pts: impl IntoIterator<Item = (impl Into<String>, impl Into<f64>, impl Into<f64>)>,
    ) -> Self {
        for (label, before, after) in pts {
            self.points.push(SlopePoint {
                label: label.into(),
                before: before.into(),
                after: after.into(),
            });
        }
        self
    }

    /// Set the column header for the left (before) endpoint.
    pub fn with_before_label(mut self, s: impl Into<String>) -> Self {
        self.before_label = Some(s.into());
        self
    }

    /// Set the column header for the right (after) endpoint.
    pub fn with_after_label(mut self, s: impl Into<String>) -> Self {
        self.after_label = Some(s.into());
        self
    }

    /// Set the color for increasing rows (after > before). Default `"#2ca02c"`.
    pub fn with_color_up(mut self, s: impl Into<String>) -> Self {
        self.color_up = s.into();
        self
    }

    /// Set the color for decreasing rows (after < before). Default `"#d62728"`.
    pub fn with_color_down(mut self, s: impl Into<String>) -> Self {
        self.color_down = s.into();
        self
    }

    /// Set the color for flat rows (after == before). Default `"#aaaaaa"`.
    pub fn with_color_flat(mut self, s: impl Into<String>) -> Self {
        self.color_flat = s.into();
        self
    }

    /// Toggle direction-based coloring.  `true` (default) uses `color_up` / `color_down` / `color_flat`.
    /// `false` uses the uniform `color` field.
    pub fn with_direction_colors(mut self, enable: bool) -> Self {
        self.color_by_direction = enable;
        self
    }

    /// Set the uniform color used when `color_by_direction` is `false`.
    pub fn with_color(mut self, s: impl Into<String>) -> Self {
        self.color = s.into();
        self
    }

    /// Set per-row color overrides.  Index corresponds to the row in `points`.
    pub fn with_group_colors(
        mut self,
        colors: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.group_colors = Some(colors.into_iter().map(Into::into).collect());
        self
    }

    /// Set dot radius in pixels. Default `6.0`.
    pub fn with_dot_radius(mut self, r: f64) -> Self {
        self.dot_radius = r;
        self
    }

    /// Set connecting-segment stroke width in pixels. Default `2.5`.
    pub fn with_line_width(mut self, w: f64) -> Self {
        self.line_width = w;
        self
    }

    /// Set dot fill opacity. Default `1.0`.
    pub fn with_dot_opacity(mut self, o: f64) -> Self {
        self.dot_opacity = o;
        self
    }

    /// Set connecting-segment stroke opacity. Default `0.7`.
    pub fn with_line_opacity(mut self, o: f64) -> Self {
        self.line_opacity = o;
        self
    }

    /// Show or hide numeric labels next to each dot. Default `false`.
    pub fn with_values(mut self, show: bool) -> Self {
        self.show_values = show;
        self
    }

    /// Set the number format used for value labels.
    pub fn with_value_format(mut self, fmt: SlopeValueFormat) -> Self {
        self.value_format = fmt;
        self
    }

    /// Attach a legend to the plot.  When `color_by_direction` is `true`,
    /// the legend shows "Increase" and "Decrease" entries.  When `false`,
    /// it shows one entry per row (or a single entry for uniform color).
    pub fn with_legend(mut self, label: impl Into<String>) -> Self {
        self.legend_label = Some(label.into());
        self
    }
}
