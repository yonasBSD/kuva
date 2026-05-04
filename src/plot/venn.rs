use std::collections::{BTreeSet, HashMap};

/// A single set in a Venn diagram.
#[derive(Debug, Clone)]
pub struct VennSet {
    pub label: String,
    /// Raw element list; `None` when pre-computed sizes are used.
    pub elements: Option<Vec<String>>,
    /// Pre-computed total size of the set (inclusive of all intersections).
    /// `None` when raw elements are provided.
    pub size: Option<usize>,
}

/// A pre-computed intersection size.
///
/// `sets` is a **sorted** Vec of set labels (length ≥ 2).
#[derive(Debug, Clone)]
pub struct VennOverlap {
    pub sets: Vec<String>,
    pub size: usize,
}

/// Builder for a Venn diagram (2, 3, or 4 sets).
///
/// Two input modes are supported:
/// - **Raw elements** via [`with_set`](Self::with_set): intersections are computed automatically.
/// - **Pre-computed sizes** via [`with_set_size`](Self::with_set_size) +
///   [`with_overlap`](Self::with_overlap): supply totals and pairwise/triple/... intersection sizes.
///
/// # Example
///
/// ```rust,no_run
/// use kuva::plot::venn::VennPlot;
/// use kuva::render::plots::Plot;
/// use kuva::render::layout::Layout;
/// use kuva::render::render::render_multiple;
/// use kuva::backend::svg::SvgBackend;
///
/// let deseq2 = vec!["BRCA1","TP53","MYC","EGFR"];
/// let edger  = vec!["TP53","MYC","KRAS","PIK3CA"];
///
/// let venn = VennPlot::new()
///     .with_set("DESeq2", deseq2.iter().map(|s| s.to_string()).collect())
///     .with_set("edgeR",  edger.iter().map(|s| s.to_string()).collect())
///     .with_percentages(true);
///
/// let plots = vec![Plot::Venn(venn)];
/// let layout = Layout::auto_from_plots(&plots).with_title("DE Gene Overlap");
/// let scene = render_multiple(plots, layout);
/// let svg = SvgBackend.render_scene(&scene);
/// std::fs::write("venn.svg", svg).unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct VennPlot {
    pub sets: Vec<VennSet>,
    pub(crate) overlaps: Vec<VennOverlap>,

    // Display options
    /// Show element counts in each region. Default: `true`.
    pub show_counts: bool,
    /// Show percentage of total elements in each region. Default: `false`.
    pub show_percentages: bool,
    /// Show set name labels. Default: `true`.
    pub show_set_labels: bool,
    /// Fill opacity for each circle/ellipse. Default: `0.25`.
    pub fill_opacity: f64,
    /// Stroke width for circle/ellipse outlines. Default: `1.5`.
    pub stroke_width: f64,

    // Proportional mode
    /// Scale circle areas proportional to set sizes. Default: `false`.
    pub proportional: bool,
    /// Display the layout stress score when `proportional = true`. Default: `false`.
    ///
    /// Stress (venneuler formula) measures how accurately the visual areas represent
    /// the target region sizes: `sqrt(Σ(aᵢ−tᵢ)² / Σtᵢ²)` where `aᵢ` is the
    /// sampled area fraction and `tᵢ` is the target fraction.
    /// A value near 0 means perfect proportional representation; values above 0.2
    /// indicate significant distortion.
    pub show_loss: bool,

    /// Optional explicit colors for each set (CSS color strings).
    /// Falls back to `category10` palette by index when `None` or shorter than `sets`.
    pub colors: Option<Vec<String>>,

    /// Legend group title. When set, a legend entry per set is added.
    pub legend_label: Option<String>,

    /// Place all region labels outside the diagram with leader lines (default: `false`).
    pub leader_lines: bool,
    /// Show coloured set-indicator dots on region labels (default: `true`).
    ///
    /// When `true`, a small coloured dot is drawn above each inline region label
    /// for every set the region belongs to.  On leader-line labels the dots appear
    /// beside the count text.  This visually distinguishes intersection regions
    /// from single-set regions at a glance.  Set to `false` for a cleaner look.
    pub show_set_indicators: bool,
}

impl Default for VennPlot {
    fn default() -> Self {
        Self::new()
    }
}

impl VennPlot {
    /// Create a Venn diagram with default settings.
    pub fn new() -> Self {
        Self {
            sets: vec![],
            overlaps: vec![],
            show_counts: true,
            show_percentages: false,
            show_set_labels: true,
            fill_opacity: 0.25,
            stroke_width: 1.5,
            proportional: false,
            show_loss: false,
            colors: None,
            legend_label: None,
            leader_lines: false,
            show_set_indicators: true,
        }
    }

    /// Add a set from a raw list of elements.
    ///
    /// Intersections with other sets are computed automatically.
    pub fn with_set(mut self, label: impl Into<String>, elements: Vec<impl Into<String>>) -> Self {
        self.sets.push(VennSet {
            label: label.into(),
            elements: Some(elements.into_iter().map(|e| e.into()).collect()),
            size: None,
        });
        self
    }

    /// Add a set with a pre-computed total size (inclusive of all intersections).
    pub fn with_set_size(mut self, label: impl Into<String>, size: usize) -> Self {
        self.sets.push(VennSet {
            label: label.into(),
            elements: None,
            size: Some(size),
        });
        self
    }

    /// Add a pre-computed intersection size.
    ///
    /// `labels` must name 2 or more sets; the size is the inclusive |A∩B| (or |A∩B∩C|, …).
    /// Labels are sorted internally for consistent lookup.
    pub fn with_overlap(
        mut self,
        labels: impl IntoIterator<Item = impl Into<String>>,
        size: usize,
    ) -> Self {
        let mut sorted: Vec<String> = labels.into_iter().map(|l| l.into()).collect();
        sorted.sort();
        self.overlaps.push(VennOverlap { sets: sorted, size });
        self
    }

    /// Show/hide element counts in each region (default: `true`).
    pub fn with_counts(mut self, v: bool) -> Self {
        self.show_counts = v;
        self
    }

    /// Show/hide percentage of total in each region (default: `false`).
    pub fn with_percentages(mut self, v: bool) -> Self {
        self.show_percentages = v;
        self
    }

    /// Show/hide set name labels (default: `true`).
    pub fn with_set_labels(mut self, v: bool) -> Self {
        self.show_set_labels = v;
        self
    }

    /// Set fill opacity for circles/ellipses (default: `0.25`).
    pub fn with_fill_opacity(mut self, v: f64) -> Self {
        self.fill_opacity = v;
        self
    }

    /// Set stroke width for circle/ellipse outlines (default: `1.5`).
    pub fn with_stroke_width(mut self, v: f64) -> Self {
        self.stroke_width = v;
        self
    }

    /// Enable proportional mode: circle areas scale with set sizes (default: `false`).
    pub fn with_proportional(mut self, v: bool) -> Self {
        self.proportional = v;
        self
    }

    /// Show the layout stress score in the corner when `proportional = true` (default: `false`).
    ///
    /// The displayed value is the venneuler stress: `sqrt(Σ(aᵢ−tᵢ)² / Σtᵢ²)`.
    /// Values near 0 indicate accurate proportional representation; above 0.2 indicates
    /// meaningful distortion.
    pub fn with_loss(mut self, v: bool) -> Self {
        self.show_loss = v;
        self
    }

    /// Override the colors for each set (CSS color strings).
    pub fn with_colors(mut self, colors: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.colors = Some(colors.into_iter().map(|c| c.into()).collect());
        self
    }

    /// Attach a legend; the label is used as the legend group title.
    pub fn with_legend(mut self, label: impl Into<String>) -> Self {
        self.legend_label = Some(label.into());
        self
    }

    /// Place region labels outside the diagram with leader lines (default: `false`).
    ///
    /// When enabled every region label is drawn outside the Venn circles with a thin
    /// grey connector line.  Small coloured dots (one per "in" set) appear next to
    /// the count so the reader can identify which region is being described.
    pub fn with_leader_lines(mut self, v: bool) -> Self {
        self.leader_lines = v;
        self
    }

    /// Show or hide the coloured set-indicator dots on leader-line labels (default: `true`).
    ///
    /// When `true`, small coloured dots (one per "in" set) appear beside each leader-line
    /// count, making it easy to see which region the label refers to.
    /// Set to `false` for a cleaner look when the diagram is unambiguous.
    pub fn with_set_indicators(mut self, v: bool) -> Self {
        self.show_set_indicators = v;
        self
    }

    // ── Internal helpers ─────────────────────────────────────────────────────

    /// Returns the color for set `i`, using `colors` if set, else falling back to
    /// the `category10` palette.
    pub(crate) fn color_for(&self, i: usize) -> String {
        use crate::render::palette::Palette;
        if let Some(ref cv) = self.colors {
            if let Some(c) = cv.get(i) {
                if !c.is_empty() {
                    return c.clone();
                }
            }
        }
        let pal = Palette::category10();
        pal[i % pal.len()].to_string()
    }

    /// Compute the **exclusive** element count for every region.
    ///
    /// Returns a `HashMap<u8, usize>` keyed by bitmask.
    /// Bit `i` of the bitmask = 1 means the region is inside set `i`.
    /// e.g. bitmask `0b101` = inside sets 0 and 2, outside set 1.
    ///
    /// Only non-zero bitmasks (1 … (1<<n)−1) are returned.
    pub(crate) fn region_sizes(&self) -> HashMap<u8, usize> {
        let n = self.sets.len();
        if n == 0 || n > 4 {
            return HashMap::new();
        }

        // Determine which input mode we're in
        let raw_mode = self.sets.iter().any(|s| s.elements.is_some());

        if raw_mode {
            self.region_sizes_raw()
        } else {
            self.region_sizes_precomputed()
        }
    }

    fn region_sizes_raw(&self) -> HashMap<u8, usize> {
        let n = self.sets.len();
        let total_masks = 1u8 << n;

        // Convert each set's element list to a BTreeSet for fast set ops
        let sets: Vec<BTreeSet<&str>> = self
            .sets
            .iter()
            .map(|s| {
                s.elements
                    .as_deref()
                    .unwrap_or(&[])
                    .iter()
                    .map(|e| e.as_str())
                    .collect()
            })
            .collect();

        let mut result = HashMap::new();
        for mask in 1..total_masks {
            // Intersection of all sets with bit=1
            let in_bits: Vec<usize> = (0..n).filter(|&i| mask & (1 << i) != 0).collect();
            let out_bits: Vec<usize> = (0..n).filter(|&i| mask & (1 << i) == 0).collect();

            // Start from the first "in" set
            let first = in_bits[0];
            let mut common: BTreeSet<&str> = sets[first].clone();
            for &i in &in_bits[1..] {
                common = common.intersection(&sets[i]).copied().collect();
            }
            // Subtract elements that appear in any "out" set
            for &i in &out_bits {
                common = common.difference(&sets[i]).copied().collect();
            }
            result.insert(mask, common.len());
        }
        result
    }

    fn region_sizes_precomputed(&self) -> HashMap<u8, usize> {
        let n = self.sets.len();
        let total_masks = 1u8 << n;

        // Build inclusive-count map: bitmask → inclusive intersection size
        let mut inclusive: HashMap<u8, usize> = HashMap::new();

        // Single sets
        for (i, s) in self.sets.iter().enumerate() {
            if let Some(sz) = s.size {
                inclusive.insert(1u8 << i, sz);
            }
        }

        // Pairwise/triple/... overlaps
        for ov in &self.overlaps {
            let mut mask = 0u8;
            for label in &ov.sets {
                if let Some(i) = self.sets.iter().position(|s| s.label == *label) {
                    mask |= 1 << i;
                }
            }
            if mask != 0 {
                inclusive.insert(mask, ov.size);
            }
        }

        // Compute exclusive counts from highest popcount downward
        // exclusive[R] = inclusive[R] - sum(exclusive[m]) for all strict supersets m of R
        let mut exclusive: HashMap<u8, usize> = HashMap::new();
        // Process masks from highest popcount to lowest
        let mut masks: Vec<u8> = (1..total_masks).collect();
        masks.sort_by_key(|m| -(m.count_ones() as i32));

        for mask in masks {
            let inc = inclusive.get(&mask).copied().unwrap_or(0);
            // Sum of exclusive counts for strict supersets
            let super_sum: usize = (1..total_masks)
                .filter(|&m| m != mask && (m & mask) == mask)
                .map(|m| exclusive.get(&m).copied().unwrap_or(0))
                .sum();
            exclusive.insert(mask, inc.saturating_sub(super_sum));
        }
        exclusive
    }

    /// Total number of elements across all sets (counting shared elements once).
    #[allow(dead_code)]
    pub(crate) fn total_elements(&self) -> usize {
        let n = self.sets.len();
        if n == 0 {
            return 0;
        }
        let raw_mode = self.sets.iter().any(|s| s.elements.is_some());
        if raw_mode {
            // Union of all element sets
            let union: BTreeSet<&str> = self
                .sets
                .iter()
                .flat_map(|s| {
                    s.elements
                        .as_deref()
                        .unwrap_or(&[])
                        .iter()
                        .map(|e| e.as_str())
                })
                .collect();
            union.len()
        } else {
            // Sum of exclusive regions
            self.region_sizes_precomputed().values().sum()
        }
    }
}
