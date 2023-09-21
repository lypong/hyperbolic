extern crate hyperbolic;
use hyperbolic::tiling::Tiling;
use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    tiling : Tiling,
    counter : u16
}

fn model(_app: &App) -> Model {
    let mut tiling = Tiling::new(4, 6, 4);
    tiling.compute();
    Model {tiling,counter:0}
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.counter+=1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw().scale(200f32);

    // Clear the background to blue.
    draw.background().color(WHITE);
    let mut counter = 0;
    for shape in  model.tiling.shapes().unwrap(){
        counter+=1;
        if counter>=model.counter{
            break;
        }
        for i in 0..shape.len() {
            let a = shape[i];
            let b = shape[(i + 1) % shape.len()];
            if let Some(geodesic) = hyperbolic::geodesic_passing_by_two_points(a, b){
                geodesic.draw(&draw);
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
