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

// Retourne le rayon du cercle sur lequel on va disposer notre
// polygone initial. Le pavage est induit par cette valeur.
fn euclidian_distance_from_center_to_vertex(p: u8, q: u8) -> f32 {
    // La formule provient de :
    //http://www.malinc.se/noneuclidean/en/poincaretiling.php
    let q: f32 = q.into();
    let p: f32 = p.into();
    (((PI / 2f32 - PI / q).tan() - (PI / p).tan())
        / ((PI / 2f32 - PI / q).tan() + (PI / p).tan()))
    .sqrt()
}

pub fn geodesic_passing_by_two_points(
    u: Point2,
    v: Point2,
) -> Option<Box<dyn reflect::Reflect>> {
    let divisor = u.x * v.y - u.y * v.x;
    // Si le diviseur vaut zéro l'équation du cercle tend vers
    // l'infini, donc on construit une droite si les points ne sont
    // pas égaux.
    if divisor == 0f32 {
        return if let Some(line) = Line::new(u, v) {
            Some(Box::new(line))
        } else {
            None
        };
    }
    // Issu de l'équation du cercle : x*x + ax + y*y + by + 1 = 0
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
    let radius = center.distance(u);
    assert_ne!(radius, f32::NAN);
    // Si le cercle est énorme, on peut faire une approximation et
    // partir du principe que c'est une ligne pour des inversions plus
    // précises.
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
