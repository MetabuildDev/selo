use glam::*;
use selo::{Area as _, Ring};

#[test]
fn ring_area() {
    let polygon = Ring::new(vec![
        Vec2::new(1.0, 1.0),
        Vec2::new(-2.0, 4.0),
        Vec2::new(-2.0, -2.0),
    ]);
    assert_eq!(polygon.area(), 9.0)
}
