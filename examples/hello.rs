extern crate hyperbolic;
use hyperbolic::tiling::Tiling;
use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    // On initialise un dessin.
    let draw = app.draw().scale(200f32);

    draw.background().color(WHITE);

    // On définit et calcule le pavage.
    let mut tiling = Tiling::new(4, 6, 8);
    tiling.compute();

    // On dessine chaque géodésique constituant le pavage.
    for geodesic in tiling.geodesics().unwrap() {
        geodesic.draw(&draw);
    }

    // On affiche notre dessin à l'écran.
    draw.to_frame(app, &frame).unwrap();
}
