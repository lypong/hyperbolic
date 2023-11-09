use core::fmt::Debug;
use nannou::prelude::*;

use crate::reflect::*;

#[derive(Debug)]
pub struct Line {
    start: Point2,
    end: Point2,
    direction: Vec2,
}

impl Line {
    pub fn new(start: Point2, end: Point2) -> Option<Self> {
        // Si les deux points sur la droite sont égaux, on ne peut pas
        // calculer le vecteur directeur de la droite. Alors, on ne
        // retourne pas de droite.
        if start == end {
            return None;
        }
        //On calcule le vecteur directeur
        let direction = end - start;
        Some(Line {
            start,
            end,
            direction,
        })
    }
    // Retourne la droite orthogonale passant par un point donné
    pub fn orthogonal_line_passing_by_point(
        &self,
        point: Point2,
    ) -> Self {
        // On calcule le vecteur orthogonal à notre droite pour
        // déterminer un 2ème point de celle-ci, en additionant notre
        // point donné avec notre vecteur.
        let orthogonal_direction =
            Vec2::new(-self.direction.y, self.direction.x);
        Line {
            start: point,
            end: point + orthogonal_direction,
            direction: orthogonal_direction,
        }
    }
    // Retourne le point d'intersection entre deux droites
    pub fn intersect(&self, line: Self) -> Option<Point2> {
        // Issu de l'équation cartésienne d'une droite:
        // x = x0
        //          + k * v, on cherche le facteur k avec x,y étant
        //            les coordonées du point de départ de notre
        //            droite et v son vecteur directeur.
        // y = y0
        let factor = (line.direction.x
            * (self.start.y - line.start.y)
            + line.direction.y * (line.start.x - self.start.x))
            / (line.direction.y * self.direction.x
                - line.direction.x * self.direction.y);
        // Si le facteur k tend vers l'infini, alors les droites sont
        // parallèles, donc il n'y pas de point d'intersection.
        if !factor.is_finite() {
            return None;
        }
        Some(Point2::new(
            self.start.x + factor * self.direction.x,
            self.start.y + factor * self.direction.y,
        ))
    }
    // Retourne la projection orthogonale d'un point sur notre droite.
    pub fn projection(&self, point: Point2) -> Point2 {
        let orthogonal = self.orthogonal_line_passing_by_point(point);
        self.intersect(orthogonal).unwrap()
    }
}

impl Reflect for Line {
    // Retourne la symmétrie d'un point par rapport à notre droite.
    fn reflect(&self, point: Point2) -> Point2 {
        // On calcule la projection orthogonale de notre droite. Le
        // point retourné est à mi-distance entre le point donné et
        // celui qu'on recherche.
        let projection = self.projection(point);
        let direction = projection - point;
        Point2::new(
            point.x + direction.x * 2f32,
            point.y + direction.y * 2f32,
        )
    }
    fn draw(&self, draw: &Draw) {
        draw.line()
            .start(self.start)
            .end(self.end)
            .color(BLACK)
            .weight(0.005);
    }
}
