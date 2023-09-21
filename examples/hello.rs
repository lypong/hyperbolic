extern crate hyperbolic;
use hyperbolic::tiling::Tiling;
use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    // Begin drawing
    let draw = app.draw().scale(200f32);

    draw.background().color(WHITE);

    let mut tiling = Tiling::new(4, 6, 3);
    tiling.compute();

    for geodesic in tiling.geodesics().unwrap() {
        geodesic.draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}
