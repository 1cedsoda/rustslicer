use nalgebra::{Point3, Vector3};
use std::fs::File;
use std::io::BufReader;
use crate::error::{SlicerError, Result};

#[derive(Debug, Clone)]
pub struct Triangle {
    pub vertices: [Point3<f64>; 3],
    pub normal: Vector3<f64>,
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub triangles: Vec<Triangle>,
    pub bounds: BoundingBox,
}

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub min: Point3<f64>,
    pub max: Point3<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LineSegment {
    pub start: Point3<f64>,
    pub end: Point3<f64>,
}

impl Mesh {
    pub fn from_stl_file(path: &str) -> Result<Self> {
        let file = File::open(path)
            .map_err(|e| SlicerError::StlReadError(format!("Failed to open file: {}", e)))?;
        
        let mut reader = BufReader::new(file);
        let stl = stl_io::read_stl(&mut reader)
            .map_err(|e| SlicerError::StlReadError(format!("Failed to parse STL: {}", e)))?;

        let mut triangles = Vec::new();
        let mut min = Point3::new(f64::MAX, f64::MAX, f64::MAX);
        let mut max = Point3::new(f64::MIN, f64::MIN, f64::MIN);

        for face in stl.faces {
            let vertices = [
                Point3::new(
                    stl.vertices[face.vertices[0]].coords[0] as f64,
                    stl.vertices[face.vertices[0]].coords[1] as f64,
                    stl.vertices[face.vertices[0]].coords[2] as f64,
                ),
                Point3::new(
                    stl.vertices[face.vertices[1]].coords[0] as f64,
                    stl.vertices[face.vertices[1]].coords[1] as f64,
                    stl.vertices[face.vertices[1]].coords[2] as f64,
                ),
                Point3::new(
                    stl.vertices[face.vertices[2]].coords[0] as f64,
                    stl.vertices[face.vertices[2]].coords[1] as f64,
                    stl.vertices[face.vertices[2]].coords[2] as f64,
                ),
            ];

            let normal = Vector3::new(
                face.normal.coords[0] as f64,
                face.normal.coords[1] as f64,
                face.normal.coords[2] as f64,
            );

            // Update bounding box
            for vertex in &vertices {
                min.x = min.x.min(vertex.x);
                min.y = min.y.min(vertex.y);
                min.z = min.z.min(vertex.z);
                max.x = max.x.max(vertex.x);
                max.y = max.y.max(vertex.y);
                max.z = max.z.max(vertex.z);
            }

            triangles.push(Triangle { vertices, normal });
        }

        if triangles.is_empty() {
            return Err(SlicerError::InvalidGeometry("STL file contains no triangles".to_string()));
        }

        Ok(Mesh {
            triangles,
            bounds: BoundingBox { min, max },
        })
    }

    pub fn validate(&self) -> Result<()> {
        if self.triangles.is_empty() {
            return Err(SlicerError::InvalidGeometry("Mesh contains no triangles".to_string()));
        }

        for (i, triangle) in self.triangles.iter().enumerate() {
            // Check for degenerate triangles
            let v0 = triangle.vertices[0];
            let v1 = triangle.vertices[1];
            let v2 = triangle.vertices[2];

            if v0 == v1 || v1 == v2 || v2 == v0 {
                return Err(SlicerError::InvalidGeometry(
                    format!("Degenerate triangle found at index {}", i)
                ));
            }
        }

        Ok(())
    }
}

impl Triangle {
    /// Intersect triangle with a plane at height z
    pub fn intersect_plane(&self, z: f64) -> Option<LineSegment> {
        let mut intersections = Vec::new();

        // Check each edge of the triangle
        for i in 0..3 {
            let v1 = self.vertices[i];
            let v2 = self.vertices[(i + 1) % 3];

            if let Some(point) = intersect_edge_with_plane(v1, v2, z) {
                intersections.push(point);
            }
        }

        // We need exactly 2 intersection points to form a line segment
        if intersections.len() == 2 {
            Some(LineSegment {
                start: intersections[0],
                end: intersections[1],
            })
        } else {
            None
        }
    }
}

fn intersect_edge_with_plane(v1: Point3<f64>, v2: Point3<f64>, z: f64) -> Option<Point3<f64>> {
    let epsilon = 1e-10;

    // Check if edge crosses the plane
    if (v1.z - z).abs() < epsilon {
        return Some(v1);
    }
    if (v2.z - z).abs() < epsilon {
        return Some(v2);
    }

    if (v1.z < z && v2.z < z) || (v1.z > z && v2.z > z) {
        return None; // Edge doesn't cross plane
    }

    // Linear interpolation to find intersection point
    let t = (z - v1.z) / (v2.z - v1.z);
    Some(Point3::new(
        v1.x + t * (v2.x - v1.x),
        v1.y + t * (v2.y - v1.y),
        z,
    ))
}

impl BoundingBox {
    pub fn dimensions(&self) -> Vector3<f64> {
        Vector3::new(
            self.max.x - self.min.x,
            self.max.y - self.min.y,
            self.max.z - self.min.z,
        )
    }
}
