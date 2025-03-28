use selo::prelude::*;

fn main() {
    // let polys = vec![
    //     Polygon(
    //         Ring::new([
    //             Vec2::new(-26.77308, 4.1095695),
    //             Vec2::new(-28.47277, 4.8949795),
    //             Vec2::new(-26.87879, 8.344502),
    //         ]),
    //         MultiRing(vec![]),
    //     ),
    //     Polygon(
    //         Ring::new([
    //             Vec2::new(-26.340359, 9.748107),
    //             Vec2::new(-21.801508, 7.650748),
    //             Vec2::new(-26.87879, 8.344502),
    //         ]),
    //         MultiRing(vec![]),
    //     ),
    //     Polygon(
    //         Ring::new([
    //             Vec2::new(-26.87879, 8.344502),
    //             Vec2::new(-22.430717, 6.28909),
    //             Vec2::new(-26.77308, 4.1095695),
    //         ]),
    //         MultiRing(vec![]),
    //     ),
    //     Polygon(
    //         Ring::new([
    //             Vec2::new(-22.430717, 6.28909),
    //             Vec2::new(-24.024706, 2.839571),
    //             Vec2::new(-26.77308, 4.1095695),
    //         ]),
    //         MultiRing(vec![]),
    //     ),
    //     Polygon(
    //         Ring::new([
    //             Vec2::new(-21.801508, 7.650748),
    //             Vec2::new(-26.340359, 9.748107),
    //             Vec2::new(-20.585045, 10.283271),
    //         ]),
    //         MultiRing(vec![]),
    //     ),
    //     Polygon(
    //         Ring::new([
    //             Vec2::new(-22.430717, 6.28909),
    //             Vec2::new(-26.87879, 8.344502),
    //             Vec2::new(-21.801508, 7.650748),
    //         ]),
    //         MultiRing(vec![]),
    //     ),
    //     Polygon(
    //         Ring::new([
    //             Vec2::new(-25.123896, 12.3806305),
    //             Vec2::new(-20.585045, 10.283271),
    //             Vec2::new(-26.340359, 9.748107),
    //         ]),
    //         MultiRing(vec![]),
    //     ),
    // ];
    // let b = polys
    //     .into_iter()
    //     .fold(selo::MultiPolygon(vec![]), |acc, x| {
    //         acc.union_approx(&x, 1e-3)
    //     });
    // println!("{b:?}");

    let poly = MultiPolygon::<DVec2>(vec![
        Polygon(
            Ring::new([
                DVec2::new(5.52192497253418, 9.565995216369629),
                DVec2::new(5.977786540985107, 13.651596069335938),
                DVec2::new(3.978739023208618, 10.279086112976074),
            ]),
            MultiRing(vec![]),
        ),
        Polygon(Ring::new([]), MultiRing(vec![])),
        Polygon(
            Ring::new([
                DVec2::new(5.521925449371338, 9.565994262695313),
                DVec2::new(16.757265090942383, 19.576366424560547),
                DVec2::new(9.516132354736328, 7.720310211181641),
                DVec2::new(10.968563079833984, 7.0491557121276855),
                DVec2::new(16.757265090942383, 19.576366424560547),
                DVec2::new(12.401240348815918, 24.453392028808594),
            ]),
            MultiRing(vec![]),
        ),
        Polygon(
            Ring::new([
                DVec2::new(5.52192497253418, 9.565995216369629),
                DVec2::new(5.5219244956970215, 9.565994262695313),
                DVec2::new(5.52192497253418, 9.565994262695313),
            ]),
            MultiRing(vec![]),
        ),
    ]);
    println!("{poly:?}");
    println!("{:?}", poly.buffer(1e-3));
}
