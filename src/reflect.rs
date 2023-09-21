use nannou::prelude::*;

pub trait Reflect: std::fmt::Debug {
    fn reflect(&self, point: Point2) -> Point2;
    fn draw(&self, draw: &Draw);
}
