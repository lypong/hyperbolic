extern crate hyperbolic;
use hyperbolic::Shape;
use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    shapes : Vec<Shape>,
    counter : u16
}

fn model(_app: &App) -> Model {
    Model {shapes:hyperbolic::init_tile(4, 6),counter:0}
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.counter+=1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw().scale(200f32);

    // Clear the background to blue.
    draw.background().color(BLACK);
    let mut counter = 0;
    for shape in  model.shapes.iter(){
        counter+=1;
        if counter>=model.counter{
            break;
        }
        for i in 0..shape.len() {
            let a = shape[i];
            let b = shape[(i + 1) % shape.len()];
            hyperbolic::geodesic_passing_by_two_points(a, b).draw(&draw);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
