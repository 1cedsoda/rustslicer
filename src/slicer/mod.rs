use crate::geometry::{Mesh, LineSegment};
use crate::error::{SlicerError, Result};
use nalgebra::Point3;
use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Layer {
    pub z: f64,
    pub contours: Vec<Contour>,
}

#[derive(Debug, Clone)]
pub struct Contour {
    pub points: Vec<Point3<f64>>,
    pub is_outer: bool,
}

pub struct Slicer {
    mesh: Mesh,
    layer_height: f64,
}

impl Slicer {
    pub fn new(mesh: Mesh, layer_height: f64) -> Result<Self> {
        if layer_height <= 0.0 {
            return Err(SlicerError::InvalidParameter(
                "Layer height must be positive".to_string()
            ));
        }

        mesh.validate()?;

        Ok(Slicer {
            mesh,
            layer_height,
        })
    }

    pub fn slice(&self) -> Result<Vec<Layer>> {
        let min_z = self.mesh.bounds.min.z;
        let max_z = self.mesh.bounds.max.z;
        let num_layers = ((max_z - min_z) / self.layer_height).ceil() as usize;

        if num_layers == 0 {
            return Err(SlicerError::SlicingError("Model has no height".to_string()));
        }

        println!("Slicing {} layers...", num_layers);

        let layers: Vec<Layer> = (0..num_layers)
            .into_par_iter()
            .map(|i| {
                let z = min_z + (i as f64 + 0.5) * self.layer_height;
                self.slice_layer(z)
            })
            .collect();

        Ok(layers)
    }

    fn slice_layer(&self, z: f64) -> Layer {
        let mut segments: Vec<LineSegment> = self.mesh.triangles
            .iter()
            .filter_map(|triangle| triangle.intersect_plane(z))
            .collect();

        let contours = build_contours(&mut segments);

        Layer { z, contours }
    }
}

fn build_contours(segments: &mut Vec<LineSegment>) -> Vec<Contour> {
    let mut contours = Vec::new();
    let epsilon = 1e-6;

    while !segments.is_empty() {
        let mut current_contour = vec![segments[0].start, segments[0].end];
        segments.remove(0);

        // Try to build a closed contour
        let mut progress = true;
        while progress && !segments.is_empty() {
            progress = false;
            let last_point = *current_contour.last().unwrap();

            // Find a segment that connects to the current contour
            for i in 0..segments.len() {
                let seg = &segments[i];
                
                if distance_2d(&last_point, &seg.start) < epsilon {
                    current_contour.push(seg.end);
                    segments.remove(i);
                    progress = true;
                    break;
                } else if distance_2d(&last_point, &seg.end) < epsilon {
                    current_contour.push(seg.start);
                    segments.remove(i);
                    progress = true;
                    break;
                }
            }
        }

        // Check if contour is closed
        let first = current_contour.first().unwrap();
        let last = current_contour.last().unwrap();
        let is_closed = distance_2d(first, last) < epsilon;

        if is_closed && current_contour.len() > 2 {
            current_contour.pop(); // Remove duplicate last point
        }

        if current_contour.len() >= 3 {
            contours.push(Contour {
                points: current_contour,
                is_outer: true, // Simplified - proper implementation would determine this
            });
        }
    }

    contours
}

fn distance_2d(p1: &Point3<f64>, p2: &Point3<f64>) -> f64 {
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
}
