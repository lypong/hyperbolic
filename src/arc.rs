use nannou::prelude::*;
use crate::circle::Circle;
use crate::reflect::Reflect;

fn control_point(point: Point2, reference_point: Point2, center: Point2) -> Point2 {
    let angle = (point - center)
        .angle_between(reference_point - center)
        .abs();
    let l = 4f32 * (angle / 4f32).tan() / 3f32;
    let director_tan = Vec2::new(point.y - center.y, center.x - point.x);
    let potential_control_point1 =
        Vec2::new(point.x + director_tan.x * l, point.y + director_tan.y * l);
    let potential_control_point2 =
        Vec2::new(point.x - director_tan.x * l, point.y - director_tan.y * l);

    if potential_control_point1.distance_squared(reference_point)
        < potential_control_point2.distance_squared(reference_point)
    {
        potential_control_point1
    } else {
        potential_control_point2
    }
}

#[derive(Debug)]
pub struct Arc{
    start: Point2,
    end: Point2,
    circle: Circle
}

impl Arc {
    pub fn new(start: Point2, end: Point2, circle: Circle) -> Option<Self> {
        if start == end{
            return None;
        }
        Some(Arc { start, end,circle })
    }
}

impl Reflect for Arc {
    fn reflect(&self, point: Point2) -> Point2 {
        self.circle.reflect(point)
    }
    fn draw(&self, draw: &Draw) {
        let mut builder = nannou::geom::path::Builder::new().with_svg();
        builder.move_to(self.start.to_array().into());
        builder.cubic_bezier_to(
            control_point(self.start, self.end, self.circle.center()).to_array().into(),
            control_point(self.end, self.start, self.circle.center()).to_array().into(),
            self.end.to_array().into(),
        );
        let path = builder.build();
        draw.path()
            .stroke()
            .tolerance(0.001)
            .weight(0.005)
            .color(BLACK)
            .events(path.iter());
    
    }
}