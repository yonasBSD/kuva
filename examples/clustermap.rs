//! Clustermap documentation examples.
//!
//! Generates canonical SVG outputs used in the kuva documentation.
//! Run with:
//!
//! ```bash
//! cargo run --example clustermap
//! ```
//!
//! SVGs are written to `docs/src/assets/clustermap/`.

use kuva::backend::svg::SvgBackend;
use kuva::plot::{Clustermap, ClustermapNorm, ColorMap};
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;
use kuva::render::render::render_multiple;
use std::fs;

const OUT: &str = "docs/src/assets/clustermap";

fn write(name: &str, plots: Vec<Plot>, layout: Layout) {
    fs::create_dir_all(OUT).unwrap();
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    fs::write(format!("{OUT}/{name}.svg"), svg).unwrap();
}

fn main() {
    // ── Basic ──────────────────────────────────────────────────────────────
    // Two-group structure: rows A/D/E (high col1/col5) vs B/C (high col2/col3)
    let data = vec![
        vec![0.9, 0.1, 0.2, 0.1, 0.8], // A
        vec![0.1, 0.8, 0.9, 0.1, 0.2], // B
        vec![0.2, 0.9, 0.8, 0.1, 0.1], // C
        vec![0.8, 0.1, 0.1, 0.2, 0.9], // D
        vec![0.9, 0.2, 0.1, 0.1, 0.8], // E
    ];
    let cm = Clustermap::new()
        .with_data(data.clone())
        .with_row_labels(["A", "B", "C", "D", "E"])
        .with_col_labels(["X1", "X2", "X3", "X4", "X5"])
        .with_legend("Value");
    let plots = vec![Plot::Clustermap(cm)];
    let layout = Layout::auto_from_plots(&plots).with_title("Clustermap");
    write("basic", plots, layout);

    // ── Gene expression ──────────────────────────────────────────────────
    // 8 genes × 6 conditions; three expression modules
    let expr = vec![
        vec![8.2, 7.9, 0.4, 0.2, 0.1, 0.3], // Gene1
        vec![7.8, 8.1, 0.3, 0.1, 0.4, 0.2], // Gene2
        vec![0.2, 0.3, 7.5, 8.0, 0.1, 0.2], // Gene3
        vec![0.1, 0.4, 8.1, 7.6, 0.3, 0.1], // Gene4
        vec![0.3, 0.1, 0.2, 0.1, 8.3, 7.9], // Gene5
        vec![0.2, 0.2, 0.1, 0.3, 7.8, 8.2], // Gene6
        vec![4.1, 0.3, 3.9, 0.2, 4.2, 0.1], // Gene7 (mixed)
        vec![0.1, 4.3, 0.2, 4.0, 0.3, 4.1], // Gene8 (mixed)
    ];
    let gene_names = [
        "Gene1", "Gene2", "Gene3", "Gene4", "Gene5", "Gene6", "Gene7", "Gene8",
    ];
    let cond_names = ["CtrlA", "CtrlB", "TreatA", "TreatB", "StimA", "StimB"];
    let cm = Clustermap::new()
        .with_data(expr)
        .with_row_labels(gene_names)
        .with_col_labels(cond_names)
        .with_color_map(ColorMap::Viridis)
        .with_legend("Expression");
    let plots = vec![Plot::Clustermap(cm)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Gene Expression Clustermap")
        .with_width(520.0)
        .with_height(420.0);
    write("gene_expression", plots, layout);

    // ── Z-score normalised ────────────────────────────────────────────────
    let cm = Clustermap::new()
        .with_data(data.clone())
        .with_row_labels(["A", "B", "C", "D", "E"])
        .with_col_labels(["X1", "X2", "X3", "X4", "X5"])
        .with_normalization(ClustermapNorm::ColZScore)
        .with_color_map(ColorMap::Inferno)
        .with_legend("Z-score");
    let plots = vec![Plot::Clustermap(cm)];
    let layout = Layout::auto_from_plots(&plots).with_title("Z-score (columns)");
    write("zscore", plots, layout);

    // ── Equal-distance dendrogram (issue #59 reproducer) ──────────────────
    // Rows A, D, E have identical profiles → they should appear at the same
    // dendrogram level, not at artificially staggered levels.
    let eq_data = vec![
        vec![5.0, 0.1, 0.1, 0.1], // A — identical profile
        vec![0.1, 5.0, 0.1, 0.1], // B
        vec![0.1, 5.0, 0.1, 0.1], // C — identical to B
        vec![5.0, 0.1, 0.1, 0.1], // D — identical to A
        vec![5.0, 0.1, 0.1, 0.1], // E — identical to A
    ];
    let cm = Clustermap::new()
        .with_data(eq_data)
        .with_row_labels(["A", "B", "C", "D", "E"])
        .with_col_labels(["c1", "c2", "c3", "c4"])
        .with_legend("Value");
    let plots = vec![Plot::Clustermap(cm)];
    let layout = Layout::auto_from_plots(&plots).with_title("Equal-distance dendrogram");
    write("equal_distance", plots, layout);

    // ── Row-only clustering ───────────────────────────────────────────────
    let cm = Clustermap::new()
        .with_data(data.clone())
        .with_row_labels(["A", "B", "C", "D", "E"])
        .with_col_labels(["X1", "X2", "X3", "X4", "X5"])
        .with_cluster_cols(false)
        .with_legend("Value");
    let plots = vec![Plot::Clustermap(cm)];
    let layout = Layout::auto_from_plots(&plots).with_title("Row clustering only");
    write("row_only", plots, layout);

    // ── No clustering ─────────────────────────────────────────────────────
    let cm = Clustermap::new()
        .with_data(data)
        .with_row_labels(["A", "B", "C", "D", "E"])
        .with_col_labels(["X1", "X2", "X3", "X4", "X5"])
        .with_cluster_rows(false)
        .with_cluster_cols(false)
        .with_values()
        .with_legend("Value");
    let plots = vec![Plot::Clustermap(cm)];
    let layout = Layout::auto_from_plots(&plots).with_title("No clustering (values shown)");
    write("no_clustering", plots, layout);
}
