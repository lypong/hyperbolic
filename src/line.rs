use nannou::prelude::*;
use core::fmt::Debug;

use crate::reflect::*;

#[derive(Debug)]
pub struct Line {
    start: Point2,
    end: Point2,
    direction: Vec2
}

impl Line {
    pub fn new(start: Point2, end: Point2) -> Option<Self> {
        if start == end {
            return None;
        }
        let direction = end-start;
        Some(Line { start, end, direction})
    }
    pub fn orthogonal_line_passing_by_point(&self,point: Point2) -> Self{
        let orthogonal_direction = Vec2::new(-self.direction.y,self.direction.x);
        Line { start: point, end: point+orthogonal_direction, direction: orthogonal_direction }
    }
    pub fn intersect(&self,line : Self) -> Option<Point2>{
        let factor = (line.direction.x*(self.start.y-line.start.y)+line.direction.y*(line.start.x-self.start.x))/(line.direction.y*self.direction.x-line.direction.x*self.direction.y);
        if !factor.is_finite(){
            return None;
        }
        Some(Point2::new(self.start.x+factor*self.direction.x,self.start.y+factor*self.direction.y))
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
        draw.line().start(self.start).end(self.end).color(BLACK).weight(0.005);
    }
}