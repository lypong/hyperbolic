extern crate hyperbolic;
use hyperbolic::tiling::Tiling;
use nannou::prelude::*;

macro_rules! slow_down {
    ($x:expr) => {
        {
            if $x.elapsed_frames() % 10 != 0 {
                return;
            }
        }
    };
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    tiling: Tiling,
    counter: u16,
}

fn model(_app: &App) -> Model {
    let mut tiling = Tiling::new(7, 3, 7);
    tiling.compute();
    Model { tiling, counter: 0 }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    slow_down!(app);
    model.counter += 1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw().scale(200f32);

    // Clear the background to blue.
    draw.background().color(WHITE);
    for (i,geodesic) in model.tiling.geodesics().unwrap().iter().enumerate() {
        if i >= model.counter.into() {
            break;
        }
        geodesic.draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}
