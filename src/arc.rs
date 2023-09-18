use nannou::prelude::*;

use crate::angle::angle_between;
use crate::circle::Circle;
use crate::reflect::Reflect;

const RESOLUTION : f32 = 32f32;

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
    fn points_of_arc(&self,resolution : f32) -> Option<Vec<Point2>> {
        let mut res = vec![];
        let angle = angle_between(self.start, self.circle.center, self.end);
        if angle == 0f32 {
            res.push(self.start);
            res.push(self.end);
            return Some(res);
        }
        let step = angle / resolution * -1f32;
        let start = (self.start - self.circle.center).angle_between(Vec2::new(0f32, 1f32));
        let end = start - angle;
        let mut current = start;
    
        if angle < 0f32 {
            while current <= end {
                current += step;
                let p = Vec2::new(current.sin() * self.circle.radius, current.cos() * self.circle.radius) + self.circle.center;
                res.push(p)
            }
        } else {
            while current >= end {
                current += step;
                let p = Vec2::new(current.sin() * self.circle.radius, current.cos() * self.circle.radius) + self.circle.center;
                res.push(p)
            }
        }
        Some(res)
    }
}

impl Reflect for Arc {
    fn reflect(&self, point: Point2) -> Point2 {
        self.circle.reflect(point)
    }
    fn draw(&self, draw: &Draw) {
        if let Some(points) = self.points_of_arc(RESOLUTION){
            draw.polyline().weight(0.005).color(WHITE).points(points);
        }
    }
}