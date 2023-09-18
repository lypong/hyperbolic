use nannou::prelude::*;

pub fn angle_between(a: Point2, joint: Point2, b: Point2) -> f32 {
    //law of cosines
    // (c^2-a^2-b^2)/-2ab = COS(C)
    (a - joint).angle_between(b - joint)
}