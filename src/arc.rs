use crate::circle::Circle;
use crate::reflect::Reflect;
use nannou::prelude::*;

// Retourne un point de contrôle à partir des deux points aux
// extremités de l'arc et du centre du cercle qui compose l'arc dont
// on veut obtenir le point de contrôle
fn control_point(
    point: Point2,
    reference_point: Point2,
    center: Point2,
) -> Point2 {
    // On calcule la mesure de l'angle AOB avec, A et B étant les
    // extremités et O le centre du cercle dont est issu notre arc.
    let angle = (point - center)
        .angle_between(reference_point - center)
        .abs();
    let l = 4f32 * (angle / 4f32).tan() / 3f32;
    // On calcule le vecteur directeur de notre tangente.
    let director_tan =
        Vec2::new(point.y - center.y, center.x - point.x);
    // On calcule les deux points sur la tangente au cercle passant
    // par notre extremité, situés à une distance l de celle-ci.
    let potential_control_point1 = Vec2::new(
        point.x + director_tan.x * l,
        point.y + director_tan.y * l,
    );
    let potential_control_point2 = Vec2::new(
        point.x - director_tan.x * l,
        point.y - director_tan.y * l,
    );
    // On vérifie lequel des deux points est le point de contrôle. Le
    // point de contrôle sera le point le plus proche de notre point
    // de référence.
    if potential_control_point1.distance_squared(reference_point)
        < potential_control_point2.distance_squared(reference_point)
    {
        potential_control_point1
    } else {
        potential_control_point2
    }
}

#[derive(Debug)]
pub struct Arc {
    start: Point2,
    end: Point2,
    circle: Circle,
}

impl Arc {
    pub fn new(
        start: Point2,
        end: Point2,
        circle: Circle,
    ) -> Option<Self> {
        //On ne retourne rien si les deux points aux extremités sont
        // égaux, car l'arc n'existerait simplement pas.
        if start == end {
            return None;
        }
        Some(Arc { start, end, circle })
    }
}

impl Reflect for Arc {
    fn reflect(&self, point: Point2) -> Point2 {
        // Retourne simplement la réflexion du point du cercle dont
        // l'arc est issu.
        self.circle.reflect(point)
    }
    fn draw(&self, draw: &Draw) {
        // On initialise un constructeur pour notre courbe de Bézier,
        // on calcule les points de contrôle de cette dernière, puis
        // on la construit.
        let mut builder =
            nannou::geom::path::Builder::new().with_svg();
        builder.move_to(self.start.to_array().into());
        builder.cubic_bezier_to(
            control_point(self.start, self.end, self.circle.center())
                .to_array()
                .into(),
            control_point(self.end, self.start, self.circle.center())
                .to_array()
                .into(),
            self.end.to_array().into(),
        );
        let path = builder.build();
        // On dessine notre courbe
        draw.path()
            .stroke()
            .tolerance(0.001)
            .weight(0.005)
            .color(BLACK)
            .events(path.iter());
    }
}
