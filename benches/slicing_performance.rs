//! Performance benchmarks

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustslicer::config::PrintProfile;
use rustslicer::geometry::Mesh;
use rustslicer::slicer::SliceEngine;

fn benchmark_slicing(c: &mut Criterion) {
    use nalgebra::Point3;
    
    // Create a simple test mesh
    let mesh = Mesh {
        vertices: vec![
            Point3::origin(),
            Point3::new(10.0, 0.0, 0.0),
            Point3::new(0.0, 10.0, 10.0),
        ],
        triangles: vec![],
        bounds: rustslicer::geometry::BoundingBox {
            min: Point3::origin(),
            max: Point3::new(10.0, 10.0, 10.0),
        },
    };

    let config = PrintProfile::default_pla();

    c.bench_function("basic_slicing", |b| {
        b.iter(|| {
            let slicer = SliceEngine::new(mesh.clone(), config.clone());
            black_box(slicer.slice().unwrap())
        })
    });
}

criterion_group!(benches, benchmark_slicing);
criterion_main!(benches);
