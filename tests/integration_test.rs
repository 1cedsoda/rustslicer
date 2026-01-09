//! Integration tests for RustSlicer

use rustslicer::config::PrintProfile;
use rustslicer::geometry::Mesh;
use rustslicer::slicer::{SliceEngine, Island};
use rustslicer::gcode::GCodeGenerator;
use nalgebra::Point3;

#[test]
fn test_config_load_and_validate() {
    let config = PrintProfile::default_pla();
    assert!(config.validate().is_ok());
    assert_eq!(config.print_settings.layer_height, 0.2);
    assert_eq!(config.material.material_type, "PLA");
}

#[test]
fn test_slicer_with_synthetic_mesh() {
    // Create a simple pyramid mesh
    let vertices = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(10.0, 0.0, 0.0),
        Point3::new(5.0, 10.0, 0.0),
        Point3::new(5.0, 5.0, 10.0),
    ];

    let mesh = Mesh {
        vertices: vertices.clone(),
        triangles: vec![
            rustslicer::geometry::Triangle {
                vertices: [0, 1, 2],
                normal: nalgebra::Vector3::new(0.0, 0.0, -1.0),
            },
            rustslicer::geometry::Triangle {
                vertices: [0, 1, 3],
                normal: nalgebra::Vector3::new(0.0, -1.0, 0.0),
            },
            rustslicer::geometry::Triangle {
                vertices: [1, 2, 3],
                normal: nalgebra::Vector3::new(1.0, 0.0, 0.0),
            },
            rustslicer::geometry::Triangle {
                vertices: [2, 0, 3],
                normal: nalgebra::Vector3::new(-1.0, 0.0, 0.0),
            },
        ],
        bounds: rustslicer::geometry::BoundingBox::from_vertices(&vertices),
    };

    let config = PrintProfile::default_pla();
    let slicer = SliceEngine::new(mesh, config);
    let layers = slicer.slice().unwrap();

    // Should generate multiple layers
    assert!(!layers.is_empty());
    
    // Check that layer indices are sequential
    for (i, layer) in layers.iter().enumerate() {
        assert_eq!(layer.layer_index, i);
    }

    // First layer should be at first_layer_height
    assert!((layers[0].z_height - 0.3).abs() < 0.001);
}

#[test]
fn test_gcode_generation_with_real_layers() {
    use rustslicer::geometry::Polygon;
    use nalgebra::Point2;

    let config = PrintProfile::default_pla();
    let generator = GCodeGenerator::new(config);

    // Create a test layer with an island
    let outline = Polygon::new(vec![
        Point2::new(5.0, 5.0),
        Point2::new(15.0, 5.0),
        Point2::new(15.0, 15.0),
        Point2::new(5.0, 15.0),
    ]);

    let island = Island {
        outline,
        holes: vec![],
    };

    let layer = rustslicer::slicer::Layer {
        z_height: 0.3,
        layer_index: 0,
        islands: vec![island],
    };

    let gcode = generator.generate(vec![layer]).unwrap();
    
    // Verify G-code contains expected elements
    assert!(gcode.contains("RustSlicer"));
    assert!(gcode.contains("LAYER 0"));
    assert!(gcode.contains("Z: 0.300"));
    assert!(gcode.contains("Island 0"));
    
    // Should have temperature commands
    assert!(gcode.contains("M104") || gcode.contains("M109"));
    
    // Should have movement commands
    assert!(gcode.contains("G0") || gcode.contains("G1"));
    
    // Should have end commands
    assert!(gcode.contains("M84") || gcode.contains("End"));
}

#[test]
fn test_full_pipeline_empty_mesh() {
    // Test with an empty mesh (no triangles)
    let mesh = Mesh {
        vertices: vec![Point3::origin()],
        triangles: vec![],
        bounds: rustslicer::geometry::BoundingBox {
            min: Point3::origin(),
            max: Point3::new(10.0, 10.0, 10.0),
        },
    };

    let config = PrintProfile::default_pla();
    
    // Slicing should succeed but produce empty layers
    let slicer = SliceEngine::new(mesh, config.clone());
    let layers = slicer.slice().unwrap();
    
    assert!(!layers.is_empty());
    assert!(layers.iter().all(|l| l.is_empty()));

    // G-code generation should still work
    let generator = GCodeGenerator::new(config);
    let gcode = generator.generate(layers).unwrap();
    
    assert!(gcode.contains("RustSlicer"));
    assert!(!gcode.is_empty());
}

#[test]
fn test_layer_properties() {
    use rustslicer::geometry::Polygon;
    use nalgebra::Point2;

    // Create a layer with multiple islands
    let island1 = Island {
        outline: Polygon::new(vec![
            Point2::new(0.0, 0.0),
            Point2::new(5.0, 0.0),
            Point2::new(5.0, 5.0),
            Point2::new(0.0, 5.0),
        ]),
        holes: vec![],
    };

    let island2 = Island {
        outline: Polygon::new(vec![
            Point2::new(10.0, 10.0),
            Point2::new(15.0, 10.0),
            Point2::new(15.0, 15.0),
            Point2::new(10.0, 15.0),
        ]),
        holes: vec![Polygon::new(vec![
            Point2::new(11.0, 11.0),
            Point2::new(14.0, 11.0),
            Point2::new(14.0, 14.0),
            Point2::new(11.0, 14.0),
        ])],
    };

    let layer = rustslicer::slicer::Layer {
        z_height: 1.0,
        layer_index: 5,
        islands: vec![island1, island2],
    };

    // Test layer properties
    assert_eq!(layer.islands.len(), 2);
    assert!(!layer.is_empty());
    assert_eq!(layer.contour_count(), 3); // 2 outlines + 1 hole
}
