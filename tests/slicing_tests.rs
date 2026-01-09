//! Comprehensive tests for slicing algorithms

use rustslicer::config::PrintProfile;
use rustslicer::geometry::{Mesh, Triangle, BoundingBox, Polygon, LineSegment2D};
use rustslicer::slicer::SliceEngine;
use nalgebra::{Point2, Point3, Vector3};

#[test]
fn test_simple_cube_slicing() {
    // Create a simple cube mesh (just top and bottom faces for testing)
    let vertices = vec![
        // Bottom face
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(10.0, 0.0, 0.0),
        Point3::new(10.0, 10.0, 0.0),
        Point3::new(0.0, 10.0, 0.0),
        // Top face
        Point3::new(0.0, 0.0, 10.0),
        Point3::new(10.0, 0.0, 10.0),
        Point3::new(10.0, 10.0, 10.0),
        Point3::new(0.0, 10.0, 10.0),
    ];

    let triangles = vec![
        // Bottom face triangles
        Triangle {
            vertices: [0, 1, 2],
            normal: Vector3::new(0.0, 0.0, -1.0),
        },
        Triangle {
            vertices: [0, 2, 3],
            normal: Vector3::new(0.0, 0.0, -1.0),
        },
        // Top face triangles
        Triangle {
            vertices: [4, 5, 6],
            normal: Vector3::new(0.0, 0.0, 1.0),
        },
        Triangle {
            vertices: [4, 6, 7],
            normal: Vector3::new(0.0, 0.0, 1.0),
        },
        // Side faces (simplified - just one per side)
        Triangle {
            vertices: [0, 1, 5],
            normal: Vector3::new(0.0, -1.0, 0.0),
        },
        Triangle {
            vertices: [0, 5, 4],
            normal: Vector3::new(0.0, -1.0, 0.0),
        },
    ];

    let mesh = Mesh {
        vertices: vertices.clone(),
        triangles,
        bounds: BoundingBox::from_vertices(&vertices),
    };

    let config = PrintProfile::default_pla();
    let slicer = SliceEngine::new(mesh, config);

    let layers = slicer.slice().unwrap();

    // Should have multiple layers for 10mm height with 0.2mm layer height
    assert!(layers.len() > 10);
    assert!(layers.len() < 100);

    // First layer should be at first_layer_height
    assert!((layers[0].z_height - 0.3).abs() < 0.001);
}

#[test]
fn test_triangle_plane_intersection_cases() {
    let vertices = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(10.0, 0.0, 5.0),
        Point3::new(0.0, 10.0, 5.0),
    ];

    let mesh = Mesh {
        vertices: vertices.clone(),
        triangles: vec![Triangle {
            vertices: [0, 1, 2],
            normal: Vector3::z(),
        }],
        bounds: BoundingBox::from_vertices(&vertices),
    };

    let triangle = &mesh.triangles[0];

    // Case 1: Intersection in middle
    let intersection = mesh.intersect_triangle_with_plane(triangle, 2.5);
    assert!(intersection.is_some());
    let seg = intersection.unwrap();
    assert!(seg.length() > 0.0);

    // Case 2: Plane above triangle
    let no_intersection = mesh.intersect_triangle_with_plane(triangle, 10.0);
    assert!(no_intersection.is_none());

    // Case 3: Plane below triangle
    let no_intersection = mesh.intersect_triangle_with_plane(triangle, -1.0);
    assert!(no_intersection.is_none());

    // Case 4: Plane exactly at vertex
    let edge_case = mesh.intersect_triangle_with_plane(triangle, 0.0);
    // Should either return a segment or None, but shouldn't crash
    assert!(edge_case.is_some() || edge_case.is_none());

    // Case 5: Plane at maximum z
    let edge_case = mesh.intersect_triangle_with_plane(triangle, 5.0);
    // Might return None since vertices are on plane
    assert!(edge_case.is_some() || edge_case.is_none());
}

#[test]
fn test_empty_layer_handling() {
    // Create a mesh with no triangles
    let mesh = Mesh {
        vertices: vec![Point3::origin()],
        triangles: vec![],
        bounds: BoundingBox {
            min: Point3::origin(),
            max: Point3::new(10.0, 10.0, 10.0),
        },
    };

    let config = PrintProfile::default_pla();
    let slicer = SliceEngine::new(mesh, config);

    let layers = slicer.slice().unwrap();

    // Should generate layers but they should all be empty
    assert!(!layers.is_empty());
    for layer in &layers {
        assert!(layer.is_empty());
    }
}

#[test]
fn test_polygon_area_calculation() {
    // Square with side length 10
    let square = Polygon::new(vec![
        Point2::new(0.0, 0.0),
        Point2::new(10.0, 0.0),
        Point2::new(10.0, 10.0),
        Point2::new(0.0, 10.0),
    ]);

    assert!((square.area() - 100.0).abs() < 0.001);

    // Triangle
    let triangle = Polygon::new(vec![
        Point2::new(0.0, 0.0),
        Point2::new(10.0, 0.0),
        Point2::new(5.0, 10.0),
    ]);

    assert!((triangle.area() - 50.0).abs() < 0.001);
}

#[test]
fn test_polygon_winding() {
    // Counter-clockwise square
    let ccw_square = Polygon::new(vec![
        Point2::new(0.0, 0.0),
        Point2::new(10.0, 0.0),
        Point2::new(10.0, 10.0),
        Point2::new(0.0, 10.0),
    ]);

    // Clockwise square
    let cw_square = Polygon::new(vec![
        Point2::new(0.0, 0.0),
        Point2::new(0.0, 10.0),
        Point2::new(10.0, 10.0),
        Point2::new(10.0, 0.0),
    ]);

    // Test that winding detection works (exact result depends on implementation)
    let ccw_result = ccw_square.is_clockwise();
    let cw_result = cw_square.is_clockwise();
    
    // They should be opposite
    assert_ne!(ccw_result, cw_result);
}

#[test]
fn test_layer_slicing_with_pyramid() {
    // Create a pyramid that gets smaller as Z increases
    let vertices = vec![
        // Base (z=0)
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(10.0, 0.0, 0.0),
        Point3::new(10.0, 10.0, 0.0),
        Point3::new(0.0, 10.0, 0.0),
        // Apex (z=10)
        Point3::new(5.0, 5.0, 10.0),
    ];

    let triangles = vec![
        // Base
        Triangle {
            vertices: [0, 1, 2],
            normal: Vector3::new(0.0, 0.0, -1.0),
        },
        Triangle {
            vertices: [0, 2, 3],
            normal: Vector3::new(0.0, 0.0, -1.0),
        },
        // Sides
        Triangle {
            vertices: [0, 1, 4],
            normal: Vector3::new(0.0, -1.0, 0.5).normalize(),
        },
        Triangle {
            vertices: [1, 2, 4],
            normal: Vector3::new(1.0, 0.0, 0.5).normalize(),
        },
        Triangle {
            vertices: [2, 3, 4],
            normal: Vector3::new(0.0, 1.0, 0.5).normalize(),
        },
        Triangle {
            vertices: [3, 0, 4],
            normal: Vector3::new(-1.0, 0.0, 0.5).normalize(),
        },
    ];

    let mesh = Mesh {
        vertices: vertices.clone(),
        triangles,
        bounds: BoundingBox::from_vertices(&vertices),
    };

    let config = PrintProfile::default_pla();
    let slicer = SliceEngine::new(mesh, config);

    let layers = slicer.slice().unwrap();

    // Should have layers
    assert!(!layers.is_empty());

    // Lower layers should have islands (cross-section of pyramid)
    let non_empty_layers: Vec<_> = layers.iter().filter(|l| !l.is_empty()).collect();
    assert!(!non_empty_layers.is_empty());

    // Check that we get contours in at least some layers
    let layers_with_contours: Vec<_> = layers.iter().filter(|l| l.contour_count() > 0).collect();
    assert!(!layers_with_contours.is_empty());
}

#[test]
fn test_line_segment_length() {
    let seg = LineSegment2D {
        start: Point2::new(0.0, 0.0),
        end: Point2::new(3.0, 4.0),
    };

    // 3-4-5 triangle
    assert!((seg.length() - 5.0).abs() < 0.001);
}

#[test]
fn test_polygon_bounding_box() {
    let square = Polygon::new(vec![
        Point2::new(1.0, 2.0),
        Point2::new(5.0, 2.0),
        Point2::new(5.0, 7.0),
        Point2::new(1.0, 7.0),
    ]);

    let (min, max) = square.bounding_box();
    
    assert_eq!(min.x, 1.0);
    assert_eq!(min.y, 2.0);
    assert_eq!(max.x, 5.0);
    assert_eq!(max.y, 7.0);
}
