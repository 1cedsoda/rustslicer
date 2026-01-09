use rustslicer::geometry::{Triangle, LineSegment};
use nalgebra::{Point3, Vector3};

#[test]
fn test_triangle_plane_intersection() {
    let triangle = Triangle {
        vertices: [
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(10.0, 0.0, 0.0),
            Point3::new(5.0, 10.0, 10.0),
        ],
        normal: Vector3::new(0.0, 0.0, 1.0),
    };

    // Test intersection at z=5
    let result = triangle.intersect_plane(5.0);
    assert!(result.is_some());

    // Test no intersection at z=20 (above triangle)
    let result = triangle.intersect_plane(20.0);
    assert!(result.is_none());
}

#[test]
fn test_triangle_vertices() {
    let triangle = Triangle {
        vertices: [
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
        ],
        normal: Vector3::new(0.0, 0.0, 1.0),
    };

    assert_eq!(triangle.vertices.len(), 3);
    assert_eq!(triangle.vertices[0], Point3::new(0.0, 0.0, 0.0));
}
