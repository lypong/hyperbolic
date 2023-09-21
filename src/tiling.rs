use nannou::prelude::*;

use crate::{
    euclidian_distance_from_center_to_vertex, geodesic_passing_by_two_points, reflect::Reflect,
    Shape,
};

pub struct Tiling {
    p: u8,
    q: u8,
    max_depth: u8,
    centers: Vec<Point2>,
    geodesics: Vec<Box<dyn Reflect>>,
    shapes: Vec<Shape>,
    computed: bool,
}

impl Tiling {
    pub fn new(p: u8, q: u8, max_depth: u8) -> Self {
        Tiling {
            p,
            q,
            max_depth,
            centers: vec![Point2::ZERO],
            geodesics: vec![],
            shapes: vec![],
            computed: false,
        }
    }
    pub fn centers(&self) -> Option<&Vec<Point2>> {
        match self.centers.as_slice() {
            &[v] if v == Point2::ZERO => None,
            _ => Some(&self.centers),
        }
    }
    pub fn geodesics(&self) -> Option<&Vec<Box<dyn Reflect>>> {
        match self.geodesics.as_slice() {
            &[] => None,
            _ => Some(&self.geodesics),
        }
    }
    pub fn shapes(&self) -> Option<&Vec<Shape>> {
        match self.shapes.as_slice() {
            &[] => None,
            _ => Some(&self.shapes),
        }
    }
    pub fn compute(&mut self) {
        if self.computed {
            return;
        }
        let radius = euclidian_distance_from_center_to_vertex(self.p, self.q);
        let mut shape = vec![];
        let mut angle = 0f32;
        let p_as_f32: f32 = self.p.into();
        for _ in 0..self.p {
            shape.push(Point2::new(angle.cos() * radius, angle.sin() * radius));
            angle += 2f32 * PI / p_as_f32;
        }
        self.shapes.push(shape.clone());

        self.tile(&shape, Point2::ZERO, 0);
        self.computed = true;
    }
    fn tile(&mut self, current_shape: &Shape, current_center: Point2, depth: u8) {
        if depth < self.max_depth {
            for i in 0..current_shape.len() {
                let a = current_shape[i];
                let b = current_shape[(i + 1) % current_shape.len()];
                if let Some(geodesic) = geodesic_passing_by_two_points(a, b) {
                    let next_center = geodesic.reflect(current_center);
                    self.centers.push(next_center);
                    let mut next_shape = vec![];
                    for j in 0..current_shape.len() {
                        let point = current_shape[j];
                        next_shape.push(geodesic.reflect(point));
                    }
                    self.tile(&next_shape, next_center, depth + 1);
                    self.shapes.push(next_shape);
                    self.geodesics.push(geodesic);
                }
            }
        }
    }
}
