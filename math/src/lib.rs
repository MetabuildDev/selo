use geo::*;
use glam::*;
use line_intersection::line_intersection;

pub trait Mirror2D {
    fn mirror_x(&self) -> Self;
    fn mirror_y(&self) -> Self;
}

impl Mirror2D for Vec2 {
    fn mirror_x(&self) -> Self {
        Vec2::new(-self.x, self.y)
    }
    fn mirror_y(&self) -> Self {
        Vec2::new(self.x, -self.y)
    }
}

pub fn coord_to_vec2(coord: Coord<f32>) -> Vec2 {
    Vec2::new(coord.x, coord.y)
}

pub fn vec2_to_coord(vec2: Vec2) -> Coord<f32> {
    Coord {
        x: vec2.x,
        y: vec2.y,
    }
}

pub fn intersect_line_2d_point(
    (start1, end1): (Vec2, Vec2),
    (start2, end2): (Vec2, Vec2),
) -> Option<Vec2> {
    let line1 = Line::new(vec2_to_coord(start1), vec2_to_coord(end1));
    let line2 = Line::new(vec2_to_coord(start2), vec2_to_coord(end2));

    line_intersection(line1, line2).and_then(|coord| match coord {
        LineIntersection::SinglePoint {
            intersection,
            is_proper,
        } => is_proper.then_some(coord_to_vec2(intersection)),
        LineIntersection::Collinear { intersection: _ } => None,
    })
}
