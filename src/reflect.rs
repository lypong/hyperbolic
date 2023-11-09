use nannou::prelude::*;

// Déclaration de notre trait Reflect. Il représente l'ensemble des
// fonctions nécessitant une implémentation de la part de chaque
// structure représentant une géodésique.
pub trait Reflect: std::fmt::Debug {
    fn reflect(&self, point: Point2) -> Point2;
    fn draw(&self, draw: &Draw);
}
