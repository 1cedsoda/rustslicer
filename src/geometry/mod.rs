//! Geometry primitives and mesh handling

use nalgebra::{Point2, Point3, Vector3};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::error::{Result, SlicerError};

/// 3D triangular mesh
#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Point3<f64>>,
    pub triangles: Vec<Triangle>,
    pub bounds: BoundingBox,
}

/// Triangle with vertex indices
#[derive(Debug, Clone)]
pub struct Triangle {
    pub vertices: [usize; 3],
    pub normal: Vector3<f64>,
}

/// Line segment in 2D (for layer contours)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineSegment2D {
    pub start: Point2<f64>,
    pub end: Point2<f64>,
}

/// Axis-aligned bounding box
#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub min: Point3<f64>,
    pub max: Point3<f64>,
}

/// 2D polygon representing a contour
#[derive(Debug, Clone)]
pub struct Polygon {
    pub points: Vec<Point2<f64>>,
}

const EPSILON: f64 = 1e-9;

impl Mesh {
    /// Load mesh from STL file
    pub fn from_stl<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = BufReader::new(
            File::open(path.as_ref())
                .map_err(|e| SlicerError::stl_read(format!("Cannot open file: {}", e)))?,
        );

        let indexed_mesh = stl_io::read_stl(&mut file)
            .map_err(|e| SlicerError::stl_read(format!("STL parse error: {}", e)))?;

        let vertices: Vec<Point3<f64>> = indexed_mesh
            .vertices
            .into_iter()
            .map(|v| Point3::new(v[0] as f64, v[1] as f64, v[2] as f64))
            .collect();

        let triangles: Vec<Triangle> = indexed_mesh
            .faces
            .into_iter()
            .map(|f| Triangle {
                vertices: [f.vertices[0], f.vertices[1], f.vertices[2]],
                normal: Vector3::new(f.normal[0] as f64, f.normal[1] as f64, f.normal[2] as f64),
            })
            .collect();

        let bounds = BoundingBox::from_vertices(&vertices);

        Ok(Mesh {
            vertices,
            triangles,
            bounds,
        })
    }

    /// Get the three vertices of a triangle
    pub fn get_triangle_vertices(&self, triangle: &Triangle) -> [Point3<f64>; 3] {
        [
            self.vertices[triangle.vertices[0]],
            self.vertices[triangle.vertices[1]],
            self.vertices[triangle.vertices[2]],
        ]
    }

    /// Intersect a triangle with a horizontal plane at height z
    /// Returns a line segment if intersection exists
    pub fn intersect_triangle_with_plane(
        &self,
        triangle: &Triangle,
        z: f64,
    ) -> Option<LineSegment2D> {
        let [v0, v1, v2] = self.get_triangle_vertices(triangle);

        // Classify vertices relative to plane
        let d0 = v0.z - z;
        let d1 = v1.z - z;
        let d2 = v2.z - z;

        // Count vertices above, below, and on plane
        let above = (d0 > EPSILON) as u8 + (d1 > EPSILON) as u8 + (d2 > EPSILON) as u8;
        let below = (d0 < -EPSILON) as u8 + (d1 < -EPSILON) as u8 + (d2 < -EPSILON) as u8;

        // Triangle doesn't intersect if all vertices on same side
        if above == 3 || below == 3 {
            return None;
        }

        // Handle degenerate cases
        if above == 0 && below == 0 {
            // All vertices on plane - degenerate, ignore
            return None;
        }

        // Find intersection points
        let mut intersections = Vec::new();

        // Check each edge
        let edges = [(v0, v1, d0, d1), (v1, v2, d1, d2), (v2, v0, d2, d0)];

        for (va, vb, da, db) in edges {
            // Skip if both vertices on same side
            if (da > EPSILON && db > EPSILON) || (da < -EPSILON && db < -EPSILON) {
                continue;
            }

            // Handle vertex exactly on plane
            if da.abs() < EPSILON {
                intersections.push(Point2::new(va.x, va.y));
                continue;
            }
            if db.abs() < EPSILON {
                intersections.push(Point2::new(vb.x, vb.y));
                continue;
            }

            // Edge crosses plane - compute intersection
            if (da > 0.0) != (db > 0.0) {
                let t = da / (da - db);
                let x = va.x + t * (vb.x - va.x);
                let y = va.y + t * (vb.y - va.y);
                intersections.push(Point2::new(x, y));
            }
        }

        // Remove duplicate points
        intersections.dedup_by(|a, b| {
            (a.x - b.x).abs() < EPSILON && (a.y - b.y).abs() < EPSILON
        });

        // Should have exactly 2 intersection points
        if intersections.len() == 2 {
            Some(LineSegment2D {
                start: intersections[0],
                end: intersections[1],
            })
        } else {
            None
        }
    }

    /// Get number of triangles
    pub fn triangle_count(&self) -> usize {
        self.triangles.len()
    }

    /// Get number of vertices
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }
}

impl LineSegment2D {
    /// Check if two line segments share an endpoint
    pub fn connects_to(&self, other: &LineSegment2D, tolerance: f64) -> Option<ConnectionType> {
        if self.end.distance_to(&other.start) < tolerance {
            Some(ConnectionType::EndToStart)
        } else if self.end.distance_to(&other.end) < tolerance {
            Some(ConnectionType::EndToEnd)
        } else if self.start.distance_to(&other.start) < tolerance {
            Some(ConnectionType::StartToStart)
        } else if self.start.distance_to(&other.end) < tolerance {
            Some(ConnectionType::StartToEnd)
        } else {
            None
        }
    }

    /// Get length of segment
    pub fn length(&self) -> f64 {
        self.start.distance_to(&self.end)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    EndToStart,
    EndToEnd,
    StartToStart,
    StartToEnd,
}

impl BoundingBox {
    /// Create bounding box from vertices
    pub fn from_vertices(vertices: &[Point3<f64>]) -> Self {
        if vertices.is_empty() {
            return Self {
                min: Point3::origin(),
                max: Point3::origin(),
            };
        }

        let mut min = vertices[0];
        let mut max = vertices[0];

        for v in vertices.iter().skip(1) {
            min.x = min.x.min(v.x);
            min.y = min.y.min(v.y);
            min.z = min.z.min(v.z);
            max.x = max.x.max(v.x);
            max.y = max.y.max(v.y);
            max.z = max.z.max(v.z);
        }

        Self { min, max }
    }

    /// Get dimensions
    pub fn dimensions(&self) -> Vector3<f64> {
        self.max - self.min
    }

    /// Get center point
    pub fn center(&self) -> Point3<f64> {
        Point3::from((self.min.coords + self.max.coords) / 2.0)
    }
}

impl Polygon {
    /// Create a new polygon from points
    pub fn new(points: Vec<Point2<f64>>) -> Self {
        Self { points }
    }

    /// Check if polygon is closed (first and last points are close)
    pub fn is_closed(&self, tolerance: f64) -> bool {
        if self.points.len() < 3 {
            return false;
        }
        let first = self.points.first().unwrap();
        let last = self.points.last().unwrap();
        first.distance_to(last) < tolerance
    }

    /// Calculate area using shoelace formula
    pub fn area(&self) -> f64 {
        if self.points.len() < 3 {
            return 0.0;
        }

        let mut area = 0.0;
        for i in 0..self.points.len() {
            let j = (i + 1) % self.points.len();
            area += self.points[i].x * self.points[j].y;
            area -= self.points[j].x * self.points[i].y;
        }

        area.abs() / 2.0
    }

    /// Check if polygon winds clockwise
    pub fn is_clockwise(&self) -> bool {
        if self.points.len() < 3 {
            return false;
        }

        let mut sum = 0.0;
        for i in 0..self.points.len() {
            let j = (i + 1) % self.points.len();
            sum += (self.points[j].x - self.points[i].x) * (self.points[j].y + self.points[i].y);
        }

        sum > 0.0
    }

    /// Get bounding box
    pub fn bounding_box(&self) -> (Point2<f64>, Point2<f64>) {
        if self.points.is_empty() {
            return (Point2::origin(), Point2::origin());
        }

        let mut min = self.points[0];
        let mut max = self.points[0];

        for p in &self.points {
            min.x = min.x.min(p.x);
            min.y = min.y.min(p.y);
            max.x = max.x.max(p.x);
            max.y = max.y.max(p.y);
        }

        (min, max)
    }
}

// Helper trait for Point2 distance
trait PointDistance {
    fn distance_to(&self, other: &Self) -> f64;
}

impl PointDistance for Point2<f64> {
    fn distance_to(&self, other: &Point2<f64>) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounding_box() {
        let vertices = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 1.0),
            Point3::new(2.0, 2.0, 2.0),
        ];

        let bbox = BoundingBox::from_vertices(&vertices);
        assert_eq!(bbox.min, Point3::new(0.0, 0.0, 0.0));
        assert_eq!(bbox.max, Point3::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn test_triangle_plane_intersection() {
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

        // Test intersection at z = 2.5
        let segment = mesh.intersect_triangle_with_plane(triangle, 2.5);
        assert!(segment.is_some());

        // Test no intersection above triangle
        let no_intersection = mesh.intersect_triangle_with_plane(triangle, 10.0);
        assert!(no_intersection.is_none());

        // Test no intersection below triangle
        let no_intersection = mesh.intersect_triangle_with_plane(triangle, -1.0);
        assert!(no_intersection.is_none());
    }

    #[test]
    fn test_polygon_area() {
        // Square with side length 2
        let square = Polygon::new(vec![
            Point2::new(0.0, 0.0),
            Point2::new(2.0, 0.0),
            Point2::new(2.0, 2.0),
            Point2::new(0.0, 2.0),
        ]);

        assert!((square.area() - 4.0).abs() < 0.001);
    }

    #[test]
    fn test_line_segment_connection() {
        let seg1 = LineSegment2D {
            start: Point2::new(0.0, 0.0),
            end: Point2::new(1.0, 0.0),
        };

        let seg2 = LineSegment2D {
            start: Point2::new(1.0, 0.0),
            end: Point2::new(1.0, 1.0),
        };

        let connection = seg1.connects_to(&seg2, 0.001);
        assert_eq!(connection, Some(ConnectionType::EndToStart));
    }
}
