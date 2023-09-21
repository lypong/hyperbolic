use nannou::prelude::*;
use core::fmt::Debug;

use crate::reflect::*;

#[derive(Debug)]
pub struct Circle {
    center: Point2,
    radius: f32,
}

impl Circle {
    pub fn new(center: Point2, radius: f32) -> Option<Self> {
        if radius < 0f32 {
            return None;
        }
        Some(Circle { center, radius })
    }
    pub fn center(&self) -> Point2 {
        self.center
    }
    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl Reflect for Circle {
    fn reflect(&self, point: Point2) -> Point2 {
        // La distance entre le centre et un point multipliée par la distance entre le centre et l'inverse du point est égale au carré du rayon. (|OI|*|OA|=r^2)
        // Le centre, le point, et son inverse son colinéaires
        let distance_of_inverted_point = self.radius.pow(2) / point.distance(self.center);
        if distance_of_inverted_point == self.radius{
            return point;
        }
        // On crée un vecteur colinéaire avec le centre, le point et son inverse, puis on le normalise et finalement retourne la distance entre le centre et l'inverse multipliée par le vecteur normal, donc l'inverse.
        let normalized_vec =
            Point2::new(point.x - self.center.x, point.y - self.center.y).normalize_or_zero();
        self.center + normalized_vec * distance_of_inverted_point
    }
    fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .resolution(64f32)
            .no_fill()
            .stroke(BLACK)
            .stroke_weight(0.005)
            .xy(self.center)
            .radius(self.radius);
    }
}