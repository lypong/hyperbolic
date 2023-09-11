use nannou::prelude::*;
use core::fmt::Debug;

pub type Shape = Vec<Point2>;
const MAX_ITERATION: u16 = 3;

pub trait Reflect : std::fmt::Debug {
    fn reflect(&self, point: Point2) -> Point2;
    fn draw(&self, draw: &Draw);
}

#[derive(Debug)]
struct Circle {
    center: Point2,
    radius: f32,
}

impl Circle {
    pub fn new(center: Point2, radius: f32) -> Self {
        assert!(radius >= 0f32);
        Circle { center, radius }
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
            .stroke(WHITE)
            .stroke_weight(0.005)
            .xy(self.center)
            .radius(self.radius);
    }
}

#[derive(Debug)]
struct Line {
    start: Point2,
    end: Point2,
    direction: Vec2
}

impl Line {
    pub fn new(start: Point2, end: Point2) -> Self {
        assert_ne!(start, end);
        let direction = end-start;
        Line { start, end, direction}
    }
    pub fn orthogonal_line_passing_by_point(&self,point: Point2) -> Self{
        let orthogonal_direction = Vec2::new(-self.direction.y,self.direction.x);
        Line { start: point, end: point+orthogonal_direction, direction: orthogonal_direction }
    }
    pub fn intersect(&self,line : Self) -> Result<Point2,()>{
        let factor = (line.direction.x*(self.start.y-line.start.y)+line.direction.y*(line.start.x-self.start.x))/(line.direction.y*self.direction.x-line.direction.x*self.direction.y);
        if !factor.is_finite(){
            return Err(());
        }
        Ok(Point2::new(self.start.x+factor*self.direction.x,self.start.y+factor*self.direction.y))
    }
    pub fn projection(&self,point: Point2) -> Point2{
        let orthogonal = self.orthogonal_line_passing_by_point(point);
        self.intersect(orthogonal).unwrap()
    }
}

impl Reflect for Line {
    fn reflect(&self, point: Point2) -> Point2 {
        let projection = self.projection(point);
        let direction = projection-point;
        Point2::new(point.x+direction.x*2f32, point.y+direction.y*2f32)
    }
    fn draw(&self, draw: &Draw) {
        draw.line().start(self.start).end(self.end);
    }
}

fn euclidian_distance_from_center_to_vertex(p: u16, q: u16) -> f32 {
    let q: f32 = q.into();
    let p: f32 = p.into();
    (((PI / 2f32 - PI / q).tan() - (PI / p).tan()) / ((PI / 2f32 - PI / q).tan() + (PI / p).tan()))
        .sqrt()
    /*(((PI / 2f32 - PI / q).tan() - (PI / p).tan()) / ((PI / 2f32 - PI / q).tan() + (PI / p).tan()))
    .sqrt()*/
}

pub fn geodesic_passing_by_two_points(u: Point2, v: Point2) -> Box<dyn Reflect> {
    let divisor = u.x * v.y - u.y * v.x;
    if divisor == 0f32 {
        // would tend to infinity -> points are perfectly opposed or are equal
        return Box::new(Line::new(u, v)); // will assert if u==v
    }
    // circle equation -> x*x + ax + y*y + by + 1 = 0
    let factor_of_x =
        (u.y * (v.x.pow(2f32) + v.y.pow(2f32)) - v.y * (u.x.pow(2f32) + u.y.pow(2f32)) + u.y - v.y)
            / divisor;

    let factor_of_y =
        (v.x * (u.x.pow(2f32) + u.y.pow(2f32)) - u.x * (v.x.pow(2f32) + v.y.pow(2f32)) + v.x - u.x)
            / divisor;
    let center = Point2::new(-factor_of_x / 2f32, -factor_of_y / 2f32);
    let radius = center.distance(u)/*(-1f32 + (factor_of_x / 2f32).pow(2f32) + (factor_of_y / 2f32).pow(2f32)).sqrt()*/;
    assert_ne!(radius, f32::NAN);
    Box::new(Circle::new(center, radius))
}

fn tile(
    current_shape: &Shape,
    current_center: Point2,
    shapes: &mut Vec<Shape>,
    centers: &mut Vec<Point2>,
    iteration: u16,
) {
    if iteration < MAX_ITERATION {
        for i in 0..current_shape.len() {
            let a = current_shape[i];
            let b = current_shape[(i + 1) % current_shape.len()];
            let geodesic = geodesic_passing_by_two_points(a, b);
            let next_center = geodesic.reflect(current_center);
            if !centers.contains(&next_center) {
                centers.push(next_center);
                let mut next_shape = vec![];
                for j in 0..current_shape.len() {
                    let point = current_shape[j];
                    next_shape.push(geodesic.reflect(point));
                }
                tile(&next_shape, next_center, shapes, centers, iteration + 1);
                shapes.push(next_shape);
            }
        }
    }
}

pub fn init_tile(p: u16, q: u16) -> Vec<Shape> {
    let radius = euclidian_distance_from_center_to_vertex(p, q);
    let mut shape = vec![];
    let mut angle = 0f32;
    let p_as_f32: f32 = p.into();
    for _ in 0..p {
        shape.push(Point2::new(angle.cos() * radius, angle.sin() * radius));
        angle += 2f32 * PI / p_as_f32;
    }
    let mut centers = vec![Point2::ZERO];
    let mut shapes = vec![shape.clone()];
    tile(&shape, Point2::ZERO, &mut shapes, &mut centers, 0);
    shapes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reflect_point_on_geodesic_with_geodesic() {
        let a = Point2::new(0.517638147,0f32);
        let b = Point2::new(-2.26266827E-8,0.517638147);
        let geodesic = geodesic_passing_by_two_points(a, b);
        assert_eq!(a,geodesic.reflect(a))
    }

    #[test]
    fn reflect_point_on_circle() {
        let point_on_circle = Point2::new(0f32, 1f32);
        let result = Circle::new(Point2::new(0f32, 0f32), 1f32).reflect(point_on_circle);
        assert_eq!(result, point_on_circle);
    }
    #[test]
    fn reflect_point_outside_of_circle() {
        let point_outside_of_circle = Point2::new(2f32, 0f32);
        let radius = 1f32;
        let result = Circle::new(Point2::new(0f32, 0f32), radius).reflect(point_outside_of_circle);
        assert_eq!(result, Point2::new(0.5, 0f32));
    }
    #[test]
    fn reflect_point_inside_of_circle() {
        let point_inside_of_circle = Point2::new(-0.5, 0f32);
        let radius = 1f32;
        let result = Circle::new(Point2::new(0f32, 0f32), radius).reflect(point_inside_of_circle);
        assert_eq!(result, Point2::new(-2f32, 0f32));
    }
}
