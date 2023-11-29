use nannou::prelude::*;

use crate::{
    euclidian_distance_from_center_to_vertex,
    geodesic_passing_by_two_points, reflect::Reflect, Shape,
};

pub struct Tiling {
    p: u8,
    q: u8,
    max_depth: u8,
    centers: Vec<Point2>,
    geodesics: Vec<Box<dyn Reflect>>,
    shapes: Vec<Shape>,
    computed: bool,
}

impl Tiling {
    pub fn new(p: u8, q: u8, max_depth: u8) -> Self {
        Tiling {
            p,
            q,
            max_depth,
            centers: vec![Point2::ZERO],
            geodesics: vec![],
            shapes: vec![],
            computed: false,
        }
    }
    pub fn centers(&self) -> Option<&Vec<Point2>> {
        // Si le tableau est possède sa valeur d'initialisation
        // ([Point2::Zero]), on ne retourne rien.
        match self.centers.as_slice() {
            &[v] if v == Point2::ZERO => None,
            _ => Some(&self.centers),
        }
    }
    pub fn geodesics(&self) -> Option<&Vec<Box<dyn Reflect>>> {
        // Si le tableau est vide, on ne retourne rien.
        match self.geodesics.as_slice() {
            &[] => None,
            _ => Some(&self.geodesics),
        }
    }
    pub fn shapes(&self) -> Option<&Vec<Shape>> {
        // Si le tableau est vide, on ne retourne rien.
        match self.shapes.as_slice() {
            &[] => None,
            _ => Some(&self.shapes),
        }
    }
    // Calcule les polygones,géodésiques et points du pavage. Cette
    // fonction permet de ne pas faire de calculs inutiles durant
    // l'initialisation du pavage.
    pub fn compute(&mut self) {
        // Si le pavage a déjà été calculé on ne le recalcule pas.
        if self.computed {
            return;
        }
        // On réinitialise nos variables.
        self.centers = vec![Point2::ZERO];
        self.geodesics = vec![];
        self.shapes = vec![];
        self.computed = true;
        if !self.is_tilable() {
            return;
        }
        // On crée notre polygone initial.
        let mut shape = vec![];
        let radius =
            euclidian_distance_from_center_to_vertex(self.p, self.q);
        // On calcule un nombre p de points, uniformément répartis sur
        // notre cercle de centre (0;0), grâce à de la trigonomètrie.
        let mut angle = 0f32;
        let p_as_f32: f32 = self.p.into();
        for _ in 0..self.p {
            shape.push(Point2::new(
                angle.cos() * radius,
                angle.sin() * radius,
            ));
            angle += 2f32 * PI / p_as_f32;
        }
        self.shapes.push(shape.clone());

        // On commence à paver
        self.tile(&shape, Point2::ZERO, 0);
        self.computed = true;
    }
    // Fonction récursive responsable de calculer les
    // "sous-polygones".
    fn tile(
        &mut self,
        current_shape: &Shape,
        current_center: Point2,
        depth: u8,
    ) {
        // Si la profondeur atteint la profondeur maximale, on arrête.
        if depth < self.max_depth {
            for i in 0..current_shape.len() {
                // On prend chaque point du polygone par paire a,b.
                let a = current_shape[i];
                let b = current_shape[(i + 1) % current_shape.len()];
                // On calcule la géodésique passant par les deux
                // points. Si une valeur est retournée, on reflète
                // notre forme actuelle dans la géodésique.
                if let Some(geodesic) =
                    geodesic_passing_by_two_points(a, b)
                {
                    // On reflète chaque point de notre forme actuelle
                    // dans la géodésique pour composer notre nouvelle
                    // forme.
                    let next_center =
                        geodesic.reflect(current_center);
                    self.centers.push(next_center);
                    let mut next_shape = vec![];
                    for j in 0..current_shape.len() {
                        let point = current_shape[j];
                        next_shape.push(geodesic.reflect(point));
                    }
                    // On appelle à nouveau avec notre polygone qui
                    // vient d'être calculé, la fonction tile en
                    // augmentant sa profondeur.
                    self.tile(&next_shape, next_center, depth + 1);
                    self.shapes.push(next_shape);
                    self.geodesics.push(geodesic);
                }
            }
        }
    }
    // Vérifie si l'on peut paver avec nos valeur de p et q
    pub fn is_tilable(&self) -> bool {
        ((self.p as i16) - 2) * ((self.q as i16) - 2) > 4
    }
    pub fn p(&self) -> u8 {
        self.p
    }
    pub fn q(&self) -> u8 {
        self.q
    }
    pub fn max_depth(&self) -> u8 {
        self.max_depth
    }
    pub fn set_p(&mut self, p: u8) {
        if self.p != p {
            self.p = p;
            self.computed = false;
        }
    }
    pub fn set_q(&mut self, q: u8) {
        if self.q != q {
            self.q = q;
            self.computed = false;
        }
    }
    pub fn set_max_depth(&mut self, max_depth: u8) {
        if self.max_depth != max_depth {
            self.max_depth = max_depth;
            self.computed = false;
        }
    }
}
