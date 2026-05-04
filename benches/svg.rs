use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use kuva::backend::svg::SvgBackend;
use kuva::render::render::{Primitive, Scene};

fn bench_svg_circles(c: &mut Criterion) {
    let mut group = c.benchmark_group("svg_circles");
    for &n in &[1_000usize, 10_000, 100_000, 1_000_000] {
        let mut scene = Scene::new(800.0, 500.0);
        for i in 0..n {
            scene.add(Primitive::Circle {
                cx: (i % 800) as f64,
                cy: (i % 500) as f64,
                r: 3.0,
                fill: kuva::render::color::Color::from("#4c72b0"),
                fill_opacity: None,
                stroke: None,
                stroke_width: None,
            });
        }

        group.bench_with_input(BenchmarkId::new("render_scene", n), &n, |b, _| {
            b.iter(|| criterion::black_box(SvgBackend.render_scene(&scene)));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_svg_circles);
criterion_main!(benches);
