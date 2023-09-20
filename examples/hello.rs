extern crate hyperbolic;
use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    // Begin drawing
    let draw = app.draw().scale(200f32);

    draw.background().color(WHITE);

    for shape in hyperbolic::init_tile(4, 6) {
        for i in 0..shape.len() {
            let a = shape[i];
            let b = shape[(i + 1) % shape.len()];
            //hyperbolic::
            if let Some(geodesic) = hyperbolic::geodesic_passing_by_two_points(a, b){
                geodesic.draw(&draw);
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
