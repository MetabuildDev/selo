use glam::Vec2;

use crate::{Line, MultiPolygon, Polygon, Ring};

#[inline]
pub fn split_ring_polygon(polygon: &Polygon<Vec2>) -> MultiPolygon<Vec2> {
    MultiPolygon(
        find_double_line(polygon)
            .and_then(|line| find_points(polygon, line))
            .map(|(point_before, point_after)| split_poly(polygon, point_before, point_after))
            .map(|(poly_split_1, poly_split_2)| {
                [
                    split_ring_polygon(&poly_split_1).0,
                    split_ring_polygon(&poly_split_2).0,
                ]
                .concat()
            })
            .unwrap_or_else(|| vec![polygon.clone()]),
    )
}

#[inline]
fn split_poly(
    polygon: &Polygon<Vec2>,
    point_before: Vec2,
    point_after: Vec2,
) -> (Polygon<Vec2>, Polygon<Vec2>) {
    let p1_lines = polygon
        .exterior()
        .lines()
        .take_while(|line| line.src() != point_before)
        .chain(std::iter::once(Line([point_before, point_after])))
        .chain(
            polygon
                .exterior()
                .lines()
                .skip_while(|line| line.src() != point_after),
        )
        .map(|line| line.src())
        .collect::<Vec<_>>();

    let p2_lines = polygon
        .exterior()
        .lines()
        .skip_while(|line| line.src() != point_before)
        .take_while(|line| line.src() != point_after)
        .chain(std::iter::once(Line([point_after, point_before])))
        .map(|line| line.src())
        .collect::<Vec<_>>();

    let make_poly =
        |points: Vec<Vec2>| Polygon::<Vec2>::new(Ring::<Vec2>::new(points), Default::default());

    (make_poly(p1_lines), make_poly(p2_lines))
}

#[inline]
fn find_points(polygon: &Polygon<Vec2>, line: Line<Vec2>) -> Option<(Vec2, Vec2)> {
    polygon
        .exterior()
        .lines()
        .chain(polygon.exterior().lines().take(2))
        .collect::<Vec<_>>()
        .windows(3)
        .find(|win| win[1] == line)
        .map(|win| (win[0], win[2]))
        .map(|(l_before, l_after)| (l_before.src(), l_after.dst()))
}

#[inline]
fn find_double_line(polygon: &Polygon<Vec2>) -> Option<Line<Vec2>> {
    polygon
        .exterior()
        .lines()
        .enumerate()
        .find_map(|(idx, line)| {
            polygon
                .exterior()
                .lines()
                .skip(idx + 1)
                .find(|other_line| other_line.eq(&line) || other_line.eq(&line.swap_coords()))
        })
}

#[test]
fn polygon_split() {
    // _ = env_logger::try_init();
    let ring = Ring::new(
        [
            Vec2 {
                x: 11.44999885559082,
                y: 4.250000476837158,
            },
            Vec2 {
                x: 8.549999237060547,
                y: 3.90000057220459,
            },
            Vec2 {
                x: 7.875,
                y: -6.825000762939453,
            },
            Vec2 {
                x: 5.850000381469727,
                y: -8.850000381469727,
            },
            Vec2 {
                x: 5.849999904632568,
                y: 6.600000381469727,
            },
            Vec2 {
                x: 13.799999237060547,
                y: 6.600000381469727,
            },
            Vec2 {
                x: 13.800000190734863,
                y: -8.850000381469727,
            },
            Vec2 {
                x: 5.850000381469727,
                y: -8.850000381469727,
            },
            Vec2 {
                x: 7.875,
                y: -6.825000762939453,
            },
            Vec2 {
                x: 11.524999618530273,
                y: -6.575000286102295,
            },
        ]
        .to_vec(),
    );
    let poly = ring.to_polygon();
    let multipoly = split_ring_polygon(&poly);
    assert_eq!(multipoly.iter().count(), 2);
    assert!(multipoly.iter().any(|p| p.exterior().lines().count() == 4))
}
