extern crate hyperbolic;
use hyperbolic::tiling::Tiling;
use nannou::prelude::*;
use nannou_egui::*;

const MAX_P : u8 = 8;
const MAX_Q : u8 = 8;
const P3_MAX_DEPTH : u8 = 8;
const P4_5_6_MAX_DEPTH : u8 = 5;
const P7_8_MAX_DEPTH : u8 = 4;
const DEFAULT_MAX_DEPTH : u8 = 3;
const POINCARE_RADIUS : u8 = 1;

fn main() {
    nannou::app(model).update(update).run()
}

struct Model {
    tiling: Tiling,
    settings: Settings,
    egui: Egui
}

struct Settings {
    p: u8,
    q: u8,
    max_depth: u8,
}

// Crée le "modèle" qui permet de transmettre nos variables dans les différentes phases de Nannou.
fn model(app: &App) -> Model {
    // On créé une fenêtre à laquelle on attache une instance d'objet représentant une interface graphique interactive.
    app.new_window().raw_event(raw_event).view(view).build().unwrap();
    let settings = Settings{p:4,q:6,max_depth:5};
    let egui = Egui::from_window(&app.main_window());
    let mut tiling = Tiling::new(settings.p, settings.q, settings.max_depth);
    tiling.compute();
    Model {tiling, settings, egui}
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let ctx = model.egui.begin_frame();
    let mut changed = false;
    // On crée nos curseurs qui permettent à l'utilisateur de fournir un input.
    egui::Window::new("Paramètres").show(&ctx, |ui| {
        changed |= ui.add(egui::Slider::new(&mut model.settings.p, 3..=MAX_P).text("p")).changed();
        changed |= ui.add(egui::Slider::new(&mut model.settings.q, 3..=MAX_Q).text("q")).changed();
        // Permets une valeur de profondeur de récursion raisonnable et adaptée à p, afin d'éviter les ralentissements.
        let max_depth_permitted = match model.settings.p {
            3 => P3_MAX_DEPTH,
            4|5|6 => P4_5_6_MAX_DEPTH,
            7|8 => P7_8_MAX_DEPTH,
            _ => DEFAULT_MAX_DEPTH,
        };
        model.settings.max_depth = model.settings.max_depth.clamp(1, max_depth_permitted);
        changed |= ui.add(egui::Slider::new(&mut model.settings.max_depth, 1..=max_depth_permitted).text("Profondeur maximale")).changed();
    });
    model.tiling.set_max_depth(model.settings.max_depth);
    // Si on a bougé un curseur on redéfinit nos variables
    if changed {
        model.tiling.set_p(model.settings.p);
        model.tiling.set_q(model.settings.q);
    }
    model.tiling.compute();
}

fn raw_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // On permet à nos curseurs de recevoir les clics de souris.
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    // On initialise un dessin.
    let draw = app.draw();
    // On dessine un fond blanc
    draw.background().color(WHITE);
    if !model.tiling.is_tilable(){
        draw.text("La combinaison de p et q n'est pas pavable.\n Veillez à augmenter leur valeur").color(BLACK);
        draw.to_frame(app, &frame).unwrap();
        model.egui.draw_to_frame(&frame).unwrap();
        return;
    }
    // On zoom sur notre dessin.
    let draw = draw.scale(200f32);
    
    // On dessine le disque de Poincaré
    draw.ellipse()
            .resolution(64f32)
            .no_fill()
            .stroke(BLACK)
            .stroke_weight(0.01)
            .radius(POINCARE_RADIUS.into());

    // On dessine chaque géodésique constituant le pavage.
    for geodesic in model.tiling.geodesics().unwrap() {
        geodesic.draw(&draw);
    }

    // On affiche notre dessin à l'écran.
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
