mod arc;
mod circle;
mod line;
mod reflect;
pub mod tiling;

use nannou::prelude::*;

use crate::arc::Arc;
use crate::circle::Circle;
use crate::line::Line;

pub type Shape = Vec<Point2>;

fn euclidian_distance_from_center_to_vertex(p: u8, q: u8) -> f32 {
    let q: f32 = q.into();
    let p: f32 = p.into();
    (((PI / 2f32 - PI / q).tan() - (PI / p).tan())
        / ((PI / 2f32 - PI / q).tan() + (PI / p).tan()))
    .sqrt()
    /*(((PI / 2f32 - PI / q).tan() - (PI / p).tan()) / ((PI / 2f32 - PI / q).tan() + (PI / p).tan()))
    .sqrt()*/
}

pub fn geodesic_passing_by_two_points(
    u: Point2,
    v: Point2,
) -> Option<Box<dyn reflect::Reflect>> {
    let divisor = u.x * v.y - u.y * v.x;
    if divisor == 0f32 {
        // would tend to infinity -> points are perfectly opposed or
        // are equal
        return if let Some(line) = Line::new(u, v) {
            Some(Box::new(line))
        } else {
            None
        };
    }
    // circle equation -> x*x + ax + y*y + by + 1 = 0
    let factor_of_x = (u.y * (v.x.pow(2f32) + v.y.pow(2f32))
        - v.y * (u.x.pow(2f32) + u.y.pow(2f32))
        + u.y
        - v.y)
        / divisor;

    let factor_of_y = (v.x * (u.x.pow(2f32) + u.y.pow(2f32))
        - u.x * (v.x.pow(2f32) + v.y.pow(2f32))
        + v.x
        - u.x)
        / divisor;
    let center =
        Point2::new(-factor_of_x / 2f32, -factor_of_y / 2f32);
    let radius = center.distance(u)/*(-1f32 + (factor_of_x / 2f32).pow(2f32) + (factor_of_y / 2f32).pow(2f32)).sqrt()*/;
    assert_ne!(radius, f32::NAN);
    if radius > 10f32 {
        return if let Some(line) = Line::new(u, v) {
            Some(Box::new(line))
        } else {
            None
        };
    }
    if let Some(circle) = Circle::new(center, radius) {
        if let Some(arc) = Arc::new(u, v, circle) {
            return Some(Box::new(arc));
        }
    }
    None
}
