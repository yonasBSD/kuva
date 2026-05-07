//! Gantt chart documentation examples.
//!
//! Generates canonical SVG outputs used in the kuva documentation.
//! Run with:
//!
//! ```bash
//! cargo run --example gantt
//! ```
//!
//! SVGs are written to `docs/src/assets/gantt/`.

use kuva::backend::svg::SvgBackend;
use kuva::plot::GanttPlot;
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;
use kuva::render::render::render_multiple;
use std::fs;

const OUT: &str = "docs/src/assets/gantt";

fn write(name: &str, plots: Vec<Plot>, layout: Layout) {
    fs::create_dir_all(OUT).unwrap();
    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    fs::write(format!("{OUT}/{name}.svg"), svg).unwrap();
}

fn main() {
    // ── Basic: four ungrouped tasks ───────────────────────────────────────────
    let gantt = GanttPlot::new()
        .with_task("Literature review", 0.0, 4.0)
        .with_task("Data collection", 2.0, 8.0)
        .with_task("Analysis", 7.0, 12.0)
        .with_task("Write-up", 11.0, 16.0);

    let plots = vec![Plot::Gantt(gantt)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Research Project")
        .with_x_label("Week")
        .with_width(600.0)
        .with_height(180.0);
    write("basic", plots, layout);

    // ── Grouped: three phases ─────────────────────────────────────────────────
    let gantt = GanttPlot::new()
        .with_task_group("Discovery", "User research", 0.0, 3.0)
        .with_task_group("Discovery", "Competitive audit", 1.0, 4.0)
        .with_task_group("Design", "Wireframes", 3.5, 6.0)
        .with_task_group("Design", "Prototyping", 5.0, 8.0)
        .with_task_group("Build", "Frontend", 7.0, 12.0)
        .with_task_group("Build", "Backend", 6.0, 11.0);

    let plots = vec![Plot::Gantt(gantt)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Product Development")
        .with_x_label("Week")
        .with_width(620.0)
        .with_height(300.0);
    write("grouped", plots, layout);

    // ── Progress + now line ───────────────────────────────────────────────────
    let gantt = GanttPlot::new()
        .with_task_group_progress("Q1", "API design", 0.0, 3.0, 1.0)
        .with_task_group_progress("Q1", "Auth service", 1.0, 4.0, 1.0)
        .with_task_group_progress("Q2", "Dashboard", 3.0, 7.0, 0.65)
        .with_task_group_progress("Q2", "Reporting", 4.0, 8.0, 0.25)
        .with_task_group("Q3", "Mobile app", 7.0, 11.0)
        .with_task_group("Q3", "Performance", 8.0, 12.0)
        .with_now_line(5.5);

    let plots = vec![Plot::Gantt(gantt)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Engineering Roadmap — progress at week 5.5")
        .with_x_label("Week")
        .with_width(650.0)
        .with_height(310.0);
    write("progress", plots, layout);

    // ── Milestones ────────────────────────────────────────────────────────────
    let gantt = GanttPlot::new()
        .with_task_group("Planning", "Requirements", 0.0, 2.0)
        .with_task_group("Planning", "Architecture", 1.0, 3.0)
        .with_milestone_group("Planning", "Sign-off", 3.0)
        .with_task_group("Execution", "Core build", 3.0, 9.0)
        .with_task_group("Execution", "Integration", 7.0, 11.0)
        .with_milestone_group("Execution", "Code freeze", 11.0)
        .with_task_group("Launch", "Testing", 10.0, 13.0)
        .with_milestone("Public launch", 14.0)
        .with_now_line(7.0);

    let plots = vec![Plot::Gantt(gantt)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Software Release Plan")
        .with_x_label("Week")
        .with_width(680.0)
        .with_height(340.0);
    write("milestones", plots, layout);

    // ── Showcase: clinical trial timeline ─────────────────────────────────────
    // Demonstrates: group_order, grouped tasks, progress fills, per-group
    // milestone diamonds, ungrouped milestone, now line, wide canvas.
    let gantt = GanttPlot::new()
        .with_group_order(["Pre-trial", "Recruitment", "Treatment", "Analysis"])
        .with_task_group_progress("Pre-trial", "Protocol writing", 0.0, 3.0, 1.0)
        .with_task_group_progress("Pre-trial", "IRB approval", 2.0, 5.0, 1.0)
        .with_task_group_progress("Pre-trial", "Site selection", 3.0, 6.0, 1.0)
        .with_milestone_group("Pre-trial", "Trial start", 6.0)
        .with_task_group_progress("Recruitment", "Screening", 6.0, 12.0, 0.75)
        .with_task_group_progress("Recruitment", "Enrollment", 7.0, 14.0, 0.45)
        .with_task_group("Treatment", "Arm A (n=150)", 12.0, 24.0)
        .with_task_group("Treatment", "Arm B (n=150)", 12.0, 24.0)
        .with_milestone_group("Treatment", "Interim analysis", 18.0)
        .with_task_group("Analysis", "Data lock", 23.0, 26.0)
        .with_task_group("Analysis", "Statistical analysis", 25.0, 30.0)
        .with_task_group("Analysis", "Report writing", 28.0, 34.0)
        .with_milestone("Primary endpoint", 24.0)
        .with_milestone("Submission", 35.0)
        .with_now_line(16.0)
        .with_bar_height(0.55);

    let plots = vec![Plot::Gantt(gantt)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Phase III Clinical Trial Timeline")
        .with_x_label("Month")
        .with_width(800.0)
        .with_height(520.0);
    write("showcase", plots, layout);
}
